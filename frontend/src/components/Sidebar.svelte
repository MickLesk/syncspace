<script>
  import { currentView } from '../stores/ui';
  
  const navItems = [
    { id: 'files', icon: '📁', label: 'Files' },
    { id: 'shared', icon: '🔗', label: 'Shared' },
    { id: 'favorites', icon: '⭐', label: 'Favorites' },
    { id: 'trash', icon: '🗑️', label: 'Trash' },
    { id: 'users', icon: '👥', label: 'Users' },
    { id: 'settings', icon: '⚙️', label: 'Settings' }
  ];
  
  function selectView(viewId) {
    currentView.set(viewId);
  }
</script>

<nav class="sidebar">
  {#each navItems as item}
    <button 
      class="nav-item" 
      class:active={$currentView === item.id}
      on:click={() => selectView(item.id)}
      type="button"
    >
      <span class="icon">{item.icon}</span>
      <span>{item.label}</span>
    </button>
  {/each}
</nav>

<style>
  .sidebar {
    width: 280px;
    background: var(--md-sys-color-surface);
    border-right: 1px solid var(--md-sys-color-outline);
    overflow-y: auto;
    padding: 12px 8px;
  }
  
  .nav-item {
    width: 100%;
    padding: 12px 16px;
    margin: 4px 0;
    border-radius: 28px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface-variant);
    transition: all 0.2s ease;
    border: none;
    background: transparent;
    display: flex;
    align-items: center;
    gap: 12px;
    text-align: left;
  }
  
  .nav-item:hover {
    background: var(--md-sys-color-secondary-container);
  }
  
  .nav-item.active {
    background: var(--md-sys-color-secondary-container);
    color: var(--md-sys-color-on-secondary-container);
    font-weight: 600;
  }
  
  .icon {
    font-size: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
  }
</style>
