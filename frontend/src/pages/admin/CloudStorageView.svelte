<script>
  import { onMount, onDestroy } from "svelte";
  import api from "../../lib/api.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import { success, error as errorToast } from "../../stores/toast.js";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import { modals, modalEvents } from "../../stores/modals.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let backends = $state([]);
  let stats = $state(null);
  let migrations = $state([]);
  let loading = $state(true);
  let selectedTab = $state("backends"); // backends, migrations

  // Modal state
  let showBackendModal = $state(false);
  let editingBackend = $state(null);
  let showMigrationModal = $state(false);

  // Form data
  let backendForm = $state({
    name: "",
    backend_type: "s3",
    is_active: true,
    priority: 100,
    config: {
      endpoint: "",
      bucket: "",
      region: "us-east-1",
      access_key: "",
      secret_key: "",
    },
  });

  let migrationForm = $state({
    name: "",
    source_backend_id: "",
    target_backend_id: "",
    file_filter: null,
  });

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    loading = true;
    try {
      const [backendsRes, statsRes, migrationsRes] = await Promise.all([
        api.cloudStorage.listBackends(),
        api.cloudStorage.getStats(),
        api.cloudStorage.listMigrations(),
      ]);
      backends = backendsRes || [];
      stats = statsRes;
      migrations = migrationsRes || [];
    } catch (err) {
      console.error("Failed to load cloud storage data:", err);
      errorToast(tr("cloudStorage.loadError"));
    } finally {
      loading = false;
    }
  }

  function openCreateBackend() {
    editingBackend = null;
    backendForm = {
      name: "",
      backend_type: "s3",
      is_active: true,
      priority: 100,
      config: {
        endpoint: "",
        bucket: "",
        region: "us-east-1",
        access_key: "",
        secret_key: "",
      },
    };
    showBackendModal = true;
  }

  function openEditBackend(backend) {
    editingBackend = backend;
    const config = JSON.parse(backend.config || "{}");
    backendForm = {
      name: backend.name,
      backend_type: backend.backend_type,
      is_active: backend.is_active === 1,
      priority: backend.priority,
      config: config,
    };
    showBackendModal = true;
  }

  async function saveBackend() {
    try {
      if (editingBackend) {
        await api.cloudStorage.updateBackend(editingBackend.id, {
          name: backendForm.name,
          config: backendForm.config,
          is_active: backendForm.is_active,
          priority: backendForm.priority,
        });
        success(tr("cloudStorage.backendUpdated"));
      } else {
        await api.cloudStorage.createBackend({
          name: backendForm.name,
          backend_type: backendForm.backend_type,
          config: backendForm.config,
          is_active: backendForm.is_active,
          priority: backendForm.priority,
        });
        success(tr("cloudStorage.backendCreated"));
      }
      showBackendModal = false;
      await loadData();
    } catch (err) {
      console.error("Failed to save backend:", err);
      errorToast(tr("cloudStorage.saveError"));
    }
  }

  async function deleteBackend(backend) {
    if (!confirm(tr("cloudStorage.confirmDelete", backend.name))) return;

    try {
      await api.cloudStorage.deleteBackend(backend.id);
      success(tr("cloudStorage.backendDeleted"));
      await loadData();
    } catch (err) {
      console.error("Failed to delete backend:", err);
      errorToast(err.message || tr("cloudStorage.deleteError"));
    }
  }

  async function setDefaultBackend(backend) {
    try {
      await api.cloudStorage.setDefault(backend.id);
      success(tr("cloudStorage.defaultSet"));
      await loadData();
    } catch (err) {
      console.error("Failed to set default:", err);
      errorToast(tr("cloudStorage.setDefaultError"));
    }
  }

  async function testConnection(backend) {
    try {
      const result = await api.cloudStorage.testConnection(backend.id);
      if (result.success) {
        success(tr("cloudStorage.connectionSuccess", result.latency_ms));
      } else {
        errorToast(tr("cloudStorage.connectionFailed"));
      }
    } catch (err) {
      console.error("Connection test failed:", err);
      errorToast(tr("cloudStorage.connectionFailed"));
    }
  }

  function openCreateMigration() {
    migrationForm = {
      name: "",
      source_backend_id: "",
      target_backend_id: "",
      file_filter: null,
    };
    showMigrationModal = true;
  }

  async function createMigration() {
    if (!migrationForm.source_backend_id || !migrationForm.target_backend_id) {
      errorToast(tr("cloudStorage.selectBackends"));
      return;
    }
    if (migrationForm.source_backend_id === migrationForm.target_backend_id) {
      errorToast(tr("cloudStorage.sameBackendError"));
      return;
    }

    try {
      await api.cloudStorage.createMigration(migrationForm);
      success(tr("cloudStorage.migrationCreated"));
      showMigrationModal = false;
      await loadData();
    } catch (err) {
      console.error("Failed to create migration:", err);
      errorToast(tr("cloudStorage.migrationError"));
    }
  }

  async function cancelMigration(migration) {
    if (!confirm(tr("cloudStorage.confirmCancelMigration"))) return;

    try {
      await api.cloudStorage.cancelMigration(migration.id);
      success(tr("cloudStorage.migrationCancelled"));
      await loadData();
    } catch (err) {
      console.error("Failed to cancel migration:", err);
      errorToast(tr("cloudStorage.cancelError"));
    }
  }

  function formatBytes(bytes) {
    if (!bytes) return "0 B";
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return (bytes / Math.pow(1024, i)).toFixed(2) + " " + sizes[i];
  }

  function getBackendIcon(type) {
    switch (type) {
      case "s3":
        return "cloud";
      case "minio":
        return "hdd-stack";
      case "gcs":
        return "google";
      case "azure_blob":
        return "microsoft";
      case "local":
        return "hdd";
      default:
        return "cloud";
    }
  }

  function getBackendColor(type) {
    switch (type) {
      case "s3":
        return "from-orange-500 to-yellow-500";
      case "minio":
        return "from-red-500 to-pink-500";
      case "gcs":
        return "from-blue-500 to-cyan-500";
      case "azure_blob":
        return "from-blue-600 to-indigo-600";
      case "local":
        return "from-gray-500 to-gray-600";
      default:
        return "from-purple-500 to-pink-500";
    }
  }

  function getStatusBadge(status) {
    switch (status) {
      case "pending":
        return "bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-300";
      case "running":
        return "bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300";
      case "completed":
        return "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-300";
      case "failed":
        return "bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-300";
      case "cancelled":
        return "bg-gray-100 text-gray-800 dark:bg-gray-900/30 dark:text-gray-300";
      default:
        return "bg-gray-100 text-gray-800";
    }
  }
