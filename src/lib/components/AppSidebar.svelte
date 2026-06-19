<script lang="ts">
  import { onMount } from 'svelte';
  import type { TreeNode } from '$lib/types';
  import { getDocumentTree, indexDirectory } from '$lib/tauri';
  import DocumentTree from './DocumentTree.svelte';

  let { open = $bindable(true) }: { open: boolean } = $props();

  let tree = $state<TreeNode[]>([]);
  let loading = $state(false);
  let indexing = $state(false);
  let indexProgress = $state({ current: 0, total: 0, filename: '' });

  onMount(async () => {
    await loadTree();
  });

  async function loadTree() {
    loading = true;
    try {
      tree = await getDocumentTree();
    } catch (e) {
      console.error('Failed to load tree:', e);
    }
    loading = false;
  }

  async function handleIndexFolder() {
    // Use Tauri dialog to pick a folder
    const { open: openDialog } = await import('@tauri-apps/plugin-dialog');
    const selected = await openDialog({ directory: true, multiple: false });
    if (!selected) return;

    const path = typeof selected === 'string' ? selected : selected as string;
    indexing = true;

    try {
      // Listen for progress events
      const { listen } = await import('@tauri-apps/api/event');
      const unlisten = await listen<{ current: number; total: number; filename: string }>(
        'index-progress',
        (event) => {
          indexProgress = event.payload;
        }
      );

      const result = await indexDirectory(path);
      unlisten();

      console.log(`Indexed ${result.indexed} files in ${result.duration_ms}ms`);
      await loadTree();
    } catch (e) {
      console.error('Indexing failed:', e);
    }

    indexing = false;
  }
</script>

<aside
  class="flex flex-col border-r border-sidebar-border bg-sidebar text-sidebar-foreground transition-all duration-300"
  class:w-64={open}
  class:w-0={!open}
  class:overflow-hidden={!open}
>
  {#if open}
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b border-sidebar-border shrink-0">
      <div class="flex items-center gap-2">
        <div class="h-6 w-6 rounded-md bg-primary flex items-center justify-center">
          <svg class="h-3.5 w-3.5 text-primary-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
          </svg>
        </div>
        <span class="text-sm font-semibold text-sidebar-foreground">Ocean Library</span>
      </div>
    </div>

    <!-- Index button -->
    <div class="px-3 py-2 border-b border-sidebar-border shrink-0">
      <button
        class="w-full inline-flex items-center justify-center gap-2 rounded-lg bg-primary/10 px-3 py-2 text-xs font-medium text-primary hover:bg-primary/20 transition-colors"
        onclick={handleIndexFolder}
        disabled={indexing}
      >
        {#if indexing}
          <svg class="h-3.5 w-3.5 animate-spin" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
          Indexando {indexProgress.current}/{indexProgress.total}...
        {:else}
          <svg class="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
          </svg>
          Indexar carpeta
        {/if}
      </button>
    </div>

    <!-- Tree -->
    <div class="flex-1 overflow-auto smooth-scroll px-1 py-2">
      {#if loading}
        <div class="flex items-center justify-center py-8">
          <svg class="h-5 w-5 animate-spin text-muted-foreground" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
        </div>
      {:else if tree.length === 0}
        <div class="px-3 py-8 text-center">
          <p class="text-xs text-muted-foreground">Sin documentos indexados</p>
          <p class="text-[10px] text-muted-foreground/60 mt-1">Indexa una carpeta para comenzar</p>
        </div>
      {:else}
        <DocumentTree nodes={tree} />
      {/if}
    </div>

    <!-- Footer -->
    <div class="border-t border-sidebar-border px-4 py-2 shrink-0">
      <p class="text-[10px] text-muted-foreground/50 text-center">
        {tree.reduce((acc, r) => acc + r.count, 0)} libros · {tree.length} religiones
      </p>
    </div>
  {/if}
</aside>
