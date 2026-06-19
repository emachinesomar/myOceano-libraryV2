<script lang="ts">
  import { toasts, type Toast } from '$lib/stores/toast';

  function getIcon(type: Toast['type']) {
    switch (type) {
      case 'error':
        return 'M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z';
      case 'success':
        return 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z';
      case 'info':
        return 'M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z';
    }
  }

  function getColors(type: Toast['type']) {
    switch (type) {
      case 'error':
        return 'border-destructive/50 bg-destructive/10 text-destructive';
      case 'success':
        return 'border-green-500/50 bg-green-500/10 text-green-600 dark:text-green-400';
      case 'info':
        return 'border-primary/50 bg-primary/10 text-primary';
    }
  }
</script>

<!-- Toast container -->
<div class="fixed bottom-4 right-4 z-[100] flex flex-col gap-2 max-w-sm">
  {#each $toasts as toast (toast.id)}
    <div
      class="flex items-start gap-3 rounded-lg border px-4 py-3 shadow-lg backdrop-blur-sm animate-slide-in-right {getColors(toast.type)}"
      role="alert"
    >
      <svg class="h-4 w-4 mt-0.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={getIcon(toast.type)} />
      </svg>
      <p class="text-sm flex-1">{toast.message}</p>
      <button
        class="shrink-0 text-current opacity-60 hover:opacity-100 transition-opacity"
        onclick={() => toasts.remove(toast.id)}
        aria-label="Cerrar"
      >
        <svg class="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
  {/each}
</div>
