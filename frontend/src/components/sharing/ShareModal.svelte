<script>
  import { createEventDispatcher } from "svelte";
  import { success, error as errorToast } from "../../../stores/toast.js";
  import Modal from "../../ui/Modal.svelte";
  import api from "../../../lib/api.js";

  const dispatch = createEventDispatcher();

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
  const expirationOptions = [
    { value: null, label: "Never expires" },
    { value: 1, label: "1 day" },
    { value: 7, label: "1 week" },
    { value: 30, label: "1 month" },
    { value: 90, label: "3 months" },
    { value: 365, label: "1 year" },
  ];

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

      success(
        `${shareType === "public" ? "Public" : "Private"} share created successfully`
      );
    } catch (err) {
      errorToast(err.message || "Failed to create share");
    } finally {
      loading = false;
    }
  }

  async function copyShareUrl() {
    try {
      await navigator.clipboard.writeText(shareUrl);
      success("Share URL copied to clipboard");
    } catch (err) {
      errorToast("Failed to copy URL");
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
  title="Share {file?.name || ''}"
  icon="share"
  size="md"
  variant="primary"
  on:close={close}
>
  {#if !showShareResult}
    <!-- Share Configuration -->
    <div class="space-y-6">
      <!-- Share Type -->
      <div class="form-control">
        <label class="label">
          <span class="label-text font-semibold">Share Type</span>
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
              <div class="font-medium">Public Link</div>
              <div class="text-sm opacity-60">
                Anyone with the link can access
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
              <div class="font-medium">Specific User</div>
              <div class="text-sm opacity-60">Share with a specific person</div>
            </div>
          </label>
        </div>
      </div>

      <!-- User Email (for private shares) -->
      {#if shareType === "private"}
        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold">User Email</span>
          </label>
          <input
            type="email"
            placeholder="Enter user email address"
            class="input input-bordered"
            bind:value={userEmail}
            required
          />
        </div>
      {/if}

      <!-- Expiration -->
      <div class="form-control">
        <label class="label">
          <span class="label-text font-semibold">Expiration</span>
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
          <span class="label-text font-semibold">Permissions</span>
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
              <div class="font-medium">View</div>
              <div class="text-sm opacity-60">Can view and download files</div>
            </div>
          </label>

          <label class="cursor-pointer flex items-center gap-3">
            <input
              type="checkbox"
              bind:checked={permissions.canWrite}
              class="checkbox checkbox-primary"
            />
            <div>
              <div class="font-medium">Edit</div>
              <div class="text-sm opacity-60">Can upload and modify files</div>
            </div>
          </label>

          <label class="cursor-pointer flex items-center gap-3">
            <input
              type="checkbox"
              bind:checked={permissions.canDelete}
              class="checkbox checkbox-primary"
            />
            <div>
              <div class="font-medium">Delete</div>
              <div class="text-sm opacity-60">Can delete files and folders</div>
            </div>
          </label>

          <label class="cursor-pointer flex items-center gap-3">
            <input
              type="checkbox"
              bind:checked={permissions.canShare}
              class="checkbox checkbox-primary"
            />
            <div>
              <div class="font-medium">Share</div>
              <div class="text-sm opacity-60">Can create additional shares</div>
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
          <h3 class="font-bold">Share Created Successfully!</h3>
          <div class="text-sm">Your {shareType} share is ready to use.</div>
        </div>
      </div>

      <!-- Share URL -->
      <div class="form-control">
        <label class="label">
          <span class="label-text font-semibold">Share URL</span>
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
            Copy
          </button>
        </div>
      </div>

      <!-- Share Details -->
      <div class="grid grid-cols-2 gap-4">
        <div class="stat bg-slate-50 dark:bg-slate-800 rounded-xl p-4">
          <div class="stat-title">Type</div>
          <div class="stat-value text-lg">
            {shareType === "public" ? "Public" : "Private"}
          </div>
        </div>
        <div class="stat bg-slate-50 dark:bg-slate-800 rounded-xl p-4">
          <div class="stat-title">Expires</div>
          <div class="stat-value text-lg">
            {expiresInDays ? `${expiresInDays} days` : "Never"}
          </div>
        </div>
      </div>

      <!-- Permissions Summary -->
      <div class="form-control">
        <label class="label">
          <span class="label-text font-semibold">Permissions</span>
        </label>
        <div class="flex flex-wrap gap-2">
          {#if permissions.canRead}
            <div class="badge badge-success gap-1">
              <i class="bi bi-eye"></i>
              View
            </div>
          {/if}
          {#if permissions.canWrite}
            <div class="badge badge-warning gap-1">
              <i class="bi bi-pencil"></i>
              Edit
            </div>
          {/if}
          {#if permissions.canDelete}
            <div class="badge badge-error gap-1">
              <i class="bi bi-trash"></i>
              Delete
            </div>
          {/if}
          {#if permissions.canShare}
            <div class="badge badge-info gap-1">
              <i class="bi bi-share"></i>
              Share
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
        Cancel
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
        Create Share
      </button>
    {:else}
      <button class="btn btn-primary gap-2" onclick={copyShareUrl}>
        <i class="bi bi-clipboard"></i>
        Copy URL
      </button>
      <button class="btn btn-ghost" onclick={close}>
        <i class="bi bi-check-lg"></i>
        Done
      </button>
    {/if}
  </div>
</Modal>


