use crate::db::Database;
use crate::parser::{infer_metadata_from_path, merge_metadata, parse_markdown_file};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use walkdir::WalkDir;

/// Result of scanning a directory for Markdown files.
pub struct ScanResult {
    pub files: Vec<PathBuf>,
    pub total: usize,
}

/// Scan a directory recursively for .md files.
pub fn scan_directory(root: &Path) -> ScanResult {
    let mut files = Vec::new();

    for entry in WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| {
            // Skip hidden directories and common non-document folders
            let name = e.file_name().to_string_lossy().to_lowercase();
            !name.starts_with('.')
                && name != "node_modules"
                && name != "target"
                && name != ".git"
        })
    {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if entry.file_type().is_file() {
            let ext = entry.path().extension().and_then(|e| e.to_str());
            if ext == Some("md") || ext == Some("markdown") || ext == Some("txt") {
                files.push(entry.path().to_path_buf());
            }
        }
    }

    let total = files.len();
    ScanResult { files, total }
}

/// Index a batch of files into the database.
/// Returns (indexed_count, skipped_count, errors).
pub fn index_files(
    db: &Arc<Database>,
    files: &[PathBuf],
    progress_fn: Option<&dyn Fn(usize, usize, &str)>,
) -> (usize, usize, Vec<String>) {
    let mut indexed = 0usize;
    let mut skipped = 0usize;
    let mut errors = Vec::new();

    for (i, path) in files.iter().enumerate() {
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Report progress
        if let Some(cb) = progress_fn {
            cb(i + 1, files.len(), &filename);
        }

        // Get file metadata for mtime check
        let meta = match std::fs::metadata(path) {
            Ok(m) => m,
            Err(e) => {
                errors.push(format!("{}: {}", path.display(), e));
                continue;
            }
        };

        let mtime = meta
            .modified()
            .ok()
            .and_then(|t| {
                let dt: chrono::DateTime<chrono::Utc> = t.into();
                Some(dt.to_rfc3339())
            })
            .unwrap_or_default();

        let path_str = path.to_string_lossy().to_string();

        // Skip if already indexed with same mtime
        if db.is_file_indexed(&path_str, &mtime).unwrap_or(false) {
            skipped += 1;
            continue;
        }

        // Parse the Markdown file
        let parsed = match parse_markdown_file(path) {
            Ok(p) => p,
            Err(e) => {
                errors.push(format!("Parse error {}: {}", path.display(), e));
                continue;
            }
        };

        // Merge frontmatter metadata with path-inferred metadata
        let inferred = infer_metadata_from_path(path);
        let metadata = merge_metadata(parsed.metadata, inferred);

        // Insert into database
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("md");

        match db.insert_file(&path_str, &filename, ext, meta.len() as i64, &mtime) {
            Ok(file_id) => {
                let tags_json = serde_json::to_string(&metadata.tags).ok();

                if let Err(e) = db.insert_metadata(
                    file_id,
                    metadata.religion.as_deref(),
                    metadata.book.as_deref(),
                    metadata.chapter.as_deref(),
                    metadata.verse.as_deref(),
                    metadata.title.as_deref(),
                    metadata.author.as_deref(),
                    metadata.language.as_deref(),
                    tags_json.as_deref(),
                ) {
                    errors.push(format!("Metadata error {}: {}", path.display(), e));
                }

                if let Err(e) = db.insert_fts(
                    &path_str,
                    metadata.title.as_deref(),
                    metadata.author.as_deref(),
                    metadata.religion.as_deref(),
                    metadata.book.as_deref(),
                    &parsed.body,
                ) {
                    errors.push(format!("FTS error {}: {}", path.display(), e));
                }

                indexed += 1;
            }
            Err(e) => {
                errors.push(format!("DB error {}: {}", path.display(), e));
            }
        }
    }

    (indexed, skipped, errors)
}

/// Full index pipeline: scan → parse → store.
pub fn full_index(
    db: &Arc<Database>,
    root: &Path,
    progress_fn: Option<&dyn Fn(usize, usize, &str)>,
) -> IndexSummary {
    let start = Instant::now();

    let scan = scan_directory(root);
    let (indexed, skipped, errors) = index_files(db, &scan.files, progress_fn);

    let duration_ms = start.elapsed().as_millis() as u64;

    IndexSummary {
        total_files: scan.total,
        indexed,
        skipped,
        errors,
        duration_ms,
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct IndexSummary {
    pub total_files: usize,
    pub indexed: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
    pub duration_ms: u64,
}
