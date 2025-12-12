<script>
  import { onMount } from "svelte";
  import api from "../../../lib/api.js";
  import { currentLang } from "../../../stores/ui.js";
  import { t } from "../../../i18n.js";
  import { showToast } from "../../../stores/toast.js";
  import StandardGlassCard from "../../../components/ui/StandardGlassCard.svelte";
  import StandardButton from "../../../components/ui/StandardButton.svelte";
  import StandardModal from "../../../components/ui/StandardModal.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let roles = $state([]);
  let loading = $state(true);
  let showCreateModal = $state(false);
  let showEditModal = $state(false);
  let editingRole = $state(null);
  let isCreating = $state(false);
  let isSaving = $state(false);

  let newRole = $state({
    name: "",
    description: "",
    permissions: [],
  });

  const availablePermissions = [
    {
      id: "admin",
      label: "Administrator",
      description: "Vollzugriff auf das System",
    },
    {
      id: "user_management",
      label: "Benutzerverwaltung",
      description: "Benutzer erstellen und verwalten",
    },
    {
      id: "file_management",
      label: "Dateiverwaltung",
      description: "Dateien hochladen und verwalten",
    },
    {
      id: "sharing",
      label: "Teilen",
      description: "Dateien und Ordner teilen",
    },
    {
      id: "backup",
      label: "Backup",
      description: "Backups erstellen und wiederherstellen",
    },
    {
      id: "settings",
      label: "Einstellungen",
      description: "Systemeinstellungen ändern",
    },
  ];

  onMount(async () => {
    await loadRoles();
  });

  async function loadRoles() {
    loading = true;
    try {
      // Mock data for now - replace with actual API call
      roles = [
        {
          id: 1,
          name: "Administrator",
          description: "Vollzugriff auf alle Funktionen",
          permissions: ["admin"],
          user_count: 2,
          is_system: true,
        },
        {
          id: 2,
          name: "Benutzer",
          description: "Standard-Benutzerrechte",
          permissions: ["file_management", "sharing"],
          user_count: 15,
          is_system: true,
        },
        {
          id: 3,
          name: "Moderator",
          description: "Erweiterte Benutzerrechte",
          permissions: [
            "user_management",
            "file_management",
            "sharing",
            "backup",
          ],
          user_count: 3,
          is_system: false,
        },
      ];
    } catch (err) {
      console.error("Error loading roles:", err);
      showToast("Fehler beim Laden der Rollen", "error");
    }
    loading = false;
  }

  function openCreateModal() {
    newRole = {
      name: "",
      description: "",
      permissions: [],
    };
    showCreateModal = true;
  }

  function openEditModal(role) {
    editingRole = { ...role };
    showEditModal = true;
  }

  async function createRole() {
    if (!newRole.name.trim()) {
      showToast("Rollenname ist erforderlich", "error");
      return;
    }

    isCreating = true;
    try {
      // Mock creation - replace with actual API call
      const role = {
        id: roles.length + 1,
        ...newRole,
        user_count: 0,
        is_system: false,
      };
      roles = [...roles, role];
      showToast("Rolle erfolgreich erstellt", "success");
      showCreateModal = false;
    } catch (err) {
      console.error("Error creating role:", err);
      showToast("Fehler beim Erstellen der Rolle", "error");
    }
    isCreating = false;
  }

  async function saveRole() {
    if (!editingRole.name.trim()) {
      showToast("Rollenname ist erforderlich", "error");
      return;
    }

    isSaving = true;
    try {
      // Mock save - replace with actual API call
      const index = roles.findIndex((r) => r.id === editingRole.id);
      if (index !== -1) {
        roles[index] = { ...editingRole };
        roles = [...roles];
      }
      showToast("Rolle erfolgreich gespeichert", "success");
      showEditModal = false;
    } catch (err) {
      console.error("Error saving role:", err);
      showToast("Fehler beim Speichern der Rolle", "error");
    }
    isSaving = false;
  }

  async function deleteRole(role) {
    if (role.is_system) {
      showToast("System-Rollen können nicht gelöscht werden", "error");
      return;
    }

    if (
      !confirm(
        `Sind Sie sicher, dass Sie die Rolle "${role.name}" löschen möchten?`
      )
    ) {
      return;
    }

    try {
      // Mock deletion - replace with actual API call
      roles = roles.filter((r) => r.id !== role.id);
      showToast("Rolle erfolgreich gelöscht", "success");
    } catch (err) {
      console.error("Error deleting role:", err);
      showToast("Fehler beim Löschen der Rolle", "error");
    }
  }

  function togglePermission(permissionId, roleObj) {
    if (roleObj.permissions.includes(permissionId)) {
      roleObj.permissions = roleObj.permissions.filter(
        (p) => p !== permissionId
      );
    } else {
      roleObj.permissions = [...roleObj.permissions, permissionId];
    }
  }

  function getPermissionLabel(permissionId) {
    const permission = availablePermissions.find((p) => p.id === permissionId);
    return permission ? permission.label : permissionId;
  }

  function openEditRole(role) {
    openEditModal(role);
  }
