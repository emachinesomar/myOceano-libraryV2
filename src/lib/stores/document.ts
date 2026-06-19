import { writable } from 'svelte/store';

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
