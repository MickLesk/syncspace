<script>
  import { onMount } from "svelte";
  import PageLayout from "../../components/layout/PageLayout.svelte";
  import StatsCard from "../../components/ui/StatsCard.svelte";
  import { success, error as errorToast } from "../../stores/toast.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import api from "../../lib/api.js";
  import {
    formatBytes as formatSize,
    formatRelativeTime,
  } from "../../lib/designSystem.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  const BIO_LIMIT = 320;

  const breadcrumbs = [
    { label: "Home", path: "#/files" },
    { label: "Profile", path: "#/user/profile" },
  ];

  function createEmptyUser() {
    return {
      username: "",
      email: "",
      displayName: "",
      bio: "",
      avatar: null,
      joinedDate: null,
      stats: {
        files: 0,
        storage: 0,
        shares: 0,
        uploads: 0,
      },
    };
  }

  function createDraft(source) {
    return {
      displayName: source.displayName || "",
      email: source.email || "",
      bio: source.bio || "",
      avatar: source.avatar || null,
    };
  }

  let loading = $state(true);
  let errorMsg = $state(null);
  let editMode = $state(false);
  let savingProfile = $state(false);
  let user = $state(createEmptyUser());
  let profileDraft = $state(createDraft(user));
  let recentActivity = $state([]);
  let activityLoading = $state(true);
  let activityError = $state(null);

  const statsCards = $derived(() => {
    const stats = user.stats || {};
    return [
      {
        id: "files",
        icon: "folder2-open",
        label: tr("profile.filesStored") || "Files stored",
        value: (stats.files ?? 0).toLocaleString(),
        sublabel: tr("profile.filesStoredHint") || "Total files synced",
        variant: "indigo",
      },
      {
        id: "storage",
        icon: "hdd-stack",
        label: tr("profile.storageFootprint") || "Storage footprint",
        value: formatSize(stats.storage || 0),
        sublabel: tr("profile.storageFootprintHint") || "Used across all spaces",
        variant: "amber",
      },
      {
        id: "shares",
        icon: "people",
        label: tr("profile.activeShares") || "Active shares",
        value: (stats.shares ?? 0).toString(),
        sublabel: tr("profile.activeSharesHint") || "Links currently live",
        variant: "emerald",
      },
      {
        id: "uploads",
        icon: "cloud-arrow-up",
        label: tr("profile.uploads") || "Uploads",
        value: (stats.uploads ?? 0).toString(),
        sublabel: tr("profile.uploadsHint") || "Total uploads recorded",
        variant: "rose",
      },
    ];
  });

  const completionFields = $derived(() => [
    { id: "avatar", label: "Add a profile photo", complete: Boolean(user.avatar) },
    {
      id: "bio",
      label: "Write a short bio",
      complete: Boolean(user.bio && user.bio.length > 0),
    },
    { id: "email", label: "Verify an email", complete: Boolean(user.email) },
    {
      id: "files",
      label: "Upload your first file",
      complete: (user.stats?.files ?? 0) > 0,
    },
    {
      id: "shares",
      label: "Create a share link",
      complete: (user.stats?.shares ?? 0) > 0,
    },
  ]);

  const completionProgress = $derived(() => {
    const fields = completionFields();
    const total = fields.length;
    const completed = fields.filter((field) => field.complete).length;
    return total === 0 ? 0 : Math.round((completed / total) * 100);
  });

  const pendingCompletion = $derived(() =>
    completionFields().filter((field) => !field.complete)
  );

  const heroHighlights = $derived(() => [
    {
      label: tr("memberSince") || "Member since",
      value: formatMemberSince(user.joinedDate),
    },
    {
      label: tr("profile.storageUsed") || "Storage used",
      value: formatSize(user.stats?.storage || 0),
    },
    {
      label: tr("profile.links") || "Active links",
      value: (user.stats?.shares ?? 0).toString(),
    },
  ]);

  const contactMethods = $derived(() => [
    {
      id: "email",
      icon: "envelope",
      label: tr("email") || "Email",
      value: user.email || tr("noEmail") || "No email added",
    },
    {
      id: "username",
      icon: "person",
      label: tr("username") || "Username",
      value: getHandle() || tr("unknown") || "Unknown",
    },
  ]);

  const quickShortcuts = $derived(() => [
    {
      id: "settings",
      icon: "sliders2",
      label: tr("profile.shortcuts.settings") || "Account settings",
      description:
        tr("profile.shortcuts.settingsDesc") ||
        "Themes, language, notifications",
      href: "#/settings",
    },
    {
      id: "security",
      icon: "shield-lock",
      label: tr("profile.shortcuts.security") || "Security center",
      description:
        tr("profile.shortcuts.securityDesc") ||
        "Passkeys, 2FA, trusted devices",
      href: "#/settings/security",
    },
    {
      id: "activity",
      icon: "clock-history",
      label: tr("profile.shortcuts.activity") || "Activity log",
      description:
        tr("profile.shortcuts.activityDesc") || "Full audit trail",
      href: "#/activity",
    },
  ]);

  const bioLength = $derived(() => profileDraft.bio?.length || 0);

  const activityItems = $derived(() =>
    recentActivity.map((entry) => ({
      ...entry,
      meta: getActivityMeta(entry.action),
    }))
  );

  const canSaveProfile = $derived(
    () =>
      !savingProfile &&
      bioLength() <= BIO_LIMIT &&
      Boolean(profileDraft.displayName.trim())
  );

  onMount(() => {
    initialize();
  });

  async function initialize() {
    loading = true;
    errorMsg = null;
    try {
      await Promise.all([loadUserProfile(), loadRecentActivity()]);
    } catch (err) {
      console.error("[Profile] Initialization failed", err);
      errorMsg = tr("failedToLoadProfile");
    } finally {
      loading = false;
    }
  }

  async function loadUserProfile() {
    try {
      const profile = await api.users.getProfile();
      const normalized = {
        username: profile.username || "",
        email: profile.email || "",
        displayName: profile.display_name || profile.username || "",
        bio: profile.bio || "",
        avatar: profile.avatar_base64 || null,
        joinedDate: profile.created_at ? new Date(profile.created_at) : null,
        stats: {
          files: profile.file_count || 0,
          storage: profile.storage_used_bytes || 0,
          shares: profile.share_count || 0,
          uploads: profile.upload_count || 0,
        },
      };
      user = normalized;
      profileDraft = createDraft(normalized);
    } catch (err) {
      console.error("[Profile] Failed to load profile", err);
      errorToast(tr("failedToLoadProfile"));
      throw err;
    }
  }

  async function loadRecentActivity(limit = 12) {
    activityLoading = true;
    activityError = null;
    try {
      const response = await api.activity.list(limit);
      recentActivity = Array.isArray(response)
        ? response
        : response?.items || [];
    } catch (err) {
      console.error("[Profile] Failed to load activity", err);
      activityError = tr("failedToLoadActivity") || "Unable to load activity";
    } finally {
      activityLoading = false;
    }
  }

  function getHandle() {
    if (!user.username) return "";
    return user.username.startsWith("@") ? user.username : `@${user.username}`;
  }

  function currentAvatar() {
    return editMode ? profileDraft.avatar ?? user.avatar : user.avatar;
  }

  function formatMemberSince(date) {
    if (!date) return tr("unknown") || "Unknown";
    return date.toLocaleDateString(undefined, {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  }

  function startEditing() {
    profileDraft = createDraft(user);
    editMode = true;
  }

  function cancelEdit() {
    editMode = false;
    profileDraft = createDraft(user);
  }

  function handleAvatarUpload(event) {
    const [file] = event.currentTarget?.files || [];
    if (!file) return;
    if (!editMode) {
      startEditing();
    }
    const reader = new FileReader();
    reader.onload = (loadEvent) => {
      profileDraft.avatar = loadEvent.target?.result;
    };
    reader.readAsDataURL(file);
  }

  async function saveProfile() {
    if (!canSaveProfile()) return;
    savingProfile = true;
    try {
      await api.users.updateProfile({
        display_name: profileDraft.displayName.trim(),
        bio: profileDraft.bio,
        email: profileDraft.email || null,
        avatar_base64: profileDraft.avatar,
      });
      success(tr("profileSavedSuccessfully") || "Profile updated");
      editMode = false;
      await loadUserProfile();
    } catch (err) {
      console.error("[Profile] Failed to save profile", err);
      errorToast(tr("failedToSaveProfile") || "Unable to save profile");
    } finally {
      savingProfile = false;
    }
  }

  function getActivityMeta(action) {
    const defaults = {
      icon: "activity",
      color: "text-slate-300",
      label: tr("activity.unknown") || "Activity",
    };
    const mapping = {
      upload: {
        icon: "cloud-arrow-up",
        color: "text-emerald-300",
        label: tr("activity.uploaded") || "Uploaded",
      },
      download: {
        icon: "cloud-arrow-down",
        color: "text-sky-200",
        label: tr("activity.downloaded") || "Downloaded",
      },
      delete: {
        icon: "trash3",
        color: "text-rose-300",
        label: tr("activity.deleted") || "Deleted",
      },
      move: {
        icon: "arrows-move",
        color: "text-amber-200",
        label: tr("activity.moved") || "Moved",
      },
      share: {
        icon: "share",
        color: "text-indigo-200",
        label: tr("activity.shared") || "Shared",
      },
      rename: {
        icon: "type",
        color: "text-purple-200",
        label: tr("activity.renamed") || "Renamed",
      },
    };
    return mapping[action] || defaults;
  }

  async function copyToClipboard(value, message = "Copied") {
    if (!value || typeof navigator === "undefined" || !navigator.clipboard) {
      return;
    }
    try {
      await navigator.clipboard.writeText(value);
      success(message);
    } catch (err) {
      console.error("[Profile] Clipboard failed", err);
      errorToast(tr("clipboardUnavailable") || "Clipboard unavailable");
    }
  }

  function shareProfileLink() {
    if (typeof window === "undefined") return;
    copyToClipboard(
      `${window.location.origin}/#/user/profile`,
      tr("profileLinkCopied") || "Profile link copied"
    );
  }

  function copyPath(path) {
    if (!path) return;
    copyToClipboard(path, tr("pathCopied") || "File path copied");
  }

  function openShortcut(shortcut) {
    if (!shortcut?.href || typeof window === "undefined") return;
    window.location.hash = shortcut.href;
  }
</script>

<PageLayout
  title={tr("profile.title") || "Your profile"}
  subtitle={
    tr("profile.subtitle") || "Personalize your SyncSpace identity and presence"
  }
  icon="person-badge"
  {breadcrumbs}
  loading={loading}
  error={errorMsg}
>
  <section class="space-y-8">
    <section
      class="relative overflow-hidden rounded-[32px] border border-white/30 bg-gradient-to-br from-indigo-500/90 via-indigo-600/80 to-slate-900 text-white shadow-[0_20px_45px_rgba(15,23,42,0.35)]"
    >
      <div class="absolute inset-0 opacity-30 bg-[radial-gradient(circle_at_top,_rgba(255,255,255,0.25),_transparent_55%)]"></div>
      <div class="relative flex flex-col gap-6 px-6 py-8 lg:flex-row lg:items-center">
        <div class="flex items-center gap-6">
          <div class="relative">
            <div
              class="h-28 w-28 rounded-[28px] border-4 border-white/30 bg-white/10 backdrop-blur flex items-center justify-center text-3xl font-semibold uppercase tracking-wide"
            >
              {#if currentAvatar()}
                <img
                  src={currentAvatar()}
                  alt={profileDraft.displayName || user.displayName || "Avatar"}
                  class="h-full w-full rounded-[20px] object-cover"
                />
              {:else}
                {(user.displayName || user.username || "?")
                  .slice(0, 2)
                  .toUpperCase()}
              {/if}
            </div>
            <label
              class="absolute -bottom-2 -right-2 inline-flex h-10 w-10 cursor-pointer items-center justify-center rounded-2xl bg-white/20 text-white shadow-lg backdrop-blur transition hover:bg-white/40"
            >
              <i class="bi bi-camera-fill"></i>
              <input type="file" accept="image/*" onchange={handleAvatarUpload} hidden />
            </label>
          </div>
          <div>
            <div class="flex flex-wrap items-center gap-3">
              <h1 class="text-3xl font-semibold tracking-tight">
                {user.displayName || tr("loading") || "Loading"}
              </h1>
              <span class="rounded-full border border-white/30 px-3 py-1 text-xs uppercase tracking-[0.4em] text-white/70">
                {tr("profile.status.active") || "Active"}
              </span>
            </div>
            <p class="text-white/80">{getHandle() || tr("unknown") || "Unknown"}</p>
            <p class="mt-2 max-w-2xl text-sm text-white/90">
              {user.bio || tr("noBioYet") || "Let others know what you are working on."}
            </p>
            <div class="mt-4 flex flex-wrap gap-4 text-xs text-white/80">
              {#each heroHighlights() as highlight}
                <div class="rounded-full border border-white/25 px-4 py-1 backdrop-blur">
                  <span class="uppercase tracking-[0.3em] text-[10px] text-white/60">{highlight.label}</span>
                  <p class="text-sm font-semibold text-white">{highlight.value}</p>
                </div>
              {/each}
            </div>
          </div>
        </div>

        <div class="flex flex-wrap gap-3">
          <button
            class="inline-flex items-center gap-2 rounded-2xl border border-white/40 px-4 py-2 text-sm font-semibold text-white transition hover:bg-white/10"
            onclick={shareProfileLink}
          >
            <i class="bi bi-share"></i>
            {tr("share") || "Share"}
          </button>
          <button
            class={`inline-flex items-center gap-2 rounded-2xl px-5 py-2 text-sm font-semibold transition ${editMode
              ? 'bg-rose-500 text-white hover:bg-rose-400'
              : 'bg-white text-indigo-600 hover:-translate-y-0.5'}`}
            onclick={editMode ? cancelEdit : startEditing}
          >
            <i class={`bi ${editMode ? 'bi-x-lg' : 'bi-pencil-square'}`}></i>
            {editMode ? tr("cancel") || "Cancel" : tr("editProfile") || "Edit profile"}
          </button>
        </div>
      </div>
    </section>

    <section class="grid gap-4 md:grid-cols-2 xl:grid-cols-4">
      {#each statsCards() as card}
        <StatsCard
          icon={card.icon}
          label={card.label}
          value={card.value}
          sublabel={card.sublabel}
          variant={card.variant}
        />
      {/each}
    </section>

    <section class="grid gap-6 lg:grid-cols-[minmax(0,1.5fr)_minmax(0,1fr)]">
      <div class="space-y-6">
        <article class="rounded-3xl border border-white/60 bg-white/80 p-6 shadow-xl dark:border-slate-800/60 dark:bg-slate-900/70">
          <header class="flex flex-wrap items-center justify-between gap-3">
            <div>
              <p class="text-xs font-semibold uppercase tracking-[0.3em] text-slate-400">
                {tr("profile.about") || "About"}
              </p>
              <h2 class="text-xl font-semibold text-slate-900 dark:text-white">
                {tr("profile.personalDetails") || "Personal details"}
              </h2>
            </div>
            {#if editMode}
              <div class="flex flex-wrap gap-2">
                <button
                  class="inline-flex items-center gap-2 rounded-2xl border border-slate-200 px-4 py-2 text-sm font-semibold text-slate-600 transition hover:border-indigo-400 hover:text-indigo-600 dark:border-slate-700 dark:text-slate-200"
                  onclick={cancelEdit}
                  disabled={savingProfile}
                >
                  {tr("cancel") || "Cancel"}
                </button>
                <button
                  class="inline-flex items-center gap-2 rounded-2xl bg-indigo-600 px-4 py-2 text-sm font-semibold text-white shadow-lg transition hover:bg-indigo-500 disabled:opacity-60"
                  onclick={saveProfile}
                  disabled={!canSaveProfile()}
                >
                  {savingProfile ? tr("saving") || "Saving" : tr("save") || "Save"}
                </button>
              </div>
            {:else}
              <button
                class="inline-flex items-center gap-2 rounded-2xl border border-slate-200 px-4 py-2 text-sm font-semibold text-slate-600 transition hover:border-indigo-400 hover:text-indigo-600 dark:border-slate-700 dark:text-slate-200"
                onclick={startEditing}
              >
                <i class="bi bi-pencil"></i>
                {tr("editProfile") || "Edit profile"}
              </button>
            {/if}
          </header>

          {#if editMode}
            <div class="mt-6 space-y-4">
              <div class="grid gap-4 md:grid-cols-2">
                <label class="space-y-2 text-sm font-medium text-slate-600 dark:text-slate-200">
                  {tr("displayName") || "Display name"}
                  <input
                    type="text"
                    bind:value={profileDraft.displayName}
                    class="w-full rounded-2xl border border-slate-200 bg-white px-4 py-2 text-slate-900 shadow-sm outline-none transition focus:border-indigo-400 focus:ring-2 focus:ring-indigo-100 dark:border-slate-700 dark:bg-slate-900 dark:text-white"
                  />
                </label>
                <label class="space-y-2 text-sm font-medium text-slate-600 dark:text-slate-200">
                  {tr("email") || "Email"}
                  <input
                    type="email"
                    bind:value={profileDraft.email}
                    class="w-full rounded-2xl border border-slate-200 bg-white px-4 py-2 text-slate-900 shadow-sm outline-none transition focus:border-indigo-400 focus:ring-2 focus:ring-indigo-100 dark:border-slate-700 dark:bg-slate-900 dark:text-white"
                  />
                </label>
              </div>

              <label class="space-y-2 text-sm font-medium text-slate-600 dark:text-slate-200">
                {tr("bio") || "Bio"}
                <textarea
                  bind:value={profileDraft.bio}
                  rows="3"
                  class={`w-full rounded-2xl border px-4 py-3 text-sm shadow-sm outline-none transition focus:ring-2 ${bioLength() > BIO_LIMIT
                    ? 'border-rose-400 focus:border-rose-400 focus:ring-rose-100'
                    : 'border-slate-200 focus:border-indigo-400 focus:ring-indigo-100 dark:border-slate-700 dark:bg-slate-900 dark:text-white'}`}
                ></textarea>
                <span class={`text-xs ${bioLength() > BIO_LIMIT ? 'text-rose-500' : 'text-slate-400'}`}>
                  {bioLength()}/{BIO_LIMIT}
                </span>
              </label>
            </div>
          {:else}
            <div class="mt-6 space-y-4">
              <p class="text-sm text-slate-600 dark:text-slate-300">
                {user.bio || tr("noBioYet") || "No bio provided yet."}
              </p>
              <dl class="grid gap-4 sm:grid-cols-2">
                <div class="rounded-2xl border border-slate-100 bg-white/60 p-4 dark:border-slate-800 dark:bg-slate-900/60">
                  <dt class="text-xs font-semibold uppercase tracking-[0.3em] text-slate-400">
                    {tr("email") || "Email"}
                  </dt>
                  <dd class="text-sm font-medium text-slate-900 dark:text-white">
                    {user.email || tr("noEmail") || "No email added"}
                  </dd>
                </div>
                <div class="rounded-2xl border border-slate-100 bg-white/60 p-4 dark:border-slate-800 dark:bg-slate-900/60">
                  <dt class="text-xs font-semibold uppercase tracking-[0.3em] text-slate-400">
                    {tr("memberSince") || "Member since"}
                  </dt>
                  <dd class="text-sm font-medium text-slate-900 dark:text-white">
                    {formatMemberSince(user.joinedDate)}
                  </dd>
                </div>
              </dl>
            </div>
          {/if}
        </article>

        <article class="rounded-3xl border border-white/60 bg-white/80 p-6 shadow-xl dark:border-slate-800/60 dark:bg-slate-900/70">
          <header class="flex flex-wrap items-center justify-between gap-3">
            <div>
              <p class="text-xs font-semibold uppercase tracking-[0.3em] text-slate-400">
                {tr("recentActivity") || "Recent activity"}
              </p>
              <h2 class="text-xl font-semibold text-slate-900 dark:text-white">
                {tr("profile.latestEvents") || "Latest events"}
              </h2>
            </div>
            <div class="flex items-center gap-2 text-sm">
              <button
                class="inline-flex h-9 w-9 items-center justify-center rounded-full border border-slate-200 text-slate-600 transition hover:border-indigo-400 hover:text-indigo-600 dark:border-slate-700 dark:text-slate-200"
                onclick={() => loadRecentActivity()}
                aria-label="Refresh activity"
              >
                <i class="bi bi-arrow-clockwise"></i>
              </button>
              <a
                href="#/activity"
                class="inline-flex items-center gap-1 text-indigo-600 hover:underline dark:text-indigo-300"
              >
                {tr("viewAll") || "View all"}
                <i class="bi bi-arrow-up-right"></i>
              </a>
            </div>
          </header>

          {#if activityLoading}
            <div class="flex items-center justify-center py-12">
              <span class="h-10 w-10 animate-spin rounded-full border-b-2 border-indigo-500"></span>
            </div>
          {:else if activityError}
            <div class="rounded-2xl border border-rose-200 bg-rose-50/70 p-5 text-sm text-rose-600 dark:border-rose-900/40 dark:bg-rose-950/30 dark:text-rose-200">
              <p class="font-semibold">{activityError}</p>
              <button
                class="mt-3 inline-flex items-center gap-2 rounded-2xl border border-rose-200 px-3 py-1 text-xs font-semibold text-rose-600 transition hover:bg-rose-100 dark:border-rose-800 dark:text-rose-200"
                onclick={() => loadRecentActivity()}
              >
                {tr("tryAgain") || "Try again"}
              </button>
            </div>
          {:else if !recentActivity.length}
            <div class="rounded-2xl border border-dashed border-slate-200 bg-slate-50/70 p-10 text-center text-sm text-slate-500 dark:border-slate-700 dark:bg-slate-900/40 dark:text-slate-300">
              {tr("profile.noActivity") || "No recent actions yet."}
            </div>
          {:else}
            <ul class="mt-6 space-y-4">
              {#each activityItems() as entry (entry.id)}
                <li class="relative pl-10">
                  <span class="absolute left-0 top-3 h-3 w-3 rounded-full bg-indigo-200"></span>
                  <div class="rounded-3xl border border-slate-100 bg-white/70 p-4 shadow-sm dark:border-slate-800 dark:bg-slate-900/70">
                    <div class="flex flex-wrap items-center justify-between gap-3">
                      <div class="flex items-center gap-2 text-sm font-semibold text-slate-900 dark:text-white">
                        <span class={`inline-flex h-9 w-9 items-center justify-center rounded-2xl bg-slate-100 dark:bg-slate-800 ${entry.meta.color}`}>
                          <i class={`bi bi-${entry.meta.icon}`}></i>
                        </span>
                        {entry.meta.label}
                      </div>
                      <span class="text-xs font-medium uppercase tracking-[0.3em] text-slate-400">
                        {formatRelativeTime(entry.created_at)}
                      </span>
                    </div>
                    <p class="mt-2 text-sm font-semibold text-slate-800 dark:text-slate-200">
                      {entry.file_name || entry.file_path || tr("unknown") || "Unknown"}
                    </p>
                    <p class="text-xs text-slate-500 dark:text-slate-400">
                      {entry.file_path || tr("noPath") || "No path reported"}
                    </p>
                    <div class="mt-3 flex flex-wrap items-center gap-2 text-xs text-slate-500">
                      {#if entry.status}
                        <span class="inline-flex items-center gap-1 rounded-full border border-slate-200 px-3 py-1 font-semibold uppercase tracking-[0.3em] text-slate-500 dark:border-slate-700">
                          <i class="bi bi-activity"></i>
                          {entry.status}
                        </span>
                      {/if}
                      {#if entry.file_size}
                        <span class="inline-flex items-center gap-1 rounded-full bg-slate-100 px-3 py-1 text-slate-600 dark:bg-slate-800 dark:text-slate-300">
                          <i class="bi bi-hdd"></i>
                          {formatSize(entry.file_size)}
                        </span>
                      {/if}
                      {#if entry.metadata?.client}
                        <span class="inline-flex items-center gap-1 rounded-full bg-slate-100 px-3 py-1 text-slate-600 dark:bg-slate-800 dark:text-slate-300">
                          <i class="bi bi-pc-display"></i>
                          {entry.metadata.client}
                        </span>
                      {/if}
                      <button
                        class="inline-flex items-center gap-1 rounded-full border border-slate-200 px-3 py-1 font-semibold text-slate-600 transition hover:border-indigo-400 hover:text-indigo-600 dark:border-slate-700 dark:text-slate-200"
                        onclick={() => copyPath(entry.file_path)}
                      >
                        <i class="bi bi-clipboard"></i>
                        {tr("copy") || "Copy"}
                      </button>
                    </div>
                  </div>
                </li>
              {/each}
            </ul>
          {/if}
        </article>
      </div>

      <div class="space-y-6">
        <article class="rounded-3xl border border-white/60 bg-white/80 p-6 shadow-xl dark:border-slate-800/60 dark:bg-slate-900/70">
          <p class="text-xs font-semibold uppercase tracking-[0.3em] text-slate-400">
            {tr("profile.completion") || "Profile completion"}
          </p>
          <div class="mt-3 flex items-end justify-between">
            <h3 class="text-4xl font-semibold text-slate-900 dark:text-white">
              {completionProgress()}%
            </h3>
            <span class="text-xs text-slate-500">
              {pendingCompletion().length}
              {" "}
              {tr("profile.stepsRemaining") || "steps remaining"}
            </span>
          </div>
          <div class="mt-4 h-3 w-full rounded-full bg-slate-100 dark:bg-slate-800">
            <div
              class="h-full rounded-full bg-gradient-to-r from-indigo-500 to-purple-500"
              style={`width:${completionProgress()}%;`}
            ></div>
          </div>
          <ul class="mt-4 space-y-3 text-sm">
            {#each pendingCompletion().slice(0, 3) as task}
              <li class="flex items-center gap-2 text-slate-600 dark:text-slate-300">
                <i class="bi bi-circle text-xs text-slate-300"></i>
                {task.label}
              </li>
            {/each}
          </ul>
        </article>

        <article class="rounded-3xl border border-white/60 bg-white/80 p-6 shadow-xl dark:border-slate-800/60 dark:bg-slate-900/70">
          <p class="text-xs font-semibold uppercase tracking-[0.3em] text-slate-400">
            {tr("profile.contact") || "Contact & presence"}
          </p>
          <div class="mt-4 space-y-4">
            {#each contactMethods() as method}
              <div class="flex items-center gap-3 rounded-2xl border border-slate-100 bg-white/70 px-4 py-3 dark:border-slate-800 dark:bg-slate-900/80">
                <span class="inline-flex h-10 w-10 items-center justify-center rounded-2xl bg-indigo-50 text-indigo-500 dark:bg-indigo-500/10 dark:text-indigo-200">
                  <i class={`bi bi-${method.icon}`}></i>
                </span>
                <div>
                  <p class="text-xs uppercase tracking-[0.3em] text-slate-400">
                    {method.label}
                  </p>
                  <p class="text-sm font-semibold text-slate-900 dark:text-white">
                    {method.value}
                  </p>
                </div>
              </div>
            {/each}
          </div>

          <div class="mt-6 space-y-3">
            {#each quickShortcuts() as shortcut}
              <button
                class="w-full rounded-2xl border border-slate-100 bg-white/60 px-4 py-3 text-left text-sm text-slate-600 transition hover:border-indigo-400 hover:text-indigo-600 dark:border-slate-800 dark:bg-slate-900/70 dark:text-slate-200"
                onclick={() => openShortcut(shortcut)}
              >
                <div class="flex items-center gap-3">
                  <span class="inline-flex h-9 w-9 items-center justify-center rounded-2xl bg-slate-100 text-slate-500 dark:bg-slate-800 dark:text-slate-200">
                    <i class={`bi bi-${shortcut.icon}`}></i>
                  </span>
                  <div>
                    <p class="font-semibold">{shortcut.label}</p>
                    <p class="text-xs text-slate-500">{shortcut.description}</p>
                  </div>
                </div>
              </button>
            {/each}
          </div>
        </article>
      </div>
    </section>
  </section>
</PageLayout>
