<script>
  import { onMount, onDestroy } from "svelte";
  import { currentLang, currentPath } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { favorites as favoritesStore } from "../../stores/favorites.js";
  import { success, error as errorToast } from "../../stores/toast.js";
  import api from "../../lib/api.js";
  import {
    formatBytes,
    formatRelativeTime,
    getFileColor,
    getFileIcon,
  } from "../../lib/designSystem.js";
  import PageLayout from "../../components/layout/PageLayout.svelte";
  import ActionBar from "../../components/layout/ActionBar.svelte";
  import ShareModal from "../../components/sharing/ShareModal.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let favoriteFiles = $state([]);
  let loading = $state(false);
  let errorMsg = $state(null);
  let searchQuery = $state("");
  let viewMode = $state("grid");
  let filterType = $state("all");
  let sortBy = $state("manual");
  let sortOrder = $state("asc");
  let dragId = $state(null);
  let savingOrder = $state(false);
  let showShareModal = $state(false);
  let shareTarget = $state(null);

  const unsubscribeFavorites = favoritesStore.subscribe(() => {
    loadFavorites();
  });

  onDestroy(() => unsubscribeFavorites?.());
  onMount(loadFavorites);

  const breadcrumbs = [
    { label: "Files", path: "#/files" },
    { label: "Favorites", path: "#/files/favorites" },
  ];

  const isReorderable = $derived(
    () =>
      filterType === "all" &&
      !searchQuery &&
      sortBy === "manual" &&
      favoriteFiles.length > 1 &&
      !loading
  );

  const filterChips = $derived(() => [
    {
      id: "all",
      label: `All (${favoriteFiles.length})`,
      icon: "stars",
      active: filterType === "all",
    },
    {
      id: "files",
      label: "Files",
      icon: "file-earmark",
      active: filterType === "files",
    },
    {
      id: "folders",
      label: "Folders",
      icon: "folder",
      active: filterType === "folders",
    },
  ]);

  const viewModeOptions = $derived(() => [
    { id: "grid", label: "Grid", icon: "grid", active: viewMode === "grid" },
    { id: "list", label: "List", icon: "list", active: viewMode === "list" },
  ]);

  const bulkActions = $derived(() => [
    {
      id: "refresh",
      label: "Refresh",
      icon: "arrow-clockwise",
      primary: false,
    },
  ]);

  let displayFavorites = $derived(() => {
    let result = [...favoriteFiles];

    if (filterType === "files") {
      result = result.filter((file) => file.itemType !== "folder");
    } else if (filterType === "folders") {
      result = result.filter((file) => file.itemType === "folder");
    }

    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      result = result.filter(
        (file) =>
          file.name.toLowerCase().includes(query) ||
          file.fullPath.toLowerCase().includes(query)
      );
    }

    if (sortBy === "manual") {
      result.sort((a, b) => (a.order ?? 0) - (b.order ?? 0));
    } else {
      result.sort((a, b) => {
        switch (sortBy) {
          case "name":
            return a.name.localeCompare(b.name);
          case "added":
            return new Date(a.createdAt) - new Date(b.createdAt);
          case "size":
            return (a.size || 0) - (b.size || 0);
          default:
            return 0;
        }
      });
    }

    if (sortOrder === "desc") {
      result.reverse();
    }

    return result;
  });

  async function loadFavorites() {
    loading = true;
    errorMsg = null;

    try {
      const favs = await api.favorites.list();
      favoriteFiles = (favs || [])
        .map((fav, index) => ({
          id: fav.id || fav.favorite_id || `${fav.item_id}-${index}`,
          itemId: fav.item_id,
          itemType: fav.item_type,
          name: fav.display_name || fav.item_id.split("/").pop() || fav.item_id,
          fullPath: fav.item_id,
          createdAt: fav.created_at,
          size: fav.size_bytes,
          mimeType: fav.mime_type,
          order: fav.position ?? index,
        }))
        .sort((a, b) => (a.order ?? 0) - (b.order ?? 0));
    } catch (err) {
      console.error("Failed to load favorites", err);
      errorMsg = tr("failedToLoadFavorites") ?? "Failed to load favorites";
      errorToast(err.message || errorMsg);
    } finally {
      loading = false;
    }
  }

  async function removeFavorite(file, event) {
    event?.stopPropagation();
    try {
      await favoritesStore.remove(file.itemId);
      success(`${file.name} removed from favorites`);
      await loadFavorites();
    } catch (err) {
      console.error("Failed to remove favorite", err);
      errorToast("Failed to remove favorite");
    }
  }

  function navigateToFile(filePath) {
    const segments = filePath.split("/");
    segments.pop();
    const dirPath = "/" + (segments.length ? `${segments.join("/")}/` : "");
    currentPath.set(dirPath);
  }

  function openShare(file, event) {
    event?.stopPropagation();
    shareTarget = {
      name: file.name,
      path: file.fullPath,
      is_dir: file.itemType === "folder",
    };
    showShareModal = true;
  }

  function handleSearch(event) {
    searchQuery = event.detail || "";
  }

  function handleFilter(event) {
    filterType = event.detail?.id || "all";
  }

  function handleViewChange(event) {
    viewMode = event.detail?.id || "grid";
  }

  function handleBulkAction(event) {
    const action = event.detail?.id;
    if (action === "refresh") {
      loadFavorites();
    }
  }

  function handleSortChange(value) {
    sortBy = value;
    if (value !== "manual") {
      dragId = null;
    }
  }

  function toggleSortOrder() {
    sortOrder = sortOrder === "asc" ? "desc" : "asc";
  }

  function handleDragStart(file) {
    if (!isReorderable) return;
    dragId = file.id;
  }

  function handleDragEnter(target) {
    if (!isReorderable || dragId === null || dragId === target.id) return;
    const from = favoriteFiles.findIndex((f) => f.id === dragId);
    const to = favoriteFiles.findIndex((f) => f.id === target.id);
    if (from === -1 || to === -1) return;
    favoriteFiles = arrayMove(favoriteFiles, from, to).map((fav, index) => ({
      ...fav,
      order: index,
    }));
  }

  async function handleDragEnd() {
    if (!isReorderable || dragId === null) {
      dragId = null;
      return;
    }
    dragId = null;
    await persistFavoriteOrder();
  }

  async function persistFavoriteOrder() {
    savingOrder = true;
    try {
      if (typeof api.favorites.reorder === "function") {
        await api.favorites.reorder(favoriteFiles.map((fav) => fav.itemId));
      }
      success("Favorites order updated");
    } catch (err) {
      console.error("Failed to save favorite order", err);
      errorToast("Failed to save favorite order");
    } finally {
      savingOrder = false;
    }
  }

  function arrayMove(list, from, to) {
    const updated = [...list];
    const [item] = updated.splice(from, 1);
    updated.splice(to, 0, item);
    return updated;
  }
