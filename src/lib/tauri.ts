import { invoke } from '@tauri-apps/api/core';
import type { SearchResponse, TreeNode, IndexResult } from './types';

/**
 * Scan and index a directory of Markdown files.
 * Runs in background thread, emits progress events.
 */
export async function indexDirectory(path: string): Promise<IndexResult> {
  return invoke<IndexResult>('index_directory', { path });
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
 * Get the total number of indexed documents.
 */
export async function getIndexStats(): Promise<{ total: number; last_indexed: string | null }> {
  return invoke<{ total: number; last_indexed: string | null }>('get_index_stats');
}
