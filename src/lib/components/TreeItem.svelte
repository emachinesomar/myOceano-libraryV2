<script lang="ts">
  import type { TreeNode } from '$lib/types';

  let { node }: { node: TreeNode } = $props();
  let expanded = $state(false);

  function toggle() {
    if (node.children.length > 0) {
      expanded = !expanded;
    }
  }

  // Dispatch a custom event when a document is selected
  function selectDocument(path: string) {
    window.dispatchEvent(new CustomEvent('select-document', { detail: { path } }));
  }
</script>

<div>
  <!-- Node header -->
  <button
    class="w-full flex items-center gap-1.5 rounded-md px-2 py-1 text-left text-sm transition-colors hover:bg-sidebar-accent group"
    onclick={toggle}
  >
    <!-- Expand/collapse icon -->
    {#if node.children.length > 0}
      <svg
        class="h-3 w-3 shrink-0 text-muted-foreground transition-transform duration-200"
        class:rotate-90={expanded}
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    {:else}
      <span class="w-3 shrink-0"></span>
    {/if}

    <!-- Icon based on type -->
    {#if node.type === 'religion'}
      <svg class="h-3.5 w-3.5 shrink-0 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
    {:else if node.type === 'book'}
      <svg class="h-3.5 w-3.5 shrink-0 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
      </svg>
    {:else}
      <svg class="h-3.5 w-3.5 shrink-0 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
      </svg>
    {/if}

    <!-- Name -->
    <span class="truncate text-sidebar-foreground group-hover:text-sidebar-accent-foreground">
      {node.name}
    </span>

    <!-- Count badge -->
    {#if node.children.length > 0}
      <span class="ml-auto text-[10px] text-muted-foreground tabular-nums">
        {node.count}
      </span>
    {/if}
  </button>

  <!-- Children -->
  {#if expanded && node.children.length > 0}
    <div class="ml-3 pl-3 border-l border-sidebar-border">
      {#each node.children as child (child.name + child.type)}
        <svelte:self node={child} />
      {/each}
    </div>
  {/if}
</div>
