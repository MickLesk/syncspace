<script>
  /**
   * Breadcrumb Component
   * Wiederverwendbare Navigation für hierarchische Strukturen
   * 
   * @component
   * @example
   * <Breadcrumb
   *   items={[
   *     { name: "Home", path: "/" },
   *     { name: "Documents", path: "/documents/" },
   *     { name: "Projects", path: "/documents/projects/" }
   *   ]}
   *   onNavigate={(path) => currentPath.set(path)}
   * />
   */
  
  import Icon from "./Icon.svelte";

  /** @type {Array<{name: string, path: string}>} - Breadcrumb items */
  export let items = [];
  
  /** @type {Function} - Callback wenn auf Breadcrumb geklickt wird */
  export let onNavigate = null;
  
  /** @type {string} - Separator zwischen Items */
  export let separator = "/";
  
  /** @type {boolean} - Zeigt Home-Icon für erstes Element */
  export let showHomeIcon = true;
  
  /** @type {"small" | "medium" | "large"} - Größe der Breadcrumbs */
  export let size = "medium";

  function handleClick(item, index) {
    if (index === items.length - 1) return; // Aktives Element nicht klickbar
    
    if (onNavigate) {
      onNavigate(item.path);
    }
  }
</script>

<nav class="breadcrumb-nav" class:size-small={size === "small"} class:size-large={size === "large"} aria-label="Breadcrumb">
  {#each items as item, i}
    {#if i > 0}
      <span class="breadcrumb-separator" aria-hidden="true">{separator}</span>
    {/if}
    <button
      class="breadcrumb-item"
      class:active={i === items.length - 1}
      onclick={() => handleClick(item, i)}
      disabled={i === items.length - 1}
      aria-current={i === items.length - 1 ? "page" : undefined}
      type="button"
    >
      {#if i === 0 && showHomeIcon}
        <Icon name="house-fill" size={size === "small" ? 12 : size === "large" ? 16 : 14} />
      {/if}
      <span class="breadcrumb-text">{item.name}</span>
    </button>
  {/each}
</nav>

<style>
  .breadcrumb-nav {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 8px;
    padding: 12px 16px;
    background: rgba(255, 255, 255, 0.5);
    backdrop-filter: blur(10px);
    border-radius: 12px;
    border: 1px solid rgba(0, 0, 0, 0.06);
    margin-bottom: 20px;
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  }

  :global([data-theme="dark"]) .breadcrumb-nav {
    background: rgba(30, 30, 38, 0.6);
    border-color: rgba(255, 255, 255, 0.08);
  }

  .breadcrumb-nav.size-small {
    padding: 8px 12px;
    gap: 6px;
    font-size: 13px;
  }

  .breadcrumb-nav.size-large {
    padding: 16px 20px;
    gap: 10px;
    font-size: 16px;
  }

  .breadcrumb-item {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background: transparent;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
    color: var(--md-sys-color-primary);
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    white-space: nowrap;
    font-family: inherit;
  }

  .size-small .breadcrumb-item {
    padding: 4px 8px;
    gap: 4px;
    font-size: 13px;
  }

  .size-large .breadcrumb-item {
    padding: 8px 16px;
    gap: 8px;
    font-size: 16px;
  }

  .breadcrumb-item:hover:not(:disabled) {
    background: rgba(99, 102, 241, 0.1);
    color: var(--md-sys-color-primary);
  }

  .breadcrumb-item:disabled,
  .breadcrumb-item.active {
    cursor: default;
    color: var(--md-sys-color-on-surface);
    font-weight: 600;
  }

  .breadcrumb-item:disabled:hover,
  .breadcrumb-item.active:hover {
    background: transparent;
  }

  .breadcrumb-separator {
    color: var(--md-sys-color-outline);
    font-size: 14px;
    user-select: none;
    opacity: 0.6;
  }

  .size-small .breadcrumb-separator {
    font-size: 12px;
  }

  .size-large .breadcrumb-separator {
    font-size: 16px;
  }

  .breadcrumb-text {
    max-width: 200px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .size-small .breadcrumb-text {
    max-width: 150px;
  }

  .size-large .breadcrumb-text {
    max-width: 250px;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .breadcrumb-nav {
      padding: 8px 12px;
      gap: 6px;
    }

    .breadcrumb-item {
      padding: 4px 8px;
      font-size: 13px;
    }

    .breadcrumb-text {
      max-width: 100px;
    }

    /* Show only last 2 breadcrumbs on mobile */
    .breadcrumb-item:not(:nth-last-child(-n+4)) {
      display: none;
    }

    .breadcrumb-separator:not(:nth-last-child(-n+3)) {
      display: none;
    }

    /* Add ellipsis indicator */
    .breadcrumb-nav::before {
      content: "...";
      color: var(--md-sys-color-outline);
      padding: 0 4px;
      order: -1;
    }
  }

  /* Animation */
  .breadcrumb-item {
    animation: fadeIn 0.2s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateX(-10px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }
</style>
