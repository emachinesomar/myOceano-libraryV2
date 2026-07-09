import { writable, derived } from 'svelte/store';

export interface SelectedDocument {
  path: string;
  snippet?: string;
}

function createDocumentStore() {
  const { subscribe, set } = writable<SelectedDocument | null>(null);

  return {
    subscribe,
    select(path: string, snippet?: string) {
      set({ path, snippet });
    },
    clear() {
      set(null);
    }
  };
}

export const selectedDocument = createDocumentStore();

/** Derived store with just the current path for comparison in TreeItem */
export const selectedPath = derived(selectedDocument, ($doc) => $doc?.path ?? null);
