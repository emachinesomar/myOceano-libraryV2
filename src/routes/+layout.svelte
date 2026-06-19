<script lang="ts">
  import type { Snippet } from 'svelte';
  import '../app.css';
  import AppSidebar from '$lib/components/AppSidebar.svelte';
  import SearchCommand from '$lib/components/SearchCommand.svelte';

  let { children }: { children: Snippet } = $props();

  let sidebarOpen = $state(true);
  let commandOpen = $state(false);

  // Global keyboard shortcut: Ctrl+K / Cmd+K
  function handleKeydown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
      e.preventDefault();
      commandOpen = !commandOpen;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="flex h-screen overflow-hidden bg-background text-foreground">
  <!-- Sidebar -->
  <AppSidebar bind:open={sidebarOpen} />

  <!-- Main content -->
  <main class="flex-1 flex flex-col overflow-hidden">
    <!-- Top bar -->
    <header class="flex items-center gap-3 border-b border-border px-4 py-2.5 shrink-0">
      <button
        class="inline-flex items-center justify-center rounded-md p-1.5 text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
        onclick={() => (sidebarOpen = !sidebarOpen)}
        aria-label="Toggle sidebar"
      >
        <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
        </svg>
      </button>

      <!-- Search trigger button -->
      <button
        class="inline-flex items-center gap-2 rounded-lg border border-border bg-muted/50 px-3 py-1.5 text-sm text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
        onclick={() => (commandOpen = true)}
      >
        <svg class="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
        <span>Buscar...</span>
        <kbd class="ml-2 inline-flex items-center rounded border border-border bg-background px-1 py-0.5 text-[10px] font-medium text-muted-foreground">
          <span class="mr-0.5">⌘</span>K
        </kbd>
      </button>

      <div class="flex-1" />

      <span class="text-xs text-muted-foreground font-medium tracking-wide uppercase">
        Ocean Library v2
      </span>
    </header>

    <!-- Page content -->
    <div class="flex-1 overflow-auto smooth-scroll">
      {@render children()}
    </div>
  </main>
</div>

<!-- Global search command palette -->
<SearchCommand bind:open={commandOpen} />
