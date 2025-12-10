<script>
  import PageWrapper from "../../components/PageWrapper.svelte";
  import PageHeader from "../../components/ui/PageHeader.svelte";
  import ModernCard from "../../components/ui/ModernCard.svelte";
  import ModernButton from "../../components/ui/ModernButton.svelte";
  import EmptyState from "../../components/ui/EmptyState.svelte";
  import LoadingState from "../../components/ui/LoadingState.svelte";
  import { success, error as errorToast } from "../../stores/toast";
  import api from "../../lib/api";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let loading = $state(true);
  let connections = $state([]);
  let showAddModal = $state(false);
  let editingConnection = $state(null);
  let testingId = $state(null);
  let syncingId = $state(null);

  // Form state
  let formData = $state({
    name: "",
    host: "",
    port: 21,
    username: "",
    password: "",
    use_ftps: false,
    passive_mode: true,
    remote_path: "/",
    local_path: "",
    sync_direction: "download",
    auto_sync: false,
    sync_interval_minutes: 60,
  });

  const syncDirections = [
    { value: "download", label: "Download (Remote → Local)" },
    { value: "upload", label: "Upload (Local → Remote)" },
    { value: "bidirectional", label: "Bidirectional" },
  ];

  $effect(() => {
    loadConnections();
  });

  async function loadConnections() {
    loading = true;
    try {
      connections = await api.ftp.list();
    } catch (e) {
      console.error("Failed to load FTP connections:", e);
      errorToast("Failed to load FTP connections");
    } finally {
      loading = false;
    }
  }

  function openAddModal() {
    editingConnection = null;
    formData = {
      name: "",
      host: "",
      port: 21,
      username: "",
      password: "",
      use_ftps: false,
      passive_mode: true,
      remote_path: "/",
      local_path: "",
      sync_direction: "download",
      auto_sync: false,
      sync_interval_minutes: 60,
    };
    showAddModal = true;
  }

  function openEditModal(conn) {
    editingConnection = conn;
    formData = {
      name: conn.name,
      host: conn.host,
      port: conn.port,
      username: conn.username,
      password: "", // Don't show password
      use_ftps: conn.use_ftps,
      passive_mode: conn.passive_mode,
      remote_path: conn.remote_path,
      local_path: conn.local_path,
      sync_direction: conn.sync_direction,
      auto_sync: conn.auto_sync,
      sync_interval_minutes: conn.sync_interval_minutes,
    };
    showAddModal = true;
  }

  function closeModal() {
    showAddModal = false;
    editingConnection = null;
  }

  async function saveConnection() {
    try {
      if (editingConnection) {
        await api.ftp.update(editingConnection.id, formData);
        success("FTP connection updated");
      } else {
        await api.ftp.create(formData);
        success("FTP connection created");
      }
      closeModal();
      loadConnections();
    } catch (e) {
      console.error("Failed to save FTP connection:", e);
      errorToast("Failed to save connection");
    }
  }

  async function deleteConnection(id) {
    if (!confirm("Are you sure you want to delete this connection?")) return;
    try {
      await api.ftp.delete(id);
      success("Connection deleted");
      loadConnections();
    } catch (e) {
      console.error("Failed to delete connection:", e);
      errorToast("Failed to delete connection");
    }
  }

  async function testConnection(id) {
    testingId = id;
    try {
      const result = await api.ftp.test(id);
      if (result.success) {
        success("Connection successful!");
      } else {
        errorToast(result.message || "Connection failed");
      }
    } catch (e) {
      console.error("Connection test failed:", e);
      errorToast("Connection test failed");
    } finally {
      testingId = null;
    }
  }

  async function triggerSync(id) {
    syncingId = id;
    try {
      const result = await api.ftp.sync(id);
      success(`Sync completed: ${result.files_synced || 0} files`);
      loadConnections();
    } catch (e) {
      console.error("Sync failed:", e);
      errorToast("Sync failed");
    } finally {
      syncingId = null;
    }
  }

  function formatDate(dateStr) {
    if (!dateStr) return "Never";
    return new Date(dateStr).toLocaleString();
  }
</script>

