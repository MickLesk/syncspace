<script lang="ts">
  interface Props {
    open?: boolean;
    title?: string;
    size?: "sm" | "md" | "lg" | "xl";
    closeButton?: boolean;
  }

  let {
    open = false,
    title,
    size = "md",
    closeButton = true,
  }: Props = $props();

  const sizeClasses = {
    sm: "max-w-sm",
    md: "max-w-md",
    lg: "max-w-lg",
    xl: "max-w-2xl",
  };

  function close() {
    open = false;
  }
</script>

{#if open}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 bg-black/50 z-40 animate-in fade-in duration-300"
    onclick={close}
    role="presentation"
  />

  <!-- Modal -->
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4" onclick={(e) => e.stopPropagation()}>
    <div
      class={`
        bg-white rounded-xl shadow-2xl w-full ${sizeClasses[size]}
        animate-in zoom-in-95 fade-in duration-300
      `}
      role="dialog"
      aria-modal="true"
    >
      <!-- Header -->
      {#if title || closeButton}
        <div
          class="flex items-center justify-between px-6 py-4 border-b border-gray-200"
        >
          {#if title}
            <h2 class="text-lg font-semibold text-gray-900">{title}</h2>
          {/if}
          {#if closeButton}
            <button
              onclick={close}
              class="text-gray-500 hover:text-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500 rounded p-1"
              aria-label="Close modal"
            >
              <i class="bi bi-x text-xl"></i>
            </button>
          {/if}
        </div>
      {/if}

      <!-- Content -->
      <div class="px-6 py-4">
        <slot />
      </div>

      <!-- Footer -->
      <div class="px-6 py-4 border-t border-gray-200 flex justify-end gap-2">
        <slot name="footer" />
      </div>
    </div>
  </div>
{/if}
