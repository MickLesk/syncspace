<script>
  import { createEventDispatcher } from "svelte";
  import { success, error as errorToast } from "../../stores/toast.js";
  import { currentLang } from "../../stores/ui";
  import { t } from "../../i18n.js";
  import Modal from "../ui/Modal.svelte";
  import FileBrowserModal from "../files/FileBrowserModal.svelte";
  import UserSearchModal from "../user/UserSearchModal.svelte";
  import api from "../../lib/api.js";

  const dispatch = createEventDispatcher();

  // Proper reactive translation function
  function tr(key, ...args) {
    return t($currentLang, key, ...args);
  }

  // Props
  let { isOpen = $bindable(false), file = null, onClose = () => {} } = $props();

  // Share state - ENHANCED (all with $state for reactivity)
  let shareType = $state("public"); // 'public' or 'users'
  let selectedUsers = $state([]); // Array of {id, username, display_name}
  let userPermissions = $state({}); // Map: user_id => 'read'|'write'|'admin'
  let allowExternal = $state(true); // Toggle for external sharing
  let requirePassword = $state(false);
  let sharePassword = $state("");
  let expiresAt = $state(null); // ISO date string from date input
  let loading = $state(false);
  let shareUrl = $state("");
  let showShareResult = $state(false);
  let showUserSearchModal = $state(false);

  // User list from backend
  let availableUsers = $state([]);
  let loadingUsers = $state(false);

  // File selection state (when no file prop provided)
  let showFileBrowser = $state(false);
  let selectedFilePaths = $state([]);

  const isFile = $derived(file && !file.is_dir);
  const isFolder = $derived(file && file.is_dir);
  const needsFileSelection = $derived(!file && selectedFilePaths.length === 0);

  // Load users when modal opens
  $effect(() => {
    if (isOpen && shareType === "users" && availableUsers.length === 0) {
      loadUsers();
    }
  });

  // Set allowExternal based on shareType
  $effect(() => {
    if (shareType === "public") {
      allowExternal = true; // Public links are always external
    } else {
      allowExternal = false; // User shares are internal by default
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

  function handleUserSelect(user) {
    toggleUserSelection(user);
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

  function setExpirationDays(days) {
    const date = new Date();
    date.setDate(date.getDate() + days);
    expiresAt = date.toISOString().slice(0, 16);
  }

  async function handleCreateShare() {
    // Get file paths to share
    const filePaths = file ? [file.path || file.file_path] : selectedFilePaths;

    if (filePaths.length === 0) {
      errorToast(tr("pleaseSelectAtLeastOneFile"));
      return;
    }

    loading = true;
    try {
      // Create shares for all selected files
      const shareResponses = [];
      for (const filePath of filePaths) {
        const requestData = {
          file_path: filePath,
          permission: allowExternal ? "read" : "none",
          expires_at: expiresAt || undefined,
          password:
            requirePassword && sharePassword ? sharePassword : undefined,
          user_ids:
            shareType === "users" ? selectedUsers.map((u) => u.id) : undefined,
          permissions:
            shareType === "users"
              ? selectedUsers.map((u) => userPermissions[u.id] || "read")
              : undefined,
          allow_external: allowExternal,
        };

        const response = await api.sharing.create(requestData);
        shareResponses.push(response.data || response);
      }

      // Use first share for URL display
      const firstShare = shareResponses[0];
      shareUrl = `${window.location.origin}/share/${firstShare.id || firstShare.share_id || firstShare.token}`;
      showShareResult = true;

      success(
        filePaths.length === 1
          ? tr("shareCreatedSuccessfully")
          : tr("sharesCreatedSuccessfully", filePaths.length)
      );
    } catch (err) {
      console.error("Share error:", err);
      errorToast(err.message || tr("failedToCreateShare"));
    } finally {
      loading = false;
    }
  }

  function handleFileSelection(selectedFiles) {
    selectedFilePaths = selectedFiles;
    showFileBrowser = false;
  }

  function removeFileFromSelection(filePath) {
    selectedFilePaths = selectedFilePaths.filter((p) => p !== filePath);
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

  // Reset on modal open (handled by effect above)
</script>

<Modal
  visible={isOpen}
  title={file ? tr("shareFileName", file.name) : tr("createShare")}
  icon="share"
  size="lg"
  variant="primary"
  onclose={close}
>
  {#snippet children()}
    <!-- File Browser Modal (when no file provided) -->
    <FileBrowserModal
      bind:isOpen={showFileBrowser}
      onSelect={handleFileSelection}
      mode="both"
      title={tr("selectFilesToShare")}
    />

    {#if !showShareResult}
      <!-- File Selection Section (when no file prop) -->
      {#if !file}
        <div
          class="mb-6 p-4 bg-gray-50 dark:bg-gray-800 rounded-xl border-2 border-dashed border-gray-300 dark:border-gray-600"
        >
          <div class="flex items-center justify-between mb-3">
            <h3
              class="text-sm font-semibold text-gray-900 dark:text-white flex items-center gap-2"
            >
              <i class="bi bi-folder2-open text-lg text-primary"></i>
              {tr("selectedFiles")} ({selectedFilePaths.length})
            </h3>
            <button
              type="button"
              class="btn btn-sm btn-primary"
              onclick={() => (showFileBrowser = true)}
            >
              <i class="bi bi-plus-circle"></i>
              {tr("selectFiles")}
            </button>
          </div>

          {#if selectedFilePaths.length === 0}
            <p
              class="text-sm text-gray-500 dark:text-gray-400 text-center py-4"
            >
              {tr("noFilesSelected")}
            </p>
          {:else}
            <div class="space-y-2 max-h-48 overflow-y-auto">
              {#each selectedFilePaths as filePath}
                <div
                  class="flex items-center justify-between p-2 bg-white dark:bg-gray-700 rounded-lg"
                >
                  <div class="flex items-center gap-2 min-w-0 flex-1">
                    <i class="bi bi-file-earmark text-primary flex-shrink-0"
                    ></i>
                    <span class="text-sm font-medium truncate"
                      >{filePath.split("/").pop()}</span
                    >
                  </div>
                  <button
                    type="button"
                    class="btn btn-ghost btn-sm btn-circle"
                    onclick={() => removeFileFromSelection(filePath)}
                  >
                    <i class="bi bi-x-lg"></i>
                  </button>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {/if}

      <!-- Share Configuration - ENHANCED -->
      <div class="space-y-5">
        <!-- Share Type -->
        <div>
          <label
            class="block text-sm font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2"
          >
            <i class="bi bi-share text-lg text-primary"></i>
            {tr("shareType")}
          </label>
          <div class="grid grid-cols-2 gap-4">
            <!-- Public Link Card -->
            <div
              class="group cursor-pointer border-2 rounded-2xl p-6 transition-all duration-200 {shareType ===
              'public'
                ? 'border-blue-500 bg-gradient-to-br from-blue-50 to-blue-100 dark:from-blue-900/30 dark:to-blue-800/20 shadow-lg scale-[1.02]'
                : 'border-gray-200 dark:border-gray-700 hover:border-blue-300 dark:hover:border-blue-600 hover:shadow-md hover:scale-[1.01]'}"
              onclick={() => (shareType = "public")}
              role="button"
              tabindex="0"
              onkeydown={(e) => e.key === "Enter" && (shareType = "public")}
            >
              <div class="flex flex-col items-center text-center gap-4">
                <div
                  class="w-16 h-16 rounded-2xl bg-gradient-to-br from-blue-500 to-blue-600 dark:from-blue-600 dark:to-blue-700 flex items-center justify-center shadow-lg {shareType ===
                  'public'
                    ? 'ring-4 ring-blue-200 dark:ring-blue-800'
                    : ''}"
                >
                  <i class="bi bi-globe text-3xl text-white"></i>
                </div>
                <div>
                  <div
                    class="font-bold text-lg text-gray-900 dark:text-white mb-1"
                  >
                    {tr("publicLink")}
                  </div>
                  <div class="text-sm text-gray-600 dark:text-gray-400">
                    {tr("anyoneWithLinkCanAccess")}
                  </div>
                </div>
              </div>
            </div>

            <!-- Specific Users Card -->
            <div
              class="group cursor-pointer border-2 rounded-2xl p-6 transition-all duration-200 {shareType ===
              'users'
                ? 'border-purple-500 bg-gradient-to-br from-purple-50 to-purple-100 dark:from-purple-900/30 dark:to-purple-800/20 shadow-lg scale-[1.02]'
                : 'border-gray-200 dark:border-gray-700 hover:border-purple-300 dark:hover:border-purple-600 hover:shadow-md hover:scale-[1.01]'}"
              onclick={() => (shareType = "users")}
              role="button"
              tabindex="0"
              onkeydown={(e) => e.key === "Enter" && (shareType = "users")}
            >
              <div class="flex flex-col items-center text-center gap-4">
                <div
                  class="w-16 h-16 rounded-2xl bg-gradient-to-br from-purple-500 to-purple-600 dark:from-purple-600 dark:to-purple-700 flex items-center justify-center shadow-lg {shareType ===
                  'users'
                    ? 'ring-4 ring-purple-200 dark:ring-purple-800'
                    : ''}"
                >
                  <i class="bi bi-people text-3xl text-white"></i>
                </div>
                <div>
                  <div
                    class="font-bold text-lg text-gray-900 dark:text-white mb-1"
                  >
                    {tr("specificUsers")}
                  </div>
                  <div class="text-sm text-gray-600 dark:text-gray-400">
                    {tr("shareWithSelectedUsers")}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- User Selection (for user-to-user shares) -->
        {#if shareType === "users"}
          <div>
            <div class="flex items-center justify-between mb-3">
              <label
                class="text-sm font-semibold text-gray-900 dark:text-white flex items-center gap-2"
              >
                <i class="bi bi-person-plus text-lg"></i>
                {tr("selectUsers")}
              </label>
              <button
                type="button"
                onclick={() => (showUserSearchModal = true)}
                class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors flex items-center gap-2"
              >
                <i class="bi bi-search"></i>
                {tr("searchUsers")}
              </button>
            </div>

            <!-- Selected Users Display -->
            {#if selectedUsers.length > 0}
              <div class="space-y-2 mb-4">
                {#each selectedUsers as user}
                  <div
                    class="flex items-center gap-3 p-3 bg-blue-50 dark:bg-blue-900/20 border-2 border-blue-500 rounded-lg"
                  >
                    <!-- User Avatar -->
                    <div class="flex-shrink-0">
                      {#if user.avatar_base64}
                        <img
                          src={`data:image/png;base64,${user.avatar_base64}`}
                          alt={user.display_name || user.username}
                          class="w-10 h-10 rounded-full object-cover"
                        />
                      {:else}
                        <div
                          class="w-10 h-10 rounded-full bg-gradient-to-br from-blue-400 to-purple-500 flex items-center justify-center text-white font-semibold"
                        >
                          {(user.display_name || user.username)
                            .slice(0, 2)
                            .toUpperCase()}
                        </div>
                      {/if}
                    </div>

                    <!-- User Info -->
                    <div class="flex-1 min-w-0">
                      <div
                        class="font-medium text-gray-900 dark:text-white truncate"
                      >
                        {user.display_name || user.username}
                      </div>
                      <div class="text-sm text-gray-500 dark:text-gray-400">
                        @{user.username}
                      </div>
                    </div>

                    <!-- Permission Selector -->
                    <select
                      bind:value={userPermissions[user.id]}
                      class="px-3 py-1 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg text-sm"
                    >
                      <option value="read">{tr("read")}</option>
                      <option value="write">{tr("write")}</option>
                      <option value="admin">{tr("admin")}</option>
                    </select>

                    <!-- Remove Button -->
                    <button
                      type="button"
                      onclick={() => toggleUserSelection(user)}
                      class="p-2 text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition-colors"
                      title={tr("remove")}
                    >
                      <i class="bi bi-x-lg"></i>
                    </button>
                  </div>
                {/each}
              </div>
            {:else}
              <div class="text-center py-8 text-gray-500 dark:text-gray-400">
                <i class="bi bi-person-plus-fill text-3xl mb-2"></i>
                <p class="text-sm">{tr("clickSearchToAddUsers")}</p>
              </div>
            {/if}
          </div>
        {/if}

        <!-- Password Protection -->
        <div
          class="border-2 rounded-2xl p-5 transition-all {requirePassword
            ? 'border-amber-400 bg-amber-50 dark:bg-amber-900/20'
            : 'border-gray-200 dark:border-gray-700'}"
        >
          <label class="flex items-center gap-4 cursor-pointer">
            <div
              class="w-12 h-12 rounded-xl bg-gradient-to-br from-amber-500 to-amber-600 dark:from-amber-600 dark:to-amber-700 flex items-center justify-center flex-shrink-0 shadow-md"
            >
              <i class="bi bi-lock-fill text-2xl text-white"></i>
            </div>
            <div class="flex-1">
              <div
                class="font-semibold text-base text-gray-900 dark:text-white"
              >
                {tr("passwordProtection")}
              </div>
              <div class="text-sm text-gray-600 dark:text-gray-400 mt-0.5">
                {tr("requirePasswordToAccess")}
              </div>
            </div>
            <input
              type="checkbox"
              bind:checked={requirePassword}
              class="toggle toggle-lg toggle-warning flex-shrink-0"
            />
          </label>

          {#if requirePassword}
            <div class="mt-4 pl-16">
              <div class="relative">
                <i
                  class="bi bi-key-fill absolute left-4 top-1/2 -translate-y-1/2 text-gray-400"
                ></i>
                <input
                  type="password"
                  placeholder={tr("enterPassword")}
                  class="input input-bordered w-full pl-12 bg-white dark:bg-gray-700 focus:ring-2 focus:ring-amber-400 {requirePassword &&
                  !sharePassword.trim()
                    ? 'border-red-400'
                    : ''}"
                  bind:value={sharePassword}
                  required
                />
              </div>
              {#if sharePassword}
                <div
                  class="mt-2 flex items-center gap-2 text-xs text-green-600 dark:text-green-400"
                >
                  <i class="bi bi-check-circle-fill"></i>
                  <span>{tr("passwordSet")}</span>
                </div>
              {:else}
                <div
                  class="mt-2 flex items-center gap-2 text-xs text-red-600 dark:text-red-400"
                >
                  <i class="bi bi-exclamation-triangle-fill"></i>
                  <span>{tr("passwordRequired")}</span>
                </div>
              {/if}
            </div>
          {/if}
        </div>

        <!-- Expiration Date -->
        <div
          class="border-2 rounded-2xl p-5 transition-all {expiresAt
            ? 'border-purple-400 bg-purple-50 dark:bg-purple-900/20'
            : 'border-gray-200 dark:border-gray-700'}"
        >
          <div class="flex items-center gap-4 mb-4">
            <div
              class="w-12 h-12 rounded-xl bg-gradient-to-br from-purple-500 to-purple-600 dark:from-purple-600 dark:to-purple-700 flex items-center justify-center flex-shrink-0 shadow-md"
            >
              <i class="bi bi-calendar-event text-2xl text-white"></i>
            </div>
            <div class="flex-1">
              <div
                class="font-semibold text-base text-gray-900 dark:text-white"
              >
                {tr("expirationDate")}
              </div>
              <div class="text-sm text-gray-600 dark:text-gray-400 mt-0.5">
                {expiresAt ? tr("shareWillExpire") : tr("neverExpires")}
              </div>
            </div>
            {#if expiresAt}
              <button
                type="button"
                class="btn btn-sm btn-ghost gap-1 text-purple-600 hover:text-purple-700 dark:text-purple-400"
                onclick={() => (expiresAt = null)}
              >
                <i class="bi bi-x-circle"></i>
                {tr("clear")}
              </button>
            {/if}
          </div>

          <!-- Quick Select Buttons -->
          <div class="grid grid-cols-4 gap-2 mb-4">
            <button
              type="button"
              class="btn btn-sm btn-outline gap-1"
              onclick={() => setExpirationDays(1)}
            >
              <i class="bi bi-hourglass-split"></i>
              <span>1 {tr("day")}</span>
            </button>
            <button
              type="button"
              class="btn btn-sm btn-outline gap-1"
              onclick={() => setExpirationDays(7)}
            >
              <i class="bi bi-calendar-week"></i>
              <span>1 {tr("week")}</span>
            </button>
            <button
              type="button"
              class="btn btn-sm btn-outline gap-1"
              onclick={() => setExpirationDays(30)}
            >
              <i class="bi bi-calendar-month"></i>
              <span>1 {tr("month")}</span>
            </button>
            <button
              type="button"
              class="btn btn-sm btn-outline gap-1"
              onclick={() => setExpirationDays(90)}
            >
              <i class="bi bi-calendar3"></i>
              <span>3 {tr("months")}</span>
            </button>
          </div>

          {#if expiresAt}
            <div
              class="flex items-start gap-2 p-3 bg-purple-100 dark:bg-purple-900/30 rounded-lg border border-purple-200 dark:border-purple-800"
            >
              <i
                class="bi bi-info-circle-fill text-purple-600 dark:text-purple-400 text-lg flex-shrink-0 mt-0.5"
              ></i>
              <div class="text-sm text-purple-700 dark:text-purple-300 flex-1">
                <div class="font-semibold">{tr("expiresOn")}:</div>
                <div class="mt-1">
                  {new Date(expiresAt).toLocaleString($currentLang, {
                    dateStyle: "full",
                    timeStyle: "short",
                  })}
                </div>
              </div>
            </div>
          {/if}
        </div>

        <!-- Permission Preview (for public shares) -->
        {#if shareType === "public"}
          <div class="alert alert-info rounded-xl shadow-sm">
            <i class="bi bi-info-circle"></i>
            <div class="text-sm">
              {tr("publicSharesHaveReadOnlyAccess")}
            </div>
          </div>
        {/if}
      </div>
    {:else}
      <!-- Share Result - ENHANCED -->
      <div class="space-y-5">
        <!-- Success Message -->
        <div class="alert alert-success rounded-xl shadow-md">
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
            <label class="label py-0 mb-2">
              <span
                class="label-text font-semibold text-base flex items-center gap-2"
              >
                <i class="bi bi-link-45deg"></i>
                {tr("shareUrl")}
              </span>
            </label>
            <div class="join w-full shadow-sm">
              <input
                type="text"
                readonly
                value={shareUrl}
                class="input input-sm input-bordered join-item flex-1 font-mono text-sm bg-gray-50 dark:bg-gray-800"
              />
              <button
                class="btn btn-sm btn-primary join-item gap-2"
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
          <div
            class="border shadow-sm rounded-lg p-4 bg-gray-50 dark:bg-gray-800"
          >
            <div class="text-sm text-gray-600 dark:text-gray-400 mb-1">
              {tr("type")}
            </div>
            <div class="font-semibold text-lg mt-1 flex items-center gap-2">
              {#if shareType === "public"}
                <i class="bi bi-globe text-blue-500"></i>
                <span>{tr("public")}</span>
              {:else}
                <i class="bi bi-people text-purple-500"></i>
                <span>{tr("users")}</span>
              {/if}
            </div>
          </div>
          <div
            class="border shadow-sm rounded-lg p-4 bg-gray-50 dark:bg-gray-800"
          >
            <div class="text-sm text-gray-600 dark:text-gray-400 mb-1">
              {tr("expires")}
            </div>
            <div class="font-semibold text-lg mt-1 flex items-center gap-2">
              {#if expiresAt}
                <i class="bi bi-calendar-event text-amber-500"></i>
                <span
                  >{new Date(expiresAt).toLocaleDateString($currentLang)}</span
                >
              {:else}
                <i class="bi bi-infinity text-green-500"></i>
                <span>{tr("never")}</span>
              {/if}
            </div>
          </div>
        </div>

        <!-- Selected Users (for user shares) -->
        {#if shareType === "users" && selectedUsers.length > 0}
          <div class="form-control">
            <label class="label py-0 mb-2">
              <span
                class="label-text font-semibold text-base flex items-center gap-2"
              >
                <i class="bi bi-people-fill"></i>
                {tr("sharedWithUsers")}
              </span>
            </label>
            <div class="space-y-2">
              {#each selectedUsers as user}
                <div
                  class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded-lg border shadow-sm"
                >
                  <div class="flex items-center gap-3">
                    <div class="avatar placeholder">
                      <div
                        class="bg-primary text-primary-content w-10 rounded-full"
                      >
                        <span class="text-sm font-semibold"
                          >{(user.display_name || user.username)
                            .charAt(0)
                            .toUpperCase()}</span
                        >
                      </div>
                    </div>
                    <div>
                      <div class="font-medium text-gray-900 dark:text-white">
                        {user.display_name || user.username}
                      </div>
                      <div class="text-sm text-gray-600 dark:text-gray-400">
                        @{user.username}
                      </div>
                    </div>
                  </div>
                  <div class="badge badge-lg gap-1 border shadow-sm">
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
            <div class="badge badge-warning badge-lg gap-2 shadow-sm">
              <i class="bi bi-lock-fill"></i>
              {tr("passwordProtected")}
            </div>
          {/if}
          {#if allowExternal}
            <div class="badge badge-info badge-lg gap-2 shadow-sm">
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
  {/snippet}

  {#snippet actions()}
    {#if !showShareResult}
      <button class="btn btn-lg btn-ghost gap-2" onclick={close}>
        <i class="bi bi-x-lg"></i>
        {tr("cancel")}
      </button>
      <button
        class="btn btn-lg btn-primary gap-2 shadow-lg hover:shadow-xl transition-all"
        onclick={handleCreateShare}
        disabled={loading ||
          (shareType === "users" && selectedUsers.length === 0) ||
          (requirePassword && !sharePassword.trim())}
      >
        {#if loading}
          <span class="loading loading-spinner loading-md"></span>
        {:else}
          <i class="bi bi-share-fill text-xl"></i>
        {/if}
        <span class="font-semibold">{tr("createShare")}</span>
      </button>
    {:else}
      {#if allowExternal}
        <button
          class="btn btn-lg btn-primary gap-2 shadow-lg"
          onclick={copyShareUrl}
        >
          <i class="bi bi-clipboard-fill text-xl"></i>
          {tr("copyUrl")}
        </button>
      {/if}
      <button class="btn btn-lg btn-ghost gap-2" onclick={close}>
        <i class="bi bi-check-lg text-xl"></i>
        {tr("done")}
      </button>
    {/if}
  {/snippet}
</Modal>

<!-- User Search Modal -->
<UserSearchModal
  bind:isOpen={showUserSearchModal}
  {selectedUsers}
  onSelect={handleUserSelect}
/>
