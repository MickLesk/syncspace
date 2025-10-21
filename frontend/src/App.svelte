<script>
  import { auth } from './stores/auth';
  import { currentTheme } from './stores/ui';
  import Login from './pages/Login.svelte';
  import Header from './components/Header.svelte';
  import Sidebar from './components/Sidebar.svelte';
  import FilesView from './pages/FilesView.svelte';
  
  // Apply theme to document
  $: {
    document.documentElement.setAttribute('data-theme', $currentTheme);
  }
</script>

{#if !$auth.isLoggedIn}
  <Login />
{:else}
  <div class="app">
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
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }
  
  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }
  
  .app-body {
    display: flex;
    flex: 1;
    overflow: hidden;
  }
  
  .main-content {
    flex: 1;
    overflow-y: auto;
    background: var(--bg);
  }
</style>
