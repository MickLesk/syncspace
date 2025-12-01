<script>
  import { onMount, onDestroy } from "svelte";
  import api from "../../lib/api.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { success, error as errorToast } from "../../stores/toast.js";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import { modals, modalEvents } from "../../stores/modals.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let roles = $state([]);
  let availablePermissions = $state([]);
  let loading = $state(true);

  let unsubscribe;

  onMount(async () => {
    await Promise.all([loadRoles(), loadAvailablePermissions()]);
    loading = false;

    // Listen for modal save events
    unsubscribe = modalEvents.on("roleSaved", async () => {
      await loadRoles();
      success(tr("rbac.roleSaved"));
    });
  });

  onDestroy(() => {
    if (unsubscribe) unsubscribe();
  });

  async function loadRoles() {
    try {
      roles = await api.rbac.listRoles(true);
    } catch (err) {
      console.error("Error loading roles:", err);
      errorToast(tr("rbac.loadRolesError"));
    }
  }

  async function loadAvailablePermissions() {
    try {
      availablePermissions = await api.rbac.listAvailablePermissions();
    } catch (err) {
      console.error("Error loading permissions:", err);
    }
  }

  function openCreateRole() {
    modals.openRoleEditor(null, availablePermissions);
  }

  function openEditRole(role) {
    modals.openRoleEditor(role, availablePermissions);
  }

  async function handleDeleteRole(role) {
    if (!confirm(tr("rbac.confirmDeleteRole", role.display_name))) return;

    try {
      await api.rbac.deleteRole(role.id);
      await loadRoles();
      success(tr("rbac.roleDeleted"));
    } catch (err) {
      console.error("Error deleting role:", err);
      errorToast(tr("rbac.deleteRoleError"));
    }
  }

  function openPermissionMatrix(role) {
    modals.openPermissionMatrix(role, availablePermissions);
  }

  function getRoleBadgeColor(role) {
    if (role.is_system) {
      return "badge-primary";
    }
    return "badge-secondary";
  }

  function getPermissionCount(role) {
    try {
      return JSON.parse(role.permissions).length;
    } catch {
      return 0;
    }
  }
</script>

<PageWrapper>
  <div class="min-h-screen bg-gradient-to-br from-base-100 to-base-200">
    <!-- Modern Header with Glass Effect -->
    <div
      class="mb-8 bg-base-100/50 backdrop-blur-xl rounded-2xl shadow-2xl border border-base-300 p-8"
    >
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="p-4 bg-gradient-to-br from-indigo-500 to-purple-500 rounded-xl shadow-lg"
          >
            <i class="bi bi-shield-check text-3xl text-white"></i>
          </div>
          <div>
            <h1
              class="text-4xl font-bold bg-gradient-to-r from-indigo-600 to-purple-600 bg-clip-text text-transparent"
            >
              {tr("rbac.title")}
            </h1>
            <p class="text-base-content/60 mt-1 text-lg">
              {tr("rbac.description")}
            </p>
          </div>
        </div>
        <button class="btn btn-primary gap-2" onclick={openCreateRole}>
          <i class="bi bi-plus-circle"></i>
          {tr("rbac.createRole")}
        </button>
      </div>
    </div>

    <!-- Roles Grid with Modern Cards -->
    {#if loading}
      <div class="flex justify-center items-center py-20">
        <span class="loading loading-spinner loading-lg text-purple-500"></span>
      </div>
    {:else if roles.length === 0}
      <div
        class="text-center py-20 bg-base-100/50 backdrop-blur-xl rounded-2xl border border-base-300"
      >
        <i class="bi bi-shield-x text-6xl text-base-content/30 mb-4"></i>
        <p class="text-lg text-base-content/60">{tr("rbac.noRoles")}</p>
      </div>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each roles as role}
          <div
            class="card bg-base-100 shadow-lg border border-gray-200 dark:border-gray-700"
          >
            <div class="card-body">
              <div class="flex justify-between items-start mb-3">
                <div class="flex-1">
                  <h2 class="card-title text-lg">
                    {role.display_name}
                    {#if role.is_system}
                      <span class="badge badge-sm badge-primary">System</span>
                    {/if}
                    {#if role.is_default}
                      <span class="badge badge-sm badge-success">Default</span>
                    {/if}
                  </h2>
                  <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
                    {role.name}
                  </p>
                </div>
              </div>

              {#if role.description}
                <p class="text-sm text-gray-700 dark:text-gray-300 mb-3">
                  {role.description}
                </p>
              {/if}

              <div class="flex items-center gap-2 mb-4">
                <i class="bi bi-shield-check text-primary"></i>
                <span class="text-sm">
                  {getPermissionCount(role)}
                  {tr("rbac.permissions")}
                </span>
              </div>

              <div class="card-actions justify-end">
                <button
                  onclick={() => openPermissionMatrix(role)}
                  class="btn btn-sm btn-ghost"
                >
                  <i class="bi bi-table mr-1"></i>
                  {tr("rbac.viewPermissions")}
                </button>
                {#if !role.is_system}
                  <button aria-label="Edit" onclick={() => openEditRole(role)} class="btn btn-sm btn-ghost"><i class="bi bi-pencil" aria-hidden="true"></i></button>
                  <button aria-label="Delete" onclick={() => handleDeleteRole(role)} class="btn btn-sm btn-ghost text-error"><i class="bi bi-trash" aria-hidden="true"></i></button>
                {/if}
              </div>
            </div>
          </div>
        {/each}
      </div>

      {#if roles.length === 0}
        <div class="text-center py-12">
          <i class="bi bi-shield-x text-4xl text-gray-400 mb-4"></i>
          <p class="text-gray-600 dark:text-gray-400">
            {tr("rbac.noRoles")}
          </p>
        </div>
      {/if}
    {/if}
  </div>
</PageWrapper>
