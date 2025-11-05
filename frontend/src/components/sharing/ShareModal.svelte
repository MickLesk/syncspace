<script>
  import { createEventDispatcher } from "svelte";
  import { success, error as errorToast } from "../../stores/toast.js";
  import { currentLanguage } from "../../stores/ui";
  import { t } from "../../i18n.js";
  import Modal from "../ui/Modal.svelte";
  import api from "../../lib/api.js";

  const dispatch = createEventDispatcher();
  const tr = $derived((key, ...args) => t($currentLanguage, key, ...args));

  // Props
  export let isOpen = false;
  export let file = null;
  export let onClose = () => {};

  // Share state
  let shareType = "public"; // 'public' or 'private'
  let userEmail = "";
  let expiresInDays = null;
  let permissions = {
    canRead: true,
    canWrite: false,
    canDelete: false,
    canShare: false,
  };

  // UI state
  let loading = false;
  let shareUrl = "";
  let showShareResult = false;

  // Expiration options
  const expirationOptions = $derived([
    { value: null, label: tr("neverExpires") },
    { value: 1, label: tr("oneDay") },
    { value: 7, label: tr("oneWeek") },
    { value: 30, label: tr("oneMonth") },
    { value: 90, label: tr("threeMonths") },
    { value: 365, label: tr("oneYear") },
  ]);

  $: isFile = file && !file.is_dir;
  $: isFolder = file && file.is_dir;

  async function handleCreateShare() {
    if (!file) return;

    loading = true;
    try {
      const options = {
        sharedWith: shareType === "private" ? userEmail : null,
        expiresInDays,
        canRead: permissions.canRead,
        canWrite: permissions.canWrite,
        canDelete: permissions.canDelete,
        canShare: permissions.canShare,
      };

      const share = await api.sharing.create(
        isFile ? file.id || file.path : null,
        isFolder ? file.id || file.path : null,
        options
      );

      // Generate shareable URL
      shareUrl = `${window.location.origin}/share/${share.share.share_link}`;
      showShareResult = true;

      success(tr("shareCreatedSuccessfully"));
    } catch (err) {
      errorToast(err.message || tr("failedToCreateShare"));
    } finally {
      loading = false;
    }
  }

  async function copyShareUrl() {
    try {
      await navigator.clipboard.writeText(shareUrl);
      success(tr("shareUrlCopiedToClipboard"));
    } catch (err) {
      errorToast(tr("failedToCopyUrl"));
    }
  }

  function close() {
    onClose();
    showShareResult = false;
    shareUrl = "";
    dispatch("close");
  }

  function reset() {
    shareType = "public";
    userEmail = "";
    expiresInDays = null;
    permissions = {
      canRead: true,
      canWrite: false,
      canDelete: false,
      canShare: false,
    };
    showShareResult = false;
    shareUrl = "";
  }

  $: if (isOpen && !showShareResult) {
    reset();
  }
</script>

<Modal
  visible={isOpen}
  title={tr("shareFileName", file?.name || "")}
  icon="share"
  size="md"
  variant="primary"
  onclose={close}
