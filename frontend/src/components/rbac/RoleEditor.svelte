<script>
  import api from "../../lib/api.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import Modal from "../ui/Modal.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let { role = null, availablePermissions = [], onSave, onCancel } = $props();

  let formData = $state({
    name: "",
    display_name: "",
    description: "",
    permissions: [],
  });

  let loading = $state(false);
  let error = $state("");

  // Initialize form with existing role data
  $effect(() => {
    if (role) {
      formData = {
        name: role.name,
        display_name: role.display_name,
        description: role.description || "",
        permissions: JSON.parse(role.permissions || "[]"),
      };
    } else {
      formData = {
        name: "",
        display_name: "",
        description: "",
        permissions: [],
      };
    }
  });

  async function handleSubmit(e) {
    e.preventDefault();
    error = "";

    if (!formData.name.trim() || !formData.display_name.trim()) {
      error = tr("rbac.nameRequired");
      return;
    }

    if (formData.permissions.length === 0) {
      error = tr("rbac.permissionsRequired");
      return;
    }

    loading = true;

    try {
      if (role) {
        // Update existing role
        await api.rbac.updateRole(role.id, {
          display_name: formData.display_name,
          description: formData.description || null,
          permissions: formData.permissions,
        });
      } else {
        // Create new role
        await api.rbac.createRole(formData);
      }
      onSave();
    } catch (err) {
      console.error("Error saving role:", err);
      error = tr("rbac.saveError");
    } finally {
      loading = false;
    }
  }

  function togglePermission(permission) {
    if (formData.permissions.includes(permission)) {
      formData.permissions = formData.permissions.filter(
        (p) => p !== permission
      );
    } else {
      formData.permissions = [...formData.permissions, permission];
    }
  }

  function selectAllPermissions() {
    formData.permissions = [...availablePermissions];
  }

  function deselectAllPermissions() {
    formData.permissions = [];
  }

  function getPermissionCategory(permission) {
    return permission.split(".")[0];
  }

  const permissionsByCategory = $derived(() => {
    const categories = {};
    availablePermissions.forEach((perm) => {
      const category = getPermissionCategory(perm);
      if (!categories[category]) {
        categories[category] = [];
      }
      categories[category].push(perm);
    });
    return categories;
  });
</script>

<Modal
  visible={true}
  title={role ? tr("rbac.editRole") : tr("rbac.createRole")}
  icon="shield-check"
  size="xl"
  onclose={onCancel}
>
  {#snippet children()}
    {#if error}
      <div class="alert alert-error mb-4">
        <i class="bi bi-exclamation-triangle"></i>
        <span>{error}</span>
      </div>
    {/if}

    <form onsubmit={handleSubmit}>
      <div class="space-y-4">
        <!-- Role Name -->
        <div class="form-control">
          <label class="label" for="role-name">
            <span class="label-text">{tr("rbac.roleName")}</span>
          </label>
          <input
            id="role-name"
            type="text"
            bind:value={formData.name}
            class="input input-bordered"
            placeholder="e.g., editor, contributor"
            disabled={role !== null}
            required
          />
          <div class="label"><span class="label-text">{tr("rbac.roleNameHint")}</span></div>
        </div>

        <!-- Display Name -->
        <div class="form-control">
          <label class="label" for="role-display-name">
            <span class="label-text">{tr("rbac.displayName")}</span>
          </label>
          <input
            id="role-display-name"
            type="text"
            bind:value={formData.display_name}
            class="input input-bordered"
            placeholder="e.g., Content Editor"
            required
          />
        </div>

        <!-- Description -->
        <div class="form-control">
          <label class="label" for="role-description">
            <span class="label-text">{tr("rbac.description")}</span>
          </label>
          <textarea
            id="role-description"
            bind:value={formData.description}
            class="textarea textarea-bordered"
            rows="2"
            placeholder={tr("rbac.descriptionPlaceholder")}
          ></textarea>
        </div>

        <!-- Permissions -->
        <div class="form-control">
          <div class="flex justify-between items-center mb-2">
            <label class="label">
              <span class="label-text font-semibold"
                >{tr("rbac.permissions")}</span
              >
            </label>
            <div class="flex gap-2">
              <button
                type="button"
                onclick={selectAllPermissions}
                class="btn btn-xs btn-ghost"
              >
                {tr("rbac.selectAll")}
              </button>
              <button
                type="button"
                onclick={deselectAllPermissions}
                class="btn btn-xs btn-ghost"
              >
                {tr("rbac.deselectAll")}
              </button>
            </div>
          </div>

          <div
            class="border border-gray-300 dark:border-gray-600 rounded-lg p-4 max-h-96 overflow-y-auto"
          >
            {#each Object.entries(permissionsByCategory()) as [category, permissions]}
              <div class="mb-4">
                <h3
                  class="font-semibold text-sm text-gray-700 dark:text-gray-300 mb-2 capitalize"
                >
                  {category}
                  {tr("rbac.permissionsCategory")}
                </h3>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                  {#each permissions as permission}
                    <label
                      class="flex items-center gap-2 cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700 p-2 rounded"
                    >
                      <input
                        type="checkbox"
                        class="checkbox checkbox-sm"
                        checked={formData.permissions.includes(permission)}
                        onchange={() => togglePermission(permission)}
                      />
                      <span class="text-sm font-mono">{permission}</span>
                    </label>
                  {/each}
                </div>
              </div>
            {/each}
          </div>

          <div class="label"><span class="label-text">
              {formData.permissions.length}
              {tr("rbac.permissionsSelected")}
            </span></div>
        </div>
      </div>
    </form>
  {/snippet}
  {#snippet actions()}
    <button type="button" onclick={onCancel} class="btn" disabled={loading}>
      {tr("common.cancel")}
    </button>
    <button
      type="submit"
      class="btn btn-primary"
      disabled={loading}
      onclick={handleSubmit}
    >
      {#if loading}
        <span class="loading loading-spinner"></span>
      {/if}
      {role ? tr("common.save") : tr("common.create")}
    </button>
  {/snippet}
</Modal>
