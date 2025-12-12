<!-- UIToast.svelte - Toast notification component -->
<script>
  import { toasts, removeToast } from "../../stores/toast.js";
  import { fly } from "svelte/transition";

  const iconMap = {
    success: "bi-check-circle-fill",
    error: "bi-x-circle-fill",
    warning: "bi-exclamation-triangle-fill",
    info: "bi-info-circle-fill",
  };

  const colorMap = {
    success: "bg-green-500/90",
    error: "bg-red-500/90",
    warning: "bg-yellow-500/90",
    info: "bg-blue-500/90",
  };
</script>

<div
  class="fixed top-4 right-4 z-[9999] flex flex-col gap-2 pointer-events-none"
>
  {#each $toasts as toast (toast.id)}
    <div
      transition:fly={{ x: 300, duration: 300 }}
      class="pointer-events-auto flex items-center gap-3 px-4 py-3 rounded-lg shadow-lg backdrop-blur-sm text-white min-w-[300px] max-w-[500px] {colorMap[
        toast.type
      ]}"
    >
      <i class="bi {iconMap[toast.type]} text-xl flex-shrink-0"></i>
      <span class="flex-1 text-sm font-medium">{toast.message}</span>
      <button
        onclick={() => removeToast(toast.id)}
        class="flex-shrink-0 hover:bg-white/20 rounded p-1 transition-colors"
        aria-label="Schließen"
      >
        <i class="bi bi-x text-lg"></i>
      </button>
    </div>
  {/each}
</div>
