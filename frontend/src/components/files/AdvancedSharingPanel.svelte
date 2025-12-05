<script>
  /**
   * Advanced File Sharing Panel
   * Create and manage share links with expiry, password, download limits
   */
  import { onMount } from "svelte";
  import {
    shareState,
    activeShares,
    loading,
    error,
    loadShares,
    createShare,
    updateShare,
    deleteShare,
    regenerateShareToken,
    copyShareLink,
    getExpiryLabel,
    isShareExpired,
    isDownloadLimitReached,
  } from "../stores/advancedSharing.js";

  let { filePath = "" } = $props();
  let { readOnly = false } = $props();

  let showCreateForm = false;
  let permission = "read";
  let expiresIn = "1month";
  let password = "";
  let downloadLimit = "";
  let copiedLinkId = null;

  onMount(async () => {
    if (filePath) {
      await loadShares(filePath);
    }
  });

  $: if (filePath) {
    loadShares(filePath);
  }

  async function handleCreateShare() {
    await createShare(filePath, {
      permission,
      expiresIn,
      password: password || undefined,
      downloadLimit: downloadLimit ? parseInt(downloadLimit) : undefined,
    });

    // Reset form
    showCreateForm = false;
    permission = "read";
    expiresIn = "1month";
    password = "";
    downloadLimit = "";
  }

  function handleCopyLink(shareToken, shareId) {
    const link = copyShareLink(shareToken);
    copiedLinkId = shareId;
    setTimeout(() => (copiedLinkId = null), 2000);
  }

  function formatDate(date) {
    if (!date) return "Never";
    return new Date(date).toLocaleDateString("de-DE", {
      month: "short",
      day: "numeric",
      year: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
    });
  }
</script>

<div
  class="flex flex-col h-full bg-white dark:bg-slate-800 rounded-lg border border-slate-200 dark:border-slate-700"
