<script>
  import { onMount } from "svelte";
  import api from "../../lib/api.js";
  import { t } from "../../i18n.js";

  // State
  let groups = $state([]);
  let selectedGroup = $state(null);
  let groupMembers = $state([]);
  let users = $state([]);
  let loading = $state(true);
  let error = $state("");
  let successMessage = $state("");
  let searchQuery = $state("");

  // Modal states
  let showCreateModal = $state(false);
  let showAddMemberModal = $state(false);
  let showDeleteModal = $state(false);
  let showSuspendModal = $state(false);

  // Form states
  let newGroupName = $state("");
  let newGroupDescription = $state("");
  let selectedUserId = $state("");
  let suspendReason = $state("");
  let suspendDuration = $state(7);
  let suspendUserId = $state("");
  let suspendUserName = $state("");
  let saving = $state(false);

  // Filtered groups
  let filteredGroups = $derived(
    groups.filter(
      (g) =>
        g.name?.toLowerCase().includes(searchQuery.toLowerCase()) ||
        g.description?.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  // Available users to add (not already in group)
  let availableUsers = $derived(
    users.filter((u) => !groupMembers.some((m) => m.user_id === u.id))
  );

  // Load groups
  async function loadGroups() {
    loading = true;
    error = "";
    try {
      groups = await api.groups.list();
    } catch (e) {
      error = e.message || $t("groups.loadError");
    } finally {
      loading = false;
    }
  }

  // Load users
  async function loadUsers() {
    try {
      const response = await api.users.getAll();
      users = Array.isArray(response) ? response : response?.users || [];
    } catch (e) {
      console.error("Failed to load users:", e);
    }
  }

  // Select a group
  async function selectGroup(group) {
    selectedGroup = group;
    try {
      const details = await api.groups.get(group.id);
      groupMembers = details.members || [];
    } catch (e) {
      error = e.message || $t("groups.loadError");
      groupMembers = [];
    }
  }

  // Create group
  async function createGroup() {
    if (!newGroupName.trim()) return;

    saving = true;
    error = "";
    try {
      await api.groups.create(newGroupName, newGroupDescription || null);
      successMessage = $t("groups.createSuccess");
      setTimeout(() => (successMessage = ""), 3000);
      closeCreateModal();
      await loadGroups();
    } catch (e) {
      error = e.message || $t("groups.createError");
    } finally {
      saving = false;
    }
  }

  // Delete group
  async function deleteGroup() {
    if (!selectedGroup) return;

    saving = true;
    error = "";
    try {
      await api.groups.delete(selectedGroup.id);
      successMessage = $t("groups.deleteSuccess");
      setTimeout(() => (successMessage = ""), 3000);
      showDeleteModal = false;
      selectedGroup = null;
      groupMembers = [];
      await loadGroups();
    } catch (e) {
      error = e.message || $t("groups.deleteError");
    } finally {
      saving = false;
    }
  }

  // Add member
  async function addMember() {
    if (!selectedGroup || !selectedUserId) return;

    saving = true;
    error = "";
    try {
      await api.groups.addMember(selectedGroup.id, selectedUserId);
      successMessage = $t("groups.memberAdded");
      setTimeout(() => (successMessage = ""), 3000);
      closeAddMemberModal();
      await selectGroup(selectedGroup);
    } catch (e) {
      error = e.message || $t("groups.addMemberError");
    } finally {
      saving = false;
    }
  }

  // Remove member
  async function removeMember(userId) {
    if (!selectedGroup) return;

    try {
      await api.groups.removeMember(selectedGroup.id, userId);
      successMessage = $t("groups.memberRemoved");
      setTimeout(() => (successMessage = ""), 3000);
      await selectGroup(selectedGroup);
    } catch (e) {
      error = e.message || $t("groups.removeMemberError");
    }
  }

  // Open suspend modal
  function openSuspendModal(userId, username) {
    suspendUserId = userId;
    suspendUserName = username;
    suspendReason = "";
    suspendDuration = 7;
    showSuspendModal = true;
  }

  // Suspend user
  async function suspendUser() {
    if (!suspendUserId || !suspendReason.trim()) return;

    saving = true;
    error = "";
    try {
      await api.groups.suspendUser(
        suspendUserId,
        suspendReason,
        suspendDuration
      );
      successMessage = $t("groups.userSuspended");
      setTimeout(() => (successMessage = ""), 3000);
      showSuspendModal = false;
      if (selectedGroup) await selectGroup(selectedGroup);
    } catch (e) {
      error = e.message || $t("groups.suspendError");
    } finally {
      saving = false;
    }
  }

  // Unsuspend user
  async function unsuspendUser(userId) {
    try {
      await api.groups.unsuspendUser(userId);
      successMessage = $t("groups.userUnsuspended");
      setTimeout(() => (successMessage = ""), 3000);
      if (selectedGroup) await selectGroup(selectedGroup);
    } catch (e) {
      error = e.message || $t("groups.unsuspendError");
    }
  }

  // Modal helpers
  function closeCreateModal() {
    showCreateModal = false;
    newGroupName = "";
    newGroupDescription = "";
  }

  function closeAddMemberModal() {
    showAddMemberModal = false;
    selectedUserId = "";
  }

  // Format date
  function formatDate(dateString) {
    if (!dateString) return "-";
    return new Date(dateString).toLocaleDateString();
  }

  onMount(() => {
    loadGroups();
    loadUsers();
  });
</script>

<div class="p-6 max-w-7xl mx-auto">
  <!-- Header -->
  <div class="flex items-center justify-between mb-6">
    <div class="flex items-center gap-3">
      <i class="bi bi-people text-2xl text-primary"></i>
      <div>
        <h1 class="text-2xl font-bold">{$t("groups.title")}</h1>
        <p class="text-sm text-base-content/60">{$t("groups.subtitle")}</p>
      </div>
    </div>
    <div class="flex gap-2">
      <button
        class="btn btn-ghost btn-sm"
        onclick={loadGroups}
        disabled={loading}
      >
        <i class="bi bi-arrow-clockwise" class:animate-spin={loading}></i>
        {$t("common.refresh")}
      </button>
      <button
        class="btn btn-primary btn-sm"
        onclick={() => (showCreateModal = true)}
      >
        <i class="bi bi-plus-lg"></i>
        {$t("groups.createGroup")}
      </button>
    </div>
  </div>

  <!-- Messages -->
  {#if error}
    <div class="alert alert-error mb-4">
      <i class="bi bi-exclamation-triangle"></i>
      <span>{error}</span>
      <button
        class="btn btn-ghost btn-sm"
        onclick={() => (error = "")}
        aria-label="Close"
      >
        <i class="bi bi-x"></i>
      </button>
    </div>
  {/if}

  {#if successMessage}
    <div class="alert alert-success mb-4">
      <i class="bi bi-check-circle"></i>
      <span>{successMessage}</span>
    </div>
  {/if}

  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <!-- Groups List -->
    <div class="lg:col-span-1">
      <div class="card bg-base-200 shadow-lg">
        <div class="card-body">
          <h2 class="card-title text-lg mb-4">
            <i class="bi bi-folder2 text-primary"></i>
            {$t("groups.allGroups")}
            <span class="badge badge-ghost">{filteredGroups.length}</span>
          </h2>

          <!-- Search -->
          <div class="form-control mb-4">
            <input
              type="text"
              class="input input-bordered input-sm"
              placeholder={$t("groups.searchPlaceholder")}
              bind:value={searchQuery}
            />
          </div>

          {#if loading}
            <div class="flex justify-center py-8">
              <span class="loading loading-spinner loading-md"></span>
            </div>
          {:else if filteredGroups.length === 0}
            <div class="text-center py-8 text-base-content/60">
              <i class="bi bi-people text-4xl mb-2"></i>
              <p>{$t("groups.noGroups")}</p>
            </div>
          {:else}
            <ul class="menu menu-compact bg-base-100 rounded-lg">
              {#each filteredGroups as group}
                <li>
                  <button
                    class="justify-between"
                    class:active={selectedGroup?.id === group.id}
                    onclick={() => selectGroup(group)}
                  >
                    <div class="flex items-center gap-2">
                      <i class="bi bi-people-fill text-primary"></i>
                      <span class="font-medium">{group.name}</span>
                    </div>
                    <i class="bi bi-chevron-right opacity-50"></i>
                  </button>
                </li>
              {/each}
            </ul>
          {/if}
        </div>
      </div>
    </div>

    <!-- Group Details -->
    <div class="lg:col-span-2">
      {#if selectedGroup}
        <div class="card bg-base-200 shadow-lg">
          <div class="card-body">
            <!-- Group Header -->
            <div class="flex items-start justify-between mb-4">
              <div>
                <h2 class="card-title text-xl">
                  <i class="bi bi-people-fill text-primary"></i>
                  {selectedGroup.name}
                </h2>
                {#if selectedGroup.description}
                  <p class="text-sm text-base-content/60 mt-1">
                    {selectedGroup.description}
                  </p>
                {/if}
                <p class="text-xs text-base-content/40 mt-2">
                  {$t("groups.createdAt")}: {formatDate(
                    selectedGroup.created_at
                  )}
                </p>
              </div>
              <div class="flex gap-2">
                <button
                  class="btn btn-error btn-sm"
                  onclick={() => (showDeleteModal = true)}
                >
                  <i class="bi bi-trash"></i>
                  {$t("common.delete")}
                </button>
              </div>
            </div>

            <div class="divider"></div>

            <!-- Members Section -->
            <div class="flex items-center justify-between mb-4">
              <h3 class="font-medium flex items-center gap-2">
                <i class="bi bi-person-fill text-secondary"></i>
                {$t("groups.members")}
                <span class="badge badge-ghost">{groupMembers.length}</span>
              </h3>
              <button
                class="btn btn-primary btn-sm"
                onclick={() => (showAddMemberModal = true)}
              >
                <i class="bi bi-person-plus"></i>
                {$t("groups.addMember")}
              </button>
            </div>

            {#if groupMembers.length === 0}
              <div
                class="text-center py-8 text-base-content/60 bg-base-100 rounded-lg"
              >
                <i class="bi bi-person-x text-4xl mb-2"></i>
                <p>{$t("groups.noMembers")}</p>
              </div>
            {:else}
              <div class="overflow-x-auto">
                <table class="table table-zebra">
                  <thead>
                    <tr>
                      <th>{$t("groups.user")}</th>
                      <th>{$t("groups.email")}</th>
                      <th>{$t("groups.addedAt")}</th>
                      <th class="text-right">{$t("common.actions")}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {#each groupMembers as member}
                      <tr>
                        <td>
                          <div class="flex items-center gap-3">
                            <div class="avatar placeholder">
                              <div
                                class="bg-neutral text-neutral-content rounded-full w-8"
                              >
                                <span class="text-sm"
                                  >{(member.username || "U")
                                    .charAt(0)
                                    .toUpperCase()}</span
                                >
                              </div>
                            </div>
                            <span class="font-medium">{member.username}</span>
                          </div>
                        </td>
                        <td>{member.email || "-"}</td>
                        <td>{formatDate(member.added_at)}</td>
                        <td class="text-right">
                          <div class="flex justify-end gap-1">
                            <button
                              class="btn btn-ghost btn-xs"
                              title={$t("groups.suspend")}
                              onclick={() =>
                                openSuspendModal(
                                  member.user_id,
                                  member.username
                                )}
                            >
                              <i class="bi bi-pause-circle"></i>
                            </button>
                            <button
                              class="btn btn-ghost btn-xs text-error"
                              title={$t("groups.removeMember")}
                              onclick={() => removeMember(member.user_id)}
                            >
                              <i class="bi bi-person-dash"></i>
                            </button>
                          </div>
                        </td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            {/if}
          </div>
        </div>
      {:else}
        <div class="card bg-base-200 shadow-lg">
          <div class="card-body text-center py-16">
            <i class="bi bi-arrow-left text-4xl text-base-content/30 mb-4"></i>
            <p class="text-base-content/60">{$t("groups.selectGroup")}</p>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>

<!-- Create Group Modal -->
{#if showCreateModal}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg flex items-center gap-2">
        <i class="bi bi-plus-circle text-primary"></i>
        {$t("groups.createGroup")}
      </h3>

      <div class="py-4 space-y-4">
        <div class="form-control">
          <div class="label">
            <span class="label-text">{$t("groups.groupName")}</span>
          </div>
          <input
            type="text"
            class="input input-bordered"
            placeholder={$t("groups.groupNamePlaceholder")}
            bind:value={newGroupName}
            aria-label={$t("groups.groupName")}
          />
        </div>

        <div class="form-control">
          <div class="label">
            <span class="label-text">{$t("groups.description")}</span>
          </div>
          <textarea
            class="textarea textarea-bordered"
            placeholder={$t("groups.descriptionPlaceholder")}
            bind:value={newGroupDescription}
            aria-label={$t("groups.description")}
          ></textarea>
        </div>
      </div>

      <div class="modal-action">
        <button
          class="btn btn-ghost"
          onclick={closeCreateModal}
          disabled={saving}
        >
          {$t("common.cancel")}
        </button>
        <button
          class="btn btn-primary"
          onclick={createGroup}
          disabled={saving || !newGroupName.trim()}
        >
          {#if saving}
            <span class="loading loading-spinner loading-sm"></span>
          {:else}
            <i class="bi bi-check-lg"></i>
          {/if}
          {$t("common.create")}
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={closeCreateModal}
      onkeydown={(e) => e.key === "Escape" && closeCreateModal()}
    ></div>
  </div>
{/if}

<!-- Add Member Modal -->
{#if showAddMemberModal}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg flex items-center gap-2">
        <i class="bi bi-person-plus text-primary"></i>
        {$t("groups.addMember")}
      </h3>

      <div class="py-4">
        <div class="form-control">
          <div class="label">
            <span class="label-text">{$t("groups.selectUser")}</span>
          </div>
          <select
            class="select select-bordered w-full"
            bind:value={selectedUserId}
            aria-label={$t("groups.selectUser")}
          >
            <option value="">{$t("groups.selectUserPlaceholder")}</option>
            {#each availableUsers as user}
              <option value={user.id}
                >{user.username} {user.email ? `(${user.email})` : ""}</option
              >
            {/each}
          </select>
        </div>
      </div>

      <div class="modal-action">
        <button
          class="btn btn-ghost"
          onclick={closeAddMemberModal}
          disabled={saving}
        >
          {$t("common.cancel")}
        </button>
        <button
          class="btn btn-primary"
          onclick={addMember}
          disabled={saving || !selectedUserId}
        >
          {#if saving}
            <span class="loading loading-spinner loading-sm"></span>
          {:else}
            <i class="bi bi-check-lg"></i>
          {/if}
          {$t("groups.add")}
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={closeAddMemberModal}
      onkeydown={(e) => e.key === "Escape" && closeAddMemberModal()}
    ></div>
  </div>
{/if}

<!-- Delete Group Modal -->
{#if showDeleteModal}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg flex items-center gap-2 text-error">
        <i class="bi bi-exclamation-triangle"></i>
        {$t("groups.deleteGroup")}
      </h3>

      <p class="py-4">{$t("groups.deleteConfirm", selectedGroup?.name)}</p>

      <div class="modal-action">
        <button
          class="btn btn-ghost"
          onclick={() => (showDeleteModal = false)}
          disabled={saving}
        >
          {$t("common.cancel")}
        </button>
        <button class="btn btn-error" onclick={deleteGroup} disabled={saving}>
          {#if saving}
            <span class="loading loading-spinner loading-sm"></span>
          {:else}
            <i class="bi bi-trash"></i>
          {/if}
          {$t("common.delete")}
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={() => (showDeleteModal = false)}
      onkeydown={(e) => e.key === "Escape" && (showDeleteModal = false)}
    ></div>
  </div>
{/if}

<!-- Suspend User Modal -->
{#if showSuspendModal}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg flex items-center gap-2">
        <i class="bi bi-pause-circle text-warning"></i>
        {$t("groups.suspendUser")}
      </h3>

      <div class="py-4 space-y-4">
        <p class="text-base-content/70">
          {$t("groups.suspendDesc", suspendUserName)}
        </p>

        <div class="form-control">
          <div class="label">
            <span class="label-text">{$t("groups.reason")}</span>
          </div>
          <textarea
            class="textarea textarea-bordered"
            placeholder={$t("groups.reasonPlaceholder")}
            bind:value={suspendReason}
            aria-label={$t("groups.reason")}
          ></textarea>
        </div>

        <div class="form-control">
          <div class="label">
            <span class="label-text">{$t("groups.duration")}</span>
          </div>
          <div class="flex gap-2">
            <input
              type="number"
              class="input input-bordered flex-1"
              bind:value={suspendDuration}
              min="1"
              max="365"
              aria-label={$t("groups.duration")}
            />
            <span class="btn btn-ghost pointer-events-none"
              >{$t("groups.days")}</span
            >
          </div>
        </div>
      </div>

      <div class="modal-action">
        <button
          class="btn btn-ghost"
          onclick={() => (showSuspendModal = false)}
          disabled={saving}
        >
          {$t("common.cancel")}
        </button>
        <button
          class="btn btn-warning"
          onclick={suspendUser}
          disabled={saving || !suspendReason.trim()}
        >
          {#if saving}
            <span class="loading loading-spinner loading-sm"></span>
          {:else}
            <i class="bi bi-pause-circle"></i>
          {/if}
          {$t("groups.suspend")}
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={() => (showSuspendModal = false)}
      onkeydown={(e) => e.key === "Escape" && (showSuspendModal = false)}
    ></div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: -1;
  }
</style>
