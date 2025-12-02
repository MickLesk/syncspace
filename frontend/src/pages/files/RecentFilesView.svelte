<script>
  import { onMount } from "svelte";
  import TanStackVirtualList from "../../components/ui/TanStackVirtualList.svelte";
  import PageLayout from "../../components/layout/PageLayout.svelte";
  import ActionBar from "../../components/layout/ActionBar.svelte";
  import ShareModal from "../../components/sharing/ShareModal.svelte";
  import api from "../../lib/api.js";
  import {
    formatBytes,
    formatRelativeTime,
    getFileIcon,
    getFileAccent,
  } from "../../lib/designSystem.js";
  import { currentPath } from "../../stores/ui.js";
  import { modals } from "../../stores/modals.js";

  let loading = $state(true);
  let errorMessage = $state(null);
  let recentFiles = $state([]);
  let searchQuery = $state("");
  let filterId = $state("all");
  let limit = $state(50);
  let viewMode = $state("timeline");
  let showShareModal = $state(false);
  let shareTarget = $state(null);

  const breadcrumbs = [
    { label: "Files", path: "#/files" },
    { label: "Recent", path: "#/files/recent" },
  ];

  const filterChips = $derived(() => [
    { id: "all", label: "All", icon: "infinity", active: filterId === "all" },
    { id: "view", label: "Viewed", icon: "eye", active: filterId === "view" },
    { id: "edit", label: "Edited", icon: "pencil", active: filterId === "edit" },
    {
      id: "download",
      label: "Downloaded",
      icon: "cloud-download",
      active: filterId === "download",
    },
    {
      id: "upload",
      label: "Uploaded",
      icon: "cloud-upload",
      active: filterId === "upload",
    },
  ]);

  const viewModes = $derived(() => [
    {
      id: "timeline",
      label: "Timeline",
      icon: "clock-history",
      active: viewMode === "timeline",
    },
    { id: "grid", label: "Grid", icon: "grid", active: viewMode === "grid" },
  ]);

  const bulkActions = [{ id: "refresh", label: "Refresh", icon: "arrow-clockwise" }];

  const filteredFiles = $derived(() => {
    let files = [...recentFiles];
    if (filterId !== "all") {
      files = files.filter((file) => file.accessType === filterId);
    }
    if (searchQuery) {
      const q = searchQuery.toLowerCase();
      files = files.filter(
        (file) =>
          file.name.toLowerCase().includes(q) ||
          file.path.toLowerCase().includes(q) ||
          file.owner?.toLowerCase().includes(q)
      );
    }
    return files;
  });

  const groupedTimeline = $derived(() => {
    const groups = new Map();
    filteredFiles().forEach((file) => {
      const bucket = resolveGroup(file.lastAccessed);
      if (!groups.has(bucket.label)) {
        groups.set(bucket.label, { ...bucket, items: [] });
      }
      groups.get(bucket.label).items.push(file);
    });

    const order = ["Today", "Yesterday", "Last 7 Days", "Older"];
    return order
      .map((label) => groups.get(label))
      .filter(Boolean)
      .map((group) => ({
        ...group,
        items: group.items.sort(
          (a, b) => new Date(b.lastAccessed) - new Date(a.lastAccessed)
        ),
      }));
  });

  const timelineVirtualItems = $derived(() => {
    const flattened = [];
    groupedTimeline().forEach((group) => {
      flattened.push({ type: "group", id: `group-${group.label}`, group });
      group.items.forEach((file) => {
        flattened.push({ type: "file", id: `${group.label}-${file.id}`, file });
      });
    });
    return flattened;
  });

  onMount(loadRecentFiles);

  async function loadRecentFiles() {
    loading = true;
    errorMessage = null;

    try {
      const payload = await api.recent.list(limit);
      recentFiles = Array.isArray(payload)
        ? payload.map((file, index) => normalizeFile(file, index))
        : [];
    } catch (error) {
      console.error("Failed to load recent files", error);
      errorMessage = error.message || "Unable to load recent files";
    } finally {
      loading = false;
    }
  }

  function normalizeFile(file, index) {
    const path = file.path || file.file_path || file.full_path || "/";
    const name = file.name || file.filename || path.split("/").pop() || "Untitled";
    const accessType = (file.access_type || file.action || "view").toLowerCase();

    return {
      id: file.id || file.file_id || `${path}-${index}`,
      name,
      path,
      owner: file.owner || file.modified_by || file.shared_by || "",
      size: file.size_bytes ?? file.size ?? 0,
      mimeType: file.mime_type || file.mimetype || "application/octet-stream",
      accessType,
      accessCount: file.access_count ?? file.count ?? 1,
      lastAccessed:
        file.last_accessed_at || file.accessed_at || file.updated_at || file.created_at,
    };
  }

  function resolveGroup(timestamp) {
    const now = new Date();
    const when = new Date(timestamp);
    const diffDays = Math.floor((now - when) / (1000 * 60 * 60 * 24));

    if (diffDays <= 0) {
      return { label: "Today", description: "Activity in the last 24 hours" };
    }
    if (diffDays === 1) {
      return { label: "Yesterday", description: "Yesterday's work" };
    }
    if (diffDays < 7) {
      return { label: "Last 7 Days", description: "From the past week" };
    }
    return { label: "Older", description: "Archival activity" };
  }

  function handleSearch(event) {
    searchQuery = event.detail || "";
  }

  function handleFilter(event) {
    filterId = event.detail?.id || "all";
  }

  function handleView(event) {
    viewMode = event.detail?.id || "timeline";
  }

  function handleBulk(event) {
    if (event.detail?.id === "refresh") {
      loadRecentFiles();
    }
  }

  function handleLimitChange(event) {
    limit = Number(event.currentTarget.value) || 20;
    loadRecentFiles();
  }

  function openShare(file, event) {
    event?.stopPropagation();
    shareTarget = { name: file.name, path: file.path, is_dir: false };
    showShareModal = true;
  }

  function closeShare() {
    showShareModal = false;
    shareTarget = null;
  }

  function navigateToFile(file, event) {
    event?.stopPropagation();
    const segments = file.path.split("/");
    segments.pop();
    const destination = segments.join("/") || "/";
    currentPath.set(destination.endsWith("/") ? destination : `${destination}/`);
    if (typeof window !== "undefined") {
      window.location.hash = "#/files";
    }
  }

  function quickPreview(file, event) {
    event?.stopPropagation();
    // Konvertiere das Recent-File Format in das erwartete Format für Preview Modal
    const previewFile = {
      name: file.name,
      path: file.path,
      size_bytes: file.size,
      mime_type: file.mimeType,
      is_directory: false,
    };
    // Alle Dateien für Navigation vorbereiten
    const allPreviewFiles = filteredFiles().map((f) => ({
      name: f.name,
      path: f.path,
      size_bytes: f.size,
      mime_type: f.mimeType,
      is_directory: false,
    }));
    const currentIndex = filteredFiles().findIndex((f) => f.id === file.id);
    modals.openPreview(previewFile, allPreviewFiles, currentIndex >= 0 ? currentIndex : 0);
  }