</script>

<PageWrapper>
  <div class="min-h-screen bg-gradient-to-br from-base-100 to-base-200 p-6">
    <div class="max-w-7xl mx-auto">
      <!-- Modern Header -->
      <div
        class="mb-8 bg-base-100/50 backdrop-blur-xl rounded-2xl shadow-2xl border border-base-300 p-8"
      >
        <div class="flex items-center justify-between mb-6">
          <div class="flex items-center gap-4">
            <div
              class="p-4 bg-gradient-to-br from-sky-500 to-blue-600 rounded-xl shadow-lg"
            >
              <i class="bi bi-cloud-arrow-up text-3xl text-white"></i>
            </div>
            <div>
              <h1
                class="text-4xl font-bold bg-gradient-to-r from-sky-600 to-blue-600 bg-clip-text text-transparent"
              >
                {tr("cloudStorage.title")}
              </h1>
              <p class="text-base-content/60 mt-1 text-lg">
                {tr("cloudStorage.description")}
              </p>
            </div>
          </div>
        </div>

        <!-- Stats Cards -->
        {#if stats}
          <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div
              class="bg-gradient-to-br from-blue-500/10 to-blue-600/10 border border-blue-500/20 rounded-xl p-4"
            >
              <div class="flex items-center justify-between">
                <i class="bi bi-hdd-stack text-2xl text-blue-500"></i>
                <span
                  class="text-xs font-semibold text-blue-600 bg-blue-500/20 px-2 py-1 rounded-full"
                >
                  {tr("cloudStorage.backends")}
                </span>
              </div>
              <div class="text-3xl font-bold text-blue-600 mt-2">
                {stats.total_backends}
              </div>
              <div class="text-sm text-base-content/60">
                {stats.active_backends}
                {tr("cloudStorage.active")}
              </div>
            </div>

            <div
              class="bg-gradient-to-br from-green-500/10 to-green-600/10 border border-green-500/20 rounded-xl p-4"
            >
              <div class="flex items-center justify-between">
                <i class="bi bi-files text-2xl text-green-500"></i>
                <span
                  class="text-xs font-semibold text-green-600 bg-green-500/20 px-2 py-1 rounded-full"
                >
                  {tr("cloudStorage.files")}
                </span>
              </div>
              <div class="text-3xl font-bold text-green-600 mt-2">
                {stats.total_files?.toLocaleString() || 0}
              </div>
              <div class="text-sm text-base-content/60">
                {tr("cloudStorage.totalFiles")}
              </div>
            </div>

            <div
              class="bg-gradient-to-br from-purple-500/10 to-purple-600/10 border border-purple-500/20 rounded-xl p-4"
            >
              <div class="flex items-center justify-between">
                <i class="bi bi-database text-2xl text-purple-500"></i>
                <span
                  class="text-xs font-semibold text-purple-600 bg-purple-500/20 px-2 py-1 rounded-full"
                >
                  {tr("cloudStorage.storage")}
                </span>
              </div>
              <div class="text-3xl font-bold text-purple-600 mt-2">
                {formatBytes(stats.total_storage_bytes)}
              </div>
              <div class="text-sm text-base-content/60">
                {tr("cloudStorage.totalStorage")}
              </div>
            </div>

            <div
              class="bg-gradient-to-br from-orange-500/10 to-orange-600/10 border border-orange-500/20 rounded-xl p-4"
            >
              <div class="flex items-center justify-between">
                <i class="bi bi-arrow-left-right text-2xl text-orange-500"></i>
                <span
                  class="text-xs font-semibold text-orange-600 bg-orange-500/20 px-2 py-1 rounded-full"
                >
                  {tr("cloudStorage.migrations")}
                </span>
              </div>
              <div class="text-3xl font-bold text-orange-600 mt-2">
                {migrations.filter((m) => m.status === "running").length}
              </div>
              <div class="text-sm text-base-content/60">
                {tr("cloudStorage.activeMigrations")}
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Tabs -->
      <div
        class="flex gap-3 mb-6 p-2 bg-base-100/50 backdrop-blur-xl rounded-xl border border-base-300"
      >
        <button
          class="flex items-center gap-2 px-6 py-3 rounded-lg font-semibold transition-all {selectedTab ===
          'backends'
            ? 'bg-gradient-to-r from-sky-500 to-blue-500 text-white shadow-lg'
            : 'hover:bg-base-300'}"
          onclick={() => (selectedTab = "backends")}
        >
          <i class="bi bi-hdd-stack"></i>
          {tr("cloudStorage.storageBackends")}
        </button>
        <button
          class="flex items-center gap-2 px-6 py-3 rounded-lg font-semibold transition-all {selectedTab ===
          'migrations'
            ? 'bg-gradient-to-r from-sky-500 to-blue-500 text-white shadow-lg'
            : 'hover:bg-base-300'}"
          onclick={() => (selectedTab = "migrations")}
        >
          <i class="bi bi-arrow-left-right"></i>
          {tr("cloudStorage.dataMigrations")}
          {#if migrations.filter((m) => m.status === "running").length > 0}
            <span class="badge badge-sm bg-orange-500 text-white border-0">
              {migrations.filter((m) => m.status === "running").length}
            </span>
          {/if}
        </button>
      </div>

      <!-- Content -->
      {#if loading}
        <div class="flex justify-center py-20">
          <span class="loading loading-spinner loading-lg text-blue-500"></span>
        </div>
      {:else if selectedTab === "backends"}
        <!-- Backends Tab -->
        <div class="space-y-6">
          <div class="flex justify-end">
            <button class="btn btn-primary gap-2" onclick={openCreateBackend}>
              <i class="bi bi-plus-circle"></i>
              {tr("cloudStorage.addBackend")}
            </button>
          </div>

          {#if backends.length === 0}
            <div
              class="text-center py-16 bg-base-100/50 backdrop-blur-xl rounded-2xl border border-base-300"
            >
              <i class="bi bi-cloud-slash text-6xl text-base-content/30 mb-4"
              ></i>
              <p class="text-lg text-base-content/60">
                {tr("cloudStorage.noBackends")}
              </p>
            </div>
          {:else}
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {#each backends as backend}
                <div
                  class="card bg-base-100 shadow-xl border border-base-300 hover:shadow-2xl transition-shadow"
                >
                  <div class="card-body">
                    <div class="flex items-start justify-between">
                      <div class="flex items-center gap-3">
                        <div
                          class="p-3 bg-gradient-to-br {getBackendColor(
                            backend.backend_type
                          )} rounded-xl"
                        >
                          <i
                            class="bi bi-{getBackendIcon(
                              backend.backend_type
                            )} text-2xl text-white"
                          ></i>
                        </div>
                        <div>
                          <h3 class="font-bold text-lg">{backend.name}</h3>
                          <p class="text-sm text-base-content/60 capitalize">
                            {backend.backend_type}
                          </p>
                        </div>
                      </div>
                      <div class="flex gap-1">
                        {#if backend.is_default}
                          <span class="badge badge-primary badge-sm"
                            >{tr("cloudStorage.default")}</span
                          >
                        {/if}
                        {#if backend.is_active}
                          <span class="badge badge-success badge-sm"
                            >{tr("cloudStorage.active")}</span
                          >
                        {:else}
                          <span class="badge badge-ghost badge-sm"
                            >{tr("cloudStorage.inactive")}</span
                          >
                        {/if}
                      </div>
                    </div>

                    <div class="divider my-2"></div>

                    <div class="grid grid-cols-2 gap-4 text-sm">
                      <div>
                        <div class="text-base-content/60">
                          {tr("cloudStorage.files")}
                        </div>
                        <div class="font-semibold">
                          {backend.file_count?.toLocaleString() || 0}
                        </div>
                      </div>
                      <div>
                        <div class="text-base-content/60">
                          {tr("cloudStorage.storage")}
                        </div>
                        <div class="font-semibold">
                          {formatBytes(backend.storage_used_bytes)}
                        </div>
                      </div>
                      <div>
                        <div class="text-base-content/60">
                          {tr("cloudStorage.priority")}
                        </div>
                        <div class="font-semibold">{backend.priority}</div>
                      </div>
                      <div>
                        <div class="text-base-content/60">
                          {tr("cloudStorage.created")}
                        </div>
                        <div class="font-semibold text-xs">
                          {new Date(backend.created_at).toLocaleDateString()}
                        </div>
                      </div>
                    </div>

                    <div class="card-actions justify-end mt-4">
                      <button
                        class="btn btn-sm btn-ghost"
                        onclick={() => testConnection(backend)}
                        title={tr("cloudStorage.testConnection")}
                      >
                        <i class="bi bi-plug"></i>
                      </button>
                      {#if !backend.is_default}
                        <button
                          class="btn btn-sm btn-ghost"
                          onclick={() => setDefaultBackend(backend)}
                          title={tr("cloudStorage.setAsDefault")}
                        >
                          <i class="bi bi-star"></i>
                        </button>
                      {/if}
                      <button
                        class="btn btn-sm btn-ghost"
                        onclick={() => openEditBackend(backend)}
                        title={tr("common.edit")}
                      >
                        <i class="bi bi-pencil"></i>
                      </button>
                      {#if backend.backend_type !== "local" || !backend.is_default}
                        <button
                          class="btn btn-sm btn-ghost text-error"
                          onclick={() => deleteBackend(backend)}
                          title={tr("common.delete")}
                        >
                          <i class="bi bi-trash"></i>
                        </button>
                      {/if}
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {:else if selectedTab === "migrations"}
        <!-- Migrations Tab -->
        <div class="space-y-6">
          <div class="flex justify-end">
            <button
              class="btn btn-primary gap-2"
              onclick={openCreateMigration}
              disabled={backends.length < 2}
            >
              <i class="bi bi-arrow-left-right"></i>
              {tr("cloudStorage.startMigration")}
            </button>
          </div>

          {#if migrations.length === 0}
            <div
              class="text-center py-16 bg-base-100/50 backdrop-blur-xl rounded-2xl border border-base-300"
            >
              <i
                class="bi bi-arrow-left-right text-6xl text-base-content/30 mb-4"
              ></i>
              <p class="text-lg text-base-content/60">
                {tr("cloudStorage.noMigrations")}
              </p>
            </div>
          {:else}
            <div
              class="overflow-x-auto bg-base-100 rounded-xl border border-base-300"
            >
              <table class="table w-full">
                <thead>
                  <tr>
                    <th>{tr("cloudStorage.migrationName")}</th>
                    <th
                      >{tr("cloudStorage.source")} → {tr(
                        "cloudStorage.target"
                      )}</th
                    >
                    <th>{tr("cloudStorage.progress")}</th>
                    <th>{tr("cloudStorage.status")}</th>
                    <th>{tr("cloudStorage.actions")}</th>
                  </tr>
                </thead>
                <tbody>
                  {#each migrations as migration}
                    {@const sourceBackend = backends.find(
                      (b) => b.id === migration.source_backend_id
                    )}
                    {@const targetBackend = backends.find(
                      (b) => b.id === migration.target_backend_id
                    )}
                    <tr>
                      <td class="font-semibold">{migration.name}</td>
                      <td>
                        <span class="badge badge-ghost"
                          >{sourceBackend?.name || "Unknown"}</span
                        >
                        <i class="bi bi-arrow-right mx-2"></i>
                        <span class="badge badge-ghost"
                          >{targetBackend?.name || "Unknown"}</span
                        >
                      </td>
                      <td>
                        {#if migration.progress_total > 0}
                          <div class="flex items-center gap-2">
                            <progress
                              class="progress progress-primary w-24"
                              value={migration.progress_current}
                              max={migration.progress_total}
                            ></progress>
                            <span class="text-sm">
                              {Math.round(
                                (migration.progress_current /
                                  migration.progress_total) *
                                  100
                              )}%
                            </span>
                          </div>
                          <div class="text-xs text-base-content/60">
                            {migration.files_migrated} / {migration.progress_total}
                            {tr("cloudStorage.files")}
                          </div>
                        {:else}
                          <span class="text-base-content/60">—</span>
                        {/if}
                      </td>
                      <td>
                        <span
                          class="px-2 py-1 rounded-full text-xs font-semibold {getStatusBadge(
                            migration.status
                          )}"
                        >
                          {migration.status}
                        </span>
                      </td>
                      <td>
                        {#if migration.status === "pending" || migration.status === "running"}
                          <button
                            class="btn btn-sm btn-ghost text-error"
                            onclick={() => cancelMigration(migration)}
                          >
                            <i class="bi bi-x-circle"></i>
                          </button>
                        {/if}
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {/if}
        </div>
      {/if}
    </div>
  </div>
</PageWrapper>

<!-- Backend Modal -->
{#if showBackendModal}
  <div class="modal modal-open">
    <div class="modal-box max-w-2xl">
      <h3 class="font-bold text-2xl mb-6 flex items-center gap-2">
        <i
          class="bi bi-{editingBackend ? 'pencil' : 'plus-circle'} text-primary"
        ></i>
        {editingBackend
          ? tr("cloudStorage.editBackend")
          : tr("cloudStorage.addBackend")}
      </h3>

      <div class="space-y-4">
        <!-- Name -->
        <div class="form-control">
          <label class="label" for="backend-name">
            <span class="label-text font-semibold"
              >{tr("cloudStorage.backendName")}</span
            >
          </label>
          <input
            id="backend-name"
            type="text"
            class="input input-bordered"
            bind:value={backendForm.name}
            placeholder="My S3 Bucket"
          />
        </div>

        <!-- Type (only for new backends) -->
        {#if !editingBackend}
          <div class="form-control">
            <label class="label" for="backend-type">
              <span class="label-text font-semibold"
                >{tr("cloudStorage.backendType")}</span
              >
            </label>
            <select
              id="backend-type"
              class="select select-bordered"
              bind:value={backendForm.backend_type}
            >
              <option value="s3">Amazon S3</option>
              <option value="minio">MinIO</option>
              <option value="gcs">Google Cloud Storage</option>
              <option value="azure_blob">Azure Blob Storage</option>
              <option value="local">Local Storage</option>
            </select>
          </div>
        {/if}

        <!-- Configuration based on type -->
        {#if backendForm.backend_type === "s3" || backendForm.backend_type === "minio"}
          <div class="grid grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label" for="config-endpoint">
                <span class="label-text">{tr("cloudStorage.endpoint")}</span>
              </label>
              <input
                id="config-endpoint"
                type="text"
                class="input input-bordered"
                bind:value={backendForm.config.endpoint}
                placeholder="s3.amazonaws.com"
              />
            </div>
            <div class="form-control">
              <label class="label" for="config-bucket">
                <span class="label-text">{tr("cloudStorage.bucket")}</span>
              </label>
              <input
                id="config-bucket"
                type="text"
                class="input input-bordered"
                bind:value={backendForm.config.bucket}
                placeholder="my-bucket"
              />
            </div>
            <div class="form-control">
              <label class="label" for="config-region">
                <span class="label-text">{tr("cloudStorage.region")}</span>
              </label>
              <input
                id="config-region"
                type="text"
                class="input input-bordered"
                bind:value={backendForm.config.region}
                placeholder="us-east-1"
              />
            </div>
            <div class="form-control">
              <label class="label" for="config-access-key">
                <span class="label-text">{tr("cloudStorage.accessKey")}</span>
              </label>
              <input
                id="config-access-key"
                type="text"
                class="input input-bordered"
                bind:value={backendForm.config.access_key}
                placeholder="AKIAIOSFODNN7EXAMPLE"
              />
            </div>
            <div class="form-control col-span-2">
              <label class="label" for="config-secret-key">
                <span class="label-text">{tr("cloudStorage.secretKey")}</span>
              </label>
              <input
                id="config-secret-key"
                type="password"
                class="input input-bordered"
                bind:value={backendForm.config.secret_key}
                placeholder="••••••••"
              />
            </div>
          </div>
        {:else if backendForm.backend_type === "local"}
          <div class="form-control">
            <label class="label" for="config-path">
              <span class="label-text">{tr("cloudStorage.basePath")}</span>
            </label>
            <input
              id="config-path"
              type="text"
              class="input input-bordered"
              bind:value={backendForm.config.base_path}
              placeholder="./data"
            />
          </div>
        {/if}

        <!-- Priority -->
        <div class="form-control">
          <label class="label" for="backend-priority">
            <span class="label-text font-semibold"
              >{tr("cloudStorage.priority")}</span
            >
          </label>
          <input
            id="backend-priority"
            type="number"
            class="input input-bordered"
            bind:value={backendForm.priority}
            min="1"
            max="1000"
          />
          <label class="label">
            <span class="label-text-alt">{tr("cloudStorage.priorityHint")}</span
            >
          </label>
        </div>

        <!-- Active Toggle -->
        <div class="form-control">
          <label class="label cursor-pointer justify-start gap-2">
            <input
              type="checkbox"
              class="checkbox checkbox-primary"
              bind:checked={backendForm.is_active}
            />
            <span class="label-text font-semibold"
              >{tr("cloudStorage.activeBackend")}</span
            >
          </label>
        </div>
      </div>

      <div class="modal-action">
        <button
          class="btn btn-ghost"
          onclick={() => (showBackendModal = false)}
        >
          {tr("common.cancel")}
        </button>
        <button class="btn btn-primary" onclick={saveBackend}>
          {editingBackend ? tr("common.save") : tr("common.create")}
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop"
      onclick={() => (showBackendModal = false)}
    ></div>
  </div>
{/if}

<!-- Migration Modal -->
{#if showMigrationModal}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-2xl mb-6 flex items-center gap-2">
        <i class="bi bi-arrow-left-right text-primary"></i>
        {tr("cloudStorage.startMigration")}
      </h3>

      <div class="space-y-4">
        <div class="form-control">
          <label class="label" for="migration-name">
            <span class="label-text font-semibold"
              >{tr("cloudStorage.migrationName")}</span
            >
          </label>
          <input
            id="migration-name"
            type="text"
            class="input input-bordered"
            bind:value={migrationForm.name}
            placeholder="S3 to MinIO Migration"
          />
        </div>

        <div class="form-control">
          <label class="label" for="migration-source">
            <span class="label-text font-semibold"
              >{tr("cloudStorage.sourceBackend")}</span
            >
          </label>
          <select
            id="migration-source"
            class="select select-bordered"
            bind:value={migrationForm.source_backend_id}
          >
            <option value="">{tr("cloudStorage.selectBackend")}</option>
            {#each backends.filter((b) => b.is_active) as backend}
              <option value={backend.id}
                >{backend.name} ({backend.backend_type})</option
              >
            {/each}
          </select>
        </div>

        <div class="form-control">
          <label class="label" for="migration-target">
            <span class="label-text font-semibold"
              >{tr("cloudStorage.targetBackend")}</span
            >
          </label>
          <select
            id="migration-target"
            class="select select-bordered"
            bind:value={migrationForm.target_backend_id}
          >
            <option value="">{tr("cloudStorage.selectBackend")}</option>
            {#each backends.filter((b) => b.is_active && b.id !== migrationForm.source_backend_id) as backend}
              <option value={backend.id}
                >{backend.name} ({backend.backend_type})</option
              >
            {/each}
          </select>
        </div>

        <div class="alert alert-info">
          <i class="bi bi-info-circle"></i>
          <span>{tr("cloudStorage.migrationWarning")}</span>
        </div>
      </div>

      <div class="modal-action">
        <button
          class="btn btn-ghost"
          onclick={() => (showMigrationModal = false)}
        >
          {tr("common.cancel")}
        </button>
        <button class="btn btn-primary" onclick={createMigration}>
          {tr("cloudStorage.startMigration")}
        </button>
      </div>
    </div>
    <div
      class="modal-backdrop"
      onclick={() => (showMigrationModal = false)}
    ></div>
  </div>
{/if}
