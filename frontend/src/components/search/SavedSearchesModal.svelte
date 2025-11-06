<script>
  import {
    savedSearches,
    recentlyUsedSearches,
    mostUsedSearches,
  } from "../../stores/savedSearches.js";
  import { success, error as errorToast } from "../../stores/toast.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  /** @type {{onSelectSearch?: Function, visible?: boolean}} */
  let { onSelectSearch = () => {}, visible = $bindable(false) } = $props();

  /** @type {string} active tab */
  let activeTab = $state("all"); // all, recent, popular
  /** @type {string | null} search being renamed */
  let renamingId = $state(null);
  /** @type {string} new name for search */
  let newName = $state("");

  // Computed searches based on active tab
  const displaySearches = $derived(() => {
    switch (activeTab) {
      case "recent":
        return $recentlyUsedSearches;
      case "popular":
        return $mostUsedSearches;
      default:
        return $savedSearches;
    }
  });

  // Format date
  function formatDate(dateStr) {
    if (!dateStr) return tr("never");
    const date = new Date(dateStr);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffMins < 1) return tr("justNow");
    if (diffMins < 60) return tr("minutesAgo", diffMins);
    if (diffHours < 24) return tr("hoursAgo", diffHours);
    if (diffDays < 7) return tr("daysAgo", diffDays);
    return date.toLocaleDateString();
  }

  // Handle search selection
  function handleSelectSearch(search) {
    savedSearches.markSearchAsUsed(search.id);
    onSelectSearch(search);
    visible = false;
  }

  // Handle delete
  async function handleDelete(searchId, event) {
    event.stopPropagation();

    if (!confirm("Are you sure you want to delete this saved search?")) return;

    try {
      await savedSearches.deleteSearch(searchId);
      success("Saved search deleted");
    } catch (err) {
      errorToast("Failed to delete search");
    }
  }

  // Start renaming
  function startRename(search, event) {
    event.stopPropagation();
    renamingId = search.id;
    newName = search.name;
  }

  // Handle rename
  async function handleRename(searchId) {
    if (!newName.trim()) {
      errorToast("Search name cannot be empty");
      return;
    }

    try {
      await savedSearches.renameSearch(searchId, newName);
      success("Search renamed");
      renamingId = null;
      newName = "";
    } catch (err) {
      errorToast("Failed to rename search");
    }
  }

  // Cancel rename
  function cancelRename() {
    renamingId = null;
    newName = "";
  }

  // Format filters for display
  function formatFilters(filters) {
    if (!filters) return "";

    const parts = [];
    if (filters.fileType && filters.fileType !== "all") {
      parts.push(`Type: ${filters.fileType}`);
    }
    if (filters.sizeMin) {
      parts.push(`Size >= ${formatBytes(filters.sizeMin)}`);
    }
    if (filters.sizeMax) {
      parts.push(`Size <= ${formatBytes(filters.sizeMax)}`);
    }
    if (filters.dateFrom) {
      parts.push(`From: ${filters.dateFrom}`);
    }
    if (filters.dateTo) {
      parts.push(`To: ${filters.dateTo}`);
    }

    return parts.join(", ") || "No filters";
  }

  // Format bytes
  function formatBytes(bytes) {
    if (!bytes) return "0 B";
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${sizes[i]}`;
  }
</script>

{#if visible}
  <dialog class="modal modal-open" aria-labelledby="saved-searches-title">
    <div class="modal-box max-w-3xl">
      <h3
        id="saved-searches-title"
        class="text-lg font-bold mb-4 flex items-center gap-2"
      >
        <svg
          class="w-5 h-5"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z"
          />
        </svg>
        Saved Searches
      </h3>

      <!-- Tabs -->
      <div class="tabs tabs-boxed mb-4">
        <button
          class="tab {activeTab === 'all' ? 'tab-active' : ''}"
          onclick={() => (activeTab = "all")}
          aria-label="All saved searches"
        >
          All ({$savedSearches.length})
        </button>
        <button
          class="tab {activeTab === 'recent' ? 'tab-active' : ''}"
          onclick={() => (activeTab = "recent")}
          aria-label="Recently used searches"
        >
          Recent
        </button>
        <button
          class="tab {activeTab === 'popular' ? 'tab-active' : ''}"
          onclick={() => (activeTab = "popular")}
          aria-label="Most used searches"
        >
          Popular
        </button>
      </div>

      <!-- Search List -->
      {#if displaySearches().length === 0}
        <div class="text-center py-8 text-base-content/70">
          <svg
            class="w-16 h-16 mx-auto mb-4 opacity-50"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
          <p class="text-lg mb-2">No saved searches yet</p>
          <p class="text-sm">Save your frequent searches for quick access</p>
        </div>
      {:else}
        <div class="space-y-2 max-h-96 overflow-y-auto">
          {#each displaySearches() as search (search.id)}
            <div
              class="card bg-base-200 hover:bg-base-300 cursor-pointer transition-all"
              onclick={() => handleSelectSearch(search)}
              role="button"
              tabindex="0"
              onkeydown={(e) => e.key === "Enter" && handleSelectSearch(search)}
            >
              <div class="card-body p-4">
                <div class="flex items-start justify-between gap-3">
                  <div class="flex-1 min-w-0">
                    <!-- Search Name -->
                    {#if renamingId === search.id}
                      <div class="flex items-center gap-2 mb-2">
                        <input
                          type="text"
                          bind:value={newName}
                          class="input input-sm input-bordered flex-1"
                          onkeydown={(e) => {
                            e.stopPropagation();
                            if (e.key === "Enter") handleRename(search.id);
                            if (e.key === "Escape") cancelRename();
                          }}
                          onclick={(e) => e.stopPropagation()}
                        />
                        <button
                          class="btn btn-sm btn-success"
                          onclick={(e) => {
                            e.stopPropagation();
                            handleRename(search.id);
                          }}
                          aria-label="Save new name"
                        >
                          <svg
                            class="w-4 h-4"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                          >
                            <path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="2"
                              d="M5 13l4 4L19 7"
                            />
                          </svg>
                        </button>
                        <button
                          class="btn btn-sm btn-ghost"
                          onclick={(e) => {
                            e.stopPropagation();
                            cancelRename();
                          }}
                          aria-label="Cancel rename"
                        >
                          <svg
                            class="w-4 h-4"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                          >
                            <path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="2"
                              d="M6 18L18 6M6 6l12 12"
                            />
                          </svg>
                        </button>
                      </div>
                    {:else}
                      <h4 class="font-bold text-base mb-1 truncate">
                        {search.name}
                      </h4>
                    {/if}

                    <!-- Search Query -->
                    <p class="text-sm text-base-content/70 mb-2 truncate">
                      "{search.query}"
                    </p>

                    <!-- Filters -->
                    <p class="text-xs text-base-content/60 truncate">
                      {formatFilters(search.filters)}
                    </p>

                    <!-- Metadata -->
                    <div
                      class="flex items-center gap-3 mt-2 text-xs text-base-content/60"
                    >
                      <span>Created: {formatDate(search.createdAt)}</span>
                      {#if search.lastUsed}
                        <span>Last used: {formatDate(search.lastUsed)}</span>
                      {/if}
                      {#if search.useCount > 0}
                        <span class="badge badge-sm"
                          >{search.useCount} uses</span
                        >
                      {/if}
                    </div>
                  </div>

                  <!-- Actions -->
                  <div class="flex items-center gap-1">
                    <button
                      class="btn btn-sm btn-ghost btn-square"
                      onclick={(e) => startRename(search, e)}
                      title="Rename search"
                      aria-label="Rename search"
                    >
                      <svg
                        class="w-4 h-4"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          stroke-width="2"
                          d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                        />
                      </svg>
                    </button>
                    <button
                      class="btn btn-sm btn-ghost btn-square text-error"
                      onclick={(e) => handleDelete(search.id, e)}
                      title="Delete search"
                      aria-label="Delete search"
                    >
                      <svg
                        class="w-4 h-4"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          stroke-width="2"
                          d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                        />
                      </svg>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          {/each}
        </div>
      {/if}

      <!-- Actions -->
      <div class="modal-action">
        <button
          class="btn"
          onclick={() => (visible = false)}
          aria-label="Close saved searches"
        >
          Close
        </button>
      </div>
    </div>

    <!-- Backdrop -->
    <button
      class="modal-backdrop bg-black/50"
      onclick={() => (visible = false)}
      onkeydown={(e) => e.key === "Escape" && (visible = false)}
      aria-label="Close modal"
      tabindex="-1"
    ></button>
  </dialog>
{/if}

<style>
  .card {
    animation: fadeIn 0.2s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-4px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
