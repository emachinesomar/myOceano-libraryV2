import { invoke } from '@tauri-apps/api/core';
import type { SearchResponse, TreeNode, IndexResult, SyncResult } from './types';

/**
 * Scan and index a directory of Markdown files.
 * Runs in background thread, emits progress events.
 */
export async function indexDirectory(path: string): Promise<IndexResult> {
  return invoke<IndexResult>('index_directory', { path });
}

/**
 * Sync index with filesystem: index new/changed files, remove deleted ones.
 * Faster than a full re-index. Emits `sync-progress` events.
 */
export async function syncDirectory(path: string): Promise<SyncResult> {
  return invoke<SyncResult>('sync_directory_command', { path });
}

/**
 * Execute a full-text search query against FTS5.
 */
export async function searchDocuments(query: string, limit?: number): Promise<SearchResponse> {
  return invoke<SearchResponse>('search_documents', { query, limit: limit ?? 50 });
}

/**
 * Get the hierarchical document tree for sidebar.
 */
export async function getDocumentTree(): Promise<TreeNode[]> {
  return invoke<TreeNode[]>('get_document_tree');
}

/**
 * Read a document's full content by path.
 */
export async function readDocument(path: string): Promise<string> {
  return invoke<string>('read_document', { path });
}

/**
 * Clear the entire index and database.
 */
export async function clearIndex(): Promise<void> {
  return invoke<void>('clear_index');
}

/**
 * Delete a single document by path.
 */
export async function deleteDocument(path: string): Promise<void> {
  return invoke<void>('delete_document', { path });
}

/**
 * Get the total number of indexed documents.
 */
export async function getIndexStats(): Promise<{ total: number; last_indexed: string | null }> {
  return invoke<{ total: number; last_indexed: string | null }>('get_index_stats');
}

/**
 * Get FTS debug statistics.
 */
export async function getFtsStats(): Promise<{ files_count: number; content_count: number; fts_count: number }> {
  return invoke<{ files_count: number; content_count: number; fts_count: number }>('get_fts_stats');
}

/**
 * Update document metadata for a given file path.
 */
export async function updateDocumentMetadata(
  path: string,
  metadata: {
    religion?: string | null;
    book?: string | null;
    chapter?: string | null;
    title?: string | null;
    author?: string | null;
    language?: string | null;
  }
): Promise<void> {
  return invoke<void>('update_document_metadata', { path, ...metadata });
}

/**
 * Get metadata for a document by path.
 */
export async function getDocumentMetadata(path: string): Promise<{
  religion: string | null;
  book: string | null;
  chapter: string | null;
  title: string | null;
  author: string | null;
  language: string | null;
}> {
  return invoke('get_document_metadata', { path });
}

/**
 * Bulk update religion for all documents with a given religion.
 */
export async function updateReligionBulk(oldReligion: string, newReligion: string): Promise<number> {
  return invoke<number>('update_religion_bulk', { oldReligion, newReligion });
}

/**
 * Bulk update book for all documents with a given religion + book.
 */
export async function updateBookBulk(religion: string, oldBook: string, newBook: string): Promise<number> {
  return invoke<number>('update_book_bulk', { religion, oldBook, newBook });
}
