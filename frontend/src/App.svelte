<script>
  import { onMount } from "svelte";
  import { auth } from "./stores/auth";
  import { currentTheme, currentView, currentLang } from "./stores/ui";
  import { userPreferences } from "./stores/preferences";
  import { initErrorReporting } from "./lib/errorReporting";

  // Setup & Auth Views
  import SetupWizard from "./pages/SetupWizard.svelte";
  import Login from "./pages/auth/Login.svelte";
  import Signup from "./pages/auth/Signup.svelte";

  // Navigation & Header
  import Sidebar from "./components/navigation/Sidebar.svelte";
  import AppHeader from "./components/ui/AppHeader.svelte";

  // File Views
  import FilesView from "./pages/files/FilesView.svelte";
  import SharedView from "./pages/files/SharedView.svelte";
  import RecentFilesView from "./pages/files/RecentFilesView.svelte";

  // System Views
  import TrashView from "./pages/trash/TrashView.svelte";
  import ActivityView from "./pages/system/ActivityView.svelte";
  import NotificationsView from "./pages/system/NotificationsView.svelte";
  import BackupView from "./pages/system/BackupView.svelte";
  import StorageView from "./pages/system/StorageView.svelte";
  import UsersView from "./pages/system/UsersView.svelte";

  // Tools Views
  import DuplicatesView from "./pages/tools/DuplicatesView.svelte";

  // Jobs & System Management
  import JobsDashboard from "./pages/JobsDashboard.svelte";

  // User & Settings Views
  import SettingsView from "./pages/settings/SettingsView.svelte";
  import UserProfileView from "./pages/user/UserProfileView.svelte";
  import UserSettingsView from "./pages/user/UserSettingsView.svelte";
  import SecurityView from "./pages/user/SecurityView.svelte";

  // Showcase (Development)
  import DesignShowcase from "./pages/showcase/DesignShowcase.svelte";

  // UI Components
  import Toast from "./components/ui/Toast.svelte";
  import LoadingOverlay from "./components/LoadingOverlay.svelte";
  import ErrorBoundary from "./components/ui/ErrorBoundary.svelte";
  import ActivityFeed from "./components/ActivityFeed.svelte";
  import ModalPortal from "./components/modals/ModalPortal.svelte";

  // State für Setup-Check
  let setupCompleted = $state(null); // null = loading, true = completed, false = needs setup
  let setupCheckDone = $state(false);

  // Check if setup is needed
  async function checkSetupStatus() {
    try {
      const response = await fetch("http://localhost:8080/api/setup/status");
      if (response.ok) {
        const data = await response.json();
        setupCompleted = data.setup_completed;
      } else {
        setupCompleted = true; // Assume setup done if error
      }
    } catch (e) {
      console.error("Setup status check failed:", e);
      setupCompleted = true; // Assume setup done if backend unreachable
    } finally {
      setupCheckDone = true;
    }
  }

  // FORCE LIGHT MODE ON APP START - aber erlaube später umschalten
  onMount(async () => {
    // Initialize error reporting
    initErrorReporting();
    console.log("✅ Error reporting initialized");

    // Check setup status first
    await checkSetupStatus();

    // Validate auth token
    await auth.validateToken();

    // Initialisiere mit Light-Mode
    if (!localStorage.getItem("theme")) {
      localStorage.setItem("theme", "light");
      document.documentElement.classList.remove("dark");
      document.documentElement.setAttribute("data-theme", "light");
    }
  });

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

  // Apply theme to document - Tailwind v4 compatible
  $effect(() => {
    if (typeof document !== "undefined") {
      const isDark =
        $currentTheme === "dark" || $currentTheme === "syncspace-dark";

      const html = document.documentElement;

      // Remove all theme classes first
      html.classList.remove("dark", "light");

      // Set the appropriate class for Tailwind v4
      if (isDark) {
        html.classList.add("dark");
        html.setAttribute("data-theme", "dark");
        html.style.colorScheme = "dark";
      } else {
        html.classList.add("light");
        html.setAttribute("data-theme", "light");
        html.style.colorScheme = "light";
      }

      console.log(
        `[App.svelte] Theme applied: ${isDark ? "dark" : "light"}, classes: ${html.className}`
      );
    }
  });

  function handleNavigate(event) {
    console.log("[App] handleNavigate event.detail:", event.detail);
    // Support both event.detail.view and event.detail (string)
    const view =
      typeof event.detail === "string" ? event.detail : event.detail.view;
    console.log("[App] Setting currentView to:", view);
    currentView.set(view);
  }

  // Check for hash-based routing (login vs signup)
  let showSignup = $state(false);
  let isMobileMenuOpen = $state(false);
  let isMobile = $state(false);
  let showActivityFeed = $state(false); // Collapsible Activity Feed

  onMount(() => {
    const checkRoute = () => {
      showSignup = window.location.hash === "#/signup";
    };
    checkRoute();
    window.addEventListener("hashchange", checkRoute);

    // Check if mobile
    const checkMobile = () => {
      isMobile = window.innerWidth < 768;
      if (!isMobile) isMobileMenuOpen = false;
    };
    checkMobile();
    window.addEventListener("resize", checkMobile);

    return () => {
      window.removeEventListener("hashchange", checkRoute);
      window.removeEventListener("resize", checkMobile);
    };
  });
