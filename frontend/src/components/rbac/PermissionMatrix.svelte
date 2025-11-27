<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import Modal from "../ui/Modal.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let { role, availablePermissions = [], onClose } = $props();

  let permissions = $state([]);

  $effect(() => {
    try {
      permissions = JSON.parse(role.permissions || "[]");
    } catch {
      permissions = [];
    }
  });

  function hasPermission(permission) {
    return permissions.includes(permission);
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
  title="{tr('rbac.permissionMatrix')} - {role.display_name}"
  icon="table"
  size="xl"
  onclose={onClose}
>
  {#snippet children()}
    <div class="mb-4">
      <p class="text-sm text-gray-600 dark:text-gray-400">
        {role.description || tr("rbac.noDescription")}
      </p>
      <div class="flex gap-2 mt-2">
        {#if role.is_system}
          <span class="badge badge-primary badge-sm">System Role</span>
        {/if}
        {#if role.is_default}
          <span class="badge badge-success badge-sm">Default Role</span>
        {/if}
      </div>
    </div>

    <div class="overflow-x-auto">
      <table class="table table-zebra w-full">
        <thead>
          <tr>
            <th>{tr("rbac.permissionCategory")}</th>
            <th>{tr("rbac.permission")}</th>
            <th class="text-center">{tr("rbac.granted")}</th>
          </tr>
        </thead>
        <tbody>
          {#each Object.entries(permissionsByCategory()) as [category, categoryPermissions]}
            {#each categoryPermissions as permission, index}
              <tr>
                {#if index === 0}
                  <td
                    rowspan={categoryPermissions.length}
                    class="font-semibold capitalize"
                  >
                    {category}
                  </td>
                {/if}
                <td>
                  <code class="text-sm">{permission}</code>
                </td>
                <td class="text-center">
                  {#if hasPermission(permission)}
                    <i class="bi bi-check-circle-fill text-success text-lg"></i>
                  {:else}
                    <i class="bi bi-x-circle text-gray-400 text-lg"></i>
                  {/if}
                </td>
              </tr>
            {/each}
          {/each}
        </tbody>
      </table>
    </div>

    <div class="mt-4 p-4 bg-base-200 rounded-lg">
      <div class="flex items-center gap-2">
        <i class="bi bi-shield-check text-primary text-xl"></i>
        <span class="font-semibold">
          {permissions.length} / {availablePermissions.length}
          {tr("rbac.permissionsGranted")}
        </span>
      </div>
    </div>
  {/snippet}
  {#snippet actions()}
    <button onclick={onClose} class="btn">
      {tr("common.close")}
    </button>
  {/snippet}
</Modal>
