mod db;
mod indexer;
mod parser;

use db::Database;
use indexer::full_index;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::Emitter;
use tauri::Manager;

/// Application state shared across commands.
struct AppState {
    db: Arc<Database>,
    #[allow(dead_code)]
    data_dir: PathBuf,
}

// ──────────────────────────── Tauri Commands ────────────────────────────

/// Index a directory of Markdown files.
/// Emits `index-progress` events to the frontend during processing.
#[tauri::command]
async fn index_directory(
    path: String,
    app: tauri::AppHandle,
) -> Result<IndexResult, String> {
    let state = app.state::<Arc<AppState>>();
    let db = Arc::clone(&state.db);
    let root = PathBuf::from(&path);

    if !root.exists() {
        return Err(format!("Directory not found: {}", path));
    }

    let result = tokio::task::spawn_blocking(move || {
        full_index(&db, &root, Some(&|current, total, filename| {
            let _ = app.emit(
                "index-progress",
                IndexProgressPayload {
                    current,
                    total,
                    filename: filename.to_string(),
                },
            );
        }))
    })
    .await
    .map_err(|e| format!("Indexing task failed: {}", e))?;

    Ok(IndexResult {
        total_files: result.total_files,
        indexed: result.indexed,
        skipped: result.skipped,
        errors: result.errors,
        duration_ms: result.duration_ms,
    })
}

/// Search documents using FTS5.
#[tauri::command]
async fn search_documents(
    query: String,
    limit: Option<i64>,
    app: tauri::AppHandle,
) -> Result<SearchResponse, String> {
    let state = app.state::<Arc<AppState>>();

    let results = {
        let db = &state.db;
        db.search(&query, limit.unwrap_or(50))
            .map_err(|e| format!("Search failed: {}", e))?
    };

    let total = results.len() as i64;

    Ok(SearchResponse {
        results: results
            .into_iter()
            .map(|r| SearchResultJson {
                id: r.rowid,
                path: r.path,
                title: r.title,
                author: r.author,
                religion: r.religion,
                book: r.book,
                snippet: r.snippet,
                paragraph: r.paragraph,
                rank: r.rank,
            })
            .collect(),
        total,
        query,
    })
}

/// Get the hierarchical document tree for the sidebar.
#[tauri::command]
async fn get_document_tree(app: tauri::AppHandle) -> Result<Vec<TreeNodeJson>, String> {
    let state = app.state::<Arc<AppState>>();

    let entries = {
        let db = &state.db;
        db.get_document_tree()
            .map_err(|e| format!("Tree query failed: {}", e))?
    };

    Ok(build_tree(entries))
}

/// Read a document's full content by path.
#[tauri::command]
async fn read_document(path: String, app: tauri::AppHandle) -> Result<String, String> {
    let state = app.state::<Arc<AppState>>();

    let db = &state.db;
    db.read_document(&path)
        .map_err(|e| format!("Read failed: {}", e))?
        .ok_or_else(|| format!("Document not found: {}", path))
}

/// Clear the entire index.
#[tauri::command]
async fn clear_index(app: tauri::AppHandle) -> Result<(), String> {
    let state = app.state::<Arc<AppState>>();

    let db = &state.db;
    db.clear_all()
        .map_err(|e| format!("Clear failed: {}", e))
}

/// Delete a single document by path.
#[tauri::command]
async fn delete_document(app: tauri::AppHandle, path: String) -> Result<(), String> {
    let state = app.state::<Arc<AppState>>();

    let db = &state.db;
    db.delete_document(&path)
        .map_err(|e| format!("Delete failed: {}", e))
}

/// Get index statistics.
#[tauri::command]
async fn get_index_stats(app: tauri::AppHandle) -> Result<IndexStats, String> {
    let state = app.state::<Arc<AppState>>();

    let db = &state.db;
    let (total, last_indexed) = db
        .get_stats()
        .map_err(|e| format!("Stats failed: {}", e))?;

    Ok(IndexStats {
        total,
        last_indexed,
    })
}

/// Debug: get FTS statistics to verify population.
#[tauri::command]
async fn get_fts_stats(app: tauri::AppHandle) -> Result<db::FtsStats, String> {
    let state = app.state::<Arc<AppState>>();
    let db = &state.db;
    db.get_fts_stats().map_err(|e| format!("FTS stats failed: {}", e))
}

/// Update document metadata for a given file path.
#[tauri::command]
async fn update_document_metadata(
    path: String,
    religion: Option<String>,
    book: Option<String>,
    chapter: Option<String>,
    title: Option<String>,
    author: Option<String>,
    language: Option<String>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let state = app.state::<Arc<AppState>>();
    let db = &state.db;
    db.update_metadata(
        &path,
        religion.as_deref(),
        book.as_deref(),
        chapter.as_deref(),
        title.as_deref(),
        author.as_deref(),
        language.as_deref(),
    )
    .map_err(|e| format!("Update metadata failed: {}", e))
}

/// Get metadata for a document by path.
#[tauri::command]
async fn get_document_metadata(
    path: String,
    app: tauri::AppHandle,
) -> Result<db::DocumentMetadataRow, String> {
    let state = app.state::<Arc<AppState>>();
    let db = &state.db;
    db.get_metadata(&path)
        .map_err(|e| format!("Get metadata failed: {}", e))?
        .ok_or_else(|| format!("Metadata not found for: {}", path))
}

