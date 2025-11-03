<script>
  let { x = 0, y = 0, items = [], onClose = () => {} } = $props();
  
  function handleClick(item) {
    if (item.action) item.action();
    onClose();
  }
</script>

{#if items.length > 0}
  <div class="context-menu scale-in" style="left: {x}px; top: {y}px;" onclick={(e) => e.stopPropagation()}>
    {#each items as item}
      {#if item.divider}
        <div class="divider my-0"></div>
      {:else}
        <button class="context-menu-item {item.danger ? 'danger' : ''}" onclick={() => handleClick(item)} disabled={item.disabled}>
          {#if item.icon}<i class="bi bi-{item.icon}"></i>{/if}
          <span>{item.label}</span>
          {#if item.shortcut}<kbd>{item.shortcut}</kbd>{/if}
        </button>
      {/if}
    {/each}
  </div>
{/if}

<style>
  .context-menu { position: fixed; z-index: 1000; min-width: 200px; background: hsl(var(--b1)); border: 1px solid hsl(var(--bc) / 0.2); border-radius: var(--rounded-box); box-shadow: 0 10px 40px hsl(var(--bc) / 0.2); padding: 0.5rem; animation: fadeIn 0.15s; }
  .context-menu-item { display: flex; align-items: center; gap: 0.75rem; width: 100%; padding: 0.5rem 0.75rem; border-radius: 0.375rem; border: none; background: none; cursor: pointer; color: hsl(var(--bc)); font-size: 0.875rem; transition: all 0.15s; }
  .context-menu-item:hover:not(:disabled) { background: hsl(var(--bc) / 0.1); }
  .context-menu-item:disabled { opacity: 0.4; cursor: not-allowed; }
  .context-menu-item.danger { color: hsl(var(--er)); }
  .context-menu-item.danger:hover:not(:disabled) { background: hsl(var(--er) / 0.1); }
  .context-menu-item kbd { margin-left: auto; font-size: 0.75rem; opacity: 0.6; }
  @keyframes fadeIn { from { opacity: 0; transform: scale(0.95); } to { opacity: 1; transform: scale(1); } }
</style>

