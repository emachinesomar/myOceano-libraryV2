<script lang="ts">
  import { tick } from 'svelte';
  import type { DocumentMetadataRow } from '$lib/types';
  import { getDocumentMetadata, updateDocumentMetadata, updateReligionBulk, updateBookBulk } from '$lib/tauri';
  import { toasts } from '$lib/stores/toast';

  let { open = $bindable(false), path = '', nodeType = 'document', nodeName = '', nodeReligion = '', onSaved = () => {} }: {
    open: boolean;
    path: string;
    nodeType?: 'religion' | 'book' | 'document';
    nodeName?: string;
    nodeReligion?: string;
    onSaved?: () => void;
  } = $props();

  let religion = $state('');
  let book = $state('');
  let chapter = $state('');
  let title = $state('');
  let author = $state('');
  let language = $state('');
  let loading = $state(false);
  let saving = $state(false);
  let inputEl = $state<HTMLInputElement>();

  // For bulk edits
  let newName = $state('');

  // Preset options
  const religionOptions = [
    "Fe bahá'í",
    "Islam",
    "Cristianismo",
    "Judaísmo",
    "Hinduismo",
    "Budismo",
    "Otra"
  ];

  const bookOptions: Record<string, string[]> = {
    "Fe bahá'í": ["Ridván", "CUJ", "Instituto Ruhí", "Carta", "Epístola", "Otro"],
    "Islam": ["Corán", "Hadiz", "Otro"],
    "Cristianismo": ["Biblia", "Evangelio", "Otro"],
    "Judaísmo": ["Torá", "Talmud", "Otro"],
    "Hinduismo": ["Bhagavad Gita", "Vedas", "Otro"],
    "Budismo": ["Dhammapada", "Sutras", "Otro"],
  };

  const languageOptions = ["Castellano", "Inglés", "Árabe", "Persa", "Francés", "Otro"];

  // Load metadata when modal opens
  $effect(() => {
    if (open) {
      if (nodeType === 'document' && path) {
        loadMetadata();
      } else {
        newName = nodeName;
      }
      tick().then(() => inputEl?.focus());
    }
  });

  async function loadMetadata() {
    loading = true;
    try {
      const meta = await getDocumentMetadata(path);
      religion = meta.religion || '';
      book = meta.book || '';
      chapter = meta.chapter || '';
      title = meta.title || '';
      author = meta.author || '';
      language = meta.language || '';
    } catch (e) {
      console.log('No metadata found, using defaults');
      religion = '';
      book = '';
      chapter = '';
      title = '';
      author = '';
      language = '';
    }
    loading = false;
  }

  async function handleSave() {
    saving = true;
    try {
      if (nodeType === 'religion') {
        // Bulk update religion
        const count = await updateReligionBulk(nodeName, newName);
        toasts.success(`Religión renombrada: ${count} documentos actualizados`);
      } else if (nodeType === 'book') {
        // Bulk update book
        const count = await updateBookBulk(nodeReligion, nodeName, newName);
        toasts.success(`Libro renombrado: ${count} documentos actualizados`);
      } else {
        // Update single document
        await updateDocumentMetadata(path, {
          religion: religion || null,
          book: book || null,
          chapter: chapter || null,
          title: title || null,
          author: author || null,
          language: language || null,
        });
        toasts.success('Metadata guardada correctamente');
      }
      onSaved();
      open = false;
    } catch (e) {
      console.error('Save failed:', e);
      toasts.error('Error al guardar');
    }
    saving = false;
  }

  function handleBackdropClick() {
    open = false;
  }

  // Get available books for current religion
  const availableBooks = $derived(bookOptions[religion] || []);

  // Modal title based on node type
  const modalTitle = $derived(
    nodeType === 'religion' ? 'Editar Religión' :
    nodeType === 'book' ? 'Editar Libro' :
    'Editar Metadata'
  );
</script>

