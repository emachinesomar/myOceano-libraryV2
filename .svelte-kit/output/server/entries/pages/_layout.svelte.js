import { a4 as attr_class, e as escape_html, a5 as ensure_array_like, a6 as attr, a7 as bind_props, a8 as store_get, a9 as stringify, aa as unsubscribe_stores } from "../../chunks/index.js";
import "@tauri-apps/api/core";
import { w as writable } from "../../chunks/index2.js";
import "../../chunks/document.js";
function html(value) {
  var html2 = String(value ?? "");
  var open = "<!---->";
  return open + html2 + "<!---->";
}
let nextId = 0;
function createToastStore() {
  const { subscribe, update } = writable([]);
  function add(message, type = "info", duration = 5e3) {
    const id = nextId++;
    update((toasts2) => [...toasts2, { id, message, type, duration }]);
    if (duration > 0) {
      setTimeout(() => remove(id), duration);
    }
    return id;
  }
  function remove(id) {
    update((toasts2) => toasts2.filter((t) => t.id !== id));
  }
  function error(message, duration = 6e3) {
    return add(message, "error", duration);
  }
  function success(message, duration = 4e3) {
    return add(message, "success", duration);
  }
  function info(message, duration = 4e3) {
    return add(message, "info", duration);
  }
  return { subscribe, add, remove, error, success, info };
}
const toasts = createToastStore();
function TreeItem($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { node } = $$props;
    let expanded = false;
    $$renderer2.push(`<div><button class="w-full flex items-center gap-1.5 rounded-md px-2 py-1 text-left text-sm transition-colors hover:bg-sidebar-accent group">`);
    if (node.children.length > 0) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<svg${attr_class("h-3 w-3 shrink-0 text-muted-foreground transition-transform duration-200", void 0, { "rotate-90": expanded })} fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path></svg>`);
    } else {
      $$renderer2.push("<!--[-1-->");
      $$renderer2.push(`<span class="w-3 shrink-0"></span>`);
    }
    $$renderer2.push(`<!--]--> `);
    if (node.type === "religion") {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<svg class="h-3.5 w-3.5 shrink-0 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>`);
    } else if (node.type === "book") {
      $$renderer2.push("<!--[1-->");
      $$renderer2.push(`<svg class="h-3.5 w-3.5 shrink-0 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path></svg>`);
    } else {
      $$renderer2.push("<!--[-1-->");
      $$renderer2.push(`<svg class="h-3.5 w-3.5 shrink-0 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>`);
    }
    $$renderer2.push(`<!--]--> <span class="truncate text-sidebar-foreground group-hover:text-sidebar-accent-foreground">${escape_html(node.name)}</span> `);
    if (node.children.length > 0) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<span class="ml-auto text-[10px] text-muted-foreground tabular-nums">${escape_html(node.count)}</span>`);
    } else {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--></button> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--></div>`);
  });
}
function treeItem($$renderer, node) {
  TreeItem($$renderer, { node });
}
function DocumentTree($$renderer, $$props) {
  let { nodes } = $$props;
  $$renderer.push(`<div class="space-y-0.5"><!--[-->`);
  const each_array = ensure_array_like(nodes);
  for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
    let node = each_array[$$index];
    treeItem($$renderer, node);
  }
  $$renderer.push(`<!--]--></div>`);
}
function AppSidebar($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { open = true } = $$props;
    let tree = [];
    let indexing = false;
    $$renderer2.push(`<aside${attr_class("flex flex-col border-r border-sidebar-border bg-sidebar text-sidebar-foreground transition-all duration-300", void 0, { "w-64": open, "w-0": !open, "overflow-hidden": !open })}>`);
    if (open) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<div class="flex items-center justify-between px-4 py-3 border-b border-sidebar-border shrink-0"><div class="flex items-center gap-2"><div class="h-6 w-6 rounded-md bg-primary flex items-center justify-center"><svg class="h-3.5 w-3.5 text-primary-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path></svg></div> <span class="text-sm font-semibold text-sidebar-foreground">Ocean Library</span></div></div> <div class="px-3 py-2 border-b border-sidebar-border shrink-0"><button class="w-full inline-flex items-center justify-center gap-2 rounded-lg bg-primary/10 px-3 py-2 text-xs font-medium text-primary hover:bg-primary/20 transition-colors"${attr("disabled", indexing, true)}>`);
      {
        $$renderer2.push("<!--[-1-->");
        $$renderer2.push(`<svg class="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path></svg> Indexar carpeta`);
      }
      $$renderer2.push(`<!--]--></button> `);
      if (tree.length > 0) {
        $$renderer2.push("<!--[0-->");
        $$renderer2.push(`<button class="w-full mt-2 inline-flex items-center justify-center gap-2 rounded-lg bg-destructive/10 px-3 py-1.5 text-xs font-medium text-destructive hover:bg-destructive/20 transition-colors"><svg class="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg> Limpiar índice</button>`);
      } else {
        $$renderer2.push("<!--[-1-->");
      }
      $$renderer2.push(`<!--]--></div> <div class="flex-1 overflow-auto smooth-scroll px-1 py-2">`);
      if (tree.length === 0) {
        $$renderer2.push("<!--[1-->");
        $$renderer2.push(`<div class="px-3 py-8 text-center"><p class="text-xs text-muted-foreground">Sin documentos indexados</p> <p class="text-[10px] text-muted-foreground/60 mt-1">Indexa una carpeta para comenzar</p></div>`);
      } else {
        $$renderer2.push("<!--[-1-->");
        DocumentTree($$renderer2, { nodes: tree });
      }
      $$renderer2.push(`<!--]--></div> <div class="border-t border-sidebar-border px-4 py-2 shrink-0"><p class="text-[10px] text-muted-foreground/50 text-center">${escape_html(tree.reduce((acc, r) => acc + r.count, 0))} libros · ${escape_html(tree.length)} religiones</p> `);
      {
        $$renderer2.push("<!--[-1-->");
      }
      $$renderer2.push(`<!--]--></div>`);
    } else {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--></aside>`);
    bind_props($$props, { open });
  });
}
function SearchCommand($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { open = false } = $$props;
    let query = "";
    let results = [];
    let selectedIndex = 0;
    if (open) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<div class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm" role="presentation" tabindex="-1"></div> <div class="fixed inset-0 z-50 flex items-start justify-center pt-[20vh]"><div class="w-full max-w-lg bg-background border border-border rounded-xl shadow-2xl overflow-hidden" role="dialog" aria-label="Buscar documentos" tabindex="0"><div class="flex items-center gap-2 px-4 border-b border-border"><input${attr("value", query)} type="text" placeholder="Buscar en textos sagrados..." class="flex-1 bg-transparent py-3 text-sm text-foreground placeholder-muted-foreground focus:outline-none"/> `);
      {
        $$renderer2.push("<!--[-1-->");
        $$renderer2.push(`<button${attr("disabled", query.trim().length < 2, true)} class="p-1.5 rounded-md text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-40 disabled:cursor-not-allowed transition-colors shrink-0" aria-label="Buscar"><svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg></button>`);
      }
      $$renderer2.push(`<!--]--></div> `);
      if (results.length > 0) {
        $$renderer2.push("<!--[0-->");
        $$renderer2.push(`<div class="max-h-80 overflow-auto smooth-scroll py-2"><!--[-->`);
        const each_array = ensure_array_like(results);
        for (let i = 0, $$length = each_array.length; i < $$length; i++) {
          let result = each_array[i];
          $$renderer2.push(`<button${attr_class("w-full flex flex-col gap-1 px-4 py-2.5 text-left transition-colors", void 0, { "bg-accent": i === selectedIndex })}><div class="flex items-center gap-2">`);
          if (result.religion) {
            $$renderer2.push("<!--[0-->");
            $$renderer2.push(`<span class="text-[10px] font-medium text-primary bg-primary/10 px-1.5 py-0.5 rounded">${escape_html(result.religion)}</span>`);
          } else {
            $$renderer2.push("<!--[-1-->");
          }
          $$renderer2.push(`<!--]--> `);
          if (result.book) {
            $$renderer2.push("<!--[0-->");
            $$renderer2.push(`<span class="text-[10px] text-muted-foreground">${escape_html(result.book)}</span>`);
          } else {
            $$renderer2.push("<!--[-1-->");
          }
          $$renderer2.push(`<!--]--></div> <span class="text-xs text-foreground font-medium truncate">${escape_html(result.title || result.path.split(/[/\\]/).pop())}</span> <span class="text-[11px] text-muted-foreground line-clamp-3">${html(result.paragraph || result.snippet)}</span></button>`);
        }
        $$renderer2.push(`<!--]--></div>`);
      } else {
        $$renderer2.push("<!--[-1-->");
      }
      $$renderer2.push(`<!--]--> <div class="flex items-center justify-between px-4 py-2 border-t border-border bg-muted/30"><div class="flex items-center gap-3 text-[10px] text-muted-foreground"><span><kbd class="px-1 py-0.5 bg-background border border-border rounded text-[9px]">↑↓</kbd> navegar</span> <span><kbd class="px-1 py-0.5 bg-background border border-border rounded text-[9px]">↵</kbd> buscar/abrir</span> <span><kbd class="px-1 py-0.5 bg-background border border-border rounded text-[9px]">esc</kbd> cerrar</span></div> `);
      if (results.length > 0) {
        $$renderer2.push("<!--[0-->");
        $$renderer2.push(`<span class="text-[10px] text-muted-foreground">${escape_html(results.length)} resultados</span>`);
      } else {
        $$renderer2.push("<!--[-1-->");
      }
      $$renderer2.push(`<!--]--></div></div></div>`);
    } else {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]-->`);
    bind_props($$props, { open });
  });
}
function Toast($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    var $$store_subs;
    function getIcon(type) {
      switch (type) {
        case "error":
          return "M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z";
        case "success":
          return "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z";
        case "info":
          return "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z";
      }
    }
    function getColors(type) {
      switch (type) {
        case "error":
          return "border-destructive/50 bg-destructive/10 text-destructive";
        case "success":
          return "border-green-500/50 bg-green-500/10 text-green-600 dark:text-green-400";
        case "info":
          return "border-primary/50 bg-primary/10 text-primary";
      }
    }
    $$renderer2.push(`<div class="fixed bottom-4 right-4 z-[100] flex flex-col gap-2 max-w-sm"><!--[-->`);
    const each_array = ensure_array_like(store_get($$store_subs ??= {}, "$toasts", toasts));
    for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
      let toast = each_array[$$index];
      $$renderer2.push(`<div${attr_class(`flex items-start gap-3 rounded-lg border px-4 py-3 shadow-lg backdrop-blur-sm animate-slide-in-right ${stringify(getColors(toast.type))}`)} role="alert"><svg class="h-4 w-4 mt-0.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"${attr("d", getIcon(toast.type))}></path></svg> <p class="text-sm flex-1">${escape_html(toast.message)}</p> <button class="shrink-0 text-current opacity-60 hover:opacity-100 transition-opacity" aria-label="Cerrar"><svg class="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg></button></div>`);
    }
    $$renderer2.push(`<!--]--></div>`);
    if ($$store_subs) unsubscribe_stores($$store_subs);
  });
}
function _layout($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { children } = $$props;
    let sidebarOpen = true;
    let commandOpen = false;
    let $$settled = true;
    let $$inner_renderer;
    function $$render_inner($$renderer3) {
      $$renderer3.push(`<div class="flex h-screen overflow-hidden bg-background text-foreground">`);
      AppSidebar($$renderer3, {
        get open() {
          return sidebarOpen;
        },
        set open($$value) {
          sidebarOpen = $$value;
          $$settled = false;
        }
      });
      $$renderer3.push(`<!----> <main class="flex-1 flex flex-col overflow-hidden"><header class="flex items-center gap-3 border-b border-border px-4 py-2.5 shrink-0"><button class="inline-flex items-center justify-center rounded-md p-1.5 text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors" aria-label="Toggle sidebar"><svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg></button> <button class="inline-flex items-center gap-2 rounded-lg border border-border bg-muted/50 px-3 py-1.5 text-sm text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"><svg class="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg> <span>Buscar...</span> <kbd class="ml-2 inline-flex items-center rounded border border-border bg-background px-1 py-0.5 text-[10px] font-medium text-muted-foreground"><span class="mr-0.5">⌘</span>K</kbd></button> <div class="flex-1"></div> <button class="inline-flex items-center justify-center rounded-md p-1.5 text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"${attr("aria-label", "Modo claro")}>`);
      {
        $$renderer3.push("<!--[0-->");
        $$renderer3.push(`<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path></svg>`);
      }
      $$renderer3.push(`<!--]--></button> <span class="text-xs text-muted-foreground font-medium tracking-wide uppercase">Ocean Library v2</span></header> <div class="flex-1 overflow-auto smooth-scroll">`);
      children($$renderer3);
      $$renderer3.push(`<!----></div></main></div> `);
      SearchCommand($$renderer3, {
        get open() {
          return commandOpen;
        },
        set open($$value) {
          commandOpen = $$value;
          $$settled = false;
        }
      });
      $$renderer3.push(`<!----> `);
      Toast($$renderer3);
      $$renderer3.push(`<!---->`);
    }
    do {
      $$settled = true;
      $$inner_renderer = $$renderer2.copy();
      $$render_inner($$inner_renderer);
    } while (!$$settled);
    $$renderer2.subsume($$inner_renderer);
  });
}
export {
  _layout as default
};
