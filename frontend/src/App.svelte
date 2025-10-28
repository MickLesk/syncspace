<script>
  import { onMount } from "svelte";
  import { auth } from "./stores/auth";
  import { currentTheme, currentView, currentLang } from "./stores/ui";
  import { userPreferences } from "./stores/preferences";

  // Auth Views
  import Login from "./pages/auth/Login.svelte";
  import Signup from "./pages/auth/Signup.svelte";

  // Navigation & Header
  import Sidebar from "./components/navigation/Sidebar.svelte";
  import AppHeader from "./components/ui/AppHeader.svelte";

  // File Views
  import FilesView from "./pages/files/FilesView.svelte";
  import SharedView from "./pages/files/SharedView.svelte";
  import FavoritesView from "./pages/files/FavoritesView.svelte";
  import RecentFilesView from "./pages/files/RecentFilesView.svelte";

  // System Views
  import TrashView from "./pages/system/TrashView.svelte";
  import ActivityView from "./pages/system/ActivityView.svelte";
  import NotificationsView from "./pages/system/NotificationsView.svelte";
  import BackupView from "./pages/system/BackupView.svelte";
  import StorageView from "./pages/system/StorageView.svelte";
  import UsersView from "./pages/system/UsersView.svelte";

  // Tools Views
  import DuplicatesView from "./pages/tools/DuplicatesView.svelte";
  import PerformanceView from "./pages/tools/PerformanceView.svelte";

  // User & Settings Views
  import SettingsView from "./pages/settings/SettingsView.svelte";
  import ProfileView from "./pages/user/ProfileView.svelte";

  // Showcase (Development)
  import DesignShowcase from "./pages/showcase/DesignShowcase.svelte";

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
