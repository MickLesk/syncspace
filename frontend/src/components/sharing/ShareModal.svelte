<script>
  import { createEventDispatcher } from "svelte";
  import { success, error as errorToast } from "../../stores/toast.js";
  import { currentLang } from "../../stores/ui";
  import { t } from "../../i18n.js";
  import Modal from "../ui/Modal.svelte";
  import api from "../../lib/api.js";

  const dispatch = createEventDispatcher();
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Props
  let { isOpen = $bindable(false), file = null, onClose = () => {} } = $props();

  // Share state - ENHANCED
  let shareType = "public"; // 'public' or 'users'
  let selectedUsers = []; // Array of {id, username, display_name}
  let userPermissions = {}; // Map: user_id => 'read'|'write'|'admin'
  let allowExternal = true; // Toggle for external sharing
  let requirePassword = false;
  let sharePassword = "";
  let expiresAt = null; // ISO date string from date input
  let loading = false;
  let shareUrl = "";
  let showShareResult = false;

  // User list from backend
  let availableUsers = [];
  let loadingUsers = false;

  const isFile = $derived(file && !file.is_dir);
  const isFolder = $derived(file && file.is_dir);

  // Load users when modal opens
  $effect(() => {
    if (isOpen && shareType === "users" && availableUsers.length === 0) {
      loadUsers();
    }
  });

  async function loadUsers() {
    loadingUsers = true;
    try {
      const response = await api.users.listAll();
      availableUsers = response.data || [];
    } catch (err) {
      errorToast(tr("failedToLoadUsers"));
    } finally {
      loadingUsers = false;
    }
  }

  function toggleUserSelection(user) {
    const index = selectedUsers.findIndex((u) => u.id === user.id);
    if (index > -1) {
      selectedUsers = selectedUsers.filter((u) => u.id !== user.id);
      delete userPermissions[user.id];
    } else {
      selectedUsers = [...selectedUsers, user];
      userPermissions[user.id] = "read"; // Default permission
    }
  }

  function isUserSelected(userId) {
    return selectedUsers.some((u) => u.id === userId);
  }

  async function handleCreateShare() {
    if (!file) return;

    loading = true;
    try {
      const requestData = {
        file_path: file.path || file.file_path,
        permission: allowExternal ? "read" : "none",
        expires_at: expiresAt || undefined,
        password: requirePassword && sharePassword ? sharePassword : undefined,
        user_ids:
          shareType === "users" ? selectedUsers.map((u) => u.id) : undefined,
        permissions:
          shareType === "users"
            ? selectedUsers.map((u) => userPermissions[u.id] || "read")
            : undefined,
        allow_external: allowExternal,
      };

      const response = await api.sharing.create(requestData);
      const share = response.data || response;

      // Generate shareable URL
      shareUrl = `${window.location.origin}/share/${share.id || share.share_id || share.token}`;
      showShareResult = true;

      success(tr("shareCreatedSuccessfully"));
    } catch (err) {
      console.error("Share error:", err);
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
    reset();
    dispatch("close");
  }

  function reset() {
    shareType = "public";
    selectedUsers = [];
    userPermissions = {};
    allowExternal = true;
    requirePassword = false;
    sharePassword = "";
    expiresAt = null;
    showShareResult = false;
    shareUrl = "";
  }

  $: if (isOpen && !showShareResult) {
    // Reset on open (but not when showing results)
  }
</script>

<Modal
  visible={isOpen}
  title={tr("shareFileName", file?.name || "")}
  icon="share"
  size="lg"
  variant="primary"
  onclose={close}
>
  {#if !showShareResult}
    <!-- Share Configuration - ENHANCED -->
    <div class="space-y-6">
      <!-- Share Type -->
      <div class="form-control">
        <label class="label">
          <span class="label-text font-semibold text-base"
            >{tr("shareType")}</span
          >
        </label>
        <div class="grid grid-cols-2 gap-4">
          <label
            class="cursor-pointer border-2 rounded-xl p-4 transition-all {shareType ===
            'public'
              ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20'
              : 'border-gray-300 dark:border-gray-600 hover:border-blue-300'}"
          >
            <input
              type="radio"
              bind:group={shareType}
              value="public"
              class="radio radio-primary mb-2"
            />
            <div>
              <div class="font-semibold text-lg flex items-center gap-2">
                <i class="bi bi-globe text-blue-600"></i>
                {tr("publicLink")}
              </div>
              <div class="text-sm opacity-70 mt-1">
                {tr("anyoneWithLinkCanAccess")}
              </div>
            </div>
          </label>

          <label
            class="cursor-pointer border-2 rounded-xl p-4 transition-all {shareType ===
            'users'
              ? 'border-purple-500 bg-purple-50 dark:bg-purple-900/20'
              : 'border-gray-300 dark:border-gray-600 hover:border-purple-300'}"
          >
            <input
              type="radio"
              bind:group={shareType}
              value="users"
              class="radio radio-primary mb-2"
            />
            <div>
              <div class="font-semibold text-lg flex items-center gap-2">
                <i class="bi bi-people text-purple-600"></i>
                {tr("specificUsers")}
              </div>
              <div class="text-sm opacity-70 mt-1">
                {tr("shareWithSelectedUsers")}
              </div>
            </div>
          </label>
        </div>
      </div>

      <!-- User Selection (for user-to-user shares) -->
      {#if shareType === "users"}
        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold text-base">
              <i class="bi bi-person-plus mr-1"></i>
              {tr("selectUsers")}
            </span>
            <span class="label-text-alt"
              >{selectedUsers.length} {tr("selected")}</span
            >
          </label>

          {#if loadingUsers}
            <div class="flex justify-center py-8">
              <span class="loading loading-spinner loading-lg text-primary"
              ></span>
            </div>
          {:else}
            <div
              class="max-h-60 overflow-y-auto border-2 border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 p-2"
            >
              {#each availableUsers as user}
                <div
                  class="flex items-center gap-3 p-3 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                >
                  <input
                    type="checkbox"
                    class="checkbox checkbox-primary"
                    checked={isUserSelected(user.id)}
                    onchange={() => toggleUserSelection(user)}
                  />
                  <div class="flex-1">
                    <div class="font-medium text-gray-900 dark:text-white">
                      {user.display_name || user.username}
                    </div>
                    <div class="text-sm text-gray-500 dark:text-gray-400">
                      @{user.username}
                    </div>
                  </div>
                  {#if isUserSelected(user.id)}
                    <select
                      bind:value={userPermissions[user.id]}
                      class="select select-sm select-bordered"
                      onclick={(e) => e.stopPropagation()}
                    >
                      <option value="read">
                        <i class="bi bi-eye"></i>
                        {tr("read")}
                      </option>
                      <option value="write">
                        <i class="bi bi-pencil"></i>
                        {tr("write")}
                      </option>
                      <option value="admin">
                        <i class="bi bi-shield-check"></i>
                        {tr("admin")}
                      </option>
                    </select>
                  {/if}
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {/if}

      <!-- External Sharing Toggle -->
      <div class="form-control">
        <label
          class="label cursor-pointer justify-start gap-3 border-2 border-gray-300 dark:border-gray-600 rounded-xl p-4"
        >
          <input
            type="checkbox"
            bind:checked={allowExternal}
            class="toggle toggle-primary"
          />
          <div class="flex-1">
            <div class="font-semibold text-base">
              <i class="bi bi-box-arrow-up-right mr-1"></i>
              {tr("allowExternalSharing")}
            </div>
            <div class="text-sm opacity-70 mt-1">
              {tr("shareLinkWithPeopleOutsideSystem")}
            </div>
          </div>
        </label>
      </div>

      <!-- Password Protection -->
      <div class="form-control">
        <label
          class="label cursor-pointer justify-start gap-3 border-2 border-gray-300 dark:border-gray-600 rounded-xl p-4"
        >
          <input
            type="checkbox"
            bind:checked={requirePassword}
            class="toggle toggle-warning"
          />
          <div class="flex-1">
            <div class="font-semibold text-base">
              <i class="bi bi-lock mr-1"></i>
              {tr("passwordProtection")}
            </div>
            <div class="text-sm opacity-70 mt-1">
              {tr("requirePasswordToAccess")}
            </div>
          </div>
        </label>

        {#if requirePassword}
          <input
            type="password"
            placeholder={tr("enterPassword")}
            class="input input-bordered mt-3"
            bind:value={sharePassword}
            required
          />
        {/if}
      </div>

      <!-- Expiration Date -->
      <div class="form-control">
        <label class="label">
          <span class="label-text font-semibold text-base">
            <i class="bi bi-calendar-event mr-1"></i>
            {tr("expirationDate")}
          </span>
          <button
            class="label-text-alt link link-primary"
            onclick={() => (expiresAt = null)}
          >
            {tr("neverExpires")}
          </button>
        </label>
        <input
          type="datetime-local"
          class="input input-bordered"
          bind:value={expiresAt}
          min={new Date().toISOString().slice(0, 16)}
        />
        {#if expiresAt}
          <label class="label">
            <span class="label-text-alt text-amber-600 dark:text-amber-400">
              <i class="bi bi-exclamation-triangle mr-1"></i>
              {tr("expiresOn")}: {new Date(expiresAt).toLocaleString(
                $currentLang
              )}
            </span>
          </label>
        {/if}
      </div>

      <!-- Permission Preview (for public shares) -->
      {#if shareType === "public"}
        <div class="alert alert-info rounded-xl">
          <i class="bi bi-info-circle"></i>
          <div class="text-sm">
            {tr("publicSharesHaveReadOnlyAccess")}
          </div>
        </div>
      {/if}
    </div>
  {:else}
    <!-- Share Result - ENHANCED -->
    <div class="space-y-6">
      <!-- Success Message -->
      <div class="alert alert-success rounded-xl shadow-lg">
        <i class="bi bi-check-circle-fill text-3xl"></i>
        <div>
          <h3 class="font-bold text-lg">{tr("shareCreatedSuccessfully")}!</h3>
          <div class="text-sm mt-1">
            {#if shareType === "public"}
              {tr("anyoneWithLinkCanAccess")}
            {:else}
              {tr("sharedWith")} {selectedUsers.length} {tr("users")}
            {/if}
          </div>
        </div>
      </div>

      <!-- Share URL -->
      {#if allowExternal}
        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold text-base">
              <i class="bi bi-link-45deg mr-1"></i>
              {tr("shareUrl")}
            </span>
          </label>
          <div class="join w-full">
            <input
              type="text"
              readonly
              value={shareUrl}
              class="input input-bordered join-item flex-1 font-mono text-sm bg-gray-50 dark:bg-gray-800"
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
      {/if}

      <!-- Share Details Grid -->
      <div class="grid grid-cols-2 gap-4">
        <div class="card bg-base-200 p-4">
          <div class="text-sm opacity-70">{tr("type")}</div>
          <div class="font-bold text-xl mt-1">
            {#if shareType === "public"}
              <i class="bi bi-globe text-blue-600"></i> {tr("public")}
            {:else}
              <i class="bi bi-people text-purple-600"></i> {tr("users")}
            {/if}
          </div>
        </div>
        <div class="card bg-base-200 p-4">
          <div class="text-sm opacity-70">{tr("expires")}</div>
          <div class="font-bold text-xl mt-1">
            {#if expiresAt}
              <i class="bi bi-calendar-event text-amber-600"></i>
              {new Date(expiresAt).toLocaleDateString($currentLang)}
            {:else}
              <i class="bi bi-infinity text-green-600"></i> {tr("never")}
            {/if}
          </div>
        </div>
      </div>

      <!-- Selected Users (for user shares) -->
      {#if shareType === "users" && selectedUsers.length > 0}
        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold text-base">
              <i class="bi bi-people-fill mr-1"></i>
              {tr("sharedWithUsers")}
            </span>
          </label>
          <div class="space-y-2">
            {#each selectedUsers as user}
              <div
                class="flex items-center justify-between p-3 bg-base-200 rounded-lg"
              >
                <div class="flex items-center gap-3">
                  <div class="avatar placeholder">
                    <div
                      class="bg-primary text-primary-content w-10 rounded-full"
                    >
                      <span class="text-sm"
                        >{(user.display_name || user.username)
                          .charAt(0)
                          .toUpperCase()}</span
                      >
                    </div>
                  </div>
                  <div>
                    <div class="font-medium">
                      {user.display_name || user.username}
                    </div>
                    <div class="text-sm opacity-70">@{user.username}</div>
                  </div>
                </div>
                <div class="badge badge-lg gap-1">
                  {#if userPermissions[user.id] === "read"}
                    <i class="bi bi-eye"></i> {tr("read")}
                  {:else if userPermissions[user.id] === "write"}
                    <i class="bi bi-pencil"></i> {tr("write")}
                  {:else if userPermissions[user.id] === "admin"}
                    <i class="bi bi-shield-check"></i> {tr("admin")}
                  {/if}
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Security Features -->
      <div class="flex flex-wrap gap-3">
        {#if requirePassword}
          <div class="badge badge-warning badge-lg gap-2">
            <i class="bi bi-lock-fill"></i>
            {tr("passwordProtected")}
          </div>
        {/if}
        {#if allowExternal}
          <div class="badge badge-info badge-lg gap-2">
            <i class="bi bi-box-arrow-up-right"></i>
            {tr("externalSharingEnabled")}
          </div>
        {/if}
        {#if !allowExternal}
          <div class="badge badge-success badge-lg gap-2">
            <i class="bi bi-shield-check"></i>
            {tr("internalOnly")}
          </div>
        {/if}
      </div>
    </div>
  {/if}

  <!-- Actions slot - outside {#if} for proper Svelte slot placement -->
  <div slot="actions" class="flex gap-3 justify-end">
    {#if !showShareResult}
      <button class="btn btn-ghost gap-2" onclick={close}>
        <i class="bi bi-x-lg"></i>
        {tr("cancel")}
      </button>
      <button
        class="btn btn-primary gap-2"
        onclick={handleCreateShare}
        disabled={loading ||
          (shareType === "users" && selectedUsers.length === 0) ||
          (requirePassword && !sharePassword.trim())}
      >
        {#if loading}
          <span class="loading loading-spinner loading-sm"></span>
        {:else}
          <i class="bi bi-share-fill"></i>
        {/if}
        {tr("createShare")}
      </button>
    {:else}
      {#if allowExternal}
        <button class="btn btn-primary gap-2" onclick={copyShareUrl}>
          <i class="bi bi-clipboard-fill"></i>
          {tr("copyUrl")}
        </button>
      {/if}
      <button class="btn btn-ghost gap-2" onclick={close}>
        <i class="bi bi-check-lg"></i>
        {tr("done")}
      </button>
    {/if}
  </div>
</Modal>
