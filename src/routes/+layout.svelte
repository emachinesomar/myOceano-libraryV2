<script lang="ts">
  import { onMount } from 'svelte';
  import type { Snippet } from 'svelte';
  import '../app.css';
  import AppSidebar from '$lib/components/AppSidebar.svelte';
  import SearchCommand from '$lib/components/SearchCommand.svelte';
  import Toast from '$lib/components/Toast.svelte';

  let { children }: { children: Snippet } = $props();

  let sidebarOpen = $state(true);
  let commandOpen = $state(false);
  let isDark = $state(true);

  onMount(() => {
    // Load saved preference, default to dark
    const stored = localStorage.getItem('ocean-dark-mode');
    isDark = stored !== null ? stored === 'true' : true;
    document.documentElement.classList.toggle('dark', isDark);
  });

  function toggleDark() {
    isDark = !isDark;
    localStorage.setItem('ocean-dark-mode', String(isDark));
    document.documentElement.classList.toggle('dark', isDark);
  }

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

      <div class="flex-1"></div>

      <!-- Dark mode toggle -->
      <button
        class="inline-flex items-center justify-center rounded-md p-1.5 text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
        onclick={toggleDark}
        aria-label={isDark ? 'Modo claro' : 'Modo oscuro'}
      >
        {#if isDark}
          <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
          </svg>
        {:else}
          <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
          </svg>
        {/if}
      </button>

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

<!-- Toast notifications -->
<Toast />