{#if open}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm"
    onclick={handleBackdropClick}
    onkeydown={(e) => { if (e.key === 'Escape') handleBackdropClick(); }}
    role="presentation"
    tabindex="-1"
  ></div>

  <!-- Modal -->
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <div
      class="w-full max-w-lg bg-background border border-border rounded-xl shadow-2xl overflow-hidden"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => { if (e.key === 'Escape') open = false; }}
      role="dialog"
      aria-label={modalTitle}
      tabindex="0"
    >
      <!-- Header -->
      <div class="flex items-center justify-between px-6 py-4 border-b border-border">
        <div class="flex items-center gap-3">
          <div class="h-8 w-8 rounded-lg bg-primary/10 flex items-center justify-center">
            {#if nodeType === 'religion'}
              <svg class="h-4 w-4 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            {:else if nodeType === 'book'}
              <svg class="h-4 w-4 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
              </svg>
            {:else}
              <svg class="h-4 w-4 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
              </svg>
            {/if}
          </div>
          <div>
            <h2 class="text-sm font-semibold">{modalTitle}</h2>
            <p class="text-[11px] text-muted-foreground truncate max-w-[300px]">
              {#if nodeType === 'religion'}
                {nodeName} — {nodeType === 'religion' ? 'todos los documentos' : ''}
              {:else if nodeType === 'book'}
                {nodeName} — {nodeReligion}
              {:else}
                {path.split(/[/\\]/).pop()}
              {/if}
            </p>
          </div>
        </div>
        <button
          onclick={() => open = false}
          class="p-1 rounded-md text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
          aria-label="Cerrar"
        >
          <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Form -->
      {#if loading}
        <div class="flex items-center justify-center py-12">
          <svg class="h-6 w-6 animate-spin text-muted-foreground" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
        </div>
      {:else}
        <div class="px-6 py-4 space-y-4 max-h-[60vh] overflow-auto">
          {#if nodeType === 'religion'}
            <!-- Religion rename -->
            <div class="space-y-1.5">
              <label for="newName" class="text-xs font-medium text-muted-foreground">Nuevo nombre de religión</label>
              <select
                id="newName"
                bind:value={newName}
                class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary"
              >
                {#each religionOptions as opt}
                  <option value={opt}>{opt}</option>
                {/each}
              </select>
              <p class="text-[10px] text-muted-foreground">Se actualizarán todos los documentos con religión "{nodeName}"</p>
            </div>

          {:else if nodeType === 'book'}
            <!-- Book rename -->
            <div class="space-y-1.5">
              <label for="newName" class="text-xs font-medium text-muted-foreground">Nuevo nombre de libro</label>
              {#if bookOptions[nodeReligion]}
                <div class="flex flex-wrap gap-1.5">
                  {#each bookOptions[nodeReligion] as opt}
                    <button
                      type="button"
                      onclick={() => newName = opt}
                      class="px-3 py-1.5 rounded-lg text-xs font-medium border transition-colors"
                      class:border-primary={newName === opt}
                      class:bg-primary={newName === opt}
                      class:text-primary-foreground={newName === opt}
                      class:border-border={newName !== opt}
                      class:bg-background={newName !== opt}
                      class:text-foreground={newName !== opt}
                      class:hover:bg-accent={newName !== opt}
                    >
                      {opt}
                    </button>
                  {/each}
                </div>
              {/if}
              <input
                id="newName"
                bind:value={newName}
                type="text"
                placeholder="Nombre del libro..."
                class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary"
              />
              <p class="text-[10px] text-muted-foreground">Se actualizarán todos los documentos con libro "{nodeName}" en {nodeReligion}</p>
            </div>

          {:else}
            <!-- Document edit (original modal) -->
            <!-- Religion -->
            <div class="space-y-1.5">
              <label for="religion" class="text-xs font-medium text-muted-foreground">Religión</label>
              <select
                id="religion"
                bind:value={religion}
                class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary"
              >
                <option value="">Sin religión</option>
                {#each religionOptions as opt}
                  <option value={opt}>{opt}</option>
                {/each}
              </select>
            </div>

            <!-- Book -->
            <div class="space-y-1.5">
              <label for="book" class="text-xs font-medium text-muted-foreground">Libro / Tipo</label>
              {#if availableBooks.length > 0}
                <div class="flex flex-wrap gap-1.5">
                  {#each availableBooks as opt}
                    <button
                      type="button"
                      onclick={() => book = opt}
                      class="px-3 py-1.5 rounded-lg text-xs font-medium border transition-colors"
                      class:border-primary={book === opt}
                      class:bg-primary={book === opt}
                      class:text-primary-foreground={book === opt}
                      class:border-border={book !== opt}
                      class:bg-background={book !== opt}
                      class:text-foreground={book !== opt}
                      class:hover:bg-accent={book !== opt}
                    >
                      {opt}
                    </button>
                  {/each}
                </div>
                {#if book && !availableBooks.includes(book)}
                  <input
                    bind:value={book}
                    type="text"
                    placeholder="Otro libro..."
                    class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary"
                  />
                {/if}
              {:else}
                <input
                  bind:value={book}
                  type="text"
                  placeholder="Nombre del libro..."
                  class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary"
                />
              {/if}
            </div>

            <!-- Chapter -->
            <div class="space-y-1.5">
              <label for="chapter" class="text-xs font-medium text-muted-foreground">Capítulo / Año</label>
              <input
                id="chapter"
                bind:value={chapter}
                type="text"
                placeholder="Ej: 2026, Libro 10, Unidad 1..."
                class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary"
              />
            </div>

            <!-- Title -->
            <div class="space-y-1.5">
              <label for="title" class="text-xs font-medium text-muted-foreground">Título</label>
              <input
                id="title"
                bind:value={title}
                type="text"
                placeholder="Título del documento..."
                class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary"
              />
            </div>

            <!-- Author -->
            <div class="space-y-1.5">
              <label for="author" class="text-xs font-medium text-muted-foreground">Autor</label>
              <input
                id="author"
                bind:value={author}
                type="text"
                placeholder="Autor..."
                class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary"
              />
            </div>

            <!-- Language -->
            <div class="space-y-1.5">
              <label for="language" class="text-xs font-medium text-muted-foreground">Idioma</label>
              <select
                id="language"
                bind:value={language}
                class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary"
              >
                <option value="">No especificado</option>
                {#each languageOptions as opt}
                  <option value={opt}>{opt}</option>
                {/each}
              </select>
            </div>
          {/if}
        </div>
      {/if}

      <!-- Footer -->
      <div class="flex items-center justify-end gap-3 px-6 py-4 border-t border-border bg-muted/30">
        <button
          onclick={() => open = false}
          class="px-4 py-2 rounded-lg text-sm font-medium text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
        >
          Cancelar
        </button>
        <button
          onclick={handleSave}
          disabled={saving || (nodeType !== 'document' && !newName)}
          class="px-4 py-2 rounded-lg text-sm font-medium bg-primary text-primary-foreground hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          {#if saving}
            Guardando...
          {:else}
            Guardar
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
