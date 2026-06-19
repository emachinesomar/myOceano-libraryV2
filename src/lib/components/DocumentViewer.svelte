<script lang="ts">
  import { onMount } from 'svelte';
  import { readDocument } from '$lib/tauri';
  import { marked } from 'marked';

  let documentPath = $state<string | null>(null);
  let content = $state('');
  let loading = $state(false);
  let error = $state('');
  let searchSnippet = $state('');
  let scrollEl = $state<HTMLDivElement>();

  // Scroll to top when document changes
  $effect(() => {
    if (content && scrollEl) {
      scrollEl.scrollTop = 0;
    }
  });

  // Configure marked for safe rendering
  marked.setOptions({
    breaks: true,
    gfm: true,
  });

  // Listen for document selection events
  onMount(() => {
    function handleSelect(e: Event) {
      const detail = (e as CustomEvent).detail;
      documentPath = detail.path;
      searchSnippet = detail.snippet || '';
      loadDocument(detail.path);
    }

    window.addEventListener('select-document', handleSelect);
    return () => window.removeEventListener('select-document', handleSelect);
  });

  async function loadDocument(path: string) {
    loading = true;
    error = '';
    content = '';
    try {
      content = await readDocument(path);
    } catch (e) {
      error = String(e);
    }
    loading = false;
  }

  // Format path for display
  function formatPath(path: string): string {
    const parts = path.replace(/\\/g, '/').split('/');
    return parts.slice(-3).join(' / ');
  }

  // Render markdown to HTML
  function renderMarkdown(text: string): string {
    return marked.parse(text) as string;
  }
</script>

<div class="h-full flex flex-col">
  {#if !documentPath}
    <!-- Empty state -->
    <div class="flex-1 flex items-center justify-center">
      <div class="text-center space-y-4">
        <div class="mx-auto h-16 w-16 rounded-2xl bg-muted flex items-center justify-center">
          <svg class="h-8 w-8 text-muted-foreground/40" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
          </svg>
        </div>
        <div>
          <p class="text-sm text-muted-foreground">Selecciona un documento para leer</p>
          <p class="text-xs text-muted-foreground/60 mt-1">Usa Ctrl+K para buscar o navega el árbol lateral</p>
        </div>
      </div>
    </div>
  {:else}
    <!-- Document header -->
    <div class="flex items-center gap-3 px-6 py-3 border-b border-border shrink-0">
      <svg class="h-4 w-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
      </svg>
      <span class="text-sm text-muted-foreground truncate">{formatPath(documentPath)}</span>
    </div>

    <!-- Search snippet preview -->
    {#if searchSnippet}
      <div class="mx-6 mt-4 p-4 rounded-lg border border-primary/20 bg-primary/5">
        <p class="text-[10px] font-medium text-primary uppercase tracking-wider mb-2">Match en tu búsqueda</p>
        <div class="text-sm text-foreground/80 leading-relaxed">
          {@html searchSnippet}
        </div>
      </div>
    {/if}

    <!-- Document content -->
    <div class="flex-1 overflow-auto smooth-scroll px-6 py-6" bind:this={scrollEl}>
      {#if loading}
        <div class="flex items-center justify-center py-12">
          <svg class="h-6 w-6 animate-spin text-muted-foreground" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
        </div>
      {:else if error}
        <div class="py-8 text-center">
          <p class="text-sm text-destructive">{error}</p>
        </div>
      {:else}
        <article class="reading-width mx-auto reading-prose text-foreground">
          {@html renderMarkdown(content)}
        </article>
      {/if}
    </div>
  {/if}
</div>
