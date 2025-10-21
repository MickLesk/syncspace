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
    <div 
      class="nav-item" 
      class:active={$currentView === item.id}
      on:click={() => selectView(item.id)}
    >
      <span class="icon">{item.icon}</span>
      <span>{item.label}</span>
    </div>
  {/each}
</nav>

<style>
  .sidebar {
    width: 280px;
    background: var(--surface);
    border-right: 1px solid var(--border);
    overflow-y: auto;
    padding: 16px 0;
  }
  
  .nav-item {
    padding: 12px 20px;
    margin: 4px 8px;
    border-radius: 12px;
    cursor: pointer;
    font-size: 14px;
    color: var(--text-secondary);
    transition: all 0.2s;
    border-left: 3px solid transparent;
    font-weight: 500;
    display: flex;
    align-items: center;
    gap: 12px;
  }
  
  .nav-item:hover {
    background: var(--bg);
  }
  
  .nav-item.active {
    background: var(--bg);
    color: var(--primary);
    border-left-color: var(--primary);
    font-weight: 600;
  }
  
  .icon {
    font-size: 18px;
  }
</style>
