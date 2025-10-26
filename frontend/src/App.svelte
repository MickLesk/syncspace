<script>
  import { auth } from "./stores/auth";
  import { currentTheme, currentView } from "./stores/ui";
  import Login from "./pages/Login.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  import AppHeader from "./components/AppHeader.svelte";
  import FilesView from "./pages/FilesView.svelte";
  import SharedView from "./pages/SharedView.svelte";
  import FavoritesView from "./pages/FavoritesView.svelte";
  import TrashView from "./pages/TrashView.svelte";
  import UsersView from "./pages/UsersView.svelte";
  import SettingsView from "./pages/SettingsView.svelte";
  import ProfileView from "./pages/ProfileView.svelte";
  import StorageView from "./pages/StorageView.svelte";
  import ActivityView from "./pages/ActivityView.svelte";
  import DuplicatesView from "./pages/DuplicatesView.svelte";
  import BackupView from "./pages/BackupView.svelte";
  import PerformanceView from "./pages/PerformanceView.svelte";
  import Toast from "./components/ui/Toast.svelte";

  // Apply theme to document
  $: {
    if (typeof document !== "undefined") {
      document.documentElement.setAttribute("data-theme", $currentTheme);
    }
  }

  function handleNavigate(event) {
    currentView.set(event.detail.view);
  }
</script>

{#if !$auth.isLoggedIn}
  <Login />
{:else}
  <div class="app-container">
    <Sidebar />
    <div class="main-wrapper">
      <AppHeader on:navigate={handleNavigate} />
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
        {:else if $currentView === "storage"}
          <StorageView />
        {:else if $currentView === "activity"}
          <ActivityView />
        {:else if $currentView === "duplicates"}
          <DuplicatesView />
        {:else if $currentView === "backup"}
          <BackupView />
        {:else if $currentView === "performance"}
          <PerformanceView />
        {/if}
      </main>
    </div>
  </div>
{/if}<Toast />

<style>
  @import "bootstrap-icons/font/bootstrap-icons.css";

  .app-container {
    height: 100vh;
    width: 100vw;
    display: flex;
    overflow: hidden;
    background: var(--md-sys-color-background);
  }

  .main-wrapper {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .main-content {
    flex: 1;
    overflow-y: auto;
    background: var(--md-sys-color-background);
  }
</style>
