use std::collections::HashSet;
use crate::db::Database;
use crate::parser::{infer_metadata_from_path, merge_metadata, parse_markdown_content};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::time::Instant;
use walkdir::WalkDir;

/// Embedded Python script for PyMuPDF fallback extraction.
const PYMUPDF_SCRIPT: &str = include_str!("../scripts/pymupdf_extract.py");

/// Extract text from a PDF file. Tries pdf-extract first, falls back to PyMuPDF.
fn extract_pdf_text(path: &Path) -> Result<String, String> {
    // Try pdf-extract first
    match pdf_extract::extract_text(path) {
        Ok(text) if !text.trim().is_empty() => return Ok(text),
        Ok(_) => {} // Empty text, try fallback
        Err(_) => {} // Error, try fallback
    }

    // Fallback: PyMuPDF via Python
    extract_pdf_text_pymupdf(path)
}

/// Extract text using PyMuPDF (Python subprocess).
fn extract_pdf_text_pymupdf(path: &Path) -> Result<String, String> {
    // Write embedded script to temp file
    let script_dir = std::env::temp_dir().join("ocean_library");
    std::fs::create_dir_all(&script_dir).map_err(|e| e.to_string())?;
    let script_path = script_dir.join("pymupdf_extract.py");
    std::fs::write(&script_path, PYMUPDF_SCRIPT).map_err(|e| e.to_string())?;

    // Find Python executable
    let python = find_python().ok_or("Python not found for PyMuPDF fallback")?;

    let output = Command::new(&python)
        .arg(script_path.to_str().unwrap_or_default())
        .arg(path.to_str().unwrap_or_default())
        .output()
        .map_err(|e| format!("Failed to run PyMuPDF: {}", e))?;

    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout).to_string();
        if text.trim().is_empty() {
            Err("PyMuPDF returned empty text".to_string())
        } else {
            Ok(text)
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("PyMuPDF failed: {}", stderr))
    }
}

/// Try to find a Python executable on the system.
fn find_python() -> Option<String> {
    for name in &["python", "python3", "py"] {
        if Command::new(name)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return Some(name.to_string());
        }
    }
    None
}

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
            if ext == Some("md") || ext == Some("markdown") || ext == Some("txt") || ext == Some("pdf") {
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

        // Parse the file based on extension
        let ext_lower = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        let (body, metadata) = if ext_lower == "pdf" {
            // PDF: extract text, infer metadata from path
            let inferred = infer_metadata_from_path(path);
            match extract_pdf_text(path) {
                Ok(text) => (text, inferred),
                Err(e) => {
                    // Index with empty body so file still appears in tree
                    errors.push(format!("PDF extract error (indexed without text): {}: {}", path.display(), e));
                    (String::new(), inferred)
                }
            }
        } else {
            // Markdown/TXT: parse frontmatter
            match std::fs::read_to_string(path).and_then(|content| {
                parse_markdown_content(&content).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
            }) {
                Ok(parsed) => {
                    let inferred = infer_metadata_from_path(path);
                    let metadata = merge_metadata(parsed.metadata, inferred);
                    (parsed.body, metadata)
                }
                Err(e) => {
                    errors.push(format!("Parse error {}: {}", path.display(), e));
                    continue;
                }
            }
        };

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
                    &body,
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

#[derive(Debug, Clone, serde::Serialize)]
pub struct SyncSummary {
    pub total_on_disk: usize,
    pub indexed: usize,
    pub removed: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
    pub duration_ms: u64,
}

/// Sync the index with the filesystem: index new/changed files, remove deleted ones.
pub fn sync_directory(
    db: &Arc<Database>,
    root: &Path,
    progress_fn: Option<&dyn Fn(usize, usize, &str)>,
) -> SyncSummary {
    let start = Instant::now();

    // Scan files on disk
    let scan = scan_directory(root);
    let disk_paths: Vec<String> = scan.files.iter().map(|p| p.to_string_lossy().to_string()).collect();
    let disk_set: HashSet<&str> = disk_paths.iter().map(|s| s.as_str()).collect();

    // Get indexed files from DB
    let indexed_paths = db.get_all_indexed_paths().unwrap_or_default();
    let db_paths: Vec<String> = indexed_paths.iter().map(|(p, _)| p.clone()).collect();
    let db_set: HashSet<&str> = db_paths.iter().map(|s| s.as_str()).collect();
    let db_mtime_map: std::collections::HashMap<&str, &str> = indexed_paths
        .iter()
        .map(|(p, m)| (p.as_str(), m.as_str()))
        .collect();

    // Classify files
    let mut to_index: Vec<&PathBuf> = Vec::new();  // new + changed
    let mut removed = 0usize;

    // New files: on disk, not in DB
    for f in &scan.files {
        let path_str = f.to_string_lossy();
        if !db_set.contains(path_str.as_ref()) {
            to_index.push(f);
        }
    }

    // Changed files: on disk, in DB, but mtime differs
    for f in &scan.files {
        let path_str = f.to_string_lossy();
        if let Some(old_mtime) = db_mtime_map.get(path_str.as_ref()) {
            if let Ok(meta) = std::fs::metadata(f) {
                if let Ok(modified) = meta.modified() {
                    let dt: chrono::DateTime<chrono::Utc> = modified.into();
                    let current_mtime = dt.to_rfc3339();
                    if old_mtime != &&current_mtime {
                        // mtime changed, re-index
                        to_index.push(f);
                    }
                }
            }
        }
    }

    // Deleted files: in DB, not on disk
    for p in &db_paths {
        if !disk_set.contains(p.as_str()) {
            if db.delete_document(p).is_ok() {
                removed += 1;
            }
        }
    }

    // Index new + changed files
    let total_to_index = to_index.len();
    let mut indexed = 0usize;
    let mut errors = Vec::new();

    for (i, path) in to_index.iter().enumerate() {
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        if let Some(cb) = progress_fn {
            cb(i + 1, total_to_index, &filename);
        }

        let path_str = path.to_string_lossy().to_string();

        // If this file already exists in the DB (changed file), delete it first
        if db_set.contains(path_str.as_str()) {
            let _ = db.delete_document(&path_str);
        }

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

        let filename_os = path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");
        let ext_lower = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

        let (body, metadata) = if ext_lower == "pdf" {
            let inferred = infer_metadata_from_path(path);
            match extract_pdf_text(path) {
                Ok(text) => (text, inferred),
                Err(e) => {
                    errors.push(format!("PDF extract error (indexed without text): {}: {}", path.display(), e));
                    (String::new(), inferred)
                }
            }
        } else {
            match std::fs::read_to_string(path).and_then(|content| {
                parse_markdown_content(&content).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
            }) {
                Ok(parsed) => {
                    let inferred = infer_metadata_from_path(path);
                    let metadata = merge_metadata(parsed.metadata, inferred);
                    (parsed.body, metadata)
                }
                Err(e) => {
                    errors.push(format!("Parse error {}: {}", path.display(), e));
                    continue;
                }
            }
        };

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("md");

        match db.insert_file(&path_str, filename_os, ext, meta.len() as i64, &mtime) {
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
                    &body,
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

    let duration_ms = start.elapsed().as_millis() as u64;
    let skipped = scan.total.saturating_sub(indexed);

    SyncSummary {
        total_on_disk: scan.total,
        indexed,
        removed,
        skipped,
        errors,
        duration_ms,
    }
}
