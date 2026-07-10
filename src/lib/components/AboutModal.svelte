<script lang="ts">
  let { open = $bindable(false) }: { open: boolean } = $props();

  const app = {
    name: 'Ocean Library v2',
    version: '0.1.0',
    description:
      'Un gestor personal de bibliotecas de textos sagrados, diseñado para indexar, organizar y buscar documentos religiosos con rapidez y precisión.',
    repo: 'https://github.com/emachinesomar/myOceano-libraryV2',
    author: 'Omar',
    year: '2026',
  };

  const features = [
    {
      title: '🔍 Búsqueda semántica FTS5',
      desc: 'SQLite FTS5 con unicode61 + remove_diacritics. Busca en español sin acentos, con AND/OR/NOT, más LIKE en rutas de archivo.',
    },
    {
      title: '🤖 Clasificación automática',
      desc: 'Reconoce 40+ palabras clave bahá\'íes y patrones (Ridván, CUJ, Ruhí) para inferir religión, libro y capítulo al instante.',
    },
    {
      title: '🌳 Árbol jerárquico inteligente',
      desc: 'Religión → Libro → Capítulo/Documento. Los Ruhí se agrupan por "Libro X" automáticamente.',
    },
    {
      title: '✏️ Edición en lote',
      desc: 'Renombrá una religión o libro entero y todos los documentos se actualizan en una sola operación.',
    },
    {
      title: '🛡️ Resistente a PDFs corruptos',
      desc: 'Si pdf-extract falla, prueba con PyMuPDF. Si ambos fallan, igual indexa el archivo para que aparezca en el árbol.',
    },
    {
      title: '🌙 Modo oscuro + accesible',
      desc: 'Dark/light mode, atajos de teclado (Ctrl+K, Escape), navegación completa sin mouse.',
    },
  ];

  const stack = [
    { icon: '🦀', label: 'Backend', value: 'Rust + Tauri v2' },
    { icon: '⚡', label: 'Frontend', value: 'Svelte 5 + SvelteKit + Tailwind CSS 3' },
    { icon: '🗄️', label: 'Base de datos', value: 'SQLite + FTS5 (unicode61 remove_diacritics 2)' },
    { icon: '📄', label: 'Extracción PDF', value: 'pdf-extract + PyMuPDF (fallback)' },
    { icon: '📦', label: 'Build', value: 'Vite 6 + pnpm + adapter-static' },
  ];

  const philosophy = [
    'Local-first: tus documentos nunca salen de tu máquina. Sin cloud, sin telemetría.',
    'Rápido por diseño: Rust para backend pesado, Svelte para UI reactiva sin Virtual DOM.',
    'Sin hinchazón: SQLite embebido, sin servidor, sin contenedores. Un solo .exe.',
    'Open source: código en GitHub para que cualquiera lo entienda y lo mejore.',
  ];

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) open = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') open = false;
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- Backdrop -->
  <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
    onclick={handleBackdropClick}
  >
    <!-- Modal -->
    <div
      class="relative w-full max-w-3xl rounded-xl border border-border bg-card p-6 shadow-2xl"
      role="dialog"
      aria-modal="true"
      aria-label="Acerca de Ocean Library"
    >
      <!-- Close button -->
      <button
        class="absolute right-4 top-4 inline-flex items-center justify-center rounded-md p-1.5 text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
        onclick={() => (open = false)}
        aria-label="Cerrar"
      >
        <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>

      <!-- ── Header + Description row ── -->
      <div class="flex items-start gap-4 mb-5">
        <div class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-primary/10">
          <svg class="h-6 w-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
          </svg>
        </div>
        <div class="flex-1">
          <div class="flex items-baseline gap-2">
            <h2 class="text-xl font-bold text-card-foreground">{app.name}</h2>
            <span class="text-xs text-muted-foreground">v{app.version}</span>
          </div>
          <p class="text-sm text-card-foreground mt-1 leading-relaxed">{app.description}</p>
        </div>
      </div>

      <!-- ── Features grid ── -->
      <section class="mb-4">
        <h3 class="text-xs font-semibold uppercase tracking-wider text-muted-foreground mb-2">
          Características
        </h3>
        <div class="grid grid-cols-2 gap-2">
          {#each features as feat}
            <div class="rounded-lg border border-border bg-muted/30 p-2.5">
              <h4 class="text-sm font-semibold text-card-foreground">{feat.title}</h4>
              <p class="text-xs text-muted-foreground leading-relaxed mt-0.5">{feat.desc}</p>
            </div>
          {/each}
        </div>
      </section>

      <!-- ── Bottom row: Stack + Philosophy + Reqs ── -->
      <div class="grid grid-cols-3 gap-4 mb-4">
        <!-- Stack -->
        <section>
          <h3 class="text-xs font-semibold uppercase tracking-wider text-muted-foreground mb-2">
            Stack
          </h3>
          <div class="space-y-1.5">
            {#each stack as item}
              <div class="flex items-center gap-2 text-xs">
                <span>{item.icon}</span>
                <span class="text-muted-foreground shrink-0 w-16">{item.label}</span>
                <span class="text-card-foreground">{item.value}</span>
              </div>
            {/each}
          </div>
        </section>

        <!-- Philosophy -->
        <section>
          <h3 class="text-xs font-semibold uppercase tracking-wider text-muted-foreground mb-2">
            Filosofía
          </h3>
          <ul class="space-y-1.5">
            {#each philosophy as point}
              <li class="flex items-start gap-1.5 text-xs text-card-foreground">
                <span class="mt-0.5 shrink-0 text-primary">◆</span>
                <span>{point}</span>
              </li>
            {/each}
          </ul>
        </section>

        <!-- Requirements -->
        <section>
          <h3 class="text-xs font-semibold uppercase tracking-wider text-muted-foreground mb-2">
            Requisitos
          </h3>
          <div class="rounded-lg bg-muted/50 p-2.5 space-y-1.5">
            <p class="text-xs text-muted-foreground leading-relaxed">
              <span class="font-medium text-card-foreground">PyMuPDF</span> solo para PDFs corruptos que
              <code class="rounded bg-background px-1 py-0.5 text-[10px] font-mono">pdf-extract</code>
              no puede leer.
            </p>
            <code class="block rounded bg-background px-2 py-1 text-[11px] font-mono text-card-foreground">pip install PyMuPDF</code>
          </div>
        </section>
      </div>

      <!-- ── Footer ── -->
      <div class="flex items-center justify-between border-t border-border pt-4">
        <a
          href={app.repo}
          target="_blank"
          rel="noopener noreferrer"
          class="inline-flex items-center gap-1.5 text-xs text-muted-foreground hover:text-primary transition-colors"
        >
          <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
          </svg>
          GitHub
        </a>
        <span class="text-xs text-muted-foreground">&copy; {app.year} {app.author}</span>
      </div>
    </div>
  </div>
{/if}
