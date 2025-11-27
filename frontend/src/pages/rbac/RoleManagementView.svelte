<script>
  import { onMount } from "svelte";
  import api from "../../lib/api.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { success, error as errorToast } from "../../stores/toast.js";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import RoleEditor from "../../components/rbac/RoleEditor.svelte";
  import PermissionMatrix from "../../components/rbac/PermissionMatrix.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let roles = $state([]);
  let availablePermissions = $state([]);
  let loading = $state(true);
  let showRoleEditor = $state(false);
  let editingRole = $state(null);
  let showPermissionMatrix = $state(false);
  let selectedRole = $state(null);

  onMount(async () => {
    await Promise.all([loadRoles(), loadAvailablePermissions()]);
    loading = false;
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
    editingRole = null;
    showRoleEditor = true;
  }

  function openEditRole(role) {
    editingRole = role;
    showRoleEditor = true;
  }

  async function handleRoleSaved() {
    showRoleEditor = false;
    await loadRoles();
    success(tr("rbac.roleSaved"));
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
    selectedRole = role;
    showPermissionMatrix = true;
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
  <div class="role-management-view">
    <div class="mb-6">
      <div class="flex justify-between items-center mb-4">
        <div>
          <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
            {tr("rbac.title")}
          </h1>
          <p class="text-gray-600 dark:text-gray-400 mt-2">
            {tr("rbac.description")}
          </p>
        </div>
        <button onclick={openCreateRole} class="btn btn-primary">
          <i class="bi bi-plus-circle mr-2"></i>
          {tr("rbac.createRole")}
        </button>
      </div>
    </div>

    {#if loading}
      <div class="flex justify-center py-12">
        <span class="loading loading-spinner loading-lg"></span>
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
                  <button
                    onclick={() => openEditRole(role)}
                    class="btn btn-sm btn-ghost"
                  >
                    <i class="bi bi-pencil"></i>
                  </button>
                  <button
                    onclick={() => handleDeleteRole(role)}
                    class="btn btn-sm btn-ghost text-error"
                  >
                    <i class="bi bi-trash"></i>
                  </button>
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

<!-- Role Editor Modal -->
{#if showRoleEditor}
  <RoleEditor
    role={editingRole}
    {availablePermissions}
    onSave={handleRoleSaved}
    onCancel={() => (showRoleEditor = false)}
  />
{/if}

<!-- Permission Matrix Modal -->
{#if showPermissionMatrix && selectedRole}
  <PermissionMatrix
    role={selectedRole}
    {availablePermissions}
    onClose={() => {
      showPermissionMatrix = false;
      selectedRole = null;
    }}
  />
{/if}

<style>
  .role-management-view {
    padding: 2rem;
    max-width: 1400px;
    margin: 0 auto;
  }
</style>
