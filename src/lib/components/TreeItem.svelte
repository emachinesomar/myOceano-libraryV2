<script lang="ts">
  import type { TreeNode } from '$lib/types';
  import { selectedDocument, selectedPath } from '$lib/stores/document';
  import { deleteDocument } from '$lib/tauri';
  import { toasts } from '$lib/stores/toast';
  import EditMetadataModal from './EditMetadataModal.svelte';
  import TreeItem from './TreeItem.svelte';

  let { node, onMetadataSaved = () => {} }: { node: TreeNode; onMetadataSaved?: () => void } = $props();
  let expanded = $state(false);
  let editModalOpen = $state(false);
  let deleting = $state(false);

  const isLeaf = $derived(node.children.length === 0);
  const isSelected = $derived(isLeaf && $selectedPath === node.path);

  function handleClick() {
    if (isLeaf && node.path) {
      selectedDocument.select(node.path);
    } else if (!isLeaf) {
      expanded = !expanded;
    }
  }

  function handleEdit(e: Event) {
    e.stopPropagation();
    editModalOpen = true;
  }

  async function handleDelete(e: Event) {
    e.stopPropagation();
    if (!node.path || deleting) return;
    if (!confirm(`¿Eliminar "${node.name}" del índice?`)) return;
    deleting = true;
    try {
      await deleteDocument(node.path);
      toasts.success(`"${node.name}" eliminado`);
      // Deselect if this was the selected doc
      if ($selectedPath === node.path) {
        selectedDocument.clear();
      }
      onMetadataSaved();
    } catch (err) {
      console.error('Delete failed:', err);
      toasts.error('Error al eliminar');
    }
    deleting = false;
  }
</script>

<div>
  <!-- Node header -->
  <div class="group relative">
    <button
      class="w-full flex items-center gap-1.5 rounded-md px-2 py-1 text-left text-sm transition-colors"
      class:hover:bg-sidebar-accent={!isSelected}
      class:bg-sidebar-accent={isSelected}
      class:text-sidebar-accent-foreground={isSelected}
      class:font-medium={isSelected}
      onclick={handleClick}
    >
      <!-- Expand/collapse icon -->
      {#if !isLeaf}
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
      {:else if node.type === 'chapter'}
        <svg class="h-3.5 w-3.5 shrink-0 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
        </svg>
      {:else}
        <svg class="h-3.5 w-3.5 shrink-0 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
      {/if}

      <!-- Name -->
      <span class="truncate text-sidebar-foreground group-hover:text-sidebar-accent-foreground"
        class:text-primary={isSelected}
      >
        {node.name}
      </span>

      <!-- Count badge -->
      {#if !isLeaf}
        <span class="ml-auto text-[10px] text-muted-foreground tabular-nums">
          {node.count}
        </span>
      {/if}
    </button>

    <!-- Action buttons (visible on hover) -->
      <div class="absolute right-1 top-1/2 -translate-y-1/2 flex gap-0.5 opacity-0 group-hover:opacity-100 transition-all">
        {#if isLeaf && node.path}
          <!-- Delete button (documents only) -->
          <button
            onclick={handleDelete}
            disabled={deleting}
            class="p-1 rounded text-muted-foreground hover:text-red-500 hover:bg-accent transition-all disabled:opacity-50"
            aria-label="Eliminar"
          >
            <svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </button>
        {/if}
        <!-- Edit button -->
        <button
          onclick={handleEdit}
          class="p-1 rounded text-muted-foreground hover:text-foreground hover:bg-accent transition-all"
          aria-label="Editar"
        >
          <svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
          </svg>
        </button>
      </div>
  </div>

  <!-- Children -->
  {#if expanded && !isLeaf}
    <div class="ml-3 pl-3 border-l border-sidebar-border">
      {#each node.children as child (child.name + child.type)}
        <TreeItem node={child} {onMetadataSaved} />
      {/each}
    </div>
  {/if}
</div>

<!-- Edit Metadata Modal -->
{#if isLeaf && node.path}
  <EditMetadataModal
    bind:open={editModalOpen}
    path={node.path}
    nodeType="document"
    onSaved={onMetadataSaved}
  />
{:else if node.type === 'religion'}
  <EditMetadataModal
    bind:open={editModalOpen}
    path=""
    nodeType="religion"
    nodeName={node.name}
    onSaved={onMetadataSaved}
  />
{:else if node.type === 'book'}
  <EditMetadataModal
    bind:open={editModalOpen}
    path=""
    nodeType="book"
    nodeName={node.name}
    nodeReligion={node.religion || ''}
    onSaved={onMetadataSaved}
  />
{/if}
