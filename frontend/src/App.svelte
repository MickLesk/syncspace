<script>
  import { onMount } from "svelte";
  import { auth } from "./stores/auth";
  import { initErrorReporting } from "./lib/errorReporting";
  import { websocketManager } from "./stores/websocket.js";
  import { currentView } from "./stores/ui.js";

  // Backend-First: Import serverState instead of separate UI/preference stores
  import {
    serverState,
    initializeServerState,
    isDarkMode,
    currentLanguage,
    toggleDarkMode,
  } from "./stores/serverState";

  // Setup & Auth Views
  import SetupWizard from "./pages/SetupWizard.svelte";
  import Login from "./pages/auth/Login.svelte";
  import Signup from "./pages/auth/Signup.svelte";

  // Navigation & Header
  import Sidebar from "./components/navigation/Sidebar.svelte";
  import AppHeader from "./components/ui/AppHeader.svelte";
  import MobileBottomNav from "./components/navigation/MobileBottomNav.svelte";

  // File Views
  import FilesView from "./pages/files/FilesView.svelte";
  import SharedView from "./pages/files/SharedView.svelte";
  import FavoritesView from "./pages/files/FavoritesView.svelte";
  import SmartFoldersView from "./pages/files/SmartFoldersView.svelte";
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
  import TagCloudView from "./pages/TagCloudView.svelte";

  // Admin View - Consolidated admin page with tabs
  import AdminView from "./pages/admin/AdminView.svelte";

  // Jobs & System Management (legacy - kept for direct access if needed)
  import JobsDashboard from "./pages/JobsDashboard.svelte";
  import JobsQueueView from "./pages/jobs/JobsQueueView.svelte";
  import RoleManagementView from "./pages/rbac/RoleManagementView.svelte";
  import WorkflowBuilderView from "./pages/workflow/WorkflowBuilderView.svelte";
  import CloudStorageView from "./pages/admin/CloudStorageView.svelte";
  import AuditComplianceView from "./pages/AuditComplianceView.svelte";
  import AdminDashboardView from "./pages/admin/AdminDashboardView.svelte";
  import WebhooksView from "./pages/admin/WebhooksView.svelte";
  import SystemHealthView from "./pages/admin/SystemHealthView.svelte";
  import StorageAnalyticsView from "./pages/analytics/StorageAnalyticsView.svelte";
  import EncryptionView from "./pages/EncryptionView.svelte";
  import QuotaManagementView from "./pages/QuotaManagementView.svelte";
  import SystemConfigView from "./pages/admin/SystemConfigView.svelte";
  import UserGroupsView from "./pages/admin/UserGroupsView.svelte";
  import GuestAccessView from "./pages/GuestAccessView.svelte";
  import ThemeCustomizationView from "./pages/ThemeCustomizationView.svelte";

  // User & Settings Views
  import SettingsHub from "./pages/settings/SettingsHub.svelte";
  import UserProfileView from "./pages/user/UserProfileView.svelte";
  import UserSettingsView from "./pages/user/UserSettingsView.svelte";
  import SecurityView from "./pages/user/SecurityView.svelte";
  import ApiTokensView from "./pages/user/ApiTokensView.svelte";

  // Showcase (Development)
  import DesignShowcase from "./pages/showcase/DesignShowcase.svelte";

  // UI Components
  import Toast from "./components/ui/Toast.svelte";
  import LoadingOverlay from "./components/LoadingOverlay.svelte";
  import ErrorBoundary from "./components/ui/ErrorBoundary.svelte";
  import ActivityFeed from "./components/ActivityFeed.svelte";
  import ModalPortal from "./components/modals/ModalPortal.svelte";
  import WebSocketStatus from "./components/system/WebSocketStatus.svelte";
  import UploadQueue from "./components/ui/UploadQueue.svelte";
  import CommandPalette from "./components/ui/CommandPalette.svelte";
  import ShortcutsModal from "./components/ui/ShortcutsModal.svelte";

  // PWA Components
  import PWAInstallPrompt from "./components/pwa/PWAInstallPrompt.svelte";
  import OfflineIndicator from "./components/offline/OfflineIndicator.svelte";

  // State für Setup-Check
  let setupCompleted = $state(null); // null = loading, true = completed, false = needs setup
  let setupCheckDone = $state(false);

  // Command Palette state
  let commandPaletteOpen = $state(false);
  let shortcutsModalOpen = $state(false);
  let currentViewState = $state("dashboard"); // Track current view for commands (start with dashboard)

  // Check if setup is needed
  async function checkSetupStatus() {
    try {
      const response = await fetch(
        `${new URL(window.location.href).protocol}//${new URL(window.location.href).hostname}:8080/api/setup/status`
      );
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

    // Load user state from backend (Backend-First Architecture)
    if ($auth.isLoggedIn) {
      try {
        console.log("📥 Loading server state...");
        await initializeServerState();
        console.log("✅ Server state loaded");

        // Connect WebSocket after user is authenticated
        console.log("🔌 Connecting WebSocket...");
        websocketManager.connect();
      } catch (err) {
        console.error("Failed to load server state:", err);
      }
    }

    // Apply initial theme immediately (before server state loads)
    applyThemeToDocument($isDarkMode);

    // Set initial view based on current URL hash, fallback to dashboard
    if ($auth.isLoggedIn) {
      const initialView = getViewFromHash(window.location.hash);
      currentView.set(initialView);
    }
  });

  // Helper function to extract view name from hash
  function getViewFromHash(hash) {
    if (!hash || hash === "" || hash === "#" || hash === "#/") {
      return "dashboard";
    }
    // Parse hash like "#/files/path/to/folder" -> "files"
    // Or "#/settings" -> "settings"
    const match = hash.match(/^#\/([a-zA-Z-]+)/);
    if (match && match[1]) {
      const viewName = match[1].toLowerCase();
      // Map hash routes to view names
      const viewMap = {
        files: "files",
        dashboard: "dashboard",
        shared: "shared",
        favorites: "favorites",
        recent: "recent",
        trash: "trash",
        activity: "activity",
        notifications: "notifications",
        backup: "backup",
        storage: "storage",
        users: "users",
        duplicates: "duplicates",
        tags: "tags",
        "tag-cloud": "tags",
        jobs: "jobs",
        "jobs-queue": "jobsQueue",
        roles: "roles",
        workflows: "workflows",
        "cloud-storage": "cloud-storage",
        encryption: "encryption",
        audit: "audit",
        admin: "admin",
        webhooks: "webhooks",
        health: "health",
        analytics: "storage-analytics",
        "storage-analytics": "storage-analytics",
        settings: "settings",
        profile: "profile",
        "user-settings": "user-settings",
        security: "security",
        "api-tokens": "api-tokens",
        showcase: "showcase",
      };
      return viewMap[viewName] || "dashboard";
    }
    return "dashboard";
  }

  // Helper function to apply theme to document
  function applyThemeToDocument(isDark) {
    if (typeof document !== "undefined") {
      const html = document.documentElement;
      html.classList.remove("dark", "light");

      if (isDark) {
        html.classList.add("dark");
        html.setAttribute("data-theme", "dark");
        html.style.colorScheme = "dark";
      } else {
        html.classList.add("light");
        html.setAttribute("data-theme", "light");
        html.style.colorScheme = "light";
      }
      console.log(`[Theme] Applied: ${isDark ? "dark" : "light"}`);
    }
  }

  // Load server state when user logs in
  $effect(() => {
    if ($auth.isLoggedIn) {
      initializeServerState()
        .then(() => {
          console.log("✅ Server state synced after login");
          // Connect WebSocket when user logs in
          console.log("🔌 User logged in, connecting WebSocket...");
          websocketManager.connect();
        })
        .catch((err) => {
          console.error("Failed to load server state:", err);
        });
    } else {
      // Disconnect WebSocket when user logs out
      console.log("🔌 User logged out, disconnecting WebSocket...");
      websocketManager.disconnect();
    }
  });

  // Apply theme to document - Tailwind v4 compatible (Backend-First)
  // This effect runs whenever isDarkMode changes
  $effect(() => {
    applyThemeToDocument($isDarkMode);
  });

  function handleNavigate(event) {
    console.log("[App] handleNavigate event.detail:", event.detail);
    // Support both event.detail.view and event.detail (string)
    const view =
      typeof event.detail === "string" ? event.detail : event.detail.view;
    console.log("[App] Setting currentView to:", view);
    currentView.set(view);
  }

  // Handle search result selection - navigate to file and open it
  function handleSearchResultSelected(event) {
    const { file, path } = event.detail;
    console.log("[App] Search result selected:", $state.snapshot(file));

    // Switch to files view if not already there
    if ($currentView !== "files") {
      currentView.set("files");
    }

    // Use custom event to tell FilesView to navigate and open this file
    window.dispatchEvent(
      new CustomEvent("openFileFromSearch", {
        detail: {
          filePath: path,
          fileName: file.name,
          fileId: file.id,
          isFolder: file.type === "folder",
        },
      })
    );
  }

  // Handle advanced search from AppHeader
  function handleAdvancedSearch(event) {
    const { query, filters, sortBy, sortOrder } = event.detail;
    console.log("[App] Advanced search triggered:", {
      query,
      filters,
      sortBy,
      sortOrder,
    });

    // Switch to files view if not already there
    if ($currentView !== "files") {
      currentView.set("files");
    }

    // Dispatch advanced search event to FilesView
    window.dispatchEvent(
      new CustomEvent("advancedSearch", {
        detail: { query, filters, sortBy, sortOrder },
      })
    );
  }

  // Check for hash-based routing (login vs signup)
  let showSignup = $state(false);
  let isMobileMenuOpen = $state(false);
  let isMobile = $state(false);
  let showActivityFeed = $state(false); // Collapsible Activity Feed

  onMount(() => {
    const checkRoute = () => {
      showSignup = window.location.hash === "#/signup";

      // Update current view based on hash (for browser back/forward, manual URL changes)
      if ($auth.isLoggedIn) {
        const viewFromHash = getViewFromHash(window.location.hash);
        if (viewFromHash && $currentView !== viewFromHash) {
          currentView.set(viewFromHash);
        }
      }
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

    // Global keyboard shortcuts
    const handleGlobalKeydown = (e) => {
      // Ctrl+? or Cmd+? for help
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === "?") {
        e.preventDefault();
        shortcutsModalOpen = true;
      }
    };
    window.addEventListener("keydown", handleGlobalKeydown);

    return () => {
      window.removeEventListener("hashchange", checkRoute);
      window.removeEventListener("resize", checkMobile);
      window.removeEventListener("keydown", handleGlobalKeydown);
    };
  });

  // Handle Command Palette selection
  async function handleCommandSelect(cmd) {
    commandPaletteOpen = false;

    switch (cmd.type) {
      case "navigate":
        currentView.set(cmd.view);
        currentViewState = cmd.view;
        break;
      case "toggle-theme":
        // Toggle theme through serverState store (Backend-First)
        await toggleDarkMode();
        break;
      case "help":
        shortcutsModalOpen = true;
        break;
      case "new-folder":
        console.log("Create new folder");
        // TODO: Trigger folder creation modal
        break;
      case "upload-file":
        console.log("Upload file");
        // TODO: Trigger upload
        break;
      case "bulk-delete":
        console.log("Bulk delete");
        // TODO: Show bulk delete UI
        break;
      case "search":
        currentView.set("search");
        break;
      case "advanced-search":
        console.log("Advanced search");
        // TODO: Show advanced search modal
        break;
      case "saved-searches":
        console.log("Saved searches");
        // TODO: Show saved searches modal
        break;
      default:
        break;
    }
  }
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
        class="mobile-menu-toggle fixed top-4 left-4 z-50 p-3 bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 md:hidden touch-target"
        aria-label={isMobileMenuOpen ? "Close menu" : "Open menu"}
      >
        <i
          class="bi bi-{isMobileMenuOpen
            ? 'x-lg'
            : 'list'} text-xl text-gray-700 dark:text-gray-200"
          aria-hidden="true"
        ></i>
      </button>
    {/if}

    <!-- Sidebar with mobile support -->
    <div
      class="sidebar-wrapper"
      class:mobile-open={isMobileMenuOpen && isMobile}
      class:mobile-closed={!isMobileMenuOpen && isMobile}
    >
      <Sidebar on:navigate={() => isMobile && (isMobileMenuOpen = false)} />
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
        on:searchResultSelected={handleSearchResultSelected}
        on:advancedSearch={handleAdvancedSearch}
        on:toggleActivityFeed={(e) => (showActivityFeed = e.detail.visible)}
        bind:showActivityFeed
      />

      <!-- PWA Install Prompt -->
      <PWAInstallPrompt />

      <!-- Offline Status Indicator -->
      <OfflineIndicator />

      <!-- WebSocket Status Indicator (Top Right) -->
      <div class="fixed top-4 right-4 z-50">
        <WebSocketStatus />
      </div>

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
              {:else if $currentView === "favorites"}
                <FavoritesView />
              {:else if $currentView === "smart-folders"}
                <SmartFoldersView />
              {:else if $currentView === "recent"}
                <RecentFilesView />
              {:else if $currentView === "trash"}
                <TrashView />
              {:else if $currentView === "users"}
                <UsersView />
              {:else if $currentView === "settings" || $currentView === "admin"}
                <SettingsHub />
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
              {:else if $currentView === "tag-cloud"}
                <TagCloudView />
              {:else if $currentView === "backup"}
                <BackupView />
              {:else if $currentView === "jobs"}
                <JobsDashboard />
              {:else if $currentView === "jobs-queue"}
                <JobsQueueView />
              {:else if $currentView === "roles"}
                <RoleManagementView />
              {:else if $currentView === "workflows"}
                <WorkflowBuilderView />
              {:else if $currentView === "cloud-storage"}
                <CloudStorageView />
              {:else if $currentView === "encryption"}
                <EncryptionView />
              {:else if $currentView === "quota"}
                <QuotaManagementView />
              {:else if $currentView === "system-config"}
                <SystemConfigView />
              {:else if $currentView === "user-groups"}
                <UserGroupsView />
              {:else if $currentView === "audit"}
                <AuditComplianceView />
              {:else if $currentView === "dashboard"}
                <AdminDashboardView />
              {:else if $currentView === "webhooks"}
                <WebhooksView />
              {:else if $currentView === "system-health"}
                <SystemHealthView />
              {:else if $currentView === "storage-analytics"}
                <StorageAnalyticsView />
              {:else if $currentView === "api-tokens"}
                <ApiTokensView />
              {:else if $currentView === "notifications"}
                <NotificationsView />
              {:else if $currentView === "guests"}
                <GuestAccessView />
              {:else if $currentView === "theme-customization"}
                <ThemeCustomizationView />
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

    <!-- Mobile Bottom Navigation -->
    {#if isMobile}
      <MobileBottomNav />
    {/if}
  </div>
{/if}

<!-- Global Components -->
<Toast />
<LoadingOverlay />

<!-- Command Palette -->
<CommandPalette
  bind:isOpen={commandPaletteOpen}
  onCommandSelect={(cmd) => handleCommandSelect(cmd)}
/>

<!-- Shortcuts Modal -->
<ShortcutsModal
  bind:isOpen={shortcutsModalOpen}
  onClose={() => (shortcutsModalOpen = false)}
/>

<!-- Global Modal Portal - All modals rendered here -->
{#if $auth.isLoggedIn}
  <ModalPortal />
{/if}

<!-- Global Upload Queue UI -->
<UploadQueue />

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

  /* Mobile bottom nav spacing */
  @media (max-width: 768px) {
    .main-content {
      padding-bottom: calc(72px + env(safe-area-inset-bottom));
    }
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
