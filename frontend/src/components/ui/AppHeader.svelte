<script>
  import { createEventDispatcher } from "svelte";
  import { currentLang, currentView } from "../../stores/ui";
  import { auth } from "../../stores/auth";
  import { t } from "../../i18n.js";
  import AdvancedSearchModal from "../search/AdvancedSearchModal.svelte";
  import BackendInfoPanel from "../backend/BackendInfoPanel.svelte";
  import Modal from "./Modal.svelte";
  import ThemeSwitcher from "./ThemeSwitcher.svelte";
  import NotificationBell from "./NotificationBell.svelte";
  import HelpDialog from "./HelpDialog.svelte";
  import { userPreferences } from "../../stores/preferences.js";
  import { wsConnected } from "@stores/websocket.js";
  import { isDarkMode, toggleDarkMode } from "../../stores/serverState.js";
  import api, { API_BASE } from "../../lib/api.js";
  import { onMount } from "svelte";

  const dispatch = createEventDispatcher();

  let backendOnline = $state(false);
  let showBackendModal = $state(false);
  let showHelpDialog = $state(false);

  let { showActivityFeed = $bindable(false) } = $props();

  function toggleActivityFeed() {
    showActivityFeed = !showActivityFeed;
    dispatch("toggleActivityFeed", { visible: showActivityFeed });
  }

  // Check backend status periodically
  onMount(() => {
    checkBackendStatus();
    const interval = setInterval(checkBackendStatus, 10000); // Every 10 seconds

    // Handle outside clicks to close search dropdown
    const handleClickOutside = (event) => {
      if (
        showSearchDropdown &&
        searchDropdownRef &&
        !searchDropdownRef.contains(event.target) &&
        !searchInputRef?.contains(event.target)
      ) {
        showSearchDropdown = false;
      }
    };

    document.addEventListener("click", handleClickOutside);

    return () => {
      clearInterval(interval);
      document.removeEventListener("click", handleClickOutside);
    };
  });

  async function checkBackendStatus() {
    try {
      const response = await fetch(`${API_BASE}/status`, {
        method: "GET",
      });
      backendOnline = response.ok;
    } catch (err) {
      backendOnline = false;
    }
  }

  // Format file size in human readable format
  function formatFileSize(bytes) {
    if (!bytes || bytes === 0) return "";
    const units = ["B", "KB", "MB", "GB", "TB"];
    let unitIndex = 0;
    let size = bytes;
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }
    return `${size.toFixed(unitIndex > 0 ? 1 : 0)} ${units[unitIndex]}`;
  }

  function openBackendModal() {
    showBackendModal = true;
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
  let searchInputRef = $state(null);
  let searchDropdownRef = $state(null); // Reference for outside click detection

  let isDark = $derived($isDarkMode);
  let userInitials = $derived(
    $auth.username ? $auth.username.substring(0, 2).toUpperCase() : "AD"
  );

  let showUserDropdown = $state(false);

  // Navigate and close dropdown
  function navigateAndClose(view) {
    showUserDropdown = false;
    dispatch("navigate", view);
  }

  // Load notifications from API
  let notifications = $state([]);

  async function loadNotifications() {
    try {
      const data = (await api.notifications?.list?.()) || [];
      notifications = data.map((n) => ({
        id: n.id,
        type: n.type || "info",
        icon: n.icon || "info-circle-fill",
        title: n.title,
        message: n.message,
        time: n.created_at || "Just now",
        read: n.read || false,
        avatar: n.avatar || null,
      }));
    } catch (error) {
      console.error("Failed to load notifications:", error);
      notifications = [];
    }
  }

  onMount(() => {
    loadNotifications();
    const interval = setInterval(loadNotifications, 60000); // Refresh every minute
    return () => clearInterval(interval);
  });

  async function markAsRead(id) {
    try {
      await api.notifications?.markRead?.(id);
      notifications = notifications.map((n) =>
        n.id === id ? { ...n, read: true } : n
      );
    } catch (error) {
      console.error("Failed to mark notification as read:", error);
    }
  }

  async function markAllAsRead() {
    try {
      await api.notifications?.markAllRead?.();
      notifications = notifications.map((n) => ({ ...n, read: true }));
    } catch (error) {
      console.error("Failed to mark all as read:", error);
    }
  }

  async function clearAllNotifications() {
    try {
      await api.notifications?.clearAll?.();
      notifications = [];
    } catch (error) {
      console.error("Failed to clear notifications:", error);
    }
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

  // Handle search input change with debounce - use suggest API for autocomplete
  function handleSearchInput(e) {
    searchQuery = e.target.value;
    showSearchDropdown = true;

    if (searchDebounce) {
      clearTimeout(searchDebounce);
    }

    if (searchQuery.trim()) {
      searchDebounce = setTimeout(async () => {
        try {
          console.log("[AppHeader] Autocomplete for:", searchQuery);
          // Use suggest API for autocomplete (uses wildcard matching)
          const response = await api.search.suggest(searchQuery, 10);
          console.log("[AppHeader] Suggest response:", response);

          // suggest returns array directly, not wrapped in results
          if (!response || !Array.isArray(response)) {
            console.warn("[AppHeader] Invalid response format:", response);
            searchResults = [];
            return;
          }

          // Transform suggest results to UI format
          // Suggest returns: { text: filename, file_type: string|null, score: number }
          searchResults = response.map((result) => {
            const fileName = result.text || "Unknown";
            const fileType = result.file_type || "";
            const isFolder = fileType === "folder" || fileType === "directory";

            // Determine icon based on file type or extension
            let icon = "file-earmark-text";
            if (isFolder) {
              icon = "folder-fill";
            } else if (fileType === "pdf" || fileName.match(/\.(pdf)$/i)) {
              icon = "file-earmark-pdf";
            } else if (
              fileType === "document" ||
              fileName.match(/\.(docx?|doc)$/i)
            ) {
              icon = "file-earmark-word";
            } else if (
              fileType === "spreadsheet" ||
              fileName.match(/\.(xlsx?|xls|csv)$/i)
            ) {
              icon = "file-earmark-excel";
            } else if (
              fileType === "image" ||
              fileName.match(/\.(png|jpe?g|gif|svg|webp|bmp)$/i)
            ) {
              icon = "file-earmark-image";
            } else if (
              fileType === "video" ||
              fileName.match(/\.(mp4|webm|avi|mov)$/i)
            ) {
              icon = "file-earmark-play";
            } else if (
              fileType === "archive" ||
              fileName.match(/\.(zip|rar|7z|tar|gz)$/i)
            ) {
              icon = "file-earmark-zip";
            } else if (
              fileType === "code" ||
              fileName.match(
                /\.(js|ts|jsx|tsx|py|java|cpp|c|h|css|html|json)$/i
              )
            ) {
              icon = "file-earmark-code";
            }

            return {
              id: result.id || fileName, // Use filename as fallback id
              name: fileName,
              path: result.path || "", // Folder path from backend
              type: isFolder ? "folder" : "file",
              icon: icon,
              score: result.score || 0,
              size: result.size_bytes || 0,
              snippet: "",
            };
          });

          console.log(
            "[AppHeader] Transformed results:",
            $state.snapshot(searchResults)
          );
        } catch (err) {
          console.error("[AppHeader] Search failed:", err);
          searchResults = [];
        }
      }, 300);
    } else {
      searchResults = [];
    }
  } // Select search result and navigate to file
  function selectSearchResult(result) {
    saveRecentSearch(searchQuery); // Save the actual search query, not the filename
    showSearchDropdown = false;

    // Dispatch with full file information for navigation
    dispatch("searchResultSelected", {
      file: result,
      query: searchQuery,
      path: result.path,
    });

    // Clear search query after selection (Question 1)
    searchQuery = "";
    searchResults = [];
  }

  // Select recent search
  function selectRecentSearch(query) {
    searchQuery = query;
    showSearchDropdown = false;
    dispatch("search", { query });
  }

  async function toggleTheme() {
    // Backend-First: Use serverState toggleDarkMode
    await toggleDarkMode();
  }

  function handleLogout() {
    // Show confirmation modal
    const tr = (key, ...args) => t($currentLang, key, ...args);
    if (confirm(tr("areYouSureLogout"))) {
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
<header
  class="sticky top-0 z-[1000] h-16 bg-gradient-to-br from-white/95 to-slate-50/90 dark:from-gray-800/95 dark:to-gray-900/90 backdrop-blur-md border-b border-blue-500/10 dark:border-blue-500/15 shadow-[0_4px_20px_rgba(59,130,246,0.08)] dark:shadow-[0_4px_20px_rgba(59,130,246,0.12)] text-gray-900 dark:text-gray-50"
>
  <div class="flex items-center justify-between h-full px-6 max-w-full mx-auto">
    <!-- Left: Logo & Current View Name -->
    <div class="flex items-center min-w-[200px]">
      <div
        class="flex items-center gap-3 cursor-pointer transition-transform duration-200 hover:scale-105"
      >
        <div
          class="w-10 h-10 rounded-xl bg-gradient-to-br from-emerald-500 via-emerald-600 to-emerald-700 flex items-center justify-center text-xl text-white shadow-[0_6px_16px_rgba(16,185,129,0.35)]"
        >
          <i class={currentViewIcon}></i>
        </div>
        <span
          class="text-xl font-bold bg-gradient-to-br from-emerald-500 via-emerald-600 to-emerald-700 bg-clip-text text-transparent tracking-tight max-lg:hidden"
          >{currentViewName}</span
        >
      </div>
    </div>

    <!-- Center: Enhanced Search Bar -->
    <div class="flex-1 flex justify-center max-w-[600px] mx-4 lg:mx-8">
      <div class="relative flex w-full gap-2">
        <div
          class="relative flex-1 flex items-center h-11 bg-gradient-to-br from-white/80 to-slate-50/70 dark:from-gray-800/80 dark:to-gray-900/70 backdrop-blur-md border border-blue-500/15 dark:border-blue-500/20 rounded-xl px-4 transition-all duration-200 shadow-[0_2px_8px_rgba(59,130,246,0.05)] dark:shadow-[0_2px_8px_rgba(59,130,246,0.08)] focus-within:bg-gradient-to-br focus-within:from-white/95 focus-within:to-slate-50/90 dark:focus-within:from-gray-700/95 dark:focus-within:to-gray-800/90 focus-within:border-blue-500/40 dark:focus-within:border-blue-500/50 focus-within:shadow-[0_8px_24px_rgba(59,130,246,0.15)] dark:focus-within:shadow-[0_8px_24px_rgba(59,130,246,0.2)]"
        >
          <i
            class="bi bi-search text-lg text-gray-900/50 dark:text-white/50 mr-3 transition-colors"
            aria-hidden="true"
          ></i>
          <input
            type="text"
            class="flex-1 border-none bg-transparent outline-none text-sm text-gray-900 dark:text-gray-50 placeholder:text-gray-900/40 dark:placeholder:text-white/40"
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
              class="flex items-center justify-center w-6 h-6 border-none bg-gray-900/10 dark:bg-white/10 rounded-full cursor-pointer transition-all mr-2 hover:bg-gray-900/20 dark:hover:bg-white/20"
              onclick={() => {
                searchQuery = "";
                searchResults = [];
                showSearchDropdown = false;
              }}
              aria-label="Clear search"
            >
              <i class="bi bi-x" aria-hidden="true"></i>
            </button>
          {/if}
          <kbd
            class="px-2 py-1 bg-gray-100 dark:bg-gray-700 border border-gray-900/15 dark:border-white/15 rounded-md text-xs font-mono text-gray-900/60 dark:text-white/60"
            >Ctrl K</kbd
          >
        </div>

        <!-- Search Dropdown -->
        {#if showSearchDropdown}
          <div
            class="absolute top-[calc(100%+0.5rem)] left-0 right-[50px] bg-white dark:bg-gray-700 border border-gray-900/10 dark:border-white/10 rounded-xl shadow-[0_10px_40px_-10px_rgba(0,0,0,0.2)] max-h-[400px] overflow-y-auto z-[1000] animate-[dropdownSlide_0.2s_ease-out]"
            bind:this={searchDropdownRef}
          >
            {#if searchQuery.trim() && searchResults.length > 0}
              <div class="p-2">
                <div
                  class="text-xs font-semibold uppercase tracking-wider text-gray-900/50 dark:text-white/50 px-3 py-2"
                >
                  Results
                </div>
                {#each searchResults as result}
                  <button
                    class="flex items-center gap-3 w-full px-3 py-3 border-none bg-transparent cursor-pointer rounded-lg transition-all text-left hover:bg-gray-900/5 dark:hover:bg-white/5"
                    onclick={() => selectSearchResult(result)}
                  >
                    <i
                      class="bi bi-{result.icon} text-lg w-6 shrink-0 {result.type ===
                      'folder'
                        ? 'text-amber-500'
                        : 'text-blue-500'}"
                    ></i>
                    <div class="flex-1 min-w-0">
                      <div
                        class="text-sm font-medium text-gray-900 dark:text-gray-50 truncate"
                      >
                        {result.name}
                      </div>
                      <div
                        class="flex items-center gap-3 text-[0.7rem] text-gray-900/50 dark:text-white/50 mt-0.5"
                      >
                        {#if result.path}
                          <span
                            class="flex items-center gap-1 max-w-[200px] truncate"
                          >
                            <i class="bi bi-folder2 text-xs" aria-hidden="true"
                            ></i>
                            {result.path}
                          </span>
                        {/if}
                        {#if result.size > 0}
                          <span class="shrink-0 opacity-80"
                            >{formatFileSize(result.size)}</span
                          >
                        {/if}
                      </div>
                    </div>
                    <i
                      class="bi bi-arrow-return-left text-xs opacity-40"
                      aria-hidden="true"
                    ></i>
                  </button>
                {/each}
              </div>
            {:else if !searchQuery.trim() && recentSearches.length > 0}
              <div class="p-2">
                <div class="flex items-center justify-between px-3 py-2">
                  <div
                    class="text-xs font-semibold uppercase tracking-wider text-gray-900/50 dark:text-white/50"
                  >
                    {t($currentLang, "recentSearches")}
                  </div>
                  <button
                    class="text-xs text-blue-500 bg-transparent border-none cursor-pointer px-2 py-1 rounded transition-all hover:bg-blue-500/10"
                    onclick={clearRecentSearches}
                    >{t($currentLang, "clear")}</button
                  >
                </div>
                {#each recentSearches.slice(0, 5) as recent}
                  <button
                    class="flex items-center gap-3 w-full px-3 py-3 border-none bg-transparent cursor-pointer rounded-lg transition-all text-left hover:bg-gray-900/5 dark:hover:bg-white/5"
                    onclick={() => selectRecentSearch(recent)}
                  >
                    <i class="bi bi-clock-history opacity-60" aria-hidden="true"
                    ></i>
                    <div class="flex-1 min-w-0">
                      <div
                        class="text-sm font-medium text-gray-900 dark:text-gray-50 truncate"
                      >
                        {recent}
                      </div>
                    </div>
                    <i
                      class="bi bi-arrow-return-left text-xs opacity-40"
                      aria-hidden="true"
                    ></i>
                  </button>
                {/each}
              </div>
            {:else if searchQuery.trim()}
              <div
                class="flex flex-col items-center justify-center py-12 px-8 text-center"
              >
                <i class="bi bi-search opacity-40 text-2xl" aria-hidden="true"
                ></i>
                <p class="text-sm opacity-60 mt-2">No results found</p>
              </div>
            {:else}
              <div
                class="flex flex-col items-center justify-center py-12 px-8 text-center"
              >
                <i class="bi bi-search opacity-40 text-2xl" aria-hidden="true"
                ></i>
                <p class="text-sm opacity-60 mt-2">Search files and folders</p>
                <div class="flex flex-col gap-2 items-center mt-3">
                  <div class="flex items-center gap-1 text-xs">
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
                  <div class="flex items-center gap-1 text-xs">
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
    <div
      class="flex items-center gap-3 min-w-[200px] justify-end relative z-[100]"
    >
      <!-- WebSocket Connection Status with Dropdown -->
      <div class="relative group">
        <button
          class="px-3 py-1.5 text-sm rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors flex items-center gap-2"
          onclick={openBackendModal}
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
      </div>

      <!-- Activity Feed Toggle Button -->
      <button
        onclick={toggleActivityFeed}
        class="relative p-2 rounded-lg transition-all duration-300 hover:scale-110 {showActivityFeed
          ? 'bg-gradient-to-br from-emerald-500 to-green-600'
          : 'hover:bg-gray-100 dark:hover:bg-gray-800'}"
        title={showActivityFeed ? "Hide Activity Feed" : "Show Activity Feed"}
      >
        <i
          class="bi bi-activity text-xl transition-colors duration-300 {showActivityFeed
            ? 'text-white'
            : 'text-emerald-500 dark:text-emerald-400'}"
        ></i>
        {#if !showActivityFeed}
          <span class="absolute -top-0.5 -right-0.5 flex h-2 w-2">
            <span
              class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"
            ></span>
            <span
              class="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"
            ></span>
          </span>
        {/if}
      </button>

      <!-- Help & Support Button -->
      <button
        onclick={() => (showHelpDialog = true)}
        class="p-2 rounded-lg transition-all duration-300 hover:scale-110 hover:bg-gray-100 dark:hover:bg-gray-800"
        title="Help & Support"
      >
        <i
          class="bi bi-question-circle text-xl text-gray-600 dark:text-gray-400"
        ></i>
      </button>

      <!-- Theme Toggle -->
      <ThemeSwitcher />

      <!-- Favorites Quick Access -->
      <button
        onclick={() => dispatch("navigate", "favorites")}
        class="p-2 rounded-lg transition-all duration-300 hover:scale-110 hover:bg-gray-100 dark:hover:bg-gray-800"
        title="Favorites"
        aria-label="View favorites"
      >
        <i class="bi bi-star-fill text-xl text-amber-500 dark:text-amber-400"
        ></i>
      </button>

      <!-- Notifications with Backend Integration -->
      <NotificationBell />

      <!-- User Menu -->
      <div class="relative">
        <button
          class="bg-transparent border-none cursor-pointer p-0"
          onclick={() => (showUserDropdown = !showUserDropdown)}
        >
          <div
            class="w-11 h-11 rounded-xl bg-gradient-to-br from-indigo-500 via-purple-500 to-fuchsia-500 flex items-center justify-center relative transition-all duration-200 border-2 border-gray-900/10 dark:border-white/10 shadow-[0_2px_8px_rgba(99,102,241,0.2)] hover:scale-105 hover:border-gray-900/20 dark:hover:border-white/20 hover:shadow-[0_4px_12px_rgba(99,102,241,0.3)]"
          >
            <span class="text-sm font-bold text-white">{userInitials}</span>
            <div
              class="absolute -bottom-0.5 -right-0.5 w-3 h-3 bg-emerald-500 border-2 border-blue-500 dark:border-gray-800 rounded-full"
            ></div>
          </div>
        </button>

        {#if showUserDropdown}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="fixed inset-0 z-[99]"
            onclick={() => (showUserDropdown = false)}
            role="button"
            tabindex="0"
          ></div>

          <div
            class="absolute right-0 top-[calc(100%+0.75rem)] z-[100] w-80 bg-white dark:bg-gray-800 border border-gray-900/10 dark:border-white/10 rounded-2xl shadow-[0_10px_40px_-10px_rgba(0,0,0,0.2)] dark:shadow-[0_10px_40px_-10px_rgba(0,0,0,0.5)] overflow-hidden animate-[slideDown_0.2s_ease]"
          >
            <div
              class="p-6 bg-gradient-to-br from-blue-500/10 to-purple-500/10 flex items-center gap-4"
            >
              <div
                class="relative w-15 h-15 rounded-full bg-gradient-to-br from-blue-500 to-purple-500 flex items-center justify-center shrink-0 text-white text-2xl font-bold border-3 border-white shadow-[0_4px_12px_rgba(0,0,0,0.1)]"
              >
                <span>{userInitials}</span>
                <div
                  class="absolute bottom-0.5 right-0.5 w-3.5 h-3.5 rounded-full bg-emerald-500 border-2 border-white"
                ></div>
              </div>
              <div class="flex-1 min-w-0">
                <p
                  class="text-lg font-bold text-gray-900 dark:text-gray-50 m-0 truncate"
                >
                  {$auth.username || "Admin"}
                </p>
                <p class="m-0 mb-2">
                  <span
                    class="px-2 py-0.5 text-xs font-medium bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-200 rounded-full"
                    >Administrator</span
                  >
                </p>
                <p
                  class="text-[0.8125rem] text-gray-900/60 dark:text-white/60 m-0 truncate"
                >
                  admin@syncspace.local
                </p>
              </div>
            </div>

            <div class="h-px bg-gray-900/8 dark:bg-white/10"></div>

            <ul class="list-none p-2 m-0">
              <li>
                <button
                  class="flex items-center gap-3.5 px-4 py-3.5 w-full cursor-pointer rounded-lg transition-all text-gray-900 dark:text-gray-50 hover:bg-gray-900/5 dark:hover:bg-white/10"
                  onclick={() => navigateAndClose("profile")}
                >
                  <i
                    class="bi bi-person-circle text-xl w-6 text-center text-gray-900/70 dark:text-white/70"
                    aria-hidden="true"
                  ></i>
                  <div class="flex-1 min-w-0">
                    <span
                      class="block text-sm font-semibold text-gray-900 dark:text-gray-50"
                      >Profile</span
                    >
                    <span
                      class="block text-xs text-gray-900/50 dark:text-white/50 mt-0.5"
                      >View and edit profile</span
                    >
                  </div>
                  <i
                    class="bi bi-chevron-right text-sm text-gray-900/30 dark:text-white/30 transition-transform group-hover:translate-x-0.5"
                    aria-hidden="true"
                  ></i>
                </button>
              </li>
              <li>
                <button
                  class="flex items-center gap-3.5 px-4 py-3.5 w-full cursor-pointer rounded-lg transition-all text-gray-900 dark:text-gray-50 hover:bg-gray-900/5 dark:hover:bg-white/10"
                  onclick={() => navigateAndClose("user-settings")}
                >
                  <i
                    class="bi bi-sliders text-xl w-6 text-center text-gray-900/70 dark:text-white/70"
                    aria-hidden="true"
                  ></i>
                  <div class="flex-1 min-w-0">
                    <span
                      class="block text-sm font-semibold text-gray-900 dark:text-gray-50"
                      >Settings</span
                    >
                    <span
                      class="block text-xs text-gray-900/50 dark:text-white/50 mt-0.5"
                      >Preferences & options</span
                    >
                  </div>
                  <i
                    class="bi bi-chevron-right text-sm text-gray-900/30 dark:text-white/30 transition-transform"
                    aria-hidden="true"
                  ></i>
                </button>
              </li>
              <li>
                <button
                  class="flex items-center gap-3.5 px-4 py-3.5 w-full cursor-pointer rounded-lg transition-all text-gray-900 dark:text-gray-50 hover:bg-gray-900/5 dark:hover:bg-white/10"
                  onclick={() => navigateAndClose("security")}
                >
                  <i
                    class="bi bi-shield-lock text-xl w-6 text-center text-gray-900/70 dark:text-white/70"
                    aria-hidden="true"
                  ></i>
                  <div class="flex-1 min-w-0">
                    <span
                      class="block text-sm font-semibold text-gray-900 dark:text-gray-50"
                      >Security</span
                    >
                    <span
                      class="block text-xs text-gray-900/50 dark:text-white/50 mt-0.5"
                      >2FA & password</span
                    >
                  </div>
                  <i
                    class="bi bi-chevron-right text-sm text-gray-900/30 dark:text-white/30 transition-transform"
                    aria-hidden="true"
                  ></i>
                </button>
              </li>
              <li>
                <button
                  class="flex items-center gap-3.5 px-4 py-3.5 w-full cursor-pointer rounded-lg transition-all text-gray-900 dark:text-gray-50 hover:bg-gray-900/5 dark:hover:bg-white/10"
                  onclick={() => navigateAndClose("storage")}
                >
                  <i
                    class="bi bi-hdd-fill text-xl w-6 text-center text-gray-900/70 dark:text-white/70"
                    aria-hidden="true"
                  ></i>
                  <div class="flex-1 min-w-0">
                    <span
                      class="block text-sm font-semibold text-gray-900 dark:text-gray-50"
                      >Storage</span
                    >
                    <span
                      class="block text-xs text-gray-900/50 dark:text-white/50 mt-0.5"
                      >Usage & analytics</span
                    >
                  </div>
                  <i
                    class="bi bi-chevron-right text-sm text-gray-900/30 dark:text-white/30 transition-transform"
                    aria-hidden="true"
                  ></i>
                </button>
              </li>
            </ul>

            <div class="h-px bg-gray-900/8 dark:bg-white/10"></div>

            <ul class="list-none p-2 m-0">
              <li>
                <button
                  class="flex items-center gap-3.5 px-4 py-3.5 w-full cursor-pointer rounded-lg transition-all text-gray-900 dark:text-gray-50 hover:bg-gray-900/5 dark:hover:bg-white/10"
                >
                  <i
                    class="bi bi-question-circle text-xl w-6 text-center text-gray-900/70 dark:text-white/70"
                    aria-hidden="true"
                  ></i>
                  <div class="flex-1 min-w-0">
                    <span
                      class="block text-sm font-semibold text-gray-900 dark:text-gray-50"
                      >Help & Support</span
                    >
                  </div>
                </button>
              </li>
            </ul>

            <div class="h-px bg-gray-900/8 dark:bg-white/10"></div>

            <div class="p-3 bg-gray-50/50 dark:bg-gray-900/50">
              <button
                class="flex items-center justify-center gap-2 w-full px-4 py-3 border-none bg-transparent text-red-500 dark:text-red-400 text-sm font-semibold rounded-lg cursor-pointer transition-all hover:bg-red-500/10 dark:hover:bg-red-400/15"
                onclick={handleLogout}
              >
                <i class="bi bi-box-arrow-right text-lg" aria-hidden="true"></i>
                <span>Log Out</span>
              </button>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
</header>

<!-- Help & Support Dialog -->
<HelpDialog bind:open={showHelpDialog} />

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
          <i
            class="bi bi-search text-lg text-gray-500 dark:text-gray-400"
            aria-hidden="true"
          ></i>
        </span>
        <input
          id="quickSearchInput"
          type="text"
          placeholder="Search for files, folders..."
          class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-l-none rounded-r-xl bg-white dark:bg-gray-900 text-gray-900 dark:text-white placeholder:text-gray-400 dark:placeholder:text-gray-500 focus:outline-none focus:ring-2 focus:ring-green-500 dark:focus:ring-green-400 focus:border-transparent"
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
      <i class="bi bi-x-lg" aria-hidden="true"></i>
      Cancel
    </button>
    <button
      type="button"
      class="px-4 py-2 text-sm font-medium text-white bg-green-600 dark:bg-green-500 rounded-xl hover:bg-green-700 dark:hover:bg-green-600 transition-colors flex items-center gap-2"
      onclick={handleSearch}
    >
      <i class="bi bi-search" aria-hidden="true"></i>
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

<!-- Backend Info Modal -->
<BackendInfoPanel
  visible={showBackendModal}
  onClose={() => (showBackendModal = false)}
/>

<style>
  /* Keyframe animations that cannot be expressed in Tailwind */
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

  /* Custom width utilities */
  .w-15 {
    width: 3.75rem;
  }
  .h-15 {
    height: 3.75rem;
  }
  .border-3 {
    border-width: 3px;
  }
</style>
