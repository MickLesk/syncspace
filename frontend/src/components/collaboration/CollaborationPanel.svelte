<script>
  import { onMount, onDestroy } from "svelte";
  import api from "../../lib/api.js";
  import { auth } from "../../stores/auth.js";
  import { success, error as errorToast } from "../../stores/toast.js";

  /** @type {{filePath?: string | null}} */
  let { filePath = null } = $props();

  /** @type {Array} active users viewing/editing */
  let presenceList = $state([]);
  /** @type {Object | null} active file lock */
  let currentLock = $state(null);
  /** @type {boolean} loading state */
  let loading = $state(false);
  /** @type {number | null} presence update interval */
  let presenceInterval = null;
  /** @type {number | null} lock heartbeat interval */
  let lockHeartbeatInterval = null;

  // Load presence data
  async function loadPresence() {
    if (!filePath) return;

    try {
      const data = await api.collaboration.getPresence(filePath);
      presenceList = data.filter((p) => p.user_id !== $auth?.userId);
    } catch (err) {
      console.error("Failed to load presence:", err);
    }
  }

  // Check for locks
  async function checkLocks() {
    if (!filePath) return;

    try {
      const locks = await api.collaboration.listLocks(filePath);
      const myLock = locks.find((l) => l.user_id === $auth?.userId);
      const otherLock = locks.find((l) => l.user_id !== $auth?.userId);

      currentLock = myLock || otherLock || null;
    } catch (err) {
      console.error("Failed to check locks:", err);
    }
  }

  // Update own presence
  async function updatePresence(activityType = "viewing") {
    if (!filePath) return;

    try {
      await api.collaboration.updatePresence(filePath, activityType);
    } catch (err) {
      console.error("Failed to update presence:", err);
    }
  }

  // Acquire file lock
  async function acquireLock() {
    if (!filePath) return;

    try {
      loading = true;
      const lock = await api.collaboration.acquireLock(filePath);
      currentLock = lock;

      // Start heartbeat
      startLockHeartbeat(lock.id);

      success("File locked for editing");
    } catch (err) {
      errorToast("Failed to acquire lock - file may be locked by another user");
    } finally {
      loading = false;
    }
  }

  // Release file lock
  async function releaseLock() {
    if (!currentLock) return;

    try {
      loading = true;
      await api.collaboration.releaseLock(currentLock.id);

      // Stop heartbeat
      stopLockHeartbeat();

      currentLock = null;
      success("File lock released");
    } catch (err) {
      errorToast("Failed to release lock");
    } finally {
      loading = false;
    }
  }

  // Start lock heartbeat
  function startLockHeartbeat(lockId) {
    stopLockHeartbeat();

    lockHeartbeatInterval = setInterval(async () => {
      try {
        await api.collaboration.renewLock(lockId);
      } catch (err) {
        console.error("Lock heartbeat failed:", err);
        errorToast("Lost file lock");
        stopLockHeartbeat();
        currentLock = null;
      }
    }, 30000); // Renew every 30 seconds
  }

  // Stop lock heartbeat
  function stopLockHeartbeat() {
    if (lockHeartbeatInterval) {
      clearInterval(lockHeartbeatInterval);
      lockHeartbeatInterval = null;
    }
  }

  // Start presence updates
  function startPresenceUpdates() {
    loadPresence();
    checkLocks();
    updatePresence("viewing");

    presenceInterval = setInterval(() => {
      loadPresence();
      checkLocks();
      updatePresence("viewing");
    }, 5000); // Update every 5 seconds
  }

  // Stop presence updates
  function stopPresenceUpdates() {
    if (presenceInterval) {
      clearInterval(presenceInterval);
      presenceInterval = null;
    }
  }

  // Cleanup on destroy
  onDestroy(() => {
    stopPresenceUpdates();
    stopLockHeartbeat();

    // Remove presence
    if (filePath && $auth?.userId) {
      api.collaboration.removePresence($auth?.userId).catch(() => {});
    }

    // Release lock if owned
    if (currentLock && currentLock.user_id === $auth?.userId) {
      api.collaboration.releaseLock(currentLock.id).catch(() => {});
    }
  });

  // Watch file path changes
  $effect(() => {
    if (filePath) {
      startPresenceUpdates();
    } else {
      stopPresenceUpdates();
    }
  });

  // Computed
  const hasOtherUsers = $derived(presenceList.length > 0);
  const isLockedByOther = $derived(
    currentLock && currentLock.user_id !== $auth?.userId
  );
  const isLockedByMe = $derived(
    currentLock && currentLock.user_id === $auth?.userId
  );

  // Format timestamp
  function formatTime(timestamp) {
    if (!timestamp) return "";
    const date = new Date(timestamp);
    return date.toLocaleTimeString();
  }

  // Get activity color
  function getActivityColor(activityType) {
    switch (activityType) {
      case "editing":
        return "badge-error";
      case "viewing":
        return "badge-info";
      case "idle":
        return "badge-ghost";
      default:
        return "badge-ghost";
    }
  }
</script>

{#if filePath}
  <div class="card bg-base-200 shadow-sm">
    <div class="card-body p-4">
      <h3 class="card-title text-base mb-3 flex items-center gap-2">
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
            d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
          />
        </svg>
        Collaboration
      </h3>

      <!-- File Lock Status -->
      <div class="mb-3">
        {#if isLockedByMe}
          <div class="alert alert-success py-2">
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
                d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
              />
            </svg>
            <span class="text-sm">You have locked this file for editing</span>
            <button
              class="btn btn-sm btn-ghost"
              onclick={releaseLock}
              disabled={loading}
            >
              Release
            </button>
          </div>
        {:else if isLockedByOther}
          <div class="alert alert-warning py-2">
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
                d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
              />
            </svg>
            <span class="text-sm"
              >File locked by {currentLock.username || "another user"}</span
            >
          </div>
        {:else}
          <button
            class="btn btn-sm btn-outline w-full gap-2"
            onclick={acquireLock}
            disabled={loading}
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
                d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
              />
            </svg>
            Lock for Editing
          </button>
        {/if}
      </div>

      <!-- Active Users -->
      {#if hasOtherUsers}
        <div class="space-y-2">
          <div class="text-sm font-medium text-base-content/70">
            Active Users ({presenceList.length})
          </div>

          {#each presenceList as presence (presence.user_id)}
            <div class="flex items-center gap-2 text-sm">
              <div class="avatar placeholder">
                <div class="bg-primary text-primary-content rounded-full w-8">
                  <span class="text-xs">{presence.username?.[0] || "U"}</span>
                </div>
              </div>

              <div class="flex-1 min-w-0">
                <div class="font-medium truncate">
                  {presence.username || "User"}
                </div>
                <div class="text-xs text-base-content/60">
                  {formatTime(presence.last_seen)}
                </div>
              </div>

              <span
                class="badge badge-sm {getActivityColor(
                  presence.activity_type
                )}"
              >
                {presence.activity_type || "viewing"}
              </span>
            </div>
          {/each}
        </div>
      {:else}
        <div class="text-sm text-base-content/60 text-center py-2">
          No other users active
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .alert {
    animation: slideIn 0.3s ease-out;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateX(-10px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }
</style>
