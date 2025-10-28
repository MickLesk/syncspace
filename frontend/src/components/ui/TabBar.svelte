<script>
  export let tabs = []; // Array of { id, label, icon? }
  export let activeTab = "";
  export let onChange = (id) => {};
  export let variant = "pills"; // pills, underline, glass
</script>

<div class="tab-bar {variant}">
  {#each tabs as tab}
    <button
      class="tab"
      class:active={activeTab === tab.id}
      onclick={() => {
        activeTab = tab.id;
        onChange(tab.id);
      }}
    >
      {#if tab.icon}
        <i class="bi {tab.icon}"></i>
      {/if}
      <span>{tab.label}</span>
    </button>
  {/each}
</div>

<style>
  .tab-bar {
    display: flex;
    gap: 8px;
    padding: 4px;
    border-radius: 12px;
    width: fit-content;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 10px 18px;
    border: none;
    background: transparent;
    color: var(--md-sys-color-on-surface-variant);
    font-family: "Inter", sans-serif;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    white-space: nowrap;
    letter-spacing: -0.2px;
  }

  .tab i {
    font-size: 16px;
  }

  /* Pills Variant (Default) */
  .pills {
    background: rgba(15, 23, 42, 0.04);
  }

  .pills .tab {
    border-radius: 10px;
  }

  .pills .tab:hover {
    background: rgba(99, 102, 241, 0.08);
    color: var(--md-sys-color-on-surface);
  }

  .pills .tab.active {
    background: linear-gradient(135deg, #6366f1, #8b5cf6);
    color: white;
    box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
  }

  /* Underline Variant */
  .underline {
    background: transparent;
    border-bottom: 2px solid rgba(15, 23, 42, 0.08);
    border-radius: 0;
    gap: 16px;
    padding: 0;
  }

  .underline .tab {
    padding: 12px 4px;
    border-radius: 0;
    position: relative;
  }

  .underline .tab::after {
    content: "";
    position: absolute;
    bottom: -2px;
    left: 0;
    right: 0;
    height: 2px;
    background: var(--md-sys-color-primary);
    transform: scaleX(0);
    transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .underline .tab.active::after {
    transform: scaleX(1);
  }

  .underline .tab:hover {
    color: var(--md-sys-color-primary);
  }

  .underline .tab.active {
    color: var(--md-sys-color-primary);
  }

  /* Glass Variant */
  .glass {
    background: var(--glass-bg);
    backdrop-filter: blur(var(--glass-blur));
    border: 1px solid var(--glass-border);
    box-shadow: var(--glass-shadow);
  }

  .glass .tab {
    border-radius: 10px;
  }

  .glass .tab:hover {
    background: rgba(255, 255, 255, 0.3);
    color: var(--md-sys-color-on-surface);
  }

  .glass .tab.active {
    background: rgba(255, 255, 255, 0.9);
    color: var(--md-sys-color-primary);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  /* Dark Mode */
  :global(.dark) .pills {
    background: rgba(255, 255, 255, 0.04);
  }

  :global(.dark) .underline {
    border-color: rgba(255, 255, 255, 0.08);
  }

  :global(.dark) .glass {
    background: rgba(30, 41, 59, 0.85);
    border-color: rgba(255, 255, 255, 0.08);
  }

  :global(.dark) .glass .tab:hover {
    background: rgba(255, 255, 255, 0.08);
  }

  :global(.dark) .glass .tab.active {
    background: rgba(255, 255, 255, 0.12);
    color: #818cf8;
  }
</style>
