<script>
  import { onMount } from "svelte";
  import api from "../../lib/api.js";
  import { success, error as errorToast } from "../../stores/toast.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import PageLayout from "../../components/layout/PageLayout.svelte";
  import ActionBar from "../../components/layout/ActionBar.svelte";
  import Tabs from "../../components/ui/Tabs.v3.svelte";
  import StatsCard from "../../components/ui/StatsCard.svelte";
  import FormField from "../../components/ui/FormField.svelte";
  import ShareModal from "../../components/sharing/ShareModal.svelte";
  import ShareAnalyticsView from "../sharing/ShareAnalyticsView.svelte";
  import TanStackVirtualList from "../../components/ui/TanStackVirtualList.svelte";
  import Modal from "../../components/ui/Modal.svelte";
  import {
    formatBytes,
    formatRelativeTime,
    getFileAccent,
    getFileIcon,
  } from "../../lib/designSystem.js";
  import { cn } from "../../lib/design-system/utils.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  const breadcrumbs = [
    { label: "Files", path: "#/files" },
    { label: "Shared", path: "#/files/shared" },
  ];

  let loading = $state(true);
  let errorMsg = $state(null);
  let sharesByMe = $state([]);
  let sharesWithMe = $state([]);
  let searchQuery = $state("");
  let filterStatus = $state("all");
  let viewMode = $state("list");
  let activeTab = $state("byMe");
  let showShareModal = $state(false);
  let shareContext = $state(null);
  let selectedShare = $state(null);
  let editPermission = $state("read");
  let editExpiresAt = $state("");
  let savingShare = $state(false);
  let deletingShareId = $state(null);
  let analyticsShareId = $state(null);

  const totalAvailable = $derived(() => sharesByMe.length + sharesWithMe.length);

  const publicLinks = $derived(() =>
    sharesByMe.filter(
      (share) =>
        share.allowExternal || share.raw?.is_public || share.raw?.share_type === "public"
    )
  );

  const currentTabItems = $derived(() => {
    if (activeTab === "withMe") return sharesWithMe;
    if (activeTab === "publicLinks") return publicLinks();
    return sharesByMe;
  });

  const filteredShares = $derived(() => {
    let list = [...currentTabItems()];

    if (filterStatus === "active") {
      list = list.filter((share) => !isExpired(share));
    } else if (filterStatus === "expired") {
      list = list.filter((share) => isExpired(share));
    } else if (filterStatus === "protected") {
      list = list.filter((share) => share.passwordProtected);
    } else if (filterStatus === "expiring") {
      list = list.filter((share) => isExpiringSoon(share));
    }

    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      list = list.filter((share) => {
        const target = `${share.name} ${share.path} ${share.owner ?? ""} ${share.permission}`;
        return target.toLowerCase().includes(query);
      });
    }

    return list.sort(
      (a, b) =>
        new Date(b.createdAt || b.raw?.created_at || 0) -
        new Date(a.createdAt || a.raw?.created_at || 0)
    );
  });

  const shareTabs = $derived(() => [
    {
      id: "byMe",
      label: "Shared by me",
      icon: "arrow-up-right",
      badge: sharesByMe.length,
    },
    {
      id: "withMe",
      label: "Shared with me",
      icon: "people",
      badge: sharesWithMe.length,
    },
    {
      id: "publicLinks",
      label: "Public links",
      icon: "globe",
      badge: publicLinks().length,
    },
  ]);

  const filterChips = $derived(() => [
    {
      id: "all",
      label: "All",
      icon: "stars",
      active: filterStatus === "all",
    },
    {
      id: "active",
      label: "Active",
      icon: "check2-circle",
      active: filterStatus === "active",
    },
    {
      id: "expiring",
      label: "Expiring soon",
      icon: "hourglass-split",
      active: filterStatus === "expiring",
    },
    {
      id: "expired",
      label: "Expired",
      icon: "x-octagon",
      active: filterStatus === "expired",
    },
    {
      id: "protected",
      label: "Protected",
      icon: "shield-lock-fill",
      active: filterStatus === "protected",
    },
  ]);

  const viewModes = $derived(() => [
    { id: "list", label: "List", icon: "list-task", active: viewMode === "list" },
    { id: "grid", label: "Grid", icon: "collection", active: viewMode === "grid" },
  ]);

  const statsCards = $derived(() => {
    const activeShares = sharesByMe.filter((share) => !isExpired(share));
    const protectedShares = sharesByMe.filter((share) => share.passwordProtected);
    const downloads = sharesByMe.reduce((sum, share) => sum + (share.downloads || 0), 0);
    const expiringSoon = sharesByMe.filter((share) => isExpiringSoon(share));

    return [
      {
        id: "active",
        icon: "shuffle",
        label: "Active shares",
        value: activeShares.length.toString(),
        sublabel: `${sharesByMe.length} total links`,
        variant: "indigo",
      },
      {
        id: "public",
        icon: "globe2",
        label: "Public links",
        value: publicLinks().length.toString(),
        sublabel: `${expiringSoon.length} expiring soon`,
        variant: "amber",
      },
      {
        id: "protected",
        icon: "shield-lock",
        label: "Protected shares",
        value: protectedShares.length.toString(),
        sublabel: `${downloads} total downloads`,
        variant: "rose",
      },
    ];
  });

  const bulkActions = [
    { id: "refresh", label: "Refresh", icon: "arrow-clockwise" },
    { id: "new", label: "New share", icon: "plus-lg", primary: true },
  ];

  const virtualHeight = $derived(() =>
    viewMode === "grid" ? "clamp(520px, 72vh, 960px)" : "clamp(420px, 68vh, 880px)"
  );

  const emptyCopy = $derived(() => {
    if (activeTab === "withMe") {
      return {
        icon: "people",
        title: "Nothing shared with you",
        message: "Collaborators will appear here once they grant you access.",
      };
    }

    if (activeTab === "publicLinks") {
      return {
        icon: "globe2",
        title: "No public links yet",
        message: "Create a link to share files externally with passwords and expirations.",
      };
    }

    return {
      icon: "share",
      title: "No shares yet",
      message: "Generate your first share to collaborate effortlessly across devices.",
    };
  });

  const canEditSelectedShare = $derived(() => selectedShare?.owned ?? false);

  $effect(() => {
    if (selectedShare) {
      editPermission = selectedShare.permission || "read";
      editExpiresAt = selectedShare.expiresAt ? toLocalInput(selectedShare.expiresAt) : "";
    } else {
      editPermission = "read";
      editExpiresAt = "";
    }
  });

  onMount(loadShares);

  async function loadShares(reselectId = null) {
    try {
      loading = true;
      errorMsg = null;

      const [ownResponse, sharedResponse] = await Promise.all([
        api.shares.list(),
        api.sharing?.listSharedWithMe
          ? api.sharing.listSharedWithMe().catch((err) => {
              console.warn("Shared-with-me endpoint unavailable", err);
              return [];
            })
          : Promise.resolve([]),
      ]);

      sharesByMe = normalizeShareList(ownResponse, true);
      sharesWithMe = normalizeShareList(sharedResponse, false);

      restoreSelection(reselectId ?? selectedShare?.id);
    } catch (err) {
      console.error("Failed to load shares", err);
      errorMsg = tr("failedToLoadShares") ?? "Failed to load shares";
      errorToast(errorMsg);
      sharesByMe = [];
      sharesWithMe = [];
      selectedShare = null;
    } finally {
      loading = false;
    }
  }

  function normalizeShareList(rawList = [], owned = false) {
    if (!Array.isArray(rawList)) return [];

    return rawList.map((share, index) => {
      const path = share.file_path || share.item_id || share.item_path || share.path || "";
      const fallbackName = path?.split("/").filter(Boolean).pop();
      const name = share.display_name || share.item_name || share.name || fallbackName || "Untitled";

      return {
        id: share.id || share.share_id || share.token || `${path}-${index}`,
        name,
        path: path || "/",
        permission: (share.permission || share.permissions || share.access || "read").toLowerCase(),
        createdAt: share.created_at || share.createdAt || null,
        expiresAt: share.expires_at || share.expiresAt || null,
        passwordProtected: Boolean(
          share.password_hash || share.password_protected || share.requires_password
        ),
        downloads: share.download_count ?? share.downloads ?? 0,
        size: share.size_bytes ?? share.size ?? null,
        owner: share.owner || share.owner_name || share.created_by || null,
        allowExternal: share.allow_external ?? share.is_public ?? false,
        allowUpload: share.allow_upload ?? false,
        lastAccessed: share.last_accessed_at || share.last_accessed || null,
        itemType: share.item_type || (share.is_dir ? "folder" : "file"),
        owned,
        raw: share,
      };
    });
  }

  function restoreSelection(preferredId) {
    if (!preferredId) {
      selectedShare = null;
      return;
    }

    const combined = [...sharesByMe, ...sharesWithMe];
    selectedShare = combined.find((share) => share.id === preferredId) || null;
  }

  function isExpired(share) {
    if (!share?.expiresAt) return false;
    return new Date(share.expiresAt) < new Date();
  }

  function isExpiringSoon(share) {
    if (!share?.expiresAt) return false;
    const expires = new Date(share.expiresAt);
    const now = new Date();
    const diffDays = (expires.getTime() - now.getTime()) / (1000 * 60 * 60 * 24);
    return diffDays > 0 && diffDays <= 7;
  }

  function toLocalInput(dateString) {
    if (!dateString) return "";
    const date = new Date(dateString);
    const tzOffset = date.getTimezoneOffset();
    const localDate = new Date(date.getTime() - tzOffset * 60000);
    return localDate.toISOString().slice(0, 16);
  }

  function fromLocalInput(value) {
    if (!value) return null;
    const date = new Date(value);
    return date.toISOString();
  }

  function handleSearch(event) {
    searchQuery = event.detail ?? "";
  }

  function handleFilter(event) {
    filterStatus = event.detail?.id || "all";
  }

  function handleView(event) {
    viewMode = event.detail?.id || "list";
  }

  function handleBulk(event) {
    const action = event.detail?.id;
    if (action === "refresh") {
      loadShares();
    } else if (action === "new") {
      shareContext = null;
      showShareModal = true;
    }
  }

  function handleTabChange(tabId) {
    activeTab = tabId;
    filterStatus = "all";
    searchQuery = "";
    selectedShare = null;
  }

  function selectShare(share) {
    if (!share) return;
    selectedShare = share;
  }

  function clearSelection() {
    selectedShare = null;
  }

  function openShareComposer(share, event) {
    event?.stopPropagation();
    shareContext = share
      ? {
          name: share.name,
          path: share.path,
          is_dir: share.itemType === "folder",
        }
      : null;
    showShareModal = true;
  }

  function handleShareModalClose() {
    showShareModal = false;
    shareContext = null;
    loadShares();
  }

  async function copyShareLink(share, event) {
    event?.stopPropagation();
    const token = share?.raw?.share_token || share?.raw?.token || share?.id;
    const link = `${window.location.origin}/sharing/public/${token}`;

    try {
      await navigator.clipboard.writeText(link);
      success(tr("shareLinkCopiedToClipboard") ?? "Link copied");
    } catch (err) {
      console.error("Failed to copy link", err);
      errorToast(tr("failedToCopyLink") ?? "Failed to copy link");
    }
  }

  async function saveShareChanges() {
    if (!selectedShare?.owned) return;

    savingShare = true;
    try {
      await api.shares.update(selectedShare.id, {
        permission: editPermission,
        expires_at: fromLocalInput(editExpiresAt),
      });
      success(tr("shareUpdatedSuccessfully") ?? "Share updated");
      await loadShares(selectedShare.id);
    } catch (err) {
      console.error("Failed to update share", err);
      errorToast(tr("failedToUpdateShare") ?? "Failed to update share");
    } finally {
      savingShare = false;
    }
  }

  async function deleteShare(share, event) {
    event?.stopPropagation();
    if (!share?.owned) return;
    if (!confirm("Delete this share?")) return;

    deletingShareId = share.id;
    try {
      await api.shares.delete(share.id);
      success(tr("shareDeletedSuccessfully") ?? "Share deleted");
      if (selectedShare?.id === share.id) {
        selectedShare = null;
      }
      await loadShares();
    } catch (err) {
      console.error("Failed to delete share", err);
      errorToast(tr("failedToDeleteShare") ?? "Failed to delete share");
    } finally {
      deletingShareId = null;
    }
  }

  function openAnalytics(share, event) {
    event?.stopPropagation();
    analyticsShareId = share?.id || null;
  }

  function closeAnalytics() {
    analyticsShareId = null;
  }

  function formatPermission(permission) {
    if (!permission) return "Unknown";
    const normalized = permission.toLowerCase();
    if (normalized === "read") return "View only";
    if (normalized === "write") return "Can edit";
    if (normalized === "admin") return "Full access";
    return permission;
  }

  function shareOwnerLabel(share) {
    if (share?.owned) return "You";
    return share?.owner || "Unknown";
  }
