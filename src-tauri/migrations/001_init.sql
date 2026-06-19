-- Ocean Library v2 — SQLite FTS5 Schema
-- Multilingual support: Arabic, Sanskrit, Hebrew, CJK, diacritics

-- Table to track indexed files
CREATE TABLE IF NOT EXISTS files (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    path        TEXT NOT NULL UNIQUE,
    filename    TEXT NOT NULL,
    extension   TEXT NOT NULL,
    size_bytes  INTEGER NOT NULL,
    mtime       TEXT NOT NULL,
    indexed_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Metadata extracted from YAML frontmatter
CREATE TABLE IF NOT EXISTS document_metadata (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    file_id     INTEGER NOT NULL UNIQUE,
    religion    TEXT,
    book        TEXT,
    chapter     TEXT,
    verse       TEXT,
    title       TEXT,
    author      TEXT,
    language    TEXT,
    tags        TEXT,  -- JSON array stored as text
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
);

-- FTS5 virtual table for full-text search
-- unicode61 tokenizer with:
--   remove_diacritics=2 — removes diacritics for languages that use them (á→a, é→e)
--   tokenchars — characters that are part of tokens (dots, hyphens, apostrophes)
--   separators — characters that separate tokens (spaces, punctuation)
CREATE VIRTUAL TABLE IF NOT EXISTS documents_fts USING fts5(
    path,
    title,
    author,
    religion,
    book,
    body,
    content='documents_content',
    content_rowid='rowid',
    tokenize='unicode61 remove_diacritics 2 tokenchars "-." apostrophes "\'"'
);

-- External content table (actual storage)
CREATE TABLE IF NOT EXISTS documents_content (
    rowid  INTEGER PRIMARY KEY,
    path   TEXT NOT NULL,
    title  TEXT,
    author TEXT,
    religion TEXT,
    book   TEXT,
    body   TEXT NOT NULL
);

-- Indexes for metadata queries
CREATE INDEX IF NOT EXISTS idx_files_path ON files(path);
CREATE INDEX IF NOT EXISTS idx_metadata_religion ON document_metadata(religion);
CREATE INDEX IF NOT EXISTS idx_metadata_book ON document_metadata(book);
CREATE INDEX IF NOT EXISTS idx_metadata_chapter ON document_metadata(chapter);
CREATE INDEX IF NOT EXISTS idx_content_rowid ON documents_content(rowid);
