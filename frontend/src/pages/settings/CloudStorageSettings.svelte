<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import {
    success as toastSuccess,
    error as toastError,
  } from "../../stores/toast.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let loading = $state(true);
  let saving = $state(false);
  let testing = $state(false);
  let totalSpace = $state(50);
  let usedSpace = $state(0);
  let fileCount = $state(0);
  let backends = $state([]);
  let showAddBackend = $state(false);

  let newBackend = $state({
    name: "",
    type: "s3",
    endpoint: "",
    bucket: "",
    region: "eu-central-1",
    accessKey: "",
    secretKey: "",
    isActive: true,
  });

  const usagePercent = $derived(
    totalSpace > 0 ? Math.min((usedSpace / totalSpace) * 100, 100) : 0
  );
  const freeSpace = $derived(Math.max(totalSpace - usedSpace, 0));

  const backendTypes = [
    { value: "local", label: "Local Storage", icon: "bi-hdd" },
    { value: "s3", label: "Amazon S3", icon: "bi-cloud" },
    { value: "minio", label: "MinIO", icon: "bi-server" },
    { value: "gcs", label: "Google Cloud Storage", icon: "bi-google" },
    { value: "azure_blob", label: "Azure Blob Storage", icon: "bi-microsoft" },
  ];

  onMount(async () => {
    await loadStorageData();
  });

  async function loadStorageData() {
    loading = true;
    try {
      const token = localStorage.getItem("authToken");
      const statsResponse = await fetch(
        "http://localhost:8080/api/storage/stats",
        {
          headers: { Authorization: `Bearer ${token}` },
        }
      );
      if (statsResponse.ok) {
        const stats = await statsResponse.json();
        usedSpace = (stats.total_storage_bytes || 0) / 1024 ** 3;
        fileCount = stats.total_files || 0;
      }
      const backendsResponse = await fetch(
        "http://localhost:8080/api/storage/backends",
        {
          headers: { Authorization: `Bearer ${token}` },
        }
      );
      if (backendsResponse.ok) {
        backends = await backendsResponse.json();
      }
    } catch (err) {
      console.error("Failed to load storage data:", err);
    } finally {
      loading = false;
    }
  }

  async function addBackend() {
    saving = true;
    try {
      const token = localStorage.getItem("authToken");
      const response = await fetch(
        "http://localhost:8080/api/storage/backends",
        {
          method: "POST",
          headers: {
            Authorization: `Bearer ${token}`,
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            name: newBackend.name,
            backend_type: newBackend.type,
            config: {
              endpoint: newBackend.endpoint,
              bucket: newBackend.bucket,
              region: newBackend.region,
              access_key: newBackend.accessKey,
              secret_key: newBackend.secretKey,
            },
            is_active: newBackend.isActive,
          }),
        }
      );
      if (response.ok) {
        toastSuccess(tr("settings.storage.backend_added"));
        showAddBackend = false;
        resetNewBackend();
        await loadStorageData();
      } else {
        const data = await response.json();
        toastError(data.error || tr("settings.storage.add_error"));
      }
    } catch (err) {
      toastError(err.message);
    } finally {
      saving = false;
    }
  }

  async function testBackendConnection(backendId) {
    testing = true;
    try {
      const token = localStorage.getItem("authToken");
      const response = await fetch(
        `http://localhost:8080/api/storage/backends/${backendId}/test`,
        {
          method: "POST",
          headers: { Authorization: `Bearer ${token}` },
        }
      );
      if (response.ok) {
        toastSuccess(tr("settings.storage.connection_ok"));
      } else {
        toastError(tr("settings.storage.connection_failed"));
      }
    } catch (err) {
      toastError(err.message);
    } finally {
      testing = false;
    }
  }

  async function setDefaultBackend(backendId) {
    try {
      const token = localStorage.getItem("authToken");
      const response = await fetch(
        `http://localhost:8080/api/storage/backends/${backendId}/set-default`,
        {
          method: "POST",
          headers: { Authorization: `Bearer ${token}` },
        }
      );
      if (response.ok) {
        toastSuccess(tr("settings.storage.default_set"));
        await loadStorageData();
      } else {
        toastError(tr("settings.storage.default_error"));
      }
    } catch (err) {
      toastError(err.message);
    }
  }

  async function deleteBackend(backendId) {
    if (!confirm(tr("settings.storage.delete_confirm"))) return;
    try {
      const token = localStorage.getItem("authToken");
      const response = await fetch(
        `http://localhost:8080/api/storage/backends/${backendId}`,
        {
          method: "DELETE",
          headers: { Authorization: `Bearer ${token}` },
        }
      );
      if (response.ok) {
        toastSuccess(tr("settings.storage.backend_deleted"));
        await loadStorageData();
      } else {
        toastError(tr("settings.storage.delete_error"));
      }
    } catch (err) {
      toastError(err.message);
    }
  }

  function resetNewBackend() {
    newBackend = {
      name: "",
      type: "s3",
      endpoint: "",
      bucket: "",
      region: "eu-central-1",
      accessKey: "",
      secretKey: "",
      isActive: true,
    };
  }

  function formatSize(gb) {
    if (gb < 1) return `${(gb * 1024).toFixed(0)} MB`;
    return `${gb.toFixed(2)} GB`;
  }

  function formatBytes(bytes) {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    if (bytes < 1024 * 1024 * 1024)
      return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
    return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
  }

  function getBackendIcon(type) {
    const found = backendTypes.find((b) => b.value === type);
    return found?.icon || "bi-hdd";
  }