</script>

<PageLayout
  title="Favorites"
  subtitle="Pin frequently accessed files and folders"
  icon="star-fill"
  {loading}
  {breadcrumbs}
  empty={!loading && !errorMsg && displayFavorites().length === 0}
  emptyProps={{
    icon: "star",
    title: "No favorites yet",
    message:
      "Star files to see them here for instant access across every device.",
  }}
>
  <svelte:fragment slot="header-actions">
    <button
      class="inline-flex items-center gap-2 rounded-2xl border border-white/30 bg-white/80 px-4 py-2 text-sm font-medium text-slate-700 shadow-sm transition hover:-translate-y-0.5 hover:border-indigo-400 hover:text-indigo-600 dark:bg-slate-900/60 dark:text-slate-200"
      onclick={loadFavorites}
      disabled={loading}
    >
      <i class={`bi bi-arrow-clockwise ${loading ? "animate-spin" : ""}`}></i>
      Refresh
    </button>
  </svelte:fragment>

  {#if !errorMsg && favoriteFiles.length > 0}
    <div class="space-y-4">
      <ActionBar
        searchValue={searchQuery}
        filterChips={filterChips()}
        viewModes={viewModeOptions()}
        bulkActions={bulkActions()}
        sticky={false}
        searchPlaceholder="Search favorites..."
        searchEnabled={true}
        on:search={handleSearch}
        on:filter={handleFilter}
        on:view={handleViewChange}
        on:bulk={handleBulkAction}
      >
        <div class="flex flex-wrap items-center gap-3">
          <div
            class="inline-flex items-center gap-2 rounded-2xl border border-slate-200/70 bg-white/70 px-3 py-1.5 text-sm font-medium dark:border-slate-700/70 dark:bg-slate-900/40"
          >
            <span class="text-xs uppercase tracking-[0.2em] text-slate-400"
              >Sort</span
            >
            <select
              class="bg-transparent text-sm font-semibold focus:outline-none"
              value={sortBy}
              on:change={(event) => handleSortChange(event.currentTarget.value)}
            >
              <option value="manual">Manual order</option>
              <option value="name">Name</option>
              <option value="added">Added date</option>
              <option value="size">Size</option>
            </select>
          </div>
          <button
            type="button"
            class="rounded-2xl border border-slate-200/70 bg-white/70 px-3 py-1.5 text-sm font-semibold text-slate-600 transition hover:border-indigo-400 hover:text-indigo-500 dark:border-slate-700/70 dark:bg-slate-900/40 dark:text-slate-200"
            onclick={toggleSortOrder}
            aria-label="Toggle sort order"
          >
            <i
              class={`bi bi-sort-${sortOrder === "asc" ? "alpha-down" : "alpha-up"}`}
            ></i>
          </button>
          {#if isReorderable}
            <span
              class="inline-flex items-center gap-2 rounded-2xl border border-emerald-200/60 bg-emerald-50/70 px-3 py-1.5 text-xs font-semibold uppercase tracking-[0.25em] text-emerald-600 dark:border-emerald-500/30 dark:bg-emerald-500/10 dark:text-emerald-200"
            >
              <i class="bi bi-arrows-move"></i>
              Drag to reorder
            </span>
          {:else}
            <span
              class="inline-flex items-center gap-2 rounded-2xl border border-slate-200/60 px-3 py-1.5 text-xs font-semibold uppercase tracking-[0.25em] text-slate-400"
            >
              <i class="bi bi-info-circle"></i>
              Enable manual sort to reorder
            </span>
          {/if}
        </div>
        <div
          slot="right"
          class="flex items-center gap-3 text-sm text-slate-500 dark:text-slate-300"
        >
          <span class="font-semibold text-slate-900 dark:text-white"
            >{displayFavorites().length}</span
          >
          <span>items</span>
        </div>
      </ActionBar>

      {#if savingOrder}
        <div
          class="flex items-center gap-2 rounded-2xl border border-indigo-200/60 bg-white/70 px-4 py-2 text-sm text-indigo-600 shadow-sm dark:border-indigo-500/30 dark:bg-indigo-500/10 dark:text-indigo-200"
        >
          <i class="bi bi-arrow-repeat animate-spin"></i>
          Saving new order...
        </div>
      {/if}
    </div>
  {/if}

  {#if errorMsg}
    <section
      class="rounded-3xl border border-red-200/60 bg-white/80 px-8 py-10 text-center text-red-600 shadow-sm dark:border-red-800/60 dark:bg-red-950/30 dark:text-red-200"
    >
      <div
        class="mb-4 inline-flex h-16 w-16 items-center justify-center rounded-2xl bg-red-500/15 text-3xl"
      >
        <i class="bi bi-exclamation-octagon"></i>
      </div>
      <h3 class="text-2xl font-semibold">{errorMsg}</h3>
      <p class="mt-2 text-sm text-red-500/80 dark:text-red-200/80">
        Something interrupted your favorites feed. Try again in a moment.
      </p>
      <button
        class="mt-6 inline-flex items-center gap-2 rounded-2xl bg-gradient-to-r from-indigo-500 to-purple-500 px-6 py-2 text-sm font-semibold text-white shadow-lg shadow-indigo-500/30 transition hover:-translate-y-0.5"
        onclick={loadFavorites}
      >
        <i class="bi bi-arrow-clockwise"></i>
        Try again
      </button>
    </section>
  {/if}

  {#if !errorMsg && displayFavorites().length > 0}
    <section
      class={`grid gap-6 ${
        viewMode === "grid"
          ? "grid-cols-1 sm:grid-cols-2 xl:grid-cols-3"
          : "grid-cols-1"
      }`}
    >
      {#each displayFavorites() as file (file.id)}
        <article
          class={`group relative overflow-hidden rounded-3xl border border-white/40 bg-white/80 p-5 text-left shadow-xl shadow-slate-900/5 transition duration-200 hover:-translate-y-0.5 hover:border-indigo-300 hover:shadow-2xl dark:border-slate-800/80 dark:bg-slate-900/70 ${
            viewMode === "list"
              ? "flex items-center gap-6"
              : "flex flex-col gap-6"
          } ${isReorderable ? "cursor-grab" : "cursor-pointer"}`}
          role="button"
          tabindex="0"
          draggable={isReorderable}
          on:click={() => navigateToFile(file.fullPath)}
          on:keydown={(event) =>
            (event.key === "Enter" || event.key === " ") &&
            navigateToFile(file.fullPath)}
          on:dragstart={() => handleDragStart(file)}
          on:dragenter={() => handleDragEnter(file)}
          on:dragend={handleDragEnd}
        >
          <div
            class={`relative ${viewMode === "list" ? "flex items-center gap-6" : "flex flex-col items-center"}`}
          >
            <div
              class={`relative flex h-20 w-20 items-center justify-center rounded-3xl text-3xl shadow-[0_20px_45px_rgba(15,23,42,0.12)] ${getFileColor(file.name)} bg-slate-50 dark:bg-slate-800/60`}
            >
              <i class={`bi bi-${getFileIcon(file.name)} text-3xl`}></i>
              <span
                class="absolute -top-2 -right-2 inline-flex h-8 w-8 items-center justify-center rounded-full bg-white text-amber-400 shadow-lg dark:bg-slate-900"
              >
                <i class="bi bi-star-fill"></i>
              </span>
            </div>
            <div
              class={`mt-4 flex flex-col ${viewMode === "list" ? "mt-0" : "items-center text-center"}`}
            >
              <h3
                class="text-lg font-semibold text-slate-900 dark:text-white"
                title={file.name}
              >
                {file.name}
              </h3>
              <p
                class="mt-1 truncate text-xs font-mono text-slate-500 dark:text-slate-400"
              >
                {file.fullPath}
              </p>
              <div
                class={`mt-3 flex flex-wrap items-center gap-2 text-xs text-slate-500 dark:text-slate-300 ${viewMode === "list" ? "justify-start" : "justify-center"}`}
              >
                {#if file.size}
                  <span
                    class="inline-flex items-center gap-1 rounded-full bg-slate-100/80 px-3 py-1 text-slate-700 dark:bg-slate-800/70 dark:text-slate-200"
                  >
                    <i class="bi bi-hdd"></i>
                    {formatBytes(file.size)}
                  </span>
                {/if}
                {#if file.createdAt}
                  <span
                    class="inline-flex items-center gap-1 rounded-full bg-slate-100/80 px-3 py-1 text-slate-700 dark:bg-slate-800/70 dark:text-slate-200"
                  >
                    <i class="bi bi-clock"></i>
                    {formatRelativeTime(file.createdAt)}
                  </span>
                {/if}
                <span
                  class="inline-flex items-center gap-1 rounded-full bg-slate-100/80 px-3 py-1 capitalize text-slate-700 dark:bg-slate-800/70 dark:text-slate-200"
                >
                  <i class="bi bi-tag"></i>
                  {file.itemType || "item"}
                </span>
              </div>
            </div>
          </div>

          <div
            class={`flex flex-wrap items-center gap-2 ${viewMode === "list" ? "ml-auto" : "w-full"}`}
          >
            <button
              type="button"
              class="inline-flex flex-1 items-center justify-center gap-2 rounded-2xl bg-gradient-to-r from-indigo-500 to-purple-500 px-4 py-2 text-sm font-semibold text-white shadow-lg shadow-indigo-500/30 transition hover:-translate-y-0.5"
              onclick={(event) => {
                event.stopPropagation();
                navigateToFile(file.fullPath);
              }}
            >
              <i class="bi bi-arrow-up-right"></i>
              Open
            </button>
            <button
              type="button"
              class="inline-flex items-center justify-center gap-2 rounded-2xl border border-slate-200/70 px-4 py-2 text-sm font-semibold text-slate-600 transition hover:border-indigo-400 hover:text-indigo-500 dark:border-slate-700/70 dark:text-slate-200"
              onclick={(event) => openShare(file, event)}
            >
              <i class="bi bi-share"></i>
              Share
            </button>
            <button
              type="button"
              class="inline-flex items-center justify-center gap-2 rounded-2xl border border-amber-200/70 bg-amber-50/80 px-4 py-2 text-sm font-semibold text-amber-700 transition hover:border-amber-400 dark:border-amber-500/40 dark:bg-amber-500/10 dark:text-amber-200"
              onclick={(event) => removeFavorite(file, event)}
            >
              <i class="bi bi-star"></i>
              Unfavorite
            </button>
          </div>
        </article>
      {/each}
    </section>
  {/if}
</PageLayout>

<ShareModal
  bind:isOpen={showShareModal}
  file={shareTarget}
  on:close={() => {
    showShareModal = false;
    shareTarget = null;
  }}
/>
