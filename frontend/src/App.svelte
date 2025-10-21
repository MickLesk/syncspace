<script>
  import { auth } from "./stores/auth";
  import { currentTheme, currentView } from "./stores/ui";
  import Login from "./pages/Login.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  import FilesView from "./pages/FilesView.svelte";
  import SharedView from "./pages/SharedView.svelte";
  import FavoritesView from "./pages/FavoritesView.svelte";
  import TrashView from "./pages/TrashView.svelte";
  import UsersView from "./pages/UsersView.svelte";
  import SettingsView from "./pages/SettingsView.svelte";
  import ProfileView from "./pages/ProfileView.svelte";
  import Toast from "./components/ui/Toast.svelte";

  // Apply theme to document
  $: {
    if (typeof document !== "undefined") {
      document.documentElement.setAttribute("data-theme", $currentTheme);
    }
  }
</script>

{#if !$auth.isLoggedIn}
  <Login />
{:else}
  <div class="app-container">
    <Sidebar />
    <main class="main-content">
      {#if $currentView === "files"}
        <FilesView />
      {:else if $currentView === "shared"}
        <SharedView />
      {:else if $currentView === "favorites"}
        <FavoritesView />
      {:else if $currentView === "trash"}
        <TrashView />
      {:else if $currentView === "users"}
        <UsersView />
      {:else if $currentView === "settings"}
        <SettingsView />
      {:else if $currentView === "profile"}
        <ProfileView />
      {/if}
    </main>
  </div>
{/if}

<Toast />

<style>
  @import 'bootstrap-icons/font/bootstrap-icons.css';

  .app-container {
    height: 100vh;
    width: 100vw;
    display: flex;
    overflow: hidden;
    background: var(--md-sys-color-background);
  }

  .main-content {
    flex: 1;
    overflow-y: auto;
    background: var(--md-sys-color-background);
  }
</style>