</script>

{#if loading}
  <div class="flex items-center justify-center py-12">
    <div
      class="animate-spin rounded-full h-8 w-8 border-b-2 border-green-500"
    ></div>
  </div>
{:else}
  <div class="space-y-6">
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
      <div
        class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700"
      >
        <div class="flex items-center gap-3">
          <div
            class="w-10 h-10 rounded-lg bg-green-100 dark:bg-green-900/30 flex items-center justify-center"
          >
            <i class="bi bi-hdd-stack-fill text-green-600 dark:text-green-400"
            ></i>
          </div>
          <div>
            <p class="text-xl font-bold text-gray-900 dark:text-white">
              {formatSize(totalSpace)}
            </p>
            <p
              class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wide"
            >
              {tr("settings.storage.total")}
            </p>
          </div>
        </div>
      </div>
      <div
        class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700"
      >
        <div class="flex items-center gap-3">
          <div
            class="w-10 h-10 rounded-lg bg-purple-100 dark:bg-purple-900/30 flex items-center justify-center"
          >
            <i class="bi bi-pie-chart-fill text-purple-600 dark:text-purple-400"
            ></i>
          </div>
          <div>
            <p class="text-xl font-bold text-gray-900 dark:text-white">
              {formatSize(usedSpace)}
            </p>
            <p
              class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wide"
            >
              {tr("settings.storage.used")}
            </p>
          </div>
        </div>
      </div>
      <div
        class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700"
      >
        <div class="flex items-center gap-3">
          <div
            class="w-10 h-10 rounded-lg bg-green-100 dark:bg-green-900/30 flex items-center justify-center"
          >
            <i
              class="bi bi-check-circle-fill text-green-600 dark:text-green-400"
            ></i>
          </div>
          <div>
            <p class="text-xl font-bold text-gray-900 dark:text-white">
              {formatSize(freeSpace)}
            </p>
            <p
              class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wide"
            >
              {tr("settings.storage.free")}
            </p>
          </div>
        </div>
      </div>
      <div
        class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700"
      >
        <div class="flex items-center gap-3">
          <div
            class="w-10 h-10 rounded-lg bg-amber-100 dark:bg-amber-900/30 flex items-center justify-center"
          >
            <i class="bi bi-files text-amber-600 dark:text-amber-400"></i>
          </div>
          <div>
            <p class="text-xl font-bold text-gray-900 dark:text-white">
              {fileCount.toLocaleString()}
            </p>
            <p
              class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wide"
            >
              {tr("settings.storage.files")}
            </p>
          </div>
        </div>
      </div>
    </div>

    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6"
    >
      <div class="flex items-center gap-3 mb-4">
        <div
          class="w-10 h-10 rounded-lg bg-green-100 dark:bg-green-900/30 flex items-center justify-center"
        >
          <i class="bi bi-speedometer2 text-green-600 dark:text-green-400"></i>
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
            {tr("settings.storage.usage")}
          </h3>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            {tr("settings.storage.usage_desc")}
          </p>
        </div>
      </div>
      <div class="space-y-2">
        <div
          class="h-4 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden"
        >
          <div
            class="h-full bg-gradient-to-r from-green-500 to-green-600 rounded-full transition-all duration-500"
            style="width: {usagePercent}%"
          ></div>
        </div>
        <div class="flex justify-between text-sm">
          <span class="text-gray-600 dark:text-gray-400"
            >{formatSize(usedSpace)}
            {tr("settings.storage.used_of")}
            {formatSize(totalSpace)}</span
          >
          <span class="font-semibold text-green-600 dark:text-green-400"
            >{usagePercent.toFixed(1)}%</span
          >
        </div>
      </div>
    </div>

    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6"
    >
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-3">
          <div
            class="w-10 h-10 rounded-lg bg-purple-100 dark:bg-purple-900/30 flex items-center justify-center"
          >
            <i class="bi bi-cloud-fill text-purple-600 dark:text-purple-400"
            ></i>
          </div>
          <div>
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
              {tr("settings.storage.backends")}
            </h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              {tr("settings.storage.backends_desc")}
            </p>
          </div>
        </div>
        <button
          onclick={() => (showAddBackend = true)}
          class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg flex items-center gap-2"
        >
          <i class="bi bi-plus-lg"></i>
          {tr("settings.storage.add_backend")}
        </button>
      </div>
      {#if backends.length === 0}
        <div class="text-center py-8 text-gray-500 dark:text-gray-400">
          <i class="bi bi-inbox text-4xl mb-3 block"></i>
          <p>{tr("settings.storage.no_backends")}</p>
          <p class="text-sm">{tr("settings.storage.no_backends_hint")}</p>
        </div>
      {:else}
        <div class="space-y-3">
          {#each backends as backend}
            <div
              class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg"
            >
              <div class="flex items-center gap-4">
                <div
                  class="w-12 h-12 rounded-lg bg-white dark:bg-gray-600 flex items-center justify-center border border-gray-200 dark:border-gray-500"
                >
                  <i
                    class="bi {getBackendIcon(
                      backend.backend_type
                    )} text-xl text-gray-600 dark:text-gray-300"
                  ></i>
                </div>
                <div>
                  <div class="flex items-center gap-2">
                    <span class="font-semibold text-gray-900 dark:text-white"
                      >{backend.name}</span
                    >
                    {#if backend.is_default}
                      <span
                        class="px-2 py-0.5 text-xs bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 rounded-full"
                        >{tr("settings.storage.default")}</span
                      >
                    {/if}
                    {#if !backend.is_active}
                      <span
                        class="px-2 py-0.5 text-xs bg-gray-100 dark:bg-gray-600 text-gray-600 dark:text-gray-300 rounded-full"
                        >{tr("settings.storage.inactive")}</span
                      >
                    {/if}
                  </div>
                  <div class="text-sm text-gray-500 dark:text-gray-400">
                    {backendTypes.find((b) => b.value === backend.backend_type)
                      ?.label || backend.backend_type} · {formatBytes(
                      backend.storage_used_bytes || 0
                    )} · {backend.file_count || 0}
                    {tr("settings.storage.files_lower")}
                  </div>
                </div>
              </div>
              <div class="flex items-center gap-2">
                <button
                  onclick={() => testBackendConnection(backend.id)}
                  disabled={testing}
                  class="p-2 text-gray-500 hover:text-green-600 hover:bg-green-50 dark:hover:bg-green-900/30 rounded-lg"
                  title={tr("settings.storage.test_connection")}
                >
                  <i class="bi bi-plug"></i>
                </button>
                {#if !backend.is_default}
                  <button
                    onclick={() => setDefaultBackend(backend.id)}
                    class="p-2 text-gray-500 hover:text-green-600 hover:bg-green-50 dark:hover:bg-green-900/30 rounded-lg"
                    title={tr("settings.storage.set_default")}
                  >
                    <i class="bi bi-star"></i>
                  </button>
                  <button
                    onclick={() => deleteBackend(backend.id)}
                    class="p-2 text-gray-500 hover:text-red-600 hover:bg-red-50 dark:hover:bg-red-900/30 rounded-lg"
                    title={tr("settings.storage.delete")}
                  >
                    <i class="bi bi-trash"></i>
                  </button>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>
{/if}

{#if showAddBackend}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-xl w-full max-w-lg mx-4 max-h-[90vh] overflow-y-auto"
    >
      <div
        class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700"
      >
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
          {tr("settings.storage.add_backend")}
        </h3>
        <button
          onclick={() => {
            showAddBackend = false;
            resetNewBackend();
          }}
          class="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
        >
          <i class="bi bi-x-lg"></i>
        </button>
      </div>
      <div class="p-4 space-y-4">
        <div>
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
            >{tr("settings.storage.backend_name")}</label
          >
          <input
            type="text"
            bind:value={newBackend.name}
            placeholder="My S3 Bucket"
            class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-green-500"
          />
        </div>
        <div>
          <label
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
            >{tr("settings.storage.backend_type")}</label
          >
          <select
            bind:value={newBackend.type}
            class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-green-500"
          >
            {#each backendTypes as type}
              <option value={type.value}>{type.label}</option>
            {/each}
          </select>
        </div>
        {#if newBackend.type !== "local"}
          <div>
            <label
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
              >{tr("settings.storage.endpoint")}</label
            >
            <input
              type="text"
              bind:value={newBackend.endpoint}
              placeholder="https://s3.amazonaws.com"
              class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-green-500"
            />
          </div>
          <div>
            <label
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
              >{tr("settings.storage.bucket")}</label
            >
            <input
              type="text"
              bind:value={newBackend.bucket}
              placeholder="my-bucket"
              class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-green-500"
            />
          </div>
          <div>
            <label
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
              >{tr("settings.storage.region")}</label
            >
            <input
              type="text"
              bind:value={newBackend.region}
              placeholder="eu-central-1"
              class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-green-500"
            />
          </div>
          <div>
            <label
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
              >{tr("settings.storage.access_key")}</label
            >
            <input
              type="text"
              bind:value={newBackend.accessKey}
              placeholder="AKIAIOSFODNN7EXAMPLE"
              class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-green-500"
            />
          </div>
          <div>
            <label
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
              >{tr("settings.storage.secret_key")}</label
            >
            <input
              type="password"
              bind:value={newBackend.secretKey}
              placeholder="••••••••"
              class="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-green-500"
            />
          </div>
        {/if}
        <label
          class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg cursor-pointer"
        >
          <input
            type="checkbox"
            bind:checked={newBackend.isActive}
            class="w-5 h-5 rounded border-gray-300 text-green-600 focus:ring-green-500"
          />
          <span class="text-gray-900 dark:text-white"
            >{tr("settings.storage.enable_backend")}</span
          >
        </label>
      </div>
      <div
        class="flex justify-end gap-3 p-4 border-t border-gray-200 dark:border-gray-700"
      >
        <button
          onclick={() => {
            showAddBackend = false;
            resetNewBackend();
          }}
          class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
          >{tr("common.cancel")}</button
        >
        <button
          onclick={addBackend}
          disabled={saving || !newBackend.name}
          class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg flex items-center gap-2 disabled:opacity-50"
        >
          {#if saving}
            <div
              class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"
            ></div>
          {:else}
            <i class="bi bi-plus-lg"></i>
          {/if}
          {tr("settings.storage.add_backend")}
        </button>
      </div>
    </div>
  </div>
{/if}