>
  <!-- Header -->
  <div
    class="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700"
  >
    <div class="flex items-center gap-2">
      <i class="bi bi-share text-lg text-green-500" / aria-hidden="true">
      <span class="font-medium text-slate-900 dark:text-white">
        Shares ({$activeShares.length})
      </span>
    </div>

    {#if !readOnly}
      <button
        on:click={() => (showCreateForm = !showCreateForm)}
        class="px-3 py-1.5 bg-green-500 hover:bg-green-600 text-white rounded text-sm font-medium transition-colors"
      >
        <i class="bi bi-plus" / aria-hidden="true">
        New Share
      </button>
    {/if}
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-y-auto p-4">
    {#if $loading}
      <div class="flex items-center justify-center h-full">
        <div class="animate-spin">
          <i class="bi bi-hourglass text-2xl text-green-500" / aria-hidden="true">
        </div>
      </div>
    {:else if $error}
      <div
        class="bg-red-50 dark:bg-red-900/20 p-3 rounded text-red-700 dark:text-red-200 text-sm"
      >
        <i class="bi bi-exclamation-circle mr-2" / aria-hidden="true">
        {$error}
      </div>
    {:else}
      <!-- Create Form -->
      {#if showCreateForm && !readOnly}
        <div class="bg-green-50 dark:bg-green-900/20 p-4 rounded mb-4 space-y-3">
          <h3 class="font-medium text-green-900 dark:text-green-100">
            Create New Share
          </h3>

          <!-- Permission -->
          <div>
            <div class="text-sm font-medium text-slate-700 dark:text-slate-300 block mb-1">
              Permission
            </div>
            <select
              bind:value={permission}
              class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded text-sm bg-white dark:bg-slate-800 text-slate-900 dark:text-white"
            >
              <option value="read">View Only</option>
              <option value="write">Edit & Upload</option>
            </select>
          </div>

          <!-- Expiry -->
          <div>
            <div class="text-sm font-medium text-slate-700 dark:text-slate-300 block mb-1">
              Expiry
            </div>
            <select
              bind:value={expiresIn}
              class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded text-sm bg-white dark:bg-slate-800 text-slate-900 dark:text-white"
            >
              <option value="never">Never</option>
              <option value="1week">1 Week</option>
              <option value="1month">1 Month</option>
              <option value="90days">90 Days</option>
              <option value="1year">1 Year</option>
            </select>
          </div>

          <!-- Password (Optional) -->
          <div>
            <div class="text-sm font-medium text-slate-700 dark:text-slate-300 block mb-1">
              Password (Optional)
            </div>
            <input
              type="password"
              placeholder="Leave empty for no password"
              bind:value={password}
              class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded text-sm bg-white dark:bg-slate-800 text-slate-900 dark:text-white"
            />
          </div>

          <!-- Download Limit (Optional) -->
          <div>
            <div class="text-sm font-medium text-slate-700 dark:text-slate-300 block mb-1">
              Download Limit (Optional)
            </div>
            <input
              type="number"
              placeholder="Leave empty for unlimited"
              min="1"
              bind:value={downloadLimit}
              class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded text-sm bg-white dark:bg-slate-800 text-slate-900 dark:text-white"
            />
          </div>

          <!-- Actions -->
          <div class="flex gap-2">
            <button
              on:click={handleCreateShare}
              class="flex-1 px-4 py-2 bg-green-500 hover:bg-green-600 text-white rounded text-sm font-medium transition-colors"
            >
              <i class="bi bi-check" / aria-hidden="true">
              Create Share
            </button>
            <button
              on:click={() => (showCreateForm = false)}
              class="flex-1 px-4 py-2 bg-slate-300 dark:bg-slate-600 text-slate-900 dark:text-white rounded text-sm font-medium transition-colors"
            >
              Cancel
            </button>
          </div>
        </div>
      {/if}

      <!-- Shares List -->
      {#if $activeShares.length === 0}
        <p class="text-center text-slate-500 dark:text-slate-400 text-sm py-8">
          No active shares
        </p>
      {:else}
        <div class="space-y-3">
          {#each $activeShares as share (share.id)}
            <div
              class="border border-slate-200 dark:border-slate-700 rounded p-3 space-y-2"
            >
              <!-- Header -->
              <div class="flex items-start justify-between gap-2">
                <div>
                  <p class="text-sm font-medium text-slate-900 dark:text-white">
                    {share.permission === "read" ? "👁️ View" : "✏️ Edit"}
                    {#if share.password}
                      <i
                        class="bi bi-lock-fill text-amber-500 ml-1"
                        title="Password protected"
                      />
                    {/if}
                  </p>
                  <p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
                    Created {new Date(share.createdAt).toLocaleDateString(
                      "de-DE"
                    )}
                  </p>
                </div>

                <div
                  class={!isShareExpired(share.expiresAt)
                    ? "px-2 py-1 rounded text-xs font-medium bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-200"
                    : "px-2 py-1 rounded text-xs font-medium bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-200"}
                >
                  {getExpiryLabel(share.expiresAt)}
                </div>
              </div>

              <!-- Share Link -->
              <div
                class="bg-slate-50 dark:bg-slate-700/50 p-2 rounded text-xs font-mono text-slate-600 dark:text-slate-400 break-all"
              >
                {share.shareToken}
              </div>

              <!-- Stats -->
              {#if share.downloadLimit}
                <p class="text-xs text-slate-600 dark:text-slate-400">
                  <i class="bi bi-download mr-1" / aria-hidden="true">
                  Downloads: {share.downloads || 0} / {share.downloadLimit}
                </p>
              {/if}

              <!-- Actions -->
              <div class="flex gap-2">
                <button
                  on:click={() => handleCopyLink(share.shareToken, share.id)}
                  class="flex-1 px-3 py-1.5 text-sm rounded transition-colors"
                  class:bg-green-100={copiedLinkId === share.id}
                  class:dark:bg-green-900={copiedLinkId === share.id}
                  class:text-green-700={copiedLinkId === share.id}
                  class:dark:text-green-200={copiedLinkId === share.id}
                  class:bg-slate-200={copiedLinkId !== share.id}
                  class:dark:bg-slate-700={copiedLinkId !== share.id}
                  class:text-slate-700={copiedLinkId !== share.id}
                  class:dark:text-slate-300={copiedLinkId !== share.id}
                >
                  <i class="bi bi-copy mr-1" / aria-hidden="true">
                  {copiedLinkId === share.id ? "Copied!" : "Copy Link"}
                </button>

                {#if !readOnly}
                  <button
                    on:click={() => regenerateShareToken(share.id)}
                    class="px-3 py-1.5 bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-200 text-sm rounded hover:bg-orange-200 dark:hover:bg-orange-900 transition-colors"
                    title="Generate new link"
                  >
                    <i class="bi bi-arrow-repeat" / aria-hidden="true">
                  </button>

                  <button
                    on:click={() => deleteShare(share.id)}
                    class="px-3 py-1.5 bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-200 text-sm rounded hover:bg-red-200 dark:hover:bg-red-900 transition-colors"
                  >
                    <i class="bi bi-trash" / aria-hidden="true">
                  </button>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      {/if}
    {/if}
  </div>
</div>
