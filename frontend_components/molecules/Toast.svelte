<script lang="ts">
  import type { ToastType, ToastPosition } from "../shared/index.js";

  interface Toast {
    id: string;
    message: string;
    type?: ToastType;
    duration?: number;
  }

  interface Props {
    position?: ToastPosition;
  }

  let { position = "bottom-right" }: Props = $props();

  let toasts: Toast[] = $state([]);

  const typeColors = {
    success: "bg-green-500",
    error: "bg-red-500",
    warning: "bg-yellow-500",
    info: "bg-blue-500",
  };

  const positionClasses = {
    "top-left": "top-4 left-4",
    "top-center": "top-4 left-1/2 -translate-x-1/2",
    "top-right": "top-4 right-4",
    "bottom-left": "bottom-4 left-4",
    "bottom-center": "bottom-4 left-1/2 -translate-x-1/2",
    "bottom-right": "bottom-4 right-4",
  };

  export function show(
    message: string,
    type: ToastType = "info",
    duration = 3000
  ) {
    const id = `toast-${Date.now()}`;
    toasts = [...toasts, { id, message, type, duration }];

    if (duration > 0) {
      setTimeout(() => {
        toasts = toasts.filter((t) => t.id !== id);
      }, duration);
    }

    return id;
  }

  export function hide(id: string) {
    toasts = toasts.filter((t) => t.id !== id);
  }
</script>

<div class={`fixed ${positionClasses[position]} z-50 flex flex-col gap-2`}>
  {#each toasts as toast (toast.id)}
    <div
      class={`
        ${typeColors[toast.type || "info"]} text-white px-6 py-3 rounded-lg shadow-lg
        animate-in fade-in slide-in-from-bottom-5 duration-300
      `}
      role="alert"
    >
      <p class="text-sm font-medium">{toast.message}</p>
    </div>
  {/each}
</div>