<PageWrapper>
  <PageHeader
    title="FTP Sync"
    subtitle="Manage FTP/FTPS connections for file synchronization"
    icon="bi-cloud-arrow-down"
  >
    <ModernButton variant="primary" onclick={openAddModal}>
      <i class="bi bi-plus-lg me-2"></i>
      Add Connection
    </ModernButton>
  </PageHeader>

  {#if loading}
    <LoadingState message="Loading FTP connections..." />
  {:else if connections.length === 0}
    <EmptyState
      icon="bi-cloud-slash"
      title="No FTP Connections"
      description="Add an FTP connection to start syncing files from remote servers."
    >
      <ModernButton variant="primary" onclick={openAddModal}>
        <i class="bi bi-plus-lg me-2"></i>
        Add Connection
      </ModernButton>
    </EmptyState>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
      {#each connections as conn}
        <ModernCard>
          <div class="p-4">
            <div class="flex items-start justify-between mb-3">
              <div class="flex items-center gap-3">
                <div class="p-2 rounded-lg bg-primary/10">
                  <i class="bi bi-hdd-network text-xl text-primary"></i>
                </div>
                <div>
                  <h3 class="font-semibold text-base-content">{conn.name}</h3>
                  <p class="text-sm text-base-content/60">
                    {conn.use_ftps ? "FTPS" : "FTP"}://{conn.host}:{conn.port}
                  </p>
                </div>
              </div>
              <div class="dropdown dropdown-end">
                <button tabindex="0" class="btn btn-ghost btn-sm btn-square" aria-label="More options">
                  <i class="bi bi-three-dots-vertical"></i>
                </button>
                <ul tabindex="0" class="dropdown-content z-10 menu p-2 shadow bg-base-100 rounded-box w-40">
                  <li><button onclick={() => openEditModal(conn)}>Edit</button></li>
                  <li><button onclick={() => deleteConnection(conn.id)} class="text-error">Delete</button></li>
                </ul>
              </div>
            </div>

            <div class="space-y-2 text-sm mb-4">
              <div class="flex justify-between">
                <span class="text-base-content/60">Remote Path:</span>
                <span class="font-mono text-xs">{conn.remote_path}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-base-content/60">Local Path:</span>
                <span class="font-mono text-xs">{conn.local_path}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-base-content/60">Direction:</span>
                <span class="badge badge-sm badge-outline">{conn.sync_direction}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-base-content/60">Auto Sync:</span>
                <span class="badge badge-sm" class:badge-success={conn.auto_sync} class:badge-ghost={!conn.auto_sync}>
                  {conn.auto_sync ? `Every ${conn.sync_interval_minutes}m` : "Manual"}
                </span>
              </div>
              <div class="flex justify-between">
                <span class="text-base-content/60">Last Sync:</span>
                <span class="text-xs">{formatDate(conn.last_sync_at)}</span>
              </div>
              {#if conn.last_sync_status}
                <div class="flex justify-between">
                  <span class="text-base-content/60">Status:</span>
                  <span class="badge badge-sm" class:badge-success={conn.last_sync_status === 'success'} class:badge-error={conn.last_sync_status === 'failed'}>
                    {conn.last_sync_status}
                  </span>
                </div>
              {/if}
            </div>

            <div class="flex gap-2">
              <ModernButton
                variant="outline"
                size="sm"
                onclick={() => testConnection(conn.id)}
                disabled={testingId === conn.id}
              >
                {#if testingId === conn.id}
                  <span class="loading loading-spinner loading-xs"></span>
                {:else}
                  <i class="bi bi-wifi me-1"></i>
                {/if}
                Test
              </ModernButton>
              <ModernButton
                variant="primary"
                size="sm"
                onclick={() => triggerSync(conn.id)}
                disabled={syncingId === conn.id}
              >
                {#if syncingId === conn.id}
                  <span class="loading loading-spinner loading-xs"></span>
                {:else}
                  <i class="bi bi-arrow-repeat me-1"></i>
                {/if}
                Sync Now
              </ModernButton>
            </div>
          </div>
        </ModernCard>
      {/each}
    </div>
  {/if}
</PageWrapper>

<!-- Add/Edit Modal -->
{#if showAddModal}
  <div class="modal modal-open">
    <div class="modal-box max-w-2xl">
      <h3 class="font-bold text-lg mb-4">
        {editingConnection ? "Edit FTP Connection" : "Add FTP Connection"}
      </h3>

      <form onsubmit={(e) => { e.preventDefault(); saveConnection(); }}>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="form-control">
            <label class="label" for="conn-name">
              <span class="label-text">Connection Name</span>
            </label>
            <input
              type="text"
              id="conn-name"
              class="input input-bordered"
              bind:value={formData.name}
              required
              placeholder="My FTP Server"
            />
          </div>

          <div class="form-control">
            <label class="label" for="conn-host">
              <span class="label-text">Host</span>
            </label>
            <input
              type="text"
              id="conn-host"
              class="input input-bordered"
              bind:value={formData.host}
              required
              placeholder="ftp.example.com"
            />
          </div>

          <div class="form-control">
            <label class="label" for="conn-port">
              <span class="label-text">Port</span>
            </label>
            <input
              type="number"
              id="conn-port"
              class="input input-bordered"
              bind:value={formData.port}
              min="1"
              max="65535"
            />
          </div>

          <div class="form-control">
            <label class="label" for="conn-username">
              <span class="label-text">Username</span>
            </label>
            <input
              type="text"
              id="conn-username"
              class="input input-bordered"
              bind:value={formData.username}
              required
            />
          </div>

          <div class="form-control md:col-span-2">
            <label class="label" for="conn-password">
              <span class="label-text">Password {editingConnection ? "(leave empty to keep current)" : ""}</span>
            </label>
            <input
              type="password"
              id="conn-password"
              class="input input-bordered"
              bind:value={formData.password}
              required={!editingConnection}
            />
          </div>

          <div class="form-control">
            <label class="label" for="conn-remote-path">
              <span class="label-text">Remote Path</span>
            </label>
            <input
              type="text"
              id="conn-remote-path"
              class="input input-bordered font-mono"
              bind:value={formData.remote_path}
              placeholder="/"
            />
          </div>

          <div class="form-control">
            <label class="label" for="conn-local-path">
              <span class="label-text">Local Path</span>
            </label>
            <input
              type="text"
              id="conn-local-path"
              class="input input-bordered font-mono"
              bind:value={formData.local_path}
              required
              placeholder="/ftp-sync/server1"
            />
          </div>

          <div class="form-control">
            <label class="label" for="conn-direction">
              <span class="label-text">Sync Direction</span>
            </label>
            <select id="conn-direction" class="select select-bordered" bind:value={formData.sync_direction}>
              {#each syncDirections as dir}
                <option value={dir.value}>{dir.label}</option>
              {/each}
            </select>
          </div>

          <div class="form-control">
            <label class="label" for="conn-interval">
              <span class="label-text">Sync Interval (minutes)</span>
            </label>
            <input
              type="number"
              id="conn-interval"
              class="input input-bordered"
              bind:value={formData.sync_interval_minutes}
              min="5"
              disabled={!formData.auto_sync}
            />
          </div>

          <div class="form-control">
            <label class="label cursor-pointer justify-start gap-3">
              <input type="checkbox" class="checkbox checkbox-primary" bind:checked={formData.use_ftps} />
              <span class="label-text">Use FTPS (TLS/SSL)</span>
            </label>
          </div>

          <div class="form-control">
            <label class="label cursor-pointer justify-start gap-3">
              <input type="checkbox" class="checkbox checkbox-primary" bind:checked={formData.passive_mode} />
              <span class="label-text">Passive Mode</span>
            </label>
          </div>

          <div class="form-control md:col-span-2">
            <label class="label cursor-pointer justify-start gap-3">
              <input type="checkbox" class="checkbox checkbox-primary" bind:checked={formData.auto_sync} />
              <span class="label-text">Enable Auto Sync</span>
            </label>
          </div>
        </div>

        <div class="modal-action">
          <button type="button" class="btn" onclick={closeModal}>Cancel</button>
          <button type="submit" class="btn btn-primary">
            {editingConnection ? "Update" : "Create"}
          </button>
        </div>
      </form>
    </div>
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={closeModal}
      onkeydown={(e) => e.key === "Escape" && closeModal()}
    ></div>
  </div>
{/if}
