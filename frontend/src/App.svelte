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

  // Dynamic imports for better code-splitting (lazy-loaded per route)
  // These are loaded on-demand when user navigates to the view
  const FilesView = () => import("./pages/files/FilesView.svelte");
  const SharedView = () => import("./pages/files/SharedView.svelte");
  const FavoritesView = () => import("./pages/files/FavoritesView.svelte");
  const SmartFoldersView = () =>
    import("./pages/files/SmartFoldersView.svelte");
  const RecentFilesView = () => import("./pages/files/RecentFilesView.svelte");

  const TrashView = () => import("./pages/trash/TrashView.svelte");
  const ActivityView = () => import("./pages/system/ActivityView.svelte");
  const NotificationsView = () =>
    import("./pages/system/NotificationsView.svelte");
  const BackupView = () => import("./pages/system/BackupView.svelte");
  const StorageView = () => import("./pages/system/StorageView.svelte");
  const UsersView = () => import("./pages/system/UsersView.svelte");

  const DuplicatesView = () => import("./pages/tools/DuplicatesView.svelte");
  const FtpSyncView = () => import("./pages/tools/FtpSyncView.svelte");
  const EmailIntegrationView = () =>
    import("./pages/tools/EmailIntegrationView.svelte");
  const ArchivesView = () => import("./pages/tools/ArchivesView.svelte");
  const CompressionView = () => import("./pages/tools/CompressionView.svelte");
  const TagCloudView = () => import("./pages/TagCloudView.svelte");

  const AdminView = () => import("./pages/admin/AdminView.svelte");
  const JobsDashboard = () => import("./pages/JobsDashboard.svelte");
  const JobsQueueView = () => import("./pages/jobs/JobsQueueView.svelte");
  const RoleManagementView = () =>
    import("./pages/rbac/RoleManagementView.svelte");
  const WorkflowBuilderView = () =>
    import("./pages/workflow/WorkflowBuilderView.svelte");
  const CloudStorageView = () =>
    import("./pages/admin/CloudStorageView.svelte");
  const AuditComplianceView = () =>
    import("./pages/AuditComplianceView.svelte");
  const AdminDashboardView = () =>
    import("./pages/admin/AdminDashboardView.svelte");
  const WebhooksView = () => import("./pages/admin/WebhooksView.svelte");
  const SystemHealthView = () =>
    import("./pages/admin/SystemHealthView.svelte");
  const StorageAnalyticsView = () =>
    import("./pages/analytics/StorageAnalyticsView.svelte");
  const EncryptionView = () => import("./pages/EncryptionView.svelte");
  const QuotaManagementView = () =>
    import("./pages/QuotaManagementView.svelte");
  const SystemConfigView = () =>
    import("./pages/admin/SystemConfigView.svelte");
  const UserGroupsView = () => import("./pages/admin/UserGroupsView.svelte");
  const GuestAccessView = () => import("./pages/GuestAccessView.svelte");
  const ThemeCustomizationView = () =>
    import("./pages/ThemeCustomizationView.svelte");

  const SettingsHub = () => import("./pages/settings/SettingsHub.svelte");
  const UserProfileView = () => import("./pages/user/UserProfileView.svelte");
  const UserSettingsView = () => import("./pages/user/UserSettingsView.svelte");
  const SecurityView = () => import("./pages/user/SecurityView.svelte");
  const ApiTokensView = () => import("./pages/user/ApiTokensView.svelte");
  const HelpView = () => import("./pages/user/HelpView.svelte");

  // UI Components (keep static - needed immediately)
  import Toast from "./components/ui/Toast.svelte";
  import LoadingOverlay from "./components/LoadingOverlay.svelte";
  import ErrorBoundary from "./components/ui/ErrorBoundary.svelte";
  import ActivityFeed from "./components/ActivityFeed.svelte";
  import ModalPortal from "./components/modals/ModalPortal.svelte";
  import WebSocketStatus from "./components/system/WebSocketStatus.svelte";
  import UploadQueue from "./components/ui/UploadQueue.svelte";
  import CommandPalette from "./components/ui/CommandPalette.svelte";
  import ShortcutsModal from "./components/ui/ShortcutsModal.svelte";
  import LazyView from "./components/ui/LazyView.svelte";

  // PWA Components (keep static - needed immediately)
  import PWAInstallPrompt from "./components/pwa/PWAInstallPrompt.svelte";
  import OfflineIndicator from "./components/offline/OfflineIndicator.svelte";

  // State für Setup-Check
  let setupCompleted = $state(null); // null = loading, true = completed, false = needs setup
  let setupCheckDone = $state(false);

  // Command Palette state
  let commandPaletteOpen = $state(false);
  let shortcutsModalOpen = $state(false);
  let currentViewState = $state("dashboard"); // Track current view for commands (start with dashboard)

  // View component mapping for lazy loading
  const viewComponents = {
    files: FilesView,
    shared: SharedView,
    favorites: FavoritesView,
    "smart-folders": SmartFoldersView,
    recent: RecentFilesView,
    trash: TrashView,
    users: UsersView,
    settings: SettingsHub,
    admin: SettingsHub,
    profile: UserProfileView,
    help: HelpView,
    "user-settings": UserSettingsView,
    security: SecurityView,
    storage: StorageView,
    activity: ActivityView,
    duplicates: DuplicatesView,
    "ftp-sync": FtpSyncView,
    "email-integration": EmailIntegrationView,
    archives: ArchivesView,
    compression: CompressionView,
    conversion: () => import("./pages/conversion/ConversionView.svelte"),
    "tag-cloud": TagCloudView,
    backup: BackupView,
    jobs: JobsDashboard,
    "jobs-queue": JobsQueueView,
    roles: RoleManagementView,
    workflows: WorkflowBuilderView,
    "cloud-storage": CloudStorageView,
    encryption: EncryptionView,
    quota: QuotaManagementView,
    "system-config": SystemConfigView,
    "user-groups": UserGroupsView,
    audit: AuditComplianceView,
    dashboard: AdminDashboardView,
    webhooks: WebhooksView,
    "system-health": SystemHealthView,
    "storage-analytics": StorageAnalyticsView,
    "api-tokens": ApiTokensView,
    notifications: NotificationsView,
    guests: GuestAccessView,
    "theme-customization": ThemeCustomizationView,
  };

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
        "ftp-sync": "ftp-sync",
        "email-integration": "email-integration",
        archives: "archives",
        compression: "compression",
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
      // Prevent shortcuts when typing in input/textarea
      const isTyping = ["INPUT", "TEXTAREA", "SELECT"].includes(
        e.target.tagName
      );
      const isContentEditable = e.target.isContentEditable;

      // Ctrl+K / Cmd+K - Command Palette (always works)
      if ((e.ctrlKey || e.metaKey) && e.key === "k") {
        e.preventDefault();
        commandPaletteOpen = true;
        return;
      }

      // Ctrl+? or Cmd+? for help
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === "?") {
        e.preventDefault();
        shortcutsModalOpen = true;
        return;
      }

      // Don't trigger other shortcuts while typing
      if (isTyping || isContentEditable) return;

      // Ctrl+F / Cmd+F - Focus search (in AppHeader)
      if ((e.ctrlKey || e.metaKey) && e.key === "f") {
        e.preventDefault();
        const searchInput = document.querySelector('input[type="search"]');
        if (searchInput) searchInput.focus();
        return;
      }

      // Ctrl+U / Cmd+U - Trigger upload
      if ((e.ctrlKey || e.metaKey) && e.key === "u") {
        e.preventDefault();
        // Trigger file upload if on files view
        if ($currentView === "files") {
          const uploadInput = document.querySelector('input[type="file"]');
          if (uploadInput) uploadInput.click();
        }
        return;
      }

      // Ctrl+N / Cmd+N - New folder
      if ((e.ctrlKey || e.metaKey) && e.key === "n") {
        e.preventDefault();
        if ($currentView === "files") {
          // Dispatch event to FilesView to open create folder modal
          window.dispatchEvent(new CustomEvent("trigger-new-folder"));
        }
        return;
      }

      // Ctrl+Shift+A / Cmd+Shift+A - Advanced search
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === "A") {
        e.preventDefault();
        window.dispatchEvent(new CustomEvent("trigger-advanced-search"));
        return;
      }

      // ESC - Close modals/panels
      if (e.key === "Escape") {
        if (commandPaletteOpen) {
          commandPaletteOpen = false;
        } else if (shortcutsModalOpen) {
          shortcutsModalOpen = false;
        } else if (showActivityFeed) {
          showActivityFeed = false;
        }
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
        // Trigger folder creation in FilesView
        if ($currentView === "files") {
          window.dispatchEvent(new CustomEvent("trigger-new-folder"));
        } else {
          // Navigate to files view first
          currentView.set("files");
          currentViewState = "files";
          setTimeout(() => {
            window.dispatchEvent(new CustomEvent("trigger-new-folder"));
          }, 100);
        }
        break;
      case "upload-file":
        // Trigger file upload
        if ($currentView === "files") {
          const uploadInput = document.querySelector('input[type="file"]');
          if (uploadInput) uploadInput.click();
        } else {
          // Navigate to files view first
          currentView.set("files");
          currentViewState = "files";
          setTimeout(() => {
            const uploadInput = document.querySelector('input[type="file"]');
            if (uploadInput) uploadInput.click();
          }, 100);
        }
        break;
      case "bulk-delete":
        // Trigger bulk delete in current view
        window.dispatchEvent(new CustomEvent("trigger-bulk-delete"));
        break;
      case "search":
        // Focus search input
        const searchInput = document.querySelector('input[type="search"]');
        if (searchInput) {
          searchInput.focus();
          searchInput.select();
        }
        break;
      case "advanced-search":
        // Trigger advanced search modal
        window.dispatchEvent(new CustomEvent("trigger-advanced-search"));
        break;
      case "saved-searches":
        // Trigger saved searches modal
        window.dispatchEvent(new CustomEvent("trigger-saved-searches"));
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
    class="h-screen w-screen max-w-full flex overflow-hidden relative bg-gray-50 dark:bg-gray-900 transition-colors"
  >
    <!-- Mobile Menu Toggle -->
    {#if isMobile}
      <button
        onclick={() => (isMobileMenuOpen = !isMobileMenuOpen)}
        class="fixed top-4 left-4 z-50 p-3 bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 md:hidden touch-target"
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
      class="transition-transform duration-300 ease-in-out md:relative md:transform-none {isMobile
        ? 'fixed left-0 top-0 h-screen z-40'
        : ''} {isMobileMenuOpen && isMobile
        ? 'translate-x-0'
        : isMobile
          ? '-translate-x-full'
          : ''}"
    >
      <Sidebar on:navigate={() => isMobile && (isMobileMenuOpen = false)} />
    </div>

    <!-- Overlay for mobile -->
    {#if isMobile && isMobileMenuOpen}
      <button
        onclick={() => (isMobileMenuOpen = false)}
        class="block fixed inset-0 bg-black bg-opacity-50 z-30"
        aria-label="Close menu"
      ></button>
    {/if}

    <div class="flex-1 flex flex-col overflow-hidden max-w-full">
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
              class="main-content flex-1 overflow-y-auto overflow-x-hidden max-w-full md:pb-0 pb-[calc(72px+env(safe-area-inset-bottom))]"
              style="scrollbar-width: none; -ms-overflow-style: none;"
            >
              <!-- Lazy-loaded views with loading state -->
              {#if viewComponents[$currentView]}
                <LazyView component={viewComponents[$currentView]} />
              {:else}
                <div class="flex items-center justify-center h-full">
                  <div class="text-center">
                    <i class="bi bi-question-circle text-6xl opacity-50"></i>
                    <p class="mt-4 text-lg">Unknown view: {$currentView}</p>
                  </div>
                </div>
              {/if}
            </main>
          </ErrorBoundary>
        </div>

        <!-- Modern Activity Feed Slide-In Panel -->
        {#if showActivityFeed}
          <!-- Backdrop with blur -->
          <div
            class="fixed inset-0 bg-black/20 dark:bg-black/40 backdrop-blur-sm z-[45] animate-fadeIn"
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
  /* Hide scrollbar for main content */
  .main-content::-webkit-scrollbar {
    display: none;
  }

  /* Activity Feed Slide Animation */
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
</style>
