import { w as writable } from "./index2.js";
function createDocumentStore() {
  const { subscribe, set } = writable(null);
  return {
    subscribe,
    select(path, snippet) {
      set({ path, snippet });
    },
    clear() {
      set(null);
    }
  };
}
createDocumentStore();