>
  {#if !showShareResult}
    <!-- Share Configuration -->
    <div class="space-y-6">
      <!-- Share Type -->
      <div class="form-control">
        <label class="label">
          <span class="label-text font-semibold">{tr("shareType")}</span>
        </label>
        <div class="grid grid-cols-2 gap-3">
          <label class="cursor-pointer">
            <input
              type="radio"
              bind:group={shareType}
              value="public"
              class="radio radio-primary"
            />
            <div class="ml-3">
              <div class="font-medium">{tr("publicLink")}</div>
              <div class="text-sm opacity-60">
                {tr("anyoneWithLinkCanAccess")}
              </div>
            </div>
          </label>

          <label class="cursor-pointer">
            <input
              type="radio"
              bind:group={shareType}
              value="private"
              class="radio radio-primary"
            />
            <div class="ml-3">
              <div class="font-medium">{tr("specificUser")}</div>
              <div class="text-sm opacity-60">
                {tr("shareWithSpecificPerson")}
              </div>
            </div>
          </label>
        </div>
      </div>

      <!-- User Email (for private shares) -->
      {#if shareType === "private"}
        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold">{tr("userEmail")}</span>
          </label>
          <input
            type="email"
            placeholder={tr("enterUserEmailAddress")}
            class="input input-bordered"
            bind:value={userEmail}
            required
          />
        </div>
      {/if}

      <!-- Expiration -->
      <div class="form-control">
        <label class="label">
          <span class="label-text font-semibold">{tr("expiration")}</span>
        </label>
        <select class="select select-bordered" bind:value={expiresInDays}>
          {#each expirationOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>

      <!-- Permissions -->
      <div class="form-control">
        <label class="label">
          <span class="label-text font-semibold">{tr("permissions")}</span>
        </label>
        <div class="space-y-3">
          <label class="cursor-pointer flex items-center gap-3">
            <input
              type="checkbox"
              bind:checked={permissions.canRead}
              class="checkbox checkbox-primary"
              disabled
            />
            <div>
              <div class="font-medium">{tr("view")}</div>
              <div class="text-sm opacity-60">
                {tr("canViewAndDownloadFiles")}
              </div>
            </div>
          </label>

          <label class="cursor-pointer flex items-center gap-3">
            <input
              type="checkbox"
              bind:checked={permissions.canWrite}
              class="checkbox checkbox-primary"
            />
            <div>
              <div class="font-medium">{tr("edit")}</div>
              <div class="text-sm opacity-60">
                {tr("canUploadAndModifyFiles")}
              </div>
            </div>
          </label>

          <label class="cursor-pointer flex items-center gap-3">
            <input
              type="checkbox"
              bind:checked={permissions.canDelete}
              class="checkbox checkbox-primary"
            />
            <div>
              <div class="font-medium">{tr("delete")}</div>
              <div class="text-sm opacity-60">
                {tr("canDeleteFilesAndFolders")}
              </div>
            </div>
          </label>

          <label class="cursor-pointer flex items-center gap-3">
            <input
              type="checkbox"
              bind:checked={permissions.canShare}
              class="checkbox checkbox-primary"
            />
            <div>
              <div class="font-medium">{tr("share")}</div>
              <div class="text-sm opacity-60">
                {tr("canCreateAdditionalShares")}
              </div>
            </div>
          </label>
        </div>
      </div>
    </div>
  {:else}
    <!-- Share Result -->
    <div class="space-y-6">
      <!-- Success Message -->
      <div class="alert alert-success rounded-xl">
        <i class="bi bi-check-circle-fill text-2xl"></i>
        <div>
          <h3 class="font-bold">{tr("shareCreatedSuccessfully")}!</h3>
          <div class="text-sm">
            {tr(
              "yourShareIsReadyToUse",
              shareType === "public" ? tr("public") : tr("private")
            )}
          </div>
        </div>
      </div>

      <!-- Share URL -->
      <div class="form-control">
        <label class="label">
          <span class="label-text font-semibold">{tr("shareUrl")}</span>
        </label>
        <div class="join w-full">
          <input
            type="text"
            readonly
            value={shareUrl}
            class="input input-bordered join-item flex-1 font-mono text-sm"
          />
          <button
            class="btn btn-primary join-item gap-2"
            onclick={copyShareUrl}
          >
            <i class="bi bi-clipboard"></i>
            {tr("copy")}
          </button>
        </div>
      </div>

      <!-- Share Details -->
      <div class="grid grid-cols-2 gap-4">
        <div class="stat bg-slate-50 dark:bg-slate-800 rounded-xl p-4">
          <div class="stat-title">{tr("type")}</div>
          <div class="stat-value text-lg">
            {shareType === "public" ? tr("public") : tr("private")}
          </div>
        </div>
        <div class="stat bg-slate-50 dark:bg-slate-800 rounded-xl p-4">
          <div class="stat-title">{tr("expires")}</div>
          <div class="stat-value text-lg">
            {expiresInDays ? tr("daysCount", expiresInDays) : tr("never")}
          </div>
        </div>
      </div>

      <!-- Permissions Summary -->
      <div class="form-control">
        <label class="label">
          <span class="label-text font-semibold">{tr("permissions")}</span>
        </label>
        <div class="flex flex-wrap gap-2">
          {#if permissions.canRead}
            <div class="badge badge-success gap-1">
              <i class="bi bi-eye"></i>
              {tr("view")}
            </div>
          {/if}
          {#if permissions.canWrite}
            <div class="badge badge-warning gap-1">
              <i class="bi bi-pencil"></i>
              {tr("edit")}
            </div>
          {/if}
          {#if permissions.canDelete}
            <div class="badge badge-error gap-1">
              <i class="bi bi-trash"></i>
              {tr("delete")}
            </div>
          {/if}
          {#if permissions.canShare}
            <div class="badge badge-info gap-1">
              <i class="bi bi-share"></i>
              {tr("share")}
            </div>
          {/if}
        </div>
      </div>
    </div>
  {/if}

  <!-- Actions slot - outside {#if} for proper Svelte slot placement -->
  <div slot="actions" class="flex gap-3 justify-end">
    {#if !showShareResult}
      <button class="btn btn-ghost" onclick={close}>
        <i class="bi bi-x-lg"></i>
        {tr("cancel")}
      </button>
      <button
        class="btn btn-primary gap-2"
        onclick={handleCreateShare}
        disabled={loading || (shareType === "private" && !userEmail.trim())}
      >
        {#if loading}
          <span class="loading loading-spinner loading-sm"></span>
        {:else}
          <i class="bi bi-share"></i>
        {/if}
        {tr("createShare")}
      </button>
    {:else}
      <button class="btn btn-primary gap-2" onclick={copyShareUrl}>
        <i class="bi bi-clipboard"></i>
        {tr("copyUrl")}
      </button>
      <button class="btn btn-ghost" onclick={close}>
        <i class="bi bi-check-lg"></i>
        {tr("done")}
      </button>
    {/if}
  </div>
</Modal>
