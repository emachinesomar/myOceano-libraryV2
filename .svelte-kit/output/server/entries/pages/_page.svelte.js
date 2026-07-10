import { ab as head } from "../../chunks/index.js";
import "clsx";
import "@tauri-apps/api/core";
import { marked } from "marked";
import "../../chunks/document.js";
function DocumentViewer($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    marked.setOptions({ breaks: true, gfm: true });
    $$renderer2.push(`<div class="h-full flex flex-col">`);
    {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<div class="flex-1 flex items-center justify-center"><div class="text-center space-y-4"><div class="mx-auto h-16 w-16 rounded-2xl bg-muted flex items-center justify-center"><svg class="h-8 w-8 text-muted-foreground/40" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path></svg></div> <div><p class="text-sm text-muted-foreground">Selecciona un documento para leer</p> <p class="text-xs text-muted-foreground/60 mt-1">Usa Ctrl+K para buscar o navega el árbol lateral</p></div></div></div>`);
    }
    $$renderer2.push(`<!--]--></div>`);
  });
}
function _page($$renderer) {
  head("1uha8ag", $$renderer, ($$renderer2) => {
    $$renderer2.title(($$renderer3) => {
      $$renderer3.push(`<title>Ocean Library</title>`);
    });
  });
  $$renderer.push(`<div class="h-full">`);
  DocumentViewer($$renderer);
  $$renderer.push(`<!----></div>`);
}
export {
  _page as default
};
