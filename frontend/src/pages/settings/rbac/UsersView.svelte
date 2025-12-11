<script>
  import { onMount } from "svelte";
  import { showToast } from "../../stores/toast.js";
  import { currentLang } from "../../stores/ui";
  import { t } from "../../i18n.js";
  import api from "../../lib/api.js";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
  import EmptyState from "../../components/ui/EmptyState.svelte";
  import LoadingState from "../../components/ui/LoadingState.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let users = $state([]);
  let loading = $state(true);
  let viewMode = $state("table");
  let selectedUsers = $state(new Set());
  let filterRole = $state("all");

  onMount(async () => {
    try {
      // Use admin endpoint to get all users with full details
      const response = await api.users.getAll();
      if (!response) {
        throw new Error("User list API not available");
      }
      users = Array.isArray(response) ? response : response.users || [];
    } catch (error) {
      console.error("Failed to load users from API:", error);
      showToast(tr("users.error_loading"), "error");
      users = [];
    } finally {
      loading = false;
    }
  });

  function getRoleBadgeClass(role) {
    const badges = {
      admin: "badge-glass-error",
      moderator: "badge-glass-warning",
      user: "badge-glass-info",
    };
    return badges[role] || "badge-glass-info";
  }

  function getStatusIndicator(status) {
    return status === "online"
      ? { color: "bg-green-500", text: tr("online") }
      : { color: "bg-gray-400 dark:bg-gray-600", text: tr("offline") };
  }

  function getUserInitials(username) {
    return username.slice(0, 2).toUpperCase();
  }

  function formatBytes(bytes) {
    return `${(bytes / 1024 ** 3).toFixed(1)} GB`;
  }

  function formatLastActive(date) {
    if (!date) return tr("never") || "Nie";
    const diff = Date.now() - new Date(date).getTime();
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);
    if (minutes < 1) return tr("justNow");
    if (minutes < 60) return tr("minutesAgo", minutes);
    if (hours < 24) return tr("hoursAgo", hours);
    return new Date(date).toLocaleDateString();
  }

  function toggleSelectUser(userId) {
    selectedUsers.has(userId)
      ? selectedUsers.delete(userId)
      : selectedUsers.add(userId);
    selectedUsers = selectedUsers;
  }

  function toggleSelectAll() {
    selectedUsers =
      selectedUsers.size === filteredUsers.length
        ? new Set()
        : new Set(filteredUsers.map((u) => u.id));
  }

  let filteredUsers = $derived(
    filterRole === "all" ? users : users.filter((u) => u.role === filterRole)
  );
</script>

