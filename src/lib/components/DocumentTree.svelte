<script lang="ts">
  import type { TreeNode } from '$lib/types';

  let { nodes }: { nodes: TreeNode[] } = $props();
</script>

<div class="space-y-0.5">
  {#each nodes as node (node.name + node.type)}
    {@render treeItem(node)}
  {/each}
</div>

{#snippet treeItem(node: TreeNode)}
  <TreeItem {node} />
{/snippet}
