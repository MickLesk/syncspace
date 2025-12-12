<!-- UIAlert.svelte - Inline alert component -->
<script>
  let {
    type = "info",
    title = "",
    dismissible = false,
    onDismiss = () => {},
    children,
    class: className = "",
  } = $props();

  let dismissed = $state(false);

  const iconMap = {
    success: "bi-check-circle-fill",
    error: "bi-x-circle-fill",
    warning: "bi-exclamation-triangle-fill",
    info: "bi-info-circle-fill",
  };

  const colorMap = {
    success: "bg-green-500/10 border-green-500/30 text-green-300",
    error: "bg-red-500/10 border-red-500/30 text-red-300",
    warning: "bg-yellow-500/10 border-yellow-500/30 text-yellow-300",
    info: "bg-blue-500/10 border-blue-500/30 text-blue-300",
  };

  function handleDismiss() {
    dismissed = true;
    onDismiss();
  }
</script>

{#if !dismissed}
  <div
    class="flex items-start gap-3 p-4 rounded-lg border backdrop-blur-sm {colorMap[
      type
    ]} {className}"
  >
    <i class="bi {iconMap[type]} text-xl flex-shrink-0 mt-0.5"></i>

    <div class="flex-1 min-w-0">
      {#if title}
        <h4 class="font-semibold mb-1">{title}</h4>
      {/if}
      <div class="text-sm opacity-90">
        {@render children?.()}
      </div>
    </div>

    {#if dismissible}
      <button
        onclick={handleDismiss}
        class="flex-shrink-0 hover:bg-white/10 rounded p-1 transition-colors"
        aria-label="Schließen"
      >
        <i class="bi bi-x text-lg"></i>
      </button>
    {/if}
  </div>
{/if}
