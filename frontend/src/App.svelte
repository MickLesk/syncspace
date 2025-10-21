<script>
  import { auth } from './stores/auth';
  import { currentTheme } from './stores/ui';
  import Login from './pages/Login.svelte';
  import Header from './components/Header.svelte';
  import Sidebar from './components/Sidebar.svelte';
  import FilesView from './pages/FilesView.svelte';
  
  // Apply theme to document
  $: {
    if (typeof document !== 'undefined') {
      document.documentElement.setAttribute('data-theme', $currentTheme);
    }
  }
</script>

{#if !$auth.isLoggedIn}
  <Login />
{:else}
  <div class="app-container">
    <Header />
    <div class="app-body">
      <Sidebar />
      <main class="main-content">
        <FilesView />
      </main>
    </div>
  </div>
{/if}

<style>
  .app-container {
    height: 100vh;
    width: 100vw;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: var(--md-sys-color-background);
  }
  
  .app-body {
    flex: 1;
    display: flex;
    overflow: hidden;
  }
  
  .main-content {
    flex: 1;
    overflow-y: auto;
    background: var(--md-sys-color-background);
  }
</style>