/// Bulk update religion for all documents with a given religion.
#[tauri::command]
async fn update_religion_bulk(
    old_religion: String,
    new_religion: String,
    app: tauri::AppHandle,
) -> Result<usize, String> {
    let state = app.state::<Arc<AppState>>();
    let db = &state.db;
    db.update_religion_bulk(&old_religion, &new_religion)
        .map_err(|e| format!("Bulk religion update failed: {}", e))
}

/// Bulk update book for all documents with a given religion + book.
#[tauri::command]
async fn update_book_bulk(
    religion: String,
    old_book: String,
    new_book: String,
    app: tauri::AppHandle,
) -> Result<usize, String> {
    let state = app.state::<Arc<AppState>>();
    let db = &state.db;
    db.update_book_bulk(&religion, &old_book, &new_book)
        .map_err(|e| format!("Bulk book update failed: {}", e))
}

// ──────────────────────────── Types ────────────────────────────

#[derive(Clone, serde::Serialize)]
struct IndexProgressPayload {
    current: usize,
    total: usize,
    filename: String,
}

#[derive(Clone, serde::Serialize)]
struct IndexResult {
    total_files: usize,
    indexed: usize,
    skipped: usize,
    errors: Vec<String>,
    duration_ms: u64,
}

#[derive(Clone, serde::Serialize)]
struct SearchResponse {
    results: Vec<SearchResultJson>,
    total: i64,
    query: String,
}

#[derive(Clone, serde::Serialize)]
struct SearchResultJson {
    id: i64,
    path: String,
    title: Option<String>,
    author: Option<String>,
    religion: Option<String>,
    book: Option<String>,
    snippet: String,
    paragraph: String,
    rank: f64,
}

#[derive(Clone, serde::Serialize)]
struct TreeNodeJson {
    name: String,
    #[serde(rename = "type")]
    node_type: String,
    count: usize,
    children: Vec<TreeNodeJson>,
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    religion: Option<String>,
}

#[derive(Clone, serde::Serialize)]
struct IndexStats {
    total: i64,
    last_indexed: Option<String>,
}

// ──────────────────────────── Tree Builder ────────────────────────────

fn build_tree(entries: Vec<db::TreeEntry>) -> Vec<TreeNodeJson> {
    use std::collections::HashMap;

    // Group by religion > book
    let mut religions: HashMap<String, HashMap<String, Vec<db::TreeEntry>>> = HashMap::new();
    for entry in entries {
        religions
            .entry(entry.religion.clone())
            .or_default()
            .entry(entry.book.clone())
            .or_default()
            .push(entry);
    }

    religions
        .into_iter()
        .map(|(religion, books)| {
            let book_nodes: Vec<TreeNodeJson> = books
                .into_iter()
                .flat_map(|(book, entries)| {
                    if book == "Instituto Ruhí" {
                        // Group Ruhí entries by chapter (Libro X)
                        let mut chapters: HashMap<String, Vec<db::TreeEntry>> = HashMap::new();
                        for e in entries {
                            chapters
                                .entry(e.chapter.clone())
                                .or_default()
                                .push(e);
                        }
                        let chapter_nodes: Vec<TreeNodeJson> = chapters
                            .into_iter()
                            .map(|(chapter, chapter_entries)| {
                                let doc_nodes: Vec<TreeNodeJson> = chapter_entries
                                    .into_iter()
                                    .map(|e| TreeNodeJson {
                                        name: e.filename.clone(),
                                        node_type: "document".to_string(),
                                        count: 1,
                                        children: vec![],
                                        path: Some(e.path),
                                        religion: None,
                                    })
                                    .collect();
                                TreeNodeJson {
                                    name: chapter,
                                    node_type: "chapter".to_string(),
                                    count: doc_nodes.len(),
                                    children: doc_nodes,
                                    path: None,
                                    religion: Some(religion.clone()),
                                }
                            })
                            .collect();
                        vec![TreeNodeJson {
                            name: book,
                            node_type: "book".to_string(),
                            count: chapter_nodes.len(),
                            children: chapter_nodes,
                            path: None,
                            religion: Some(religion.clone()),
                        }]
                    } else {
                        // Non-Ruhí: group under "Libros" parent
                        let doc_nodes: Vec<TreeNodeJson> = entries
                            .into_iter()
                            .map(|e| TreeNodeJson {
                                name: e.filename.clone(),
                                node_type: "document".to_string(),
                                count: 1,
                                children: vec![],
                                path: Some(e.path),
                                religion: None,
                            })
                            .collect();
                        vec![TreeNodeJson {
                            name: book,
                            node_type: "book".to_string(),
                            count: doc_nodes.len(),
                            children: doc_nodes,
                            path: None,
                            religion: Some(religion.clone()),
                        }]
                    }
                })
                .collect();

            TreeNodeJson {
                name: religion,
                node_type: "religion".to_string(),
                count: book_nodes.len(),
                children: book_nodes,
                path: None,
                religion: None,
            }
        })
        .collect()
}

// ──────────────────────────── Entry Point ────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to resolve app data dir");
            std::fs::create_dir_all(&data_dir).ok();

            let db_path = data_dir.join("ocean_library.db");
            let database = Database::open(&db_path).expect("Failed to open database");
            database
                .initialize_schema()
                .expect("Failed to initialize schema");

            let state = Arc::new(AppState {
                db: Arc::new(database),
                data_dir,
            });

            app.manage(state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            index_directory,
            search_documents,
            get_document_tree,
            read_document,
            clear_index,
            delete_document,
            get_index_stats,
            get_fts_stats,
            update_document_metadata,
            get_document_metadata,
            update_religion_bulk,
            update_book_bulk,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
