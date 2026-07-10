/** Metadata extracted from Markdown YAML frontmatter */
export interface DocumentMetadata {
  religion: string | null;
  book: string | null;
  chapter: string | null;
  verse: string | null;
  title: string | null;
  author: string | null;
  language: string | null;
  tags: string[];
}

/** A single document in the index */
export interface IndexedDocument {
  id: number;
  path: string;
  filename: string;
  metadata: DocumentMetadata;
  body_preview: string;
  indexed_at: string;
}

/** Search result from FTS5 */
export interface SearchResult {
  id: number;
  path: string;
  title: string | null;
  author: string | null;
  religion: string | null;
  book: string | null;
  snippet: string;
  paragraph: string;
  rank: number;
}

/** Tree node for sidebar navigation */
export interface TreeNode {
  name: string;
  type: 'religion' | 'book' | 'chapter' | 'document';
  count: number;
  children: TreeNode[];
  path?: string;
  religion?: string; // Inherited religion name for book nodes
}

/** Search response with pagination */
export interface SearchResponse {
  results: SearchResult[];
  total: number;
  query: string;
}

/** Index progress event from backend */
export interface IndexProgress {
  current: number;
  total: number;
  filename: string;
  phase: 'scanning' | 'parsing' | 'indexing';
}

/** Index result summary */
export interface IndexResult {
  total_files: number;
  indexed: number;
  skipped: number;
  errors: string[];
  duration_ms: number;
}

/** Sync result summary */
export interface SyncResult {
  total_on_disk: number;
  indexed: number;
  removed: number;
  skipped: number;
  errors: string[];
  duration_ms: number;
}

/** Document metadata row for editing */
export interface DocumentMetadataRow {
  religion: string | null;
  book: string | null;
  chapter: string | null;
  title: string | null;
  author: string | null;
  language: string | null;
}