</script>

<PageLayout
  title="Shared files"
  subtitle="Monitor every link, permission, and access in one dashboard"
  icon="share-fill"
  breadcrumbs={breadcrumbs}
  {loading}
  showSidebar={Boolean(selectedShare)}
>
  <svelte:fragment slot="header-actions">
    <button
      class="inline-flex items-center gap-2 rounded-2xl border border-white/40 bg-white/80 px-4 py-2 text-sm font-medium text-slate-700 shadow-sm transition hover:-translate-y-0.5 hover:border-indigo-400 hover:text-indigo-600 dark:bg-slate-900/60 dark:text-slate-200"
      onclick={loadShares}
      disabled={loading}
    >
      <i class={`bi bi-arrow-clockwise ${loading ? "animate-spin" : ""}`}></i>
      Refresh
    </button>
    <button
      class="inline-flex items-center gap-2 rounded-2xl bg-gradient-to-r from-indigo-500 to-purple-500 px-4 py-2 text-sm font-semibold text-white shadow-lg shadow-indigo-500/30 transition hover:-translate-y-0.5"
      onclick={() => {
        shareContext = null;
        showShareModal = true;
      }}
    >
      <i class="bi bi-plus-lg"></i>
      New share
    </button>
  </svelte:fragment>

  {#if !loading}
    <section class="grid gap-6 lg:grid-cols-3">
      {#each statsCards() as stat (stat.id)}
        <StatsCard
          icon={stat.icon}
          label={stat.label}
          value={stat.value}
          sublabel={stat.sublabel}
          variant={stat.variant}
        />
      {/each}
    </section>
  {/if}

  <section class="space-y-6">
    <Tabs
      tabs={shareTabs()}
      bind:activeTab
      variant="glass"
      size="md"
      fullWidth={true}
      ontabchange={handleTabChange}
    />

    {#if errorMsg}
      <div class="rounded-3xl border border-red-200/60 bg-white/80 px-6 py-5 text-red-600 shadow-sm dark:border-red-800/60 dark:bg-red-950/20 dark:text-red-200">
        <div class="flex items-center gap-3">
          <div class="h-12 w-12 rounded-2xl bg-red-500/15 text-2xl flex items-center justify-center">
            <i class="bi bi-exclamation-triangle-fill"></i>
          </div>
          <div>
            <p class="font-semibold text-lg">{errorMsg}</p>
            <p class="text-sm text-red-500/80 dark:text-red-200/80">Please try refreshing the shares list.</p>
          </div>
        </div>
      </div>
    {/if}

    {#if (!errorMsg && (totalAvailable() > 0 || searchQuery)) || filteredShares().length > 0}
      <ActionBar
        sticky={false}
        searchValue={searchQuery}
        filterChips={filterChips()}
        viewModes={viewModes()}
        bulkActions={bulkActions}
        searchPlaceholder="Search shares..."
        on:search={handleSearch}
        on:filter={handleFilter}
        on:view={handleView}
        on:bulk={handleBulk}
      >
        <div class="flex flex-wrap items-center gap-3 text-xs text-slate-500 dark:text-slate-300">
          <span class="inline-flex items-center gap-1 rounded-2xl border border-slate-200/70 bg-white/70 px-3 py-1.5 font-semibold dark:border-slate-700/70 dark:bg-slate-900/40">
            <i class="bi bi-layers"></i>
            {filteredShares().length} visible
          </span>
          <span class="inline-flex items-center gap-1 rounded-2xl border border-slate-200/70 bg-white/70 px-3 py-1.5 font-semibold dark:border-slate-700/70 dark:bg-slate-900/40">
            <i class="bi bi-person"></i>
            {activeTab === "withMe" ? "Incoming" : activeTab === "publicLinks" ? "Public" : "Outgoing"}
          </span>
        </div>
        <div slot="right" class="flex items-center gap-3 text-sm text-slate-500 dark:text-slate-300">
          <span class="font-semibold text-slate-900 dark:text-white">{totalAvailable()}</span>
          <span>total shares</span>
        </div>
      </ActionBar>
    {/if}

    {#if !loading && !errorMsg && filteredShares().length === 0}
      <section class="flex flex-col items-center justify-center rounded-3xl border border-dashed border-slate-300/80 bg-white/70 px-10 py-16 text-center text-slate-500 shadow-inner dark:border-slate-700/70 dark:bg-slate-900/40 dark:text-slate-300">
        <div class="mb-5 inline-flex h-16 w-16 items-center justify-center rounded-2xl bg-slate-100 text-3xl text-slate-400 dark:bg-slate-800">
          <i class={`bi bi-${emptyCopy().icon}`}></i>
        </div>
        <h3 class="text-2xl font-semibold text-slate-900 dark:text-white">{emptyCopy().title}</h3>
        <p class="mt-2 max-w-xl text-sm text-slate-500 dark:text-slate-300">{emptyCopy().message}</p>
        {#if activeTab !== "withMe"}
          <button
            class="mt-6 inline-flex items-center gap-2 rounded-2xl bg-gradient-to-r from-indigo-500 to-purple-500 px-5 py-2 text-sm font-semibold text-white shadow-lg shadow-indigo-500/30 transition hover:-translate-y-0.5"
            onclick={() => {
              shareContext = null;
              showShareModal = true;
            }}
          >
            <i class="bi bi-plus-circle"></i>
            Create share
          </button>
        {/if}
      </section>
    {:else if filteredShares().length > 0}
      <section class="rounded-3xl border border-white/40 bg-white/80 p-1 shadow-xl shadow-slate-900/5 dark:border-slate-800/70 dark:bg-slate-900/50">
        <TanStackVirtualList
          items={filteredShares()}
          estimateSize={() => (viewMode === "grid" ? 320 : 170)}
          isGrid={viewMode === "grid"}
          columns="auto"
          persistKey={`shares-${activeTab}-${viewMode}`}
          style={`height: ${virtualHeight()};`}
          class="rounded-3xl"
        >
          {#snippet children(share)}
            {#if viewMode === "grid"}
              <div class="h-full">
                <article
                  class={cn(
                    "flex h-full flex-col gap-4 rounded-3xl border border-white/40 bg-white/90 p-5 text-left shadow-lg transition duration-200 hover:-translate-y-0.5 hover:border-indigo-300 hover:shadow-2xl dark:border-slate-800/70 dark:bg-slate-900/70",
                    selectedShare?.id === share.id && "ring-2 ring-indigo-400"
                  )}
                  role="button"
                  tabindex="0"
                  onclick={() => selectShare(share)}
                  on:keydown={(event) =>
                    (event.key === "Enter" || event.key === " ") && selectShare(share)
                  }
                >
                  <header class="flex items-center gap-4">
                    <div class={`h-14 w-14 rounded-2xl text-2xl flex items-center justify-center shadow ${getFileAccent(share.name)}`}>
                      <i class={`bi bi-${getFileIcon(share.name)}`}></i>
                    </div>
                    <div class="flex-1 min-w-0">
                      <h3 class="truncate text-lg font-semibold text-slate-900 dark:text-white" title={share.name}>
                        {share.name}
                      </h3>
                      <p class="truncate text-xs text-slate-500 dark:text-slate-300" title={share.path}>
                        {share.path}
                      </p>
                    </div>
                    <div class="flex flex-wrap gap-2 justify-end">
                      <span class="inline-flex items-center gap-1 rounded-full bg-slate-100 px-3 py-1 text-xs font-semibold text-slate-600 dark:bg-slate-800/80 dark:text-slate-200">
                        <i class="bi bi-key"></i>
                        {formatPermission(share.permission)}
                      </span>
                      {#if share.passwordProtected}
                        <span class="inline-flex items-center gap-1 rounded-full bg-rose-100 px-3 py-1 text-xs font-semibold text-rose-600 dark:bg-rose-500/20 dark:text-rose-100">
                          <i class="bi bi-shield-lock"></i>
                          Protected
                        </span>
                      {/if}
                      {#if isExpired(share)}
                        <span class="inline-flex items-center gap-1 rounded-full bg-slate-900 px-3 py-1 text-xs font-semibold text-white dark:bg-slate-800">
                          <i class="bi bi-x-octagon"></i>
                          Expired
                        </span>
                      {:else if isExpiringSoon(share)}
                        <span class="inline-flex items-center gap-1 rounded-full bg-amber-100 px-3 py-1 text-xs font-semibold text-amber-600 dark:bg-amber-500/20 dark:text-amber-100">
                          <i class="bi bi-hourglass-split"></i>
                          Expiring soon
                        </span>
                      {/if}
                    </div>
                  </header>

                  <div class="grid grid-cols-2 gap-3 text-xs text-slate-500 dark:text-slate-300">
                    <span class="inline-flex items-center gap-1">
                      <i class="bi bi-clock"></i>
                      {share.createdAt ? formatRelativeTime(share.createdAt) : "Unknown"}
                    </span>
                    <span class="inline-flex items-center gap-1">
                      <i class="bi bi-person"></i>
                      {shareOwnerLabel(share)}
                    </span>
                    <span class="inline-flex items-center gap-1">
                      <i class="bi bi-download"></i>
                      {share.downloads} downloads
                    </span>
                    {#if share.size}
                      <span class="inline-flex items-center gap-1">
                        <i class="bi bi-hdd"></i>
                        {formatBytes(share.size)}
                      </span>
                    {:else}
                      <span class="inline-flex items-center gap-1">
                        <i class="bi bi-arrow-repeat"></i>
                        {share.allowUpload ? "Upload enabled" : "Download only"}
                      </span>
                    {/if}
                  </div>

                  <footer class="mt-auto flex flex-wrap gap-2">
                    <button
                      class="inline-flex flex-1 items-center justify-center gap-1 rounded-2xl border border-slate-200/80 px-3 py-1.5 text-xs font-semibold text-slate-600 transition hover:border-indigo-400 hover:text-indigo-600 dark:border-slate-700/70 dark:text-slate-200"
                      onclick={(event) => copyShareLink(share, event)}
                    >
                      <i class="bi bi-clipboard"></i>
                      Copy link
                    </button>
                    <button
                      class="inline-flex flex-1 items-center justify-center gap-1 rounded-2xl border border-slate-200/80 px-3 py-1.5 text-xs font-semibold text-slate-600 transition hover:border-indigo-400 hover:text-indigo-600 dark:border-slate-700/70 dark:text-slate-200"
                      onclick={(event) => openShareComposer(share, event)}
                    >
                      <i class="bi bi-share"></i>
                      Reshare
                    </button>
                    {#if share.owned}
                      <button
                        class="inline-flex items-center gap-1 rounded-2xl border border-slate-200/80 px-3 py-1.5 text-xs font-semibold text-slate-600 transition hover:border-indigo-400 hover:text-indigo-600 dark:border-slate-700/70 dark:text-slate-200"
                        onclick={(event) => openAnalytics(share, event)}
                      >
                        <i class="bi bi-graph-up"></i>
                        Analytics
                      </button>
                    {/if}
                  </footer>
                </article>
              </div>
            {:else}
              <article
                class={cn(
                  "mb-4 flex flex-col gap-4 rounded-3xl border border-white/40 bg-white/90 p-5 text-left shadow-lg transition duration-200 hover:-translate-y-0.5 hover:border-indigo-300 hover:shadow-2xl dark:border-slate-800/70 dark:bg-slate-900/70",
                  selectedShare?.id === share.id && "ring-2 ring-indigo-400"
                )}
                role="button"
                tabindex="0"
                onclick={() => selectShare(share)}
                on:keydown={(event) =>
                  (event.key === "Enter" || event.key === " ") && selectShare(share)
                }
              >
                <div class="flex items-start gap-4">
                  <div class={`h-14 w-14 rounded-2xl text-2xl flex items-center justify-center shadow ${getFileAccent(share.name)}`}>
                    <i class={`bi bi-${getFileIcon(share.name)}`}></i>
                  </div>
                  <div class="flex-1 min-w-0">
                    <div class="flex flex-wrap items-center gap-2">
                      <h3 class="truncate text-lg font-semibold text-slate-900 dark:text-white" title={share.name}>
                        {share.name}
                      </h3>
                      {#if share.passwordProtected}
                        <span class="inline-flex items-center gap-1 rounded-full bg-rose-100 px-2 py-0.5 text-[11px] font-semibold uppercase tracking-wide text-rose-600 dark:bg-rose-500/20 dark:text-rose-100">
                          <i class="bi bi-shield-lock"></i>
                          Protected
                        </span>
                      {/if}
                      {#if isExpired(share)}
                        <span class="inline-flex items-center gap-1 rounded-full bg-slate-900 px-2 py-0.5 text-[11px] font-semibold uppercase tracking-wide text-white">
                          <i class="bi bi-x-octagon"></i>
                          Expired
                        </span>
                      {:else if isExpiringSoon(share)}
                        <span class="inline-flex items-center gap-1 rounded-full bg-amber-50 px-2 py-0.5 text-[11px] font-semibold uppercase tracking-wide text-amber-600 dark:bg-amber-500/20 dark:text-amber-100">
                          <i class="bi bi-hourglass"></i>
                          Soon
                        </span>
                      {/if}
                    </div>
                    <p class="truncate text-sm text-slate-500 dark:text-slate-300" title={share.path}>
                      {share.path}
                    </p>
                  </div>
                </div>

                <div class="flex flex-wrap items-center gap-4 text-xs text-slate-500 dark:text-slate-300">
                  <span class="inline-flex items-center gap-1">
                    <i class="bi bi-key"></i>
                    {formatPermission(share.permission)}
                  </span>
                  <span class="inline-flex items-center gap-1">
                    <i class="bi bi-clock-history"></i>
                    {share.createdAt ? formatRelativeTime(share.createdAt) : "Unknown"}
                  </span>
                  <span class="inline-flex items-center gap-1">
                    <i class="bi bi-person"></i>
                    {shareOwnerLabel(share)}
                  </span>
                  <span class="inline-flex items-center gap-1">
                    <i class="bi bi-download"></i>
                    {share.downloads} downloads
                  </span>
                  {#if share.expiresAt}
                    <span class="inline-flex items-center gap-1">
                      <i class="bi bi-calendar-event"></i>
                      Expires {formatRelativeTime(share.expiresAt)}
                    </span>
                  {/if}
                </div>

                <div class="flex flex-wrap gap-2">
                  <button
                    class="inline-flex items-center gap-2 rounded-2xl border border-slate-200/80 px-3 py-1.5 text-xs font-semibold text-slate-600 transition hover:border-indigo-400 hover:text-indigo-600 dark:border-slate-700/70 dark:text-slate-200"
                    onclick={(event) => copyShareLink(share, event)}
                  >
                    <i class="bi bi-clipboard"></i>
                    Copy link
                  </button>
                  <button
                    class="inline-flex items-center gap-2 rounded-2xl border border-slate-200/80 px-3 py-1.5 text-xs font-semibold text-slate-600 transition hover:border-indigo-400 hover:text-indigo-600 dark:border-slate-700/70 dark:text-slate-200"
                    onclick={(event) => openShareComposer(share, event)}
                  >
                    <i class="bi bi-share"></i>
                    Reshare
                  </button>
                  {#if share.owned}
                    <button
                      class="inline-flex items-center gap-2 rounded-2xl border border-slate-200/80 px-3 py-1.5 text-xs font-semibold text-slate-600 transition hover:border-indigo-400 hover:text-indigo-600 dark:border-slate-700/70 dark:text-slate-200"
                      onclick={(event) => openAnalytics(share, event)}
                    >
                      <i class="bi bi-graph-up"></i>
                      Analytics
                    </button>
                    <button
                      class="inline-flex items-center gap-2 rounded-2xl border border-slate-200/80 px-3 py-1.5 text-xs font-semibold text-slate-600 transition hover:border-rose-400 hover:text-rose-600 dark:border-slate-700/70 dark:text-slate-200"
                      onclick={(event) => deleteShare(share, event)}
                      disabled={deletingShareId === share.id}
                    >
                      <i class={`bi bi-trash ${deletingShareId === share.id ? "animate-spin" : ""}`}></i>
                      Delete
                    </button>
                  {/if}
                </div>
              </article>
            {/if}
          {/snippet}
        </TanStackVirtualList>
      </section>
    {/if}
  </section>

  {#if selectedShare}
    <svelte:fragment slot="sidebar">
      <div class="flex flex-col gap-6">
        <div>
          <p class="text-xs font-semibold uppercase tracking-[0.3em] text-slate-400">Selection</p>
          <div class="mt-3 flex items-center gap-3">
            <div class={`h-12 w-12 rounded-2xl text-xl flex items-center justify-center shadow ${getFileAccent(selectedShare.name)}`}>
              <i class={`bi bi-${getFileIcon(selectedShare.name)}`}></i>
            </div>
            <div>
              <h4 class="text-base font-semibold text-slate-900 dark:text-white">{selectedShare.name}</h4>
              <p class="text-xs text-slate-500 dark:text-slate-300">{shareOwnerLabel(selectedShare)} • {formatPermission(selectedShare.permission)}</p>
            </div>
          </div>
          <button
            class="mt-3 inline-flex items-center gap-1 text-xs font-semibold text-indigo-600 hover:underline dark:text-indigo-300"
            onclick={clearSelection}
          >
            <i class="bi bi-x"></i>
            Deselect
          </button>
        </div>

        <div class="space-y-4">
          <FormField
            label="Permission"
            name="permission"
            type="select"
            options={[
              { value: "read", label: "View only" },
              { value: "write", label: "Can edit" },
              { value: "admin", label: "Full access" },
            ]}
            bind:value={editPermission}
            disabled={!canEditSelectedShare()}
          />

          <FormField
            label="Expires"
            name="expires"
            type="datetime-local"
            hint="Leave empty to remove expiration"
            bind:value={editExpiresAt}
            disabled={!canEditSelectedShare()}
          />

          <div class="rounded-2xl border border-slate-200/70 bg-slate-50 px-4 py-3 text-xs text-slate-500 dark:border-slate-700/60 dark:bg-slate-800/60 dark:text-slate-300">
            <p class="font-semibold text-slate-700 dark:text-white">Metadata</p>
            <ul class="mt-2 space-y-1">
              <li class="flex items-center gap-2">
                <i class="bi bi-clock-history"></i>
                Created {selectedShare.createdAt ? formatRelativeTime(selectedShare.createdAt) : "Unknown"}
              </li>
              {#if selectedShare.lastAccessed}
                <li class="flex items-center gap-2">
                  <i class="bi bi-activity"></i>
                  Last accessed {formatRelativeTime(selectedShare.lastAccessed)}
                </li>
              {/if}
              <li class="flex items-center gap-2">
                <i class="bi bi-upload"></i>
                {selectedShare.allowUpload ? "Upload allowed" : "Download only"}
              </li>
            </ul>
          </div>
        </div>

        <div class="space-y-3">
          <button
            class="w-full rounded-2xl bg-gradient-to-r from-indigo-500 to-purple-500 px-4 py-2 text-sm font-semibold text-white shadow-lg shadow-indigo-500/30 transition hover:-translate-y-0.5 disabled:opacity-50"
            onclick={saveShareChanges}
            disabled={!canEditSelectedShare() || savingShare}
          >
            {savingShare ? "Saving..." : "Save changes"}
          </button>
          {#if canEditSelectedShare()}
            <button
              class="w-full rounded-2xl border border-rose-200/70 px-4 py-2 text-sm font-semibold text-rose-600 transition hover:border-rose-400 dark:border-rose-500/30 dark:text-rose-200"
              onclick={(event) => deleteShare(selectedShare, event)}
              disabled={deletingShareId === selectedShare.id}
            >
              Delete share
            </button>
          {:else}
            <p class="text-xs text-slate-500 dark:text-slate-300">
              Only the owner can adjust permissions for this link.
            </p>
          {/if}
        </div>
      </div>
    </svelte:fragment>
  {/if}
</PageLayout>

<ShareModal
  bind:isOpen={showShareModal}
  file={shareContext}
  onClose={handleShareModalClose}
/>

<Modal
  visible={Boolean(analyticsShareId)}
  title="Share analytics"
  icon="graph-up"
  size="xl"
  variant="primary"
  onclose={closeAnalytics}
>
  {#snippet children()}
    {#if analyticsShareId}
      <ShareAnalyticsView shareId={analyticsShareId} />
    {/if}
  {/snippet}
</Modal>
