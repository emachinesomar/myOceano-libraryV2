import { writable } from 'svelte/store';

export interface Toast {
  id: number;
  message: string;
  type: 'error' | 'success' | 'info';
  duration?: number;
}

let nextId = 0;

function createToastStore() {
  const { subscribe, update } = writable<Toast[]>([]);

  function add(message: string, type: Toast['type'] = 'info', duration = 5000) {
    const id = nextId++;
    update((toasts) => [...toasts, { id, message, type, duration }]);

    if (duration > 0) {
      setTimeout(() => remove(id), duration);
    }
    return id;
  }

  function remove(id: number) {
    update((toasts) => toasts.filter((t) => t.id !== id));
  }

  function error(message: string, duration = 6000) {
    return add(message, 'error', duration);
  }

  function success(message: string, duration = 4000) {
    return add(message, 'success', duration);
  }

  function info(message: string, duration = 4000) {
    return add(message, 'info', duration);
  }

  return { subscribe, add, remove, error, success, info };
}

export const toasts = createToastStore();
