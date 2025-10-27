<script>
  import { onMount } from "svelte";
  import { auth } from "./stores/auth";
  import { currentTheme, currentView, currentLang } from "./stores/ui";
  import { userPreferences } from "./stores/preferences";
  import Login from "./pages/Login.svelte";
  import Signup from "./pages/Signup.svelte";
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
  import NotificationsView from "./pages/NotificationsView.svelte";
  import RecentFilesView from "./pages/RecentFilesView.svelte";
  import DesignShowcase from "./pages/DesignShowcase.svelte";
  import Toast from "./components/ui/Toast.svelte";

  // Load user preferences on mount if logged in
  onMount(async () => {
    if ($auth.isLoggedIn) {
      try {
        await Promise.all([
          userPreferences.load(),
          currentTheme.loadFromBackend?.(),
          currentLang.loadFromBackend?.(),
        ]);
      } catch (err) {
        console.error("Failed to load user data:", err);
      }
    }
  });

  // Reload preferences when user logs in
  $effect(() => {
    if ($auth.isLoggedIn) {
      Promise.all([
        userPreferences.load(),
        currentTheme.loadFromBackend?.(),
        currentLang.loadFromBackend?.(),
      ]).catch((err) => {
        console.error("Failed to load user data:", err);
      });
    }
  });

  // Apply theme to document
  $effect(() => {
    if (typeof document !== "undefined") {
      document.documentElement.setAttribute("data-theme", $currentTheme);
    }
  });

  function handleNavigate(event) {
    currentView.set(event.detail.view);
  }

  // Check for hash-based routing (login vs signup)
  let showSignup = $state(false);

  onMount(() => {
    const checkRoute = () => {
      showSignup = window.location.hash === "#/signup";
    };
    checkRoute();
    window.addEventListener("hashchange", checkRoute);
    return () => window.removeEventListener("hashchange", checkRoute);
  });
</script>

{#if !$auth.isLoggedIn}
  {#if showSignup}
    <Signup />
  {:else}
    <Login />
  {/if}
{:else}
  <div class="app-container">
    <Sidebar />
    <div class="main-wrapper">
      <AppHeader on:navigate={handleNavigate} />
      <main class="main-content">
        {#if $currentView === "design"}
          <DesignShowcase />
        {:else if $currentView === "files"}
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
        {:else if $currentView === "notifications"}
          <NotificationsView />
        {:else if $currentView === "recent"}
          <RecentFilesView />
        {/if}
      </main>
    </div>
  </div>
{/if}<Toast />

<style>
  /* Bootstrap Icons loaded from CDN in index.html */

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