<PageWrapper gradient>
  <PageHeader
    title={tr("users")}
    subtitle="{filteredUsers.length} {filteredUsers.length !== 1
      ? tr('usersPlural')
      : tr('userSingular')}"
    icon="people-fill"
  >
    {#snippet actions()}
      <ModernButton variant="gradient">
        <i class="bi bi-plus-lg mr-2" aria-hidden="true"></i>
        {tr("addUser")}
      </ModernButton>
    {/snippet}
  </PageHeader>

  <!-- View Mode and Filters -->
  <ModernCard variant="glass" class="mb-6">
    <div class="p-4">
      <div class="flex items-center justify-between flex-wrap gap-4 mb-4">
        <!-- View Mode Toggle -->
        <div
          class="flex rounded-lg overflow-hidden border-2 border-gray-200 dark:border-gray-700"
        >
          <button
            onclick={() => (viewMode = "table")}
            class="px-3 py-2 text-sm transition-colors {viewMode === 'table'
              ? 'bg-primary-600 text-white'
              : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700'}"
            aria-label={tr("tableView")}
          >
            <i class="bi bi-table" aria-hidden="true"></i>
          </button>
          <button
            onclick={() => (viewMode = "cards")}
            class="px-3 py-2 text-sm border-l-2 border-gray-200 dark:border-gray-700 transition-colors {viewMode ===
            'cards'
              ? 'bg-primary-600 text-white'
              : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700'}"
            aria-label={tr("cardsView")}
          >
            <i class="bi bi-grid-3x3-gap" aria-hidden="true"></i>
          </button>
        </div>
      </div>

      <!-- Filters -->
      <div class="flex items-center justify-between flex-wrap gap-4">
        <div
          class="flex rounded-lg overflow-hidden border-2 border-gray-200 dark:border-gray-700"
        >
          <ModernButton
            variant={filterRole === "all" ? "primary" : "ghost"}
            onclick={() => (filterRole = "all")}
            class="rounded-none"
          >
            {tr("all")}
          </ModernButton>
          <ModernButton
            variant={filterRole === "admin" ? "primary" : "ghost"}
            onclick={() => (filterRole = "admin")}
            class="rounded-none border-l-2 border-gray-200 dark:border-gray-700"
          >
            {tr("admins")}
          </ModernButton>
          <ModernButton
            variant={filterRole === "user" ? "primary" : "ghost"}
            onclick={() => (filterRole = "user")}
            class="rounded-none border-l-2 border-gray-200 dark:border-gray-700"
          >
            {tr("users")}
          </ModernButton>
        </div>

        {#if selectedUsers.size > 0}
          <ModernCard variant="glass" class="px-4 py-2">
            <div class="flex items-center gap-4">
              <span
                class="text-sm font-medium text-gray-900 dark:text-gray-100"
              >
                {tr("selectedCount", selectedUsers.size)}
              </span>
              <ModernButton variant="danger" size="sm">
                <i class="bi bi-trash mr-1" aria-hidden="true"></i>
                {tr("delete")}
              </ModernButton>
            </div>
          </ModernCard>
        {/if}
      </div>
    </div>
  </ModernCard>

  {#if loading}
    <LoadingState variant="table" count={5} message={tr("loadingUsers")} />
  {:else if filteredUsers.length === 0}
    <EmptyState
      icon="👥"
      title={tr("noUsersFound")}
      description={filterRole === "all"
        ? tr("noUsersInSystem")
        : `No ${filterRole}s found`}
    />
  {:else if viewMode === "table"}
    <!-- Table View -->
    <ModernCard variant="glass" class="overflow-hidden">
      <div class="overflow-x-auto">
        <table class="w-full">
          <thead>
            <tr
              class="border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50"
            >
              <th class="px-6 py-4 text-left">
                <input
                  type="checkbox"
                  class="w-4 h-4 rounded border-gray-300 dark:border-gray-600 text-primary-600 focus:ring-primary-500"
                  onchange={toggleSelectAll}
                  checked={selectedUsers.size === filteredUsers.length}
                />
              </th>
              <th
                class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                >{tr("userTableUser")}</th
              >
              <th
                class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                >{tr("userTableRole")}</th
              >
              <th
                class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                >{tr("userTableStatus")}</th
              >
              <th
                class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                >{tr("userTableFiles")}</th
              >
              <th
                class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                >{tr("userTableStorage")}</th
              >
              <th
                class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                >{tr("userTableLastActive")}</th
              >
              <th
                class="px-6 py-4 text-left text-xs font-semibold uppercase tracking-wider text-gray-700 dark:text-gray-300"
                >{tr("userTableActions")}</th
              >
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
            {#each filteredUsers as user (user.id)}
              <tr
                class="hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors {selectedUsers.has(
                  user.id
                )
                  ? 'bg-primary-50 dark:bg-primary-900/20'
                  : ''}"
              >
                <td class="px-6 py-4">
                  <input
                    type="checkbox"
                    class="w-4 h-4 rounded border-gray-300 dark:border-gray-600 text-primary-600 focus:ring-primary-500"
                    checked={selectedUsers.has(user.id)}
                    onchange={() => toggleSelectUser(user.id)}
                  />
                </td>
                <td class="px-6 py-4">
                  <div class="flex items-center gap-3">
                    <div class="relative">
                      <div
                        class="w-10 h-10 rounded-full gradient-bg-primary flex items-center justify-center text-white font-semibold text-sm"
                      >
                        {getUserInitials(user.username)}
                      </div>
                      <div
                        class="absolute -bottom-0.5 -right-0.5 w-3.5 h-3.5 rounded-full border-2 border-white dark:border-gray-800 {getStatusIndicator(
                          user.status
                        ).color}"
                      ></div>
                    </div>
                    <div>
                      <div
                        class="font-semibold text-gray-900 dark:text-gray-100"
                      >
                        {user.username}
                      </div>
                      <div class="text-sm text-gray-500 dark:text-gray-400">
                        {user.email}
                      </div>
                    </div>
                  </div>
                </td>
                <td class="px-6 py-4">
                  <span class={getRoleBadgeClass(user.role)}>
                    {user.role}
                  </span>
                </td>
                <td class="px-6 py-4">
                  <span class="text-sm text-gray-700 dark:text-gray-300">
                    {getStatusIndicator(user.status).text}
                  </span>
                </td>
                <td class="px-6 py-4 text-gray-700 dark:text-gray-300">
                  {(user.filesCount || 0).toLocaleString()}</td
                >
                <td class="px-6 py-4 text-gray-700 dark:text-gray-300">
                  {formatBytes(user.storageUsed || 0)}</td
                >
                <td class="px-6 py-4 text-sm text-gray-500 dark:text-gray-400"
                  >{formatLastActive(user.lastActive)}</td
                >
                <td class="px-6 py-4">
                  <div class="flex gap-2">
                    <ModernButton
                      variant="ghost"
                      size="sm"
                      aria-label={tr("editUser")}
                    >
                      <i class="bi bi-pencil" aria-hidden="true"></i>
                    </ModernButton>
                    <ModernButton
                      variant="danger"
                      size="sm"
                      aria-label={tr("deleteUser")}
                    >
                      <i class="bi bi-trash" aria-hidden="true"></i>
                    </ModernButton>
                  </div>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </ModernCard>
  {:else}
    <!-- Cards View -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each filteredUsers as user (user.id)}
        <ModernCard
          variant="glass"
          hoverable
          class="relative {selectedUsers.has(user.id)
            ? 'ring-2 ring-primary-500'
            : ''}"
        >
          <div class="p-6">
            <!-- Selection Checkbox -->
            <div class="absolute top-4 right-4 flex items-center gap-2">
              <input
                type="checkbox"
                class="w-4 h-4 rounded border-gray-300 dark:border-gray-600 text-primary-600 focus:ring-primary-500"
                checked={selectedUsers.has(user.id)}
                onchange={() => toggleSelectUser(user.id)}
              />
              <div
                class="w-3 h-3 rounded-full {getStatusIndicator(user.status)
                  .color}"
              ></div>
            </div>

            <!-- Avatar -->
            <div class="flex flex-col items-center mb-4">
              <div
                class="w-20 h-20 rounded-full gradient-bg-primary flex items-center justify-center text-white font-bold text-2xl mb-3"
              >
                {getUserInitials(user.username)}
              </div>
              <h3 class="text-xl font-bold text-gray-900 dark:text-gray-100">
                {user.username}
              </h3>
              <p class="text-sm text-gray-500 dark:text-gray-400 mb-2">
                {user.email}
              </p>
              <span class={getRoleBadgeClass(user.role)}>
                {user.role}
              </span>
            </div>

            <!-- Stats -->
            <div
              class="grid grid-cols-2 gap-4 mb-4 pt-4 border-t border-gray-200 dark:border-gray-700"
            >
              <div class="text-center">
                <div
                  class="flex items-center justify-center gap-2 text-gray-500 dark:text-gray-400 mb-1"
                >
                  <i class="bi bi-files" aria-hidden="true"></i>
                  <span class="text-xs uppercase tracking-wider"
                    >{tr("userTableFiles")}</span
                  >
                </div>
                <div
                  class="text-lg font-semibold text-gray-900 dark:text-gray-100"
                >
                  {user.filesCount.toLocaleString()}
                </div>
              </div>
              <div class="text-center">
                <div
                  class="flex items-center justify-center gap-2 text-gray-500 dark:text-gray-400 mb-1"
                >
                  <i class="bi bi-hdd" aria-hidden="true"></i>
                  <span class="text-xs uppercase tracking-wider"
                    >{tr("userTableStorage")}</span
                  >
                </div>
                <div
                  class="text-lg font-semibold text-gray-900 dark:text-gray-100"
                >
                  {formatBytes(user.storageUsed)}
                </div>
              </div>
            </div>

            <!-- Footer -->
            <div
              class="flex items-center justify-between pt-4 border-t border-gray-200 dark:border-gray-700"
            >
              <span class="text-xs text-gray-500 dark:text-gray-400">
                {formatLastActive(user.lastActive)}
              </span>
              <div class="flex gap-2">
                <ModernButton
                  variant="ghost"
                  size="sm"
                  aria-label={tr("editUser")}
                >
                  <i class="bi bi-pencil" aria-hidden="true"></i>
                </ModernButton>
                <ModernButton
                  variant="danger"
                  size="sm"
                  aria-label={tr("deleteUser")}
                >
                  <i class="bi bi-trash" aria-hidden="true"></i>
                </ModernButton>
              </div>
            </div>
          </div>
        </ModernCard>
      {/each}
    </div>
  {/if}
</PageWrapper>
