<script lang="ts">
  import { tick } from 'svelte';
  import type { SearchResult } from '$lib/types';
  import { searchDocuments } from '$lib/tauri';
  import { toasts } from '$lib/stores/toast';
  import { selectedDocument } from '$lib/stores/document';

  let { open = $bindable(false) }: { open: boolean } = $props();

  let query = $state('');
  let results = $state<SearchResult[]>([]);
  let loading = $state(false);
  let selectedIndex = $state(0);
  let inputEl = $state<HTMLInputElement>();

  // Focus input when opened
  $effect(() => {
    if (open) {
      query = '';
      results = [];
      selectedIndex = 0;
      tick().then(() => inputEl?.focus());
    }
  });

  // Debounced search
  let searchTimeout: ReturnType<typeof setTimeout>;
  $effect(() => {
    const q = query.trim();
    clearTimeout(searchTimeout);
    if (q.length < 2) {
      results = [];
      return;
    }
    searchTimeout = setTimeout(async () => {
      loading = true;
      try {
        const response = await searchDocuments(q);
        results = response.results;
        selectedIndex = 0;
      } catch (e) {
        console.error('Search failed:', e);
        toasts.error('Error en la búsqueda');
        results = [];
      }
      loading = false;
    }, 200);
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      open = false;
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      selectedIndex = Math.max(selectedIndex - 1, 0);
    } else if (e.key === 'Enter' && results.length > 0) {
      e.preventDefault();
      selectResult(results[selectedIndex]);
    }
  }

  function selectResult(result: SearchResult) {
    selectedDocument.select(result.path, result.snippet);
    open = false;
  }

  function handleBackdropClick() {
    open = false;
  }
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

  <!-- Command palette -->
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-[20vh]">
    <div
      class="w-full max-w-lg bg-background border border-border rounded-xl shadow-2xl overflow-hidden"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => { if (e.key === 'Escape') open = false; }}
      role="dialog"
      aria-label="Buscar documentos"
      tabindex="0"
    >
      <!-- Search input -->
      <div class="flex items-center gap-3 px-4 border-b border-border">
        <svg class="h-4 w-4 shrink-0 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
        <input
          bind:this={inputEl}
          bind:value={query}
          onkeydown={handleKeydown}
          type="text"
          placeholder="Buscar en textos sagrados..."
          class="flex-1 bg-transparent py-3 text-sm text-foreground placeholder-muted-foreground focus:outline-none"
        />
        {#if loading}
          <svg class="h-4 w-4 animate-spin text-muted-foreground" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
        {/if}
      </div>

      <!-- Results -->
      {#if results.length > 0}
        <div class="max-h-80 overflow-auto smooth-scroll py-2">
          {#each results as result, i (result.id)}
            <button
              class="w-full flex flex-col gap-1 px-4 py-2.5 text-left transition-colors"
              class:bg-accent={i === selectedIndex}
              onclick={() => selectResult(result)}
              onmouseenter={() => (selectedIndex = i)}
            >
              <div class="flex items-center gap-2">
                {#if result.religion}
                  <span class="text-[10px] font-medium text-primary bg-primary/10 px-1.5 py-0.5 rounded">
                    {result.religion}
                  </span>
                {/if}
                {#if result.book}
                  <span class="text-[10px] text-muted-foreground">
                    {result.book}
                  </span>
                {/if}
              </div>
              <span class="text-xs text-foreground font-medium truncate">
                {result.title || result.path.split(/[/\\]/).pop()}
              </span>
              <span class="text-[11px] text-muted-foreground line-clamp-2">
                {@html result.snippet}
              </span>
            </button>
          {/each}
        </div>
      {:else if query.trim().length >= 2 && !loading}
        <div class="py-8 text-center">
          <p class="text-sm text-muted-foreground">Sin resultados para "{query}"</p>
        </div>
      {/if}

      <!-- Footer -->
      <div class="flex items-center justify-between px-4 py-2 border-t border-border bg-muted/30">
        <div class="flex items-center gap-3 text-[10px] text-muted-foreground">
          <span><kbd class="px-1 py-0.5 bg-background border border-border rounded text-[9px]">↑↓</kbd> navegar</span>
          <span><kbd class="px-1 py-0.5 bg-background border border-border rounded text-[9px]">↵</kbd> abrir</span>
          <span><kbd class="px-1 py-0.5 bg-background border border-border rounded text-[9px]">esc</kbd> cerrar</span>
        </div>
        {#if results.length > 0}
          <span class="text-[10px] text-muted-foreground">{results.length} resultados</span>
        {/if}
      </div>
    </div>
  </div>
{/if}