</script>

<PageLayout
  title="Recent files"
  subtitle="Follow edits, shares, and downloads across your workspace"
  icon="clock-history"
  breadcrumbs={breadcrumbs}
  {loading}
  empty={!loading && !errorMessage && filteredFiles().length === 0}
  emptyProps={{
    icon: "clock-history",
    title: "No recent activity yet",
    message: "Work with files to populate this timeline",
  }}
>
  <svelte:fragment slot="header-actions">
    <button
      class="inline-flex items-center gap-2 rounded-2xl border border-white/40 bg-white/80 px-4 py-2 text-sm font-medium text-slate-700 shadow-sm transition hover:-translate-y-0.5 hover:border-indigo-400 hover:text-indigo-600 dark:bg-slate-900/60 dark:text-slate-200"
      onclick={loadRecentFiles}
      disabled={loading}
    >
      <i class={`bi bi-arrow-clockwise ${loading ? "animate-spin" : ""}`}></i>
      Refresh
    </button>
  </svelte:fragment>

  {#if !errorMessage && recentFiles.length > 0}
    <ActionBar
      searchValue={searchQuery}
      filterChips={filterChips()}
      viewModes={viewModes()}
      bulkActions={bulkActions}
      sticky={false}
      searchPlaceholder="Search recent files..."
      on:search={handleSearch}
      on:filter={handleFilter}
      on:view={handleView}
      on:bulk={handleBulk}
    >
      <div class="flex flex-wrap items-center gap-3">
        <label class="inline-flex items-center gap-2 rounded-2xl border border-slate-200/70 bg-white/70 px-3 py-1.5 text-sm font-medium dark:border-slate-700/70 dark:bg-slate-900/40">
          <span class="text-xs uppercase tracking-[0.2em] text-slate-400">Limit</span>
          <select
            class="bg-transparent text-sm font-semibold focus:outline-none"
            value={limit}
            on:change={handleLimitChange}
          >
            <option value="20">Last 20</option>
            <option value="50">Last 50</option>
            <option value="100">Last 100</option>
          </select>
        </label>
        <span class="text-xs text-slate-500 dark:text-slate-300">
          {filteredFiles().length} visible entries
        </span>
      </div>
      <div slot="right" class="flex items-center gap-3 text-sm text-slate-500 dark:text-slate-300">
        <span class="font-semibold text-slate-900 dark:text-white">{recentFiles.length}</span>
        <span>total events</span>
      </div>
    </ActionBar>
  {/if}

  {#if errorMessage}
    <section class="rounded-3xl border border-red-200/60 bg-white/80 px-8 py-10 text-center text-red-600 shadow-sm dark:border-red-800/60 dark:bg-red-950/30 dark:text-red-200">
      <div class="mb-4 inline-flex h-16 w-16 items-center justify-center rounded-2xl bg-red-500/15 text-3xl">
        <i class="bi bi-exclamation-octagon"></i>
      </div>
      <h3 class="text-2xl font-semibold">{errorMessage}</h3>
      <p class="mt-2 text-sm text-red-500/80 dark:text-red-200/80">
        Something interrupted your recent activity feed.
      </p>
      <button
        class="mt-6 inline-flex items-center gap-2 rounded-2xl bg-gradient-to-r from-indigo-500 to-purple-500 px-6 py-2 text-sm font-semibold text-white shadow-lg shadow-indigo-500/30 transition hover:-translate-y-0.5"
        onclick={loadRecentFiles}
      >
        <i class="bi bi-arrow-clockwise"></i>
        Try again
      </button>
    </section>
  {:else if !loading && filteredFiles().length > 0}
    {#if viewMode === "timeline"}
      <section class="timeline-panel">
        <TanStackVirtualList
          items={timelineVirtualItems()}
          estimateSize={() => 140}
          persistKey="recent-files-timeline"
          style="height: clamp(420px, 70vh, 900px);"
          class="timeline-virtual"
        >
          {#snippet children(entry)}
            {#if entry.type === "group"}
              <div class="timeline-group">
                <div class="timeline-marker">
                  <span class="timeline-dot"></span>
                </div>
                <div>
                  <h3>{entry.group.label}</h3>
                  <p>{entry.group.description}</p>
                </div>
              </div>
            {:else}
              <div class="timeline-item">
                <div class="timeline-marker">
                  <span class="timeline-dot timeline-dot--item"></span>
                  <span class="timeline-line"></span>
                </div>
                <article class="timeline-card">
                  <header>
                    <div class={`file-icon ${getFileAccent(entry.file.name)}`}>
                      <i class={`bi bi-${getFileIcon(entry.file.name)}`}></i>
                    </div>
                    <div>
                      <h4 title={entry.file.name}>{entry.file.name}</h4>
                      <p>{entry.file.path}</p>
                    </div>
                    <div class="actions">
                      <button class="ghost" onclick={(event) => quickPreview(entry.file, event)}>
                        <i class="bi bi-eye"></i>
                      </button>
                      <button class="ghost" onclick={(event) => openShare(entry.file, event)}>
                        <i class="bi bi-share"></i>
                      </button>
                      <button class="solid" onclick={(event) => navigateToFile(entry.file, event)}>
                        Open
                      </button>
                    </div>
                  </header>
                  <footer>
                    <span>
                      <i class="bi bi-clock"></i>
                      {formatRelativeTime(entry.file.lastAccessed)}
                    </span>
                    {#if entry.file.size}
                      <span>
                        <i class="bi bi-hdd"></i>
                        {formatBytes(entry.file.size)}
                      </span>
                    {/if}
                    <span>
                      <i class="bi bi-activity"></i>
                      {entry.file.accessCount}× {entry.file.accessType}
                    </span>
                    {#if entry.file.owner}
                      <span>
                        <i class="bi bi-person"></i>
                        {entry.file.owner}
                      </span>
                    {/if}
                  </footer>
                </article>
              </div>
            {/if}
          {/snippet}
        </TanStackVirtualList>
      </section>
    {:else}
      <section class="grid-view">
        <div class="grid gap-6 sm:grid-cols-2 xl:grid-cols-3">
          {#each filteredFiles() as file (file.id)}
            <article class="recent-card" onclick={(event) => navigateToFile(file, event)}>
              <div class="recent-card__head">
                <div class={`file-icon ${getFileAccent(file.name)}`}>
                  <i class={`bi bi-${getFileIcon(file.name)}`}></i>
                </div>
                <div class="time-chip">
                  <i class="bi bi-clock-history"></i>
                  {formatRelativeTime(file.lastAccessed)}
                </div>
              </div>
              <h3 title={file.name}>{file.name}</h3>
              <p>{file.path}</p>
              <div class="meta">
                <span>
                  <i class="bi bi-hdd"></i>
                  {formatBytes(file.size)}
                </span>
                <span>
                  <i class="bi bi-activity"></i>
                  {file.accessCount}× {file.accessType}
                </span>
              </div>
              <div class="actions">
                <button class="ghost" onclick={(event) => quickPreview(file, event)}>
                  <i class="bi bi-eye"></i>
                  Preview
                </button>
                <button class="ghost" onclick={(event) => openShare(file, event)}>
                  <i class="bi bi-share"></i>
                  Share
                </button>
                <button class="solid" onclick={(event) => navigateToFile(file, event)}>
                  Open
                </button>
              </div>
            </article>
          {/each}
        </div>
      </section>
    {/if}
  {/if}
</PageLayout>

<ShareModal bind:isOpen={showShareModal} file={shareTarget} on:close={closeShare} />

<style>
  .timeline-panel {
    border-radius: 32px;
    border: 1px solid rgba(255, 255, 255, 0.25);
    background: rgba(255, 255, 255, 0.75);
    box-shadow: 0 50px 80px -40px rgba(15, 23, 42, 0.35);
    backdrop-filter: blur(24px);
    overflow: hidden;
  }

  :global(.dark) .timeline-panel {
    background: rgba(15, 23, 42, 0.85);
    border-color: rgba(148, 163, 184, 0.15);
  }

  .timeline-virtual {
    padding: 2rem;
  }

  .timeline-group {
    display: flex;
    gap: 1.5rem;
    align-items: center;
    margin-bottom: 1rem;
  }

  .timeline-group h3 {
    font-size: 1.25rem;
    font-weight: 600;
    margin: 0;
    color: rgb(15 23 42);
  }

  .timeline-group p {
    font-size: 0.9rem;
    margin: 0.25rem 0 0 0;
    color: rgb(100 116 139);
  }

  :global(.dark) .timeline-group h3 {
    color: rgb(248 250 252);
  }

  :global(.dark) .timeline-group p {
    color: rgb(148 163 184);
  }

  .timeline-item {
    display: flex;
    gap: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .timeline-marker {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 24px;
  }

  .timeline-dot {
    width: 14px;
    height: 14px;
    border-radius: 999px;
    background: linear-gradient(135deg, #667eea, #764ba2);
    box-shadow: 0 6px 12px rgba(102, 126, 234, 0.45);
  }

  .timeline-dot--item {
    margin-top: 0.35rem;
  }

  .timeline-line {
    flex: 1;
    width: 2px;
    background: linear-gradient(180deg, rgba(102, 126, 234, 0.4), transparent);
    margin-top: 0.25rem;
  }

  .timeline-card {
    flex: 1;
    background: rgba(255, 255, 255, 0.9);
    border: 1px solid rgba(255, 255, 255, 0.4);
    border-radius: 24px;
    padding: 1.5rem;
    box-shadow: 0 25px 50px -15px rgba(15, 23, 42, 0.35);
    backdrop-filter: blur(18px);
  }

  :global(.dark) .timeline-card {
    background: rgba(15, 23, 42, 0.85);
    border-color: rgba(148, 163, 184, 0.15);
  }

  .timeline-card header {
    display: flex;
    gap: 1.25rem;
    align-items: center;
  }

  .file-icon {
    width: 60px;
    height: 60px;
    border-radius: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
  }

  .timeline-card h4 {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: rgb(15 23 42);
  }

  .timeline-card p {
    margin: 0.25rem 0 0;
    font-size: 0.85rem;
    color: rgb(100 116 139);
  }

  :global(.dark) .timeline-card h4 {
    color: rgb(248 250 252);
  }

  :global(.dark) .timeline-card p {
    color: rgb(148 163 184);
  }

  .timeline-card .actions {
    margin-left: auto;
    display: flex;
    gap: 0.5rem;
  }

  .timeline-card .actions button {
    border-radius: 999px;
    padding: 0.4rem 0.9rem;
    font-size: 0.8rem;
    font-weight: 600;
    border: 1px solid rgba(148, 163, 184, 0.3);
    background: transparent;
    color: rgb(71 85 105);
    transition: 200ms ease;
  }

  .timeline-card .actions button.ghost:hover {
    border-color: rgba(102, 126, 234, 0.8);
    color: rgb(79 70 229);
  }

  .timeline-card .actions button.solid {
    background: linear-gradient(135deg, #667eea, #764ba2);
    border: none;
    color: white;
    box-shadow: 0 10px 25px rgba(102, 126, 234, 0.35);
  }

  .timeline-card footer {
    margin-top: 1.25rem;
    display: flex;
    flex-wrap: wrap;
    gap: 0.8rem;
    font-size: 0.8rem;
    color: rgb(71 85 105);
  }

  .timeline-card footer span {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.35rem 0.8rem;
    border-radius: 999px;
    background: rgba(241, 245, 249, 0.7);
  }

  :global(.dark) .timeline-card footer span {
    background: rgba(30, 41, 59, 0.9);
    color: rgb(226 232 240);
  }

  .grid-view .recent-card {
    background: rgba(255, 255, 255, 0.9);
    border: 1px solid rgba(255, 255, 255, 0.5);
    border-radius: 24px;
    padding: 1.5rem;
    box-shadow: 0 20px 45px rgba(15, 23, 42, 0.12);
    backdrop-filter: blur(12px);
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    transition: 200ms ease;
    cursor: pointer;
  }

  .recent-card:hover {
    transform: translateY(-4px);
    border-color: rgba(102, 126, 234, 0.7);
  }

  .recent-card__head {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .recent-card .time-chip {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.45rem 0.9rem;
    border-radius: 999px;
    background: rgba(99, 102, 241, 0.1);
    color: rgb(79 70 229);
    font-size: 0.8rem;
    font-weight: 600;
  }

  .recent-card h3 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: rgb(15 23 42);
  }

  .recent-card p {
    margin: 0;
    font-size: 0.85rem;
    color: rgb(100 116 139);
  }

  .recent-card .meta {
    display: flex;
    gap: 1rem;
    font-size: 0.85rem;
    color: rgb(71 85 105);
  }

  .recent-card .meta span {
    display: inline-flex;
    gap: 0.35rem;
    align-items: center;
  }

  .recent-card .actions {
    display: flex;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .recent-card .actions button {
    flex: 1;
    min-width: 120px;
    border-radius: 999px;
    padding: 0.6rem 1rem;
    font-size: 0.85rem;
    font-weight: 600;
    border: 1px solid rgba(148, 163, 184, 0.3);
    background: transparent;
    color: rgb(71 85 105);
    transition: 200ms ease;
  }

  .recent-card .actions button.ghost:hover {
    border-color: rgba(102, 126, 234, 0.9);
    color: rgb(79 70 229);
  }

  .recent-card .actions button.solid {
    background: linear-gradient(135deg, #667eea, #764ba2);
    border: none;
    color: white;
    box-shadow: 0 10px 25px rgba(102, 126, 234, 0.35);
  }

  @media (max-width: 768px) {
    .timeline-item {
      flex-direction: column;
    }

    .timeline-card header {
      flex-direction: column;
      align-items: flex-start;
    }

    .timeline-card .actions {
      width: 100%;
      justify-content: flex-start;
      flex-wrap: wrap;
    }

    .recent-card .meta {
      flex-direction: column;
    }
  }
</style>
