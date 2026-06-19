use rusqlite::{Connection, Result as SqlResult};
use std::path::Path;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    /// Open or create the database at the given path.
    pub fn open(db_path: &Path) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;

        // Enable WAL mode for better concurrent access
        conn.execute_batch("PRAGMA journal_mode=WAL;")?;
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Initialize schema from migration SQL.
    pub fn initialize_schema(&self) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(include_str!("../migrations/001_init.sql"))
    }

    /// Check if a file is already indexed (by path + mtime).
    pub fn is_file_indexed(&self, path: &str, mtime: &str) -> SqlResult<bool> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT COUNT(1) FROM files WHERE path = ?1 AND mtime = ?2",
        )?;
        let count: i64 = stmt.query_row(rusqlite::params![path, mtime], |row| row.get(0))?;
        Ok(count > 0)
    }

    /// Insert a file record and return its ID.
    pub fn insert_file(
        &self,
        path: &str,
        filename: &str,
        extension: &str,
        size_bytes: i64,
        mtime: &str,
    ) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO files (path, filename, extension, size_bytes, mtime) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![path, filename, extension, size_bytes, mtime],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Insert document metadata.
    pub fn insert_metadata(
        &self,
        file_id: i64,
        religion: Option<&str>,
        book: Option<&str>,
        chapter: Option<&str>,
        verse: Option<&str>,
        title: Option<&str>,
        author: Option<&str>,
        language: Option<&str>,
        tags: Option<&str>,
    ) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO document_metadata (file_id, religion, book, chapter, verse, title, author, language, tags) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![file_id, religion, book, chapter, verse, title, author, language, tags],
        )?;
        Ok(())
    }

    /// Insert into FTS5 content table and the virtual table.
    pub fn insert_fts(
        &self,
        path: &str,
        title: Option<&str>,
        author: Option<&str>,
        religion: Option<&str>,
        book: Option<&str>,
        body: &str,
    ) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();

        // Insert into external content table
        conn.execute(
            "INSERT INTO documents_content (path, title, author, religion, book, body) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![path, title, author, religion, book, body],
        )?;

        let rowid = conn.last_insert_rowid();

        // Insert into FTS5 virtual table
        conn.execute(
            "INSERT INTO documents_fts (rowid, path, title, author, religion, book, body) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![rowid, path, title, author, religion, book, body],
        )?;

        Ok(())
    }

    /// Search FTS5 with a query string. Returns results with snippets.
    pub fn search(
        &self,
        query: &str,
        limit: i64,
    ) -> SqlResult<Vec<SearchRow>> {
        let conn = self.conn.lock().unwrap();

        // Sanitize query for FTS5: wrap each term in quotes for safety
        let safe_query = sanitize_fts_query(query);

        let sql = format!(
            "
            SELECT
                dc.rowid,
                dc.path,
                dc.title,
                dc.author,
                dc.religion,
                dc.book,
                snippet(documents_fts, 5, '<mark>', '</mark>', '...', 48) as snippet,
                rank
            FROM documents_fts
            JOIN documents_content dc ON dc.rowid = documents_fts.rowid
            WHERE documents_fts MATCH ?1
            ORDER BY rank
            LIMIT ?2
            "
        );

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt
            .query_map(rusqlite::params![safe_query, limit], |row| {
                Ok(SearchRow {
                    rowid: row.get(0)?,
                    path: row.get(1)?,
                    title: row.get(2)?,
                    author: row.get(3)?,
                    religion: row.get(4)?,
                    book: row.get(5)?,
                    snippet: row.get(6)?,
                    rank: row.get(7)?,
                })
            })?
            .collect::<SqlResult<Vec<_>>>()?;

        Ok(rows)
    }

    /// Get the hierarchical tree of documents grouped by religion > book > chapter.
    pub fn get_document_tree(&self) -> SqlResult<Vec<TreeEntry>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "
            SELECT
                COALESCE(dm.religion, 'Sin religión') as religion,
                COALESCE(dm.book, f.filename) as book,
                COALESCE(dm.chapter, '—') as chapter,
                f.path,
                f.filename
            FROM files f
            LEFT JOIN document_metadata dm ON dm.file_id = f.id
            ORDER BY religion, book, chapter
            ",
        )?;

        let entries = stmt
            .query_map([], |row| {
                Ok(TreeEntry {
                    religion: row.get(0)?,
                    book: row.get(1)?,
                    chapter: row.get(2)?,
                    path: row.get(3)?,
                    filename: row.get(4)?,
                })
            })?
            .collect::<SqlResult<Vec<_>>>()?;

        Ok(entries)
    }

    /// Read a document's full body by path.
    pub fn read_document(&self, doc_path: &str) -> SqlResult<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt =
            conn.prepare("SELECT body FROM documents_content WHERE path = ?1 LIMIT 1")?;
        let result = stmt.query_row(rusqlite::params![doc_path], |row| row.get(0));
        match result {
            Ok(body) => Ok(Some(body)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Clear all data from the index.
    pub fn clear_all(&self) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "
            DELETE FROM documents_fts;
            DELETE FROM documents_content;
            DELETE FROM document_metadata;
            DELETE FROM files;
            ",
        )?;
        Ok(())
    }

    /// Get index statistics.
    pub fn get_stats(&self) -> SqlResult<(i64, Option<String>)> {
        let conn = self.conn.lock().unwrap();
        let total: i64 =
            conn.query_row("SELECT COUNT(*) FROM files", [], |row| row.get(0))?;
        let last: Option<String> = conn
            .query_row(
                "SELECT MAX(indexed_at) FROM files",
                [],
                |row| row.get(0),
            )
            .ok();
        Ok((total, last))
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchRow {
    pub rowid: i64,
    pub path: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub religion: Option<String>,
    pub book: Option<String>,
    pub snippet: String,
    pub rank: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TreeEntry {
    pub religion: String,
    pub book: String,
    pub chapter: String,
    pub path: String,
    pub filename: String,
}

/// Sanitize a user query for FTS5 MATCH syntax.
/// Wraps each term in quotes to prevent syntax errors.
fn sanitize_fts_query(query: &str) -> String {
    let terms: Vec<&str> = query
        .split_whitespace()
        .filter(|t| !t.is_empty())
        .collect();

    if terms.is_empty() {
        return String::new();
    }

    // Check for boolean operators
    let mut result = Vec::new();
    let mut i = 0;
    while i < terms.len() {
        let term = terms[i];
        match term.to_uppercase().as_str() {
            "AND" | "OR" | "NOT" => {
                result.push(term.to_string());
            }
            _ => {
                // Quote each term for FTS5 safety
                let clean = term.trim_matches(|c: char| c == '"' || c == '\'');
                if !clean.is_empty() {
                    result.push(format!("\"{}\"", clean));
                }
            }
        }
        i += 1;
    }

    result.join(" ")
}