</script>

{#if !setupCheckDone || $auth.isValidating}
  <!-- Show loading while checking setup status or validating token -->
  <div
    class="flex items-center justify-center h-screen bg-gray-50 dark:bg-gray-900"
  >
    <div class="text-center">
      <div
        class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-500 mx-auto mb-4"
      ></div>
      <p class="text-gray-600 dark:text-gray-400">Loading...</p>
    </div>
  </div>
{:else if setupCompleted === false}
  <!-- Show Setup Wizard if setup not completed -->
  <SetupWizard />
{:else if !$auth.isLoggedIn}
  {#if showSignup}
    <Signup />
  {:else}
    <Login />
  {/if}
{:else}
  <div
    class="app-container bg-gray-50 dark:bg-gray-900 transition-colors overflow-x-hidden"
  >
    <!-- Mobile Menu Toggle -->
    {#if isMobile}
      <button
        onclick={() => (isMobileMenuOpen = !isMobileMenuOpen)}
        class="mobile-menu-toggle fixed top-4 left-4 z-50 p-2 bg-white dark:bg-gray-800 rounded-lg shadow-lg md:hidden"
        aria-label="Toggle menu"
      >
        <span class="text-2xl">{isMobileMenuOpen ? "✕" : "☰"}</span>
      </button>
    {/if}

    <!-- Sidebar with mobile support -->
    <div
      class="sidebar-wrapper"
      class:mobile-open={isMobileMenuOpen && isMobile}
      class:mobile-closed={!isMobileMenuOpen && isMobile}
    >
      <Sidebar />
    </div>

    <!-- Overlay for mobile -->
    {#if isMobile && isMobileMenuOpen}
      <button
        onclick={() => (isMobileMenuOpen = false)}
        class="mobile-overlay fixed inset-0 bg-black bg-opacity-50 z-30"
        aria-label="Close menu"
      ></button>
    {/if}

    <div class="main-wrapper overflow-x-hidden">
      <AppHeader
        on:navigate={handleNavigate}
        on:toggleActivityFeed={(e) => (showActivityFeed = e.detail.visible)}
        bind:showActivityFeed
      />

      <div style="display: flex; height: 100%; overflow-x: hidden;">
        <div style="flex: 1; min-width: 0; overflow-x: hidden;">
          <ErrorBoundary>
            <main
              class="main-content bg-gray-50 dark:bg-gray-900 overflow-x-hidden"
            >
              {#if $currentView === "design"}
                <DesignShowcase />
              {:else if $currentView === "files"}
                <FilesView />
              {:else if $currentView === "shared"}
                <SharedView />
              {:else if $currentView === "trash"}
                <TrashView />
              {:else if $currentView === "users"}
                <UsersView />
              {:else if $currentView === "settings"}
                <SettingsView />
              {:else if $currentView === "profile"}
                <UserProfileView />
              {:else if $currentView === "user-settings"}
                <UserSettingsView />
              {:else if $currentView === "security"}
                <SecurityView />
              {:else if $currentView === "storage"}
                <StorageView />
              {:else if $currentView === "activity"}
                <ActivityView />
              {:else if $currentView === "duplicates"}
                <DuplicatesView />
              {:else if $currentView === "backup"}
                <BackupView />
              {:else if $currentView === "jobs"}
                <JobsDashboard />
              {:else if $currentView === "notifications"}
                <NotificationsView />
              {:else if $currentView === "recent"}
                <RecentFilesView />
              {/if}
            </main>
          </ErrorBoundary>
        </div>

        <!-- Modern Activity Feed Slide-In Panel -->
        {#if showActivityFeed}
          <!-- Backdrop with blur -->
          <div
            class="fixed inset-0 bg-black/20 dark:bg-black/40 backdrop-blur-sm z-[45] animate-fade-in"
            role="button"
            tabindex="0"
            onclick={() => (showActivityFeed = false)}
            onkeydown={(e) => e.key === "Escape" && (showActivityFeed = false)}
          ></div>

          <!-- Panel -->
          <div
            class="fixed top-16 right-0 h-[calc(100vh-4rem)] w-96 shadow-2xl z-[50] overflow-hidden"
            style="animation: slideInRight 0.4s cubic-bezier(0.68, -0.55, 0.265, 1.55);"
          >
            <ActivityFeed />
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<!-- Global Components -->
<Toast />
<LoadingOverlay />

<!-- Global Modal Portal - All modals rendered here -->
{#if $auth.isLoggedIn}
  <ModalPortal />
{/if}

<style>
  /* Bootstrap Icons loaded from CDN in index.html */

  .app-container {
    height: 100vh;
    width: 100vw;
    max-width: 100vw;
    display: flex;
    overflow: hidden;
    overflow-x: hidden;
    position: relative;
  }

  .main-wrapper {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    overflow-x: hidden;
    max-width: 100%;
  }

  .main-content {
    flex: 1;
    overflow-y: auto; /* Allow scrolling for content - needed for infinite scroll */
    overflow-x: hidden;
    max-width: 100%;
    background: transparent !important;
    box-shadow: none !important;
    /* Hide scrollbar but keep functionality */
    scrollbar-width: none; /* Firefox */
    -ms-overflow-style: none; /* IE/Edge */
  }

  .main-content::-webkit-scrollbar {
    display: none; /* Chrome/Safari/Opera */
  }

  /* Mobile Sidebar Support */
  .sidebar-wrapper {
    transition: transform 0.3s ease-in-out;
  }

  @media (max-width: 768px) {
    .sidebar-wrapper {
      position: fixed;
      left: 0;
      top: 0;
      height: 100vh;
      z-index: 40;
      transform: translateX(-100%);
    }

    .sidebar-wrapper.mobile-open {
      transform: translateX(0);
    }

    .mobile-menu-toggle {
      display: block;
    }

    .main-wrapper {
      width: 100%;
    }
  }

  @media (min-width: 769px) {
    .mobile-menu-toggle {
      display: none;
    }

    .sidebar-wrapper {
      position: relative;
      transform: none !important;
    }
  }

  .mobile-overlay {
    display: block;
  }

  /* Activity Feed Animations */
  @keyframes slideInRight {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .animate-fade-in {
    animation: fadeIn 0.3s ease-out;
  }
</style>
