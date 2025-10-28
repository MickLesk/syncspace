<script>
  import { createEventDispatcher } from "svelte";
  import { currentTheme, currentLang, currentView } from "../../stores/ui";
  import { auth } from "../../stores/auth";
  import { t } from "../../i18n.js";
  import AdvancedSearchModal from "../search/AdvancedSearchModal.svelte";
  import Modal from "./Modal.svelte";
  import ThemeSwitcher from "./ThemeSwitcher.svelte";
  import NotificationBell from "./NotificationBell.svelte";
  import { userPreferences } from "../../stores/preferences.js";
  import { wsConnected } from "../../stores/websocket.js";
  import api from "../../lib/api.js";
  import { onMount } from "svelte";

  const dispatch = createEventDispatcher();

  let serverInfo = $state(null);
  let showServerInfo = $state(false);
  let backendOnline = $state(false);

  // Check backend status periodically
  onMount(() => {
    checkBackendStatus();
    const interval = setInterval(checkBackendStatus, 3000);
    return () => clearInterval(interval);
  });

  async function checkBackendStatus() {
    try {
      const response = await fetch("http://localhost:8080/health", {
        method: "GET",
      });
      backendOnline = response.ok;
    } catch (err) {
      backendOnline = false;
    }
  }

  async function loadServerInfo() {
    if (showServerInfo) {
      showServerInfo = false;
      return;
    }
    if (!serverInfo) {
      try {
        serverInfo = await api.config.getInfo();
      } catch (err) {
        console.error("Failed to load server info:", err);
      }
    }
    showServerInfo = true;
  }

  function closeServerInfo() {
    showServerInfo = false;
  }

  // Map view IDs to display names with icons
  const viewNames = {
    files: { name: "Files", icon: "bi-folder-fill" },
    shared: { name: "Shared", icon: "bi-people-fill" },
    favorites: { name: "Favorites", icon: "bi-star-fill" },
    trash: { name: "Trash", icon: "bi-trash3-fill" },
    users: { name: "Users", icon: "bi-person-fill" },
    settings: { name: "Settings", icon: "bi-gear-fill" },
    profile: { name: "Profile", icon: "bi-person-circle" },
    storage: { name: "Storage", icon: "bi-hdd-fill" },
    activity: { name: "Activity", icon: "bi-clock-history" },
    duplicates: { name: "Duplicates", icon: "bi-files" },
    backup: { name: "Backup", icon: "bi-cloud-arrow-down-fill" },
    gallery: { name: "Gallery", icon: "bi-images" },
  };

  let currentViewName = $derived(viewNames[$currentView]?.name || "SyncSpace");
  let currentViewIcon = $derived(
    viewNames[$currentView]?.icon || "bi-cloud-arrow-up-fill"
  );

  let searchQuery = $state("");
  let showSearchModal = $state(false);
  let showAdvancedSearch = $state(false);
  let showSearchDropdown = $state(false);
  let searchResults = $state([]);
  let searchDebounce = null;
  let searchInputRef = null;

  let isDark = $derived($currentTheme === "dark");
  let userInitials = $derived(
    $auth.username ? $auth.username.substring(0, 2).toUpperCase() : "AD"
  );

  let showUserDropdown = $state(false);

  // Mock notifications data
  let notifications = $state([
    {
      id: 1,
      type: "success",
      icon: "check-circle-fill",
      title: "File uploaded successfully",
      message: "Document.pdf has been uploaded",
      time: "2 minutes ago",
      read: false,
      avatar: null,
    },
    {
      id: 2,
      type: "info",
      icon: "share-fill",
      title: "New share request",
      message: 'John Doe shared "Project Files" with you',
      time: "1 hour ago",
      read: false,
      avatar: "JD",
    },
    {
      id: 3,
      type: "warning",
      icon: "exclamation-triangle-fill",
      title: "Storage almost full",
      message: "85% of storage capacity used",
      time: "3 hours ago",
      read: true,
      avatar: null,
    },
  ]);

  function markAsRead(id) {
    notifications = notifications.map((n) =>
      n.id === id ? { ...n, read: true } : n
    );
  }

  function markAllAsRead() {
    notifications = notifications.map((n) => ({ ...n, read: true }));
  }

  function clearAllNotifications() {
    notifications = [];
  }

  let unreadNotifications = $derived(notifications.filter((n) => !n.read));
  let unreadCount = $derived(unreadNotifications.length);

  // Load recent searches from backend preferences
  let recentSearches = $derived($userPreferences.recent_searches || []);

  // Save search to recent searches via backend
  async function saveRecentSearch(query) {
    if (!query.trim()) return;

    let searches = [...recentSearches];
    searches = searches.filter((s) => s !== query);
    searches = [query, ...searches].slice(0, 10);

    await userPreferences.updatePreferences({ recent_searches: searches });
  }

  // Clear recent searches
  async function clearRecentSearches() {
    await userPreferences.updatePreferences({ recent_searches: [] });
  }

  // Handle search input change with debounce
  function handleSearchInput(e) {
    searchQuery = e.target.value;
    showSearchDropdown = true;

    if (searchDebounce) {
      clearTimeout(searchDebounce);
    }

    if (searchQuery.trim()) {
      searchDebounce = setTimeout(async () => {
        // TODO: Call actual search API
        // For now, mock results
        searchResults = [
          {
            name: "Document.pdf",
            path: "/documents/Document.pdf",
            type: "file",
            icon: "file-earmark-pdf",
          },
          {
            name: "Images",
            path: "/images",
            type: "folder",
            icon: "folder-fill",
          },
          {
            name: "Project Report.docx",
            path: "/work/Project Report.docx",
            type: "file",
            icon: "file-earmark-word",
          },
        ].filter((item) =>
          item.name.toLowerCase().includes(searchQuery.toLowerCase())
        );
      }, 300);
    } else {
      searchResults = [];
    }
  }

  // Select search result
  function selectSearchResult(result) {
    searchQuery = result.name;
    saveRecentSearch(result.name);
    showSearchDropdown = false;
    dispatch("search", { query: result.name, path: result.path });
  }

  // Select recent search
  function selectRecentSearch(query) {
    searchQuery = query;
    showSearchDropdown = false;
    dispatch("search", { query });
  }

  function toggleTheme() {
    const newTheme = isDark ? "light" : "dark";
    currentTheme.set(newTheme);
    document.documentElement.setAttribute(
      "data-theme",
      newTheme === "dark" ? "syncspace-dark" : "syncspace"
    );
  }

  function handleLogout() {
    // Show confirmation modal
    if (confirm("Are you sure you want to log out?")) {
      auth.logout();
      window.location.href = "/";
    }
  }

  function handleSearch(e) {
    e.preventDefault();
    if (searchQuery.trim()) {
      saveRecentSearch(searchQuery);
      dispatch("search", { query: searchQuery });
      showSearchModal = false;
      showSearchDropdown = false;
    }
  }

  function handleAdvancedSearch(e) {
    const { query, filters, sortBy, sortOrder } = e.detail;
    dispatch("advancedSearch", { query, filters, sortBy, sortOrder });
  }

  function handleKeydown(e) {
    // Ctrl+K for Quick Search
    if ((e.ctrlKey || e.metaKey) && e.key === "k") {
      e.preventDefault();
      showSearchModal = true;
    }
    // Ctrl+Shift+F for Advanced Search
    if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === "F") {
      e.preventDefault();
      showAdvancedSearch = true;
    }
    // Escape to close dropdown
    if (e.key === "Escape" && showSearchDropdown) {
      showSearchDropdown = false;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- Material 3 Expressive Header with Glassmorphism -->
<header class="app-header">
  <div class="header-container">
    <!-- Left: Logo & Current View Name -->
    <div class="header-left">
      <div class="brand-logo">
        <div class="logo-icon">
          <i class={currentViewIcon}></i>
        </div>
        <span class="brand-name">{currentViewName}</span>
      </div>
    </div>

    <!-- Center: Enhanced Search Bar -->
    <div class="header-center">
      <div class="search-container-new">
        <div class="search-input-wrapper">
          <i class="bi bi-search search-icon-new"></i>
          <input
            type="text"
            class="search-input-new"
            placeholder="{t($currentLang, 'searchPlaceholder')}..."
            bind:value={searchQuery}
            bind:this={searchInputRef}
            oninput={handleSearchInput}
            onfocus={() => {
              showSearchDropdown = true;
            }}
            onkeydown={(e) => e.key === "Enter" && handleSearch(e)}
          />
          {#if searchQuery}
            <button
              class="search-clear-btn"
              onclick={() => {
                searchQuery = "";
                searchResults = [];
              }}
              aria-label="Clear search"
            >
              <i class="bi bi-x"></i>
            </button>
          {/if}
          <kbd class="search-kbd-new">Ctrl K</kbd>
        </div>

        <button
          class="advanced-button-new"
          onclick={() => (showAdvancedSearch = true)}
          title="{t($currentLang, 'advancedSearch')} (Ctrl+Shift+F)"
        >
          <i class="bi bi-funnel"></i>
        </button>

        <!-- Search Dropdown -->
        {#if showSearchDropdown}
          <div class="search-dropdown">
            {#if searchQuery.trim() && searchResults.length > 0}
              <div class="search-section">
                <div class="search-section-title">Results</div>
                {#each searchResults as result}
                  <button
                    class="search-result-item"
                    onclick={() => selectSearchResult(result)}
                  >
                    <i
                      class="bi bi-{result.icon} text-{result.type === 'folder'
                        ? 'warning'
                        : 'info'}"
                    ></i>
                    <div class="search-result-content">
                      <div class="search-result-name">{result.name}</div>
                      <div class="search-result-path">{result.path}</div>
                    </div>
                    <i class="bi bi-arrow-return-left text-xs opacity-40"></i>
                  </button>
                {/each}
              </div>
            {:else if !searchQuery.trim() && recentSearches.length > 0}
              <div class="search-section">
                <div class="search-section-header">
                  <div class="search-section-title">Recent Searches</div>
                  <button
                    class="search-clear-all"
                    onclick={clearRecentSearches}
                  >
                    Clear
                  </button>
                </div>
                {#each recentSearches as recent}
                  <button
                    class="search-result-item"
                    onclick={() => selectRecentSearch(recent)}
                  >
                    <i class="bi bi-clock-history opacity-60"></i>
                    <div class="search-result-content">
                      <div class="search-result-name">{recent}</div>
                    </div>
                    <i class="bi bi-arrow-return-left text-xs opacity-40"></i>
                  </button>
                {/each}
              </div>
            {:else if searchQuery.trim()}
              <div class="search-empty">
                <i class="bi bi-search opacity-40 text-2xl"></i>
                <p class="text-sm opacity-60 mt-2">No results found</p>
              </div>
            {:else}
              <div class="search-empty">
                <i class="bi bi-search opacity-40 text-2xl"></i>
                <p class="text-sm opacity-60 mt-2">Search files and folders</p>
                <div class="search-shortcuts mt-3">
                  <div class="search-shortcut-item">
                    <kbd
                      class="px-2 py-1 text-xs font-semibold bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded"
                      >Ctrl</kbd
                    >
                    <span>+</span>
                    <kbd
                      class="px-2 py-1 text-xs font-semibold bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded"
                      >K</kbd
                    >
                    <span class="text-xs opacity-60 ml-2">Quick search</span>
                  </div>
                  <div class="search-shortcut-item">
                    <kbd
                      class="px-2 py-1 text-xs font-semibold bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded"
                      >Ctrl</kbd
                    >
                    <span>+</span>
                    <kbd
                      class="px-2 py-1 text-xs font-semibold bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded"
                      >Shift</kbd
                    >
                    <span>+</span>
                    <kbd
                      class="px-2 py-1 text-xs font-semibold bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded"
                      >F</kbd
                    >
                    <span class="text-xs opacity-60 ml-2">Advanced</span>
                  </div>
                </div>
              </div>
            {/if}
          </div>
        {/if}
      </div>
    </div>

    <!-- Right: Actions -->
    <div class="header-right">
      <!-- WebSocket Connection Status with Dropdown -->
      <div class="relative group">
        <button
          class="px-3 py-1.5 text-sm rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors flex items-center gap-2"
          onclick={loadServerInfo}
        >
          <span class="relative flex h-3 w-3">
            {#if backendOnline}
              <span
                class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-500 opacity-75"
              ></span>
              <span
                class="relative inline-flex rounded-full h-3 w-3 bg-green-500"
              ></span>
            {:else}
              <span class="relative inline-flex rounded-full h-3 w-3 bg-red-500"
              ></span>
            {/if}
          </span>
          <span
            class="text-sm font-medium {backendOnline
              ? 'text-gray-700 dark:text-gray-200'
              : 'text-red-600 dark:text-red-400'}"
            >{backendOnline ? "Online" : "Offline"}</span
          >
        </button>

        {#if showServerInfo && serverInfo}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div class="fixed inset-0 z-[99]" onclick={closeServerInfo}></div>
          <div
            class="absolute right-0 top-full mt-2 z-[100] w-80 p-4 shadow-2xl bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-2xl"
          >
            <div>
              <div class="flex justify-between items-start mb-2">
                <h3
                  class="text-sm font-bold flex items-center gap-2 text-gray-900 dark:text-white"
                >
                  <i class="bi bi-server"></i>
                  Backend Status
                </h3>
                <button
                  class="w-6 h-6 rounded-full flex items-center justify-center hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-500 dark:text-gray-400 transition-colors"
                  onclick={closeServerInfo}
                >
                  <i class="bi bi-x-lg text-xs"></i>
                </button>
              </div>
              <div class="h-px bg-gray-200 dark:bg-gray-700 my-2"></div>
              <div class="grid grid-cols-2 gap-2 text-xs">
                <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-2">
                  <div class="text-xs text-gray-500 dark:text-gray-400">
                    Status
                  </div>
                  <div
                    class="text-sm font-semibold {backendOnline
                      ? 'text-green-600 dark:text-green-400'
                      : 'text-red-600 dark:text-red-400'}"
                  >
                    {backendOnline ? "Online" : "Offline"}
                  </div>
                </div>
                <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-2">
                  <div class="text-xs text-gray-500 dark:text-gray-400">
                    Version
                  </div>
                  <div
                    class="text-sm font-semibold text-gray-900 dark:text-white"
                  >
                    {serverInfo.rust_version || "N/A"}
                  </div>
                </div>
                <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-2">
                  <div class="text-xs text-gray-500 dark:text-gray-400">
                    CPU Cores
                  </div>
                  <div
                    class="text-sm font-semibold text-gray-900 dark:text-white"
                  >
                    {serverInfo.cpu_cores || "N/A"}
                  </div>
                </div>
                <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-2">
                  <div class="text-xs text-gray-500 dark:text-gray-400">
                    Memory
                  </div>
                  <div
                    class="text-sm font-semibold text-gray-900 dark:text-white"
                  >
                    {serverInfo.memory_total || "N/A"}
                  </div>
                </div>
              </div>
              {#if serverInfo.features}
                <div class="mt-3">
                  <p class="text-xs text-gray-500 dark:text-gray-400 mb-1">
                    Features:
                  </p>
                  <div class="flex flex-wrap gap-1">
                    {#each Object.entries(serverInfo.features) as [feature, enabled]}
                      {#if enabled}
                        <span
                          class="px-2 py-0.5 text-xs font-medium bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200 rounded-full"
                          >{feature}</span
                        >
                      {/if}
                    {/each}
                  </div>
                </div>
              {/if}
            </div>
          </div>
        {/if}
      </div>

      <!-- Theme Toggle -->
      <ThemeSwitcher />

      <!-- Notifications with Backend Integration -->
      <NotificationBell />

      <!-- User Menu -->
      <div class="user-menu-container">
        <button
          class="user-avatar-button"
          onclick={() => (showUserDropdown = !showUserDropdown)}
        >
          <div class="user-avatar">
            <span class="user-initials">{userInitials}</span>
            <div class="user-status-indicator"></div>
          </div>
        </button>

        {#if showUserDropdown}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="fixed inset-0 z-[99]"
            onclick={() => (showUserDropdown = false)}
          ></div>

          <div class="user-dropdown-new">
            <div class="user-dropdown-header-new">
              <div class="user-avatar-large-new">
                <span class="user-initials-large">{userInitials}</span>
                <div class="user-status-indicator-large"></div>
              </div>
              <div class="user-info-new">
                <p class="user-name-new">{$auth.username || "Admin"}</p>
                <p class="user-role-new">
                  <span
                    class="px-2 py-0.5 text-xs font-medium bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200 rounded-full"
                    >Administrator</span
                  >
                </p>
                <p class="user-email-new">admin@syncspace.local</p>
              </div>
            </div>

            <div class="divider-new"></div>

            <ul class="user-menu-new">
              <li>
                <button
                  class="user-menu-item-new"
                  onclick={() => dispatch("navigate", "profile")}
                >
                  <i class="bi bi-person-circle"></i>
                  <div class="menu-item-content">
                    <span class="menu-item-label">Profile</span>
                    <span class="menu-item-desc">View and edit profile</span>
                  </div>
                  <i class="bi bi-chevron-right menu-item-arrow"></i>
                </button>
              </li>
              <li>
                <button
                  class="user-menu-item-new"
                  onclick={() => dispatch("navigate", "settings")}
                >
                  <i class="bi bi-gear-fill"></i>
                  <div class="menu-item-content">
                    <span class="menu-item-label">Settings</span>
                    <span class="menu-item-desc">Preferences & options</span>
                  </div>
                  <i class="bi bi-chevron-right menu-item-arrow"></i>
                </button>
              </li>
              <li>
                <button
                  class="user-menu-item-new"
                  onclick={() => dispatch("navigate", "storage")}
                >
                  <i class="bi bi-hdd-fill"></i>
                  <div class="menu-item-content">
                    <span class="menu-item-label">Storage</span>
                    <span class="menu-item-desc">Manage your files</span>
                  </div>
                  <i class="bi bi-chevron-right menu-item-arrow"></i>
                </button>
              </li>
            </ul>

            <div class="divider-new"></div>

            <ul class="user-menu-new">
              <li>
                <button class="user-menu-item-new">
                  <i class="bi bi-question-circle"></i>
                  <div class="menu-item-content">
                    <span class="menu-item-label">Help & Support</span>
                  </div>
                </button>
              </li>
            </ul>

            <div class="divider-new"></div>

            <div class="user-dropdown-footer-new">
              <button class="logout-button-new" onclick={handleLogout}>
                <i class="bi bi-box-arrow-right"></i>
                <span>Log Out</span>
              </button>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
</header>

<!-- Material 3 Quick Search Modal -->
<Modal
  visible={showSearchModal}
  title="Quick Search"
  icon="search"
  size="md"
  variant="primary"
  on:close={() => (showSearchModal = false)}
>
  <form onsubmit={handleSearch}>
    <div class="mb-4">
      <label
        class="flex justify-between items-center mb-2"
        for="quickSearchInput"
      >
        <span class="text-sm font-medium text-gray-700 dark:text-gray-200"
          >Search Query</span
        >
        <span class="text-xs text-gray-500 dark:text-gray-400">
          <kbd
            class="px-1.5 py-0.5 text-xs bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded"
            >Ctrl</kbd
          >
          +
          <kbd
            class="px-1.5 py-0.5 text-xs bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded"
            >K</kbd
          >
        </span>
      </label>
      <div class="flex">
        <span
          class="px-4 bg-gray-50 dark:bg-gray-800 flex items-center rounded-l-xl border border-r-0 border-gray-300 dark:border-gray-600"
        >
          <i class="bi bi-search text-lg text-gray-500 dark:text-gray-400"></i>
        </span>
        <input
          id="quickSearchInput"
          type="text"
          placeholder="Search for files, folders..."
          class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-l-none rounded-r-xl bg-white dark:bg-gray-900 text-gray-900 dark:text-white placeholder:text-gray-400 dark:placeholder:text-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 focus:border-transparent"
          bind:value={searchQuery}
        />
      </div>
      <div class="mt-1">
        <span class="text-xs text-gray-500 dark:text-gray-400">
          💡 Use <strong>Ctrl+Shift+F</strong> for advanced search with filters
        </span>
      </div>
    </div>
  </form>

  <div slot="actions">
    <button
      type="button"
      class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-200 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors flex items-center gap-2"
      onclick={() => (showSearchModal = false)}
    >
      <i class="bi bi-x-lg"></i>
      Cancel
    </button>
    <button
      type="button"
      class="px-4 py-2 text-sm font-medium text-white bg-blue-600 dark:bg-blue-500 rounded-xl hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors flex items-center gap-2"
      onclick={handleSearch}
    >
      <i class="bi bi-search"></i>
      Search
    </button>
  </div>
</Modal>

<!-- Advanced Search Modal -->
<AdvancedSearchModal
  visible={showAdvancedSearch}
  on:search={handleAdvancedSearch}
  on:close={() => (showAdvancedSearch = false)}
/>

<style>
  /* Material 3 Expressive Header */
  .app-header {
    position: sticky;
    top: 0;
    z-index: 1000;
    height: 64px;
    background: white;
    border-bottom: 3px solid transparent;
    border-image: linear-gradient(90deg, #6366f1 0%, #8b5cf6 50%, #d946ef 100%)
      1;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
    color: #111827;
  }

  /* Dark Mode */
  :global(.dark) .app-header {
    background: #1f2937;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
    color: #f9fafb;
  }

  .header-container {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 100%;
    padding: 0 1.5rem;
    max-width: 100%;
    margin: 0 auto;
  }

  /* Left Section - Brand */
  .header-left {
    display: flex;
    align-items: center;
    min-width: 200px;
  }

  .brand-logo {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    cursor: pointer;
    transition: transform 0.2s ease;
  }

  .brand-logo:hover {
    transform: scale(1.05);
  }

  .logo-icon {
    width: 40px;
    height: 40px;
    border-radius: 12px;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 50%, #d946ef 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
    color: white;
    box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
  }

  .brand-name {
    font-size: 1.25rem;
    font-weight: 700;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 50%, #d946ef 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    letter-spacing: -0.02em;
  }

  /* Center Section - Search */
  .header-center {
    flex: 1;
    display: flex;
    justify-content: center;
    max-width: 600px;
    margin: 0 2rem;
  }

  .search-container {
    display: flex;
    width: 100%;
    gap: 0.5rem;
  }

  .search-button {
    flex: 1;
    height: 44px;
    background: #f9fafb;
    border: 1px solid rgba(17, 24, 39, 0.2);
    border-radius: 12px;
    padding: 0 1rem;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    color: #111827;
    transition: all 0.2s ease;
    cursor: pointer;
  }

  .search-button:hover {
    background: #f3f4f6;
    border-color: rgba(17, 24, 39, 0.3);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .search-icon {
    font-size: 1.125rem;
    color: rgba(17, 24, 39, 0.7);
  }

  .search-text {
    flex: 1;
    text-align: left;
    font-size: 0.875rem;
    color: rgba(17, 24, 39, 0.6);
  }

  .search-kbd {
    padding: 0.25rem 0.5rem;
    background: #f3f4f6;
    border: 1px solid rgba(17, 24, 39, 0.2);
    border-radius: 6px;
    font-size: 0.75rem;
    font-family: ui-monospace, monospace;
    color: rgba(17, 24, 39, 0.7);
  }

  .advanced-button {
    width: 44px;
    height: 44px;
    background: #f9fafb;
    border: 1px solid rgba(17, 24, 39, 0.2);
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #111827;
    font-size: 1.125rem;
    transition: all 0.2s ease;
    cursor: pointer;
  }

  .advanced-button:hover {
    background: #f3f4f6;
    border-color: rgba(17, 24, 39, 0.3);
    transform: translateY(-1px);
  }

  /* New Enhanced Search Styles */
  .search-container-new {
    position: relative;
    display: flex;
    width: 100%;
    gap: 0.5rem;
  }

  .search-input-wrapper {
    position: relative;
    flex: 1;
    display: flex;
    align-items: center;
    height: 44px;
    background: #f9fafb;
    border: 2px solid rgba(17, 24, 39, 0.1);
    border-radius: 12px;
    padding: 0 1rem;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .search-input-wrapper:focus-within {
    background: white;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .search-icon-new {
    font-size: 1.125rem;
    color: rgba(17, 24, 39, 0.5);
    margin-right: 0.75rem;
    transition: color 0.2s;
  }

  .search-input-wrapper:focus-within .search-icon-new {
    color: #3b82f6;
  }

  .search-input-new {
    flex: 1;
    border: none;
    background: transparent;
    outline: none;
    font-size: 0.875rem;
    color: #111827;
  }

  .search-input-new::placeholder {
    color: rgba(17, 24, 39, 0.4);
  }

  .search-clear-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    border: none;
    background: rgba(17, 24, 39, 0.1);
    border-radius: 50%;
    cursor: pointer;
    transition: all 0.2s;
    margin-right: 0.5rem;
  }

  .search-clear-btn:hover {
    background: rgba(17, 24, 39, 0.2);
  }

  .search-kbd-new {
    padding: 0.25rem 0.5rem;
    background: #f3f4f6;
    border: 1px solid rgba(17, 24, 39, 0.15);
    border-radius: 6px;
    font-size: 0.75rem;
    font-family: ui-monospace, monospace;
    color: rgba(17, 24, 39, 0.6);
  }

  .advanced-button-new {
    width: 44px;
    height: 44px;
    background: #f9fafb;
    border: 2px solid rgba(17, 24, 39, 0.1);
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: rgba(17, 24, 39, 0.7);
    font-size: 1.125rem;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    cursor: pointer;
  }

  .advanced-button-new:hover {
    background: #f3f4f6;
    border-color: rgba(59, 130, 246, 0.3);
    color: #3b82f6;
    transform: translateY(-1px);
  }

  /* Search Dropdown */
  .search-dropdown {
    position: absolute;
    top: calc(100% + 0.5rem);
    left: 0;
    right: 50px;
    background: white;
    border: 1px solid rgba(17, 24, 39, 0.1);
    border-radius: 12px;
    box-shadow: 0 10px 40px -10px rgba(0, 0, 0, 0.2);
    max-height: 400px;
    overflow-y: auto;
    z-index: 1000;
    animation: dropdownSlide 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  @keyframes dropdownSlide {
    from {
      opacity: 0;
      transform: translateY(-8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .search-section {
    padding: 0.5rem;
  }

  .search-section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 0.75rem;
  }

  .search-section-title {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: rgba(17, 24, 39, 0.5);
    padding: 0.5rem 0.75rem;
  }

  .search-clear-all {
    font-size: 0.75rem;
    color: #3b82f6;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    transition: all 0.2s;
  }

  .search-clear-all:hover {
    background: rgba(59, 130, 246, 0.1);
  }

  .search-result-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    padding: 0.75rem;
    border: none;
    background: transparent;
    cursor: pointer;
    border-radius: 8px;
    transition: all 0.15s cubic-bezier(0.4, 0, 0.2, 1);
    text-align: left;
  }

  .search-result-item:hover {
    background: rgba(17, 24, 39, 0.05);
  }

  .search-result-item i:first-child {
    font-size: 1.125rem;
    width: 1.5rem;
    flex-shrink: 0;
  }

  .search-result-content {
    flex: 1;
    min-width: 0;
  }

  .search-result-name {
    font-size: 0.875rem;
    font-weight: 500;
    color: #111827;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .search-result-path {
    font-size: 0.75rem;
    color: rgba(17, 24, 39, 0.5);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .search-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem 2rem;
    text-align: center;
  }

  .search-shortcuts {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    align-items: center;
  }

  .search-shortcut-item {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.75rem;
  }

  /* Right Section - Actions */
  .header-right {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    min-width: 200px;
    justify-content: flex-end;
    position: relative;
    z-index: 100;
  }

  .action-button {
    width: 44px;
    height: 44px;
    background: #f9fafb;
    border: 1px solid rgba(17, 24, 39, 0.2);
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #111827;
    font-size: 1.125rem;
    transition: all 0.2s ease;
    cursor: pointer;
    position: relative;
  }

  .action-button:hover {
    background: #f3f4f6;
    border-color: rgba(17, 24, 39, 0.3);
    transform: translateY(-1px);
  }

  /* Theme Toggle Animation */
  .theme-icon-wrapper {
    position: relative;
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .theme-icon {
    animation: themeRotate 0.5s ease;
  }

  @keyframes themeRotate {
    from {
      transform: rotate(0deg) scale(0.8);
      opacity: 0;
    }
    to {
      transform: rotate(360deg) scale(1);
      opacity: 1;
    }
  }

  /* Notification Badge */
  .notification-icon-wrapper {
    position: relative;
  }

  .notification-badge {
    position: absolute;
    top: -6px;
    right: -6px;
    width: 18px;
    height: 18px;
    background: #ef4444;
    color: white;
    border-radius: 50%;
    font-size: 0.625rem;
    font-weight: 700;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 2px solid #3b82f6;
    animation: pulse 2s infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      transform: scale(1);
    }
    50% {
      transform: scale(1.1);
    }
  }

  /* User Avatar */
  .user-menu-container {
    position: relative;
  }

  .user-avatar-button {
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
  }

  .user-avatar {
    width: 44px;
    height: 44px;
    border-radius: 12px;
    background: linear-gradient(135deg, #6366f1, #8b5cf6, #d946ef);
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    transition: all 0.2s ease;
    border: 2px solid rgba(17, 24, 39, 0.1);
    box-shadow: 0 2px 8px rgba(99, 102, 241, 0.2);
  }

  .user-avatar-button:hover .user-avatar {
    transform: scale(1.05);
    border-color: rgba(17, 24, 39, 0.2);
    box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
  }

  .user-initials {
    font-size: 0.875rem;
    font-weight: 700;
    color: white;
  }

  .user-status-indicator {
    position: absolute;
    bottom: -2px;
    right: -2px;
    width: 12px;
    height: 12px;
    background: #10b981;
    border: 2px solid #3b82f6;
    border-radius: 50%;
  }

  /* Dropdown Styles */
  .dropdown-content {
    animation: slideDown 0.2s ease;
    margin-top: 0.75rem;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  /* Notification Dropdown */
  .notification-dropdown {
    width: 360px;
    background: white;
    border: 1px solid #f3f4f6;
    border-radius: 16px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
    overflow: hidden;
  }

  .notification-header {
    padding: 1rem 1.25rem;
    border-bottom: 1px solid #f3f4f6;
  }

  .notification-title {
    font-size: 0.875rem;
    font-weight: 600;
    color: #111827;
  }

  .notification-list {
    list-style: none;
    padding: 0;
    margin: 0;
    max-height: 400px;
    overflow-y: auto;
  }

  .notification-item {
    all: unset;
    display: flex;
    align-items: start;
    gap: 0.75rem;
    padding: 1rem 1.25rem;
    width: 100%;
    cursor: pointer;
    transition: background 0.2s ease;
    border-bottom: 1px solid #f3f4f6;
  }

  .notification-item:hover {
    background: #f9fafb;
  }

  .notification-item:last-child {
    border-bottom: none;
  }

  .notification-icon {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    font-size: 1rem;
  }

  .notification-icon.success {
    background: rgba(16, 185, 129, 0.15);
    color: #10b981;
  }

  .notification-icon.info {
    background: rgba(59, 130, 246, 0.15);
    color: #3b82f6;
  }

  .notification-icon.warning {
    background: rgba(245, 158, 11, 0.15);
    color: #f59e0b;
  }

  .notification-content {
    flex: 1;
    min-width: 0;
  }

  .notification-text {
    font-size: 0.875rem;
    font-weight: 500;
    color: #111827;
    margin: 0;
  }

  .notification-time {
    font-size: 0.75rem;
    color: rgba(17, 24, 39, 0.6);
    margin: 0.25rem 0 0 0;
  }

  .notification-footer {
    padding: 0.75rem;
    border-top: 1px solid #f3f4f6;
  }

  /* New Notification Dropdown Styles */
  .notification-dropdown-new {
    width: 400px;
    background: white !important;
    border: 1px solid rgba(17, 24, 39, 0.1);
    border-radius: 16px;
    box-shadow: 0 10px 40px -10px rgba(0, 0, 0, 0.3);
    overflow: hidden;
    max-height: 600px;
    display: flex;
    flex-direction: column;
    backdrop-filter: none !important;
    -webkit-backdrop-filter: none !important;
  }

  .notification-header-new {
    padding: 1rem 1.25rem;
    border-bottom: 1px solid rgba(17, 24, 39, 0.08);
    background: #f9fafb !important;
  }

  .notification-title-new {
    font-size: 1rem;
    font-weight: 700;
    color: #111827;
    margin: 0 0 0.75rem 0;
  }

  .notification-header-actions {
    display: flex;
    gap: 0.5rem;
  }

  .notification-action-btn {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.375rem 0.75rem;
    font-size: 0.75rem;
    font-weight: 500;
    border: none;
    background: rgba(17, 24, 39, 0.05);
    color: rgba(17, 24, 39, 0.7);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .notification-action-btn:hover {
    background: rgba(17, 24, 39, 0.1);
    color: #111827;
  }

  .notification-action-btn.text-error {
    color: #ef4444;
  }

  .notification-action-btn.text-error:hover {
    background: rgba(239, 68, 68, 0.1);
  }

  .notification-list-new {
    list-style: none;
    padding: 0;
    margin: 0;
    overflow-y: auto;
    flex: 1;
  }

  .notification-item-new {
    all: unset;
    display: flex;
    align-items: start;
    gap: 0.875rem;
    padding: 1rem 1.25rem;
    width: 100%;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    border-bottom: 1px solid rgba(17, 24, 39, 0.05);
    position: relative;
  }

  .notification-item-new:hover {
    background: rgba(17, 24, 39, 0.03);
  }

  .notification-item-new.unread {
    background: rgba(59, 130, 246, 0.03);
  }

  .notification-item-new.unread:hover {
    background: rgba(59, 130, 246, 0.06);
  }

  .notification-avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: linear-gradient(135deg, #3b82f6, #8b5cf6);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    color: white;
    font-size: 0.875rem;
    font-weight: 600;
  }

  .notification-icon-new {
    width: 40px;
    height: 40px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    font-size: 1.125rem;
  }

  .notification-icon-new.success {
    background: rgba(16, 185, 129, 0.15);
    color: #10b981;
  }

  .notification-icon-new.info {
    background: rgba(59, 130, 246, 0.15);
    color: #3b82f6;
  }

  .notification-icon-new.warning {
    background: rgba(245, 158, 11, 0.15);
    color: #f59e0b;
  }

  .notification-content-new {
    flex: 1;
    min-width: 0;
  }

  .notification-title-text {
    font-size: 0.875rem;
    font-weight: 600;
    color: #111827;
    margin: 0 0 0.25rem 0;
  }

  .notification-message {
    font-size: 0.8125rem;
    color: rgba(17, 24, 39, 0.7);
    margin: 0 0 0.5rem 0;
    line-height: 1.4;
  }

  .notification-time-new {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.75rem;
    color: rgba(17, 24, 39, 0.5);
    margin: 0;
  }

  .notification-unread-indicator {
    position: absolute;
    top: 1.25rem;
    right: 1.25rem;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
  }

  .notification-footer-new {
    padding: 0.75rem;
    border-top: 1px solid rgba(17, 24, 39, 0.08);
    background: rgba(#f9fafb, 0.3);
  }

  .notification-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    text-align: center;
  }

  /* User Dropdown */
  .user-dropdown {
    width: 280px;
    background: white;
    border: 1px solid #f3f4f6;
    border-radius: 16px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
    overflow: hidden;
  }

  .user-dropdown-header {
    padding: 1.25rem;
    background: linear-gradient(
      135deg,
      rgba(99, 102, 241, 0.08),
      rgba(139, 92, 246, 0.08),
      rgba(217, 70, 239, 0.08)
    );
    border-bottom: 1px solid #f3f4f6;
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .user-avatar-large {
    width: 56px;
    height: 56px;
    border-radius: 14px;
    background: linear-gradient(135deg, #6366f1, #8b5cf6, #d946ef);
    display: flex;
    align-items: center;
    justify-center: center;
    border: 2px solid white;
    box-shadow: 0 4px 12px rgba(99, 102, 241, 0.2);
  }

  .user-initials-large {
    font-size: 1.25rem;
    font-weight: 700;
    color: white;
  }

  .user-info {
    flex: 1;
    min-width: 0;
  }

  .user-name {
    font-size: 0.9375rem;
    font-weight: 600;
    color: #111827;
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .user-email {
    font-size: 0.75rem;
    color: rgba(17, 24, 39, 0.6);
    margin: 0.25rem 0 0 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .user-menu {
    list-style: none;
    padding: 0.5rem;
    margin: 0;
  }

  .user-menu-item {
    all: unset;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    width: 100%;
    cursor: pointer;
    border-radius: 10px;
    transition: all 0.2s ease;
    font-size: 0.875rem;
    color: #111827;
  }

  .user-menu-item:hover {
    background: #f9fafb;
  }

  .user-menu-item i {
    font-size: 1.125rem;
    width: 20px;
    text-align: center;
  }

  .user-menu-item.logout {
    color: #ef4444;
  }

  .user-menu-item.logout:hover {
    background: rgba(239, 68, 68, 0.1);
  }

  .divider {
    height: 1px;
    background: #f3f4f6;
    margin: 0.5rem 0;
  }

  /* Responsive */
  @media (max-width: 1024px) {
    .brand-name {
      display: none;
    }

    .header-center {
      margin: 0 1rem;
    }
  }

  /* New User Dropdown Styles */
  .user-dropdown-new {
    position: absolute;
    right: 0;
    top: calc(100% + 0.75rem);
    z-index: 100;
    width: 320px;
    background: white;
    border: 1px solid rgba(17, 24, 39, 0.1);
    border-radius: 16px;
    box-shadow: 0 10px 40px -10px rgba(0, 0, 0, 0.2);
    overflow: hidden;
    animation: slideDown 0.2s ease;
  }

  .user-dropdown-header-new {
    padding: 1.5rem;
    background: linear-gradient(
      135deg,
      rgba(59, 130, 246, 0.1),
      rgba(139, 92, 246, 0.1)
    );
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .user-avatar-large-new {
    position: relative;
    width: 60px;
    height: 60px;
    border-radius: 50%;
    background: linear-gradient(135deg, #3b82f6, #8b5cf6);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    color: white;
    font-size: 1.5rem;
    font-weight: 700;
    border: 3px solid white;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .user-status-indicator-large {
    position: absolute;
    bottom: 2px;
    right: 2px;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: #10b981;
    border: 2px solid white;
  }

  .user-info-new {
    flex: 1;
    min-width: 0;
  }

  .user-name-new {
    font-size: 1.125rem;
    font-weight: 700;
    color: #111827;
    margin: 0 0 0.25rem 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .user-role-new {
    margin: 0 0 0.5rem 0;
  }

  .user-email-new {
    font-size: 0.8125rem;
    color: rgba(17, 24, 39, 0.6);
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .divider-new {
    height: 1px;
    background: rgba(17, 24, 39, 0.08);
    margin: 0;
  }

  .user-menu-new {
    list-style: none;
    padding: 0.5rem;
    margin: 0;
  }

  .user-menu-item-new {
    all: unset;
    display: flex;
    align-items: center;
    gap: 0.875rem;
    padding: 0.875rem 1rem;
    width: 100%;
    cursor: pointer;
    border-radius: 10px;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    color: #111827;
  }

  .user-menu-item-new:hover {
    background: rgba(17, 24, 39, 0.05);
  }

  .user-menu-item-new > i:first-child {
    font-size: 1.25rem;
    width: 24px;
    text-align: center;
    color: rgba(17, 24, 39, 0.7);
  }

  .menu-item-content {
    flex: 1;
    min-width: 0;
  }

  .menu-item-label {
    display: block;
    font-size: 0.875rem;
    font-weight: 600;
    color: #111827;
  }

  .menu-item-desc {
    display: block;
    font-size: 0.75rem;
    color: rgba(17, 24, 39, 0.5);
    margin-top: 0.125rem;
  }

  .menu-item-arrow {
    font-size: 0.875rem;
    color: rgba(17, 24, 39, 0.3);
    transition: transform 0.2s;
  }

  .user-menu-item-new:hover .menu-item-arrow {
    transform: translateX(2px);
    color: rgba(17, 24, 39, 0.6);
  }

  .user-dropdown-footer-new {
    padding: 0.75rem;
    background: rgba(249, 250, 251, 0.5);
  }

  .logout-button-new {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    width: 100%;
    padding: 0.75rem 1rem;
    border: none;
    background: transparent;
    color: #ef4444;
    font-size: 0.875rem;
    font-weight: 600;
    border-radius: 10px;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .logout-button-new:hover {
    background: rgba(239, 68, 68, 0.1);
  }

  .logout-button-new i {
    font-size: 1.125rem;
  }

  @media (max-width: 768px) {
    .search-text {
      display: none;
    }

    .search-kbd {
      display: none;
    }

    .header-center {
      max-width: 400px;
    }
  }
</style>