</script>

<!-- Modern Header -->
<div class="mb-8 flex items-center justify-between">
  <div>
    <h1 class="text-3xl font-bold text-white mb-2">
      <i class="bi bi-person-badge-fill mr-3"></i>
      Rollen & Rechte
    </h1>
    <p class="text-white/80">
      Verwalten Sie Benutzerrollen und deren Berechtigungen
    </p>
  </div>
  <StandardButton variant="primary" onclick={openCreateModal}>
    <i class="bi bi-plus-lg mr-2"></i>
    Neue Rolle
  </StandardButton>
</div>

{#if loading}
  <StandardGlassCard>
    <div class="flex items-center justify-center py-12">
      <div class="loading loading-spinner loading-lg"></div>
      <span class="ml-4 text-white/80">Lade Rollen...</span>
    </div>
  </StandardGlassCard>
{:else}
  <!-- Roles Grid -->
  <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
    {#each roles as role (role.id)}
      <StandardGlassCard>
        <div class="p-6">
          <!-- Role Header -->
          <div class="flex items-start justify-between mb-4">
            <div class="flex-1">
              <h3 class="text-xl font-semibold text-white mb-1">{role.name}</h3>
              <p class="text-white/70 text-sm">{role.description}</p>
            </div>
            {#if role.is_system}
              <span class="badge badge-primary badge-sm">System</span>
            {:else}
              <span class="badge badge-secondary badge-sm"
                >Benutzerdefiniert</span
              >
            {/if}
          </div>

          <!-- User Count -->
          <div class="flex items-center text-white/60 text-sm mb-4">
            <i class="bi bi-people-fill mr-2"></i>
            {role.user_count} Benutzer
          </div>

          <!-- Permissions -->
          <div class="mb-4">
            <h4 class="text-white/80 font-medium mb-2">Berechtigungen:</h4>
            <div class="flex flex-wrap gap-2">
              {#each role.permissions as permissionId}
                <span class="badge badge-outline badge-sm">
                  {getPermissionLabel(permissionId)}
                </span>
              {/each}
              {#if role.permissions.length === 0}
                <span class="text-white/40 text-sm">Keine Berechtigungen</span>
              {/if}
            </div>
          </div>

          <!-- Actions -->
          <div class="flex gap-2 pt-4 border-t border-white/10">
            <StandardButton
              variant="ghost"
              size="sm"
              onclick={() => openEditModal(role)}
              class="flex-1"
            >
              <i class="bi bi-pencil mr-1"></i>
              Bearbeiten
            </StandardButton>
            {#if !role.is_system}
              <StandardButton
                variant="outline-error"
                size="sm"
                onclick={() => deleteRole(role)}
              >
                <i class="bi bi-trash"></i>
              </StandardButton>
            {/if}
          </div>
        </div>
      </StandardGlassCard>
    {/each}
  </div>
{/if}

<!-- Create Role Modal -->
<StandardModal bind:showModal={showCreateModal} title="Neue Rolle erstellen">
  <div class="space-y-4">
    <div>
      <label class="block text-sm font-medium text-white mb-2">Rollenname</label
      >
      <input
        type="text"
        bind:value={newRole.name}
        class="input input-bordered w-full bg-white/5 border-white/20 text-white"
        placeholder="z.B. Editor, Reviewer..."
      />
    </div>

    <div>
      <label class="block text-sm font-medium text-white mb-2"
        >Beschreibung</label
      >
      <textarea
        bind:value={newRole.description}
        class="textarea textarea-bordered w-full bg-white/5 border-white/20 text-white"
        rows="3"
        placeholder="Beschreibung der Rolle..."
      ></textarea>
    </div>

    <div>
      <label class="block text-sm font-medium text-white mb-3"
        >Berechtigungen</label
      >
      <div class="space-y-2">
        {#each availablePermissions as permission}
          <label
            class="flex items-center p-3 rounded-lg bg-white/5 border border-white/10 hover:bg-white/10 cursor-pointer"
          >
            <input
              type="checkbox"
              class="checkbox checkbox-primary mr-3"
              checked={newRole.permissions.includes(permission.id)}
              onchange={() => togglePermission(permission.id, newRole)}
            />
            <div class="flex-1">
              <div class="text-white font-medium">{permission.label}</div>
              <div class="text-white/60 text-sm">{permission.description}</div>
            </div>
          </label>
        {/each}
      </div>
    </div>
  </div>

  <svelte:fragment slot="actions">
    <StandardButton variant="ghost" onclick={() => (showCreateModal = false)}>
      Abbrechen
    </StandardButton>
    <StandardButton
      variant="primary"
      onclick={createRole}
      disabled={isCreating}
      class={isCreating ? "loading" : ""}
    >
      {isCreating ? "Erstelle..." : "Rolle erstellen"}
    </StandardButton>
  </svelte:fragment>
</StandardModal>

<!-- Edit Role Modal -->
{#if editingRole}
  <StandardModal bind:showModal={showEditModal} title="Rolle bearbeiten">
    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-white mb-2"
          >Rollenname</label
        >
        <input
          type="text"
          bind:value={editingRole.name}
          class="input input-bordered w-full bg-white/5 border-white/20 text-white"
          disabled={editingRole.is_system}
        />
        {#if editingRole.is_system}
          <p class="text-yellow-400 text-sm mt-1">
            System-Rollen können nicht umbenannt werden
          </p>
        {/if}
      </div>

      <div>
        <label class="block text-sm font-medium text-white mb-2"
          >Beschreibung</label
        >
        <textarea
          bind:value={editingRole.description}
          class="textarea textarea-bordered w-full bg-white/5 border-white/20 text-white"
          rows="3"
          disabled={editingRole.is_system}
        ></textarea>
      </div>

      <div>
        <label class="block text-sm font-medium text-white mb-3"
          >Berechtigungen</label
        >
        <div class="space-y-2">
          {#each availablePermissions as permission}
            <label
              class="flex items-center p-3 rounded-lg bg-white/5 border border-white/10 hover:bg-white/10 cursor-pointer"
            >
              <input
                type="checkbox"
                class="checkbox checkbox-primary mr-3"
                checked={editingRole.permissions.includes(permission.id)}
                disabled={editingRole.is_system}
                onchange={() => togglePermission(permission.id, editingRole)}
              />
              <div class="flex-1">
                <div class="text-white font-medium">{permission.label}</div>
                <div class="text-white/60 text-sm">
                  {permission.description}
                </div>
              </div>
            </label>
          {/each}
        </div>
        {#if editingRole.is_system}
          <p class="text-yellow-400 text-sm mt-2">
            System-Rollen-Berechtigungen können nicht geändert werden
          </p>
        {/if}
      </div>
    </div>

    <svelte:fragment slot="actions">
      <StandardButton variant="ghost" onclick={() => (showEditModal = false)}>
        Abbrechen
      </StandardButton>
      <StandardButton
        variant="primary"
        onclick={saveRole}
        disabled={isSaving || editingRole.is_system}
        class={isSaving ? "loading" : ""}
      >
        {isSaving ? "Speichere..." : "Speichern"}
      </StandardButton>
    </svelte:fragment>
  </StandardModal>
{/if}
