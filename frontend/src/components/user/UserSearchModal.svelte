<script>
  import { createEventDispatcher } from "svelte";
  import Modal from "../ui/Modal.svelte";
  import { currentLang } from "../../stores/ui";
  import { t } from "../../i18n.js";
  import api from "../../lib/api.js";
  import { error as errorToast } from "../../stores/toast.js";

  const dispatch = createEventDispatcher();
  const tr = (key, ...args) => t($currentLang, key, ...args);

  let {
    isOpen = $bindable(false),
    selectedUsers = [],
    onSelect = null,
  } = $props();

  let users = $state([]);
  let filteredUsers = $state([]);
  let loading = $state(false);
  let searchQuery = $state("");
  let roleFilter = $state("");
  let statusFilter = $state("active");

  // Load users when modal opens
  $effect(() => {
    if (isOpen && users.length === 0) {
      loadUsers();
    }
  });

  // Filter users based on search and filters
  $effect(() => {
    if (!searchQuery && !roleFilter) {
      filteredUsers = users;
    } else {
      filteredUsers = users.filter((user) => {
        const matchesSearch =
          !searchQuery ||
          user.username?.toLowerCase().includes(searchQuery.toLowerCase()) ||
          user.display_name
            ?.toLowerCase()
            .includes(searchQuery.toLowerCase()) ||
          user.email?.toLowerCase().includes(searchQuery.toLowerCase());

        const matchesRole = !roleFilter || user.role === roleFilter;

        return matchesSearch && matchesRole;
      });
    }
  });

  async function loadUsers() {
    loading = true;
    try {
      const response = await api.users.listAll(null, statusFilter);
      users = response.data || [];
      filteredUsers = users;
    } catch (err) {
      console.error("Failed to load users:", err);
      errorToast(tr("failedToLoadUsers"));
    } finally {
      loading = false;
    }
  }

  function isUserSelected(user) {
    return selectedUsers.some((u) => u.id === user.id);
  }

  function handleUserClick(user) {
    if (onSelect) {
      onSelect(user);
    } else {
      dispatch("select", user);
    }
  }

  function getAvatarInitials(user) {
    if (user.display_name) {
      return user.display_name
        .split(" ")
        .map((n) => n[0])
        .join("")
        .toUpperCase()
        .slice(0, 2);
    }
    return user.username.slice(0, 2).toUpperCase();
  }

  function getRoleBadgeColor(role) {
    switch (role) {
      case "admin":
        return "bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-200";
      case "moderator":
        return "bg-purple-100 dark:bg-purple-900 text-purple-700 dark:text-purple-200";
      default:
        return "bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300";
    }
  }
</script>

<Modal bind:isOpen title={tr("selectUsers")} size="large">
  <!-- Search and Filters -->
  <div class="mb-6 space-y-4">
    <!-- Search Input -->
    <div class="relative">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder={tr("searchUsers")}
        class="w-full px-4 py-2 pl-10 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
      />
      <i
        class="bi bi-search absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
      ></i>
    </div>

    <!-- Filters -->
    <div class="flex gap-3">
      <select
        bind:value={roleFilter}
        class="px-3 py-2 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg"
      >
        <option value="">{tr("allRoles")}</option>
        <option value="user">{tr("user")}</option>
        <option value="moderator">{tr("moderator")}</option>
        <option value="admin">{tr("admin")}</option>
      </select>

      <select
        bind:value={statusFilter}
        onchange={loadUsers}
        class="px-3 py-2 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg"
      >
        <option value="active">{tr("activeUsers")}</option>
        <option value="inactive">{tr("inactiveUsers")}</option>
        <option value="">{tr("allUsers")}</option>
      </select>
    </div>
  </div>

  <!-- User List -->
  {#if loading}
    <div class="flex justify-center py-12">
      <div
        class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"
      ></div>
    </div>
  {:else if filteredUsers.length === 0}
    <div class="text-center py-12 text-gray-500 dark:text-gray-400">
      <i class="bi bi-people text-4xl mb-3"></i>
      <p>{tr("noUsersFound")}</p>
    </div>
  {:else}
    <div class="space-y-2 max-h-96 overflow-y-auto">
      {#each filteredUsers as user (user.id)}
        <button
          type="button"
          onclick={() => handleUserClick(user)}
          class="w-full flex items-center gap-3 p-3 rounded-lg border-2 transition-all hover:border-blue-500 hover:bg-blue-50 dark:hover:bg-blue-900/20"
          class:border-blue-500={isUserSelected(user)}
          class:bg-blue-50={isUserSelected(user)}
          class:border-gray-200={!isUserSelected(user)}
        >
          <!-- Avatar -->
          <div class="flex-shrink-0">
            {#if user.avatar_base64}
              <img
                src={`data:image/png;base64,${user.avatar_base64}`}
                alt={user.display_name || user.username}
                class="w-12 h-12 rounded-full object-cover"
              />
            {:else}
              <div
                class="w-12 h-12 rounded-full bg-gradient-to-br from-blue-400 to-purple-500 flex items-center justify-center text-white font-semibold"
              >
                {getAvatarInitials(user)}
              </div>
            {/if}
          </div>

          <!-- User Info -->
          <div class="flex-1 text-left">
            <div class="flex items-center gap-2">
              <span class="font-medium text-gray-900 dark:text-gray-100">
                {user.display_name || user.username}
              </span>
              {#if user.role !== "user"}
                <span
                  class="px-2 py-0.5 text-xs font-medium rounded-full {getRoleBadgeColor(
                    user.role
                  )}"
                >
                  {user.role}
                </span>
              {/if}
            </div>
            <div class="text-sm text-gray-500 dark:text-gray-400">
              @{user.username}
              {#if user.email}
                · {user.email}
              {/if}
            </div>
          </div>

          <!-- Selection Indicator -->
          {#if isUserSelected(user)}
            <div class="flex-shrink-0">
              <i class="bi bi-check-circle-fill text-blue-500 text-xl"></i>
            </div>
          {/if}
        </button>
      {/each}
    </div>
  {/if}

  <!-- Footer with selection count -->
  <div
    slot="footer"
    class="flex items-center justify-between pt-4 border-t border-gray-200 dark:border-gray-700"
  >
    <span class="text-sm text-gray-600 dark:text-gray-400">
      {selectedUsers.length}
      {selectedUsers.length === 1 ? tr("userSelected") : tr("usersSelected")}
    </span>
    <button
      type="button"
      onclick={() => (isOpen = false)}
      class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
    >
      {tr("done")}
    </button>
  </div>
</Modal>
