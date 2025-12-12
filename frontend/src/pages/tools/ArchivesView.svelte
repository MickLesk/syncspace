<script>
  import { onMount } from "svelte";
  import UIInput from "../../../components/ui/UIInput.svelte";
  import UITextarea from "../../../components/ui/UITextarea.svelte";
  import UISelect from "../../../components/ui/UISelect.svelte";
  import UIToggle from "../../../components/ui/UIToggle.svelte";
  import UICheckbox from "../../../components/ui/UICheckbox.svelte";
  import UIModal from "../../../components/ui/UIModal.svelte";
  import UIButton from "../../../components/ui/UIButton.svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import api from "../../lib/api.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let loading = $state(true);
  let creating = $state(false);
  let extracting = $state(false);
  let error = $state("");
  let success = $state("");

  // Archives list
  let archives = $state([]);
  let currentPath = $state("");
  let formats = $state([]);

  // Create Archive Modal
  let showCreateModal = $state(false);
  let createForm = $state({
    files: [],
    archiveName: "",
    format: "zip",
    compressionLevel: 6,
    password: "",
    destination: "",
  });

  // Extract Archive Modal
  let showExtractModal = $state(false);
  let extractForm = $state({
    archivePath: "",
    destination: "",
    password: "",
    flatten: false,
  });

  // Preview Archive Modal
  let showPreviewModal = $state(false);
  let previewLoading = $state(false);
  let previewData = $state(null);

  // File Selection
  let selectedFiles = $state([]);

  onMount(async () => {
    await Promise.all([loadArchives(), loadFormats()]);
  });

  async function loadArchives(path = "") {
    loading = true;
    error = "";
    try {
      const response = await api.archives.list(path);
      archives = response.archives || [];
      currentPath = path;
    } catch (e) {
      console.error("Failed to load archives:", e);
      error = "Failed to load archives";
    } finally {
      loading = false;
    }
  }

  async function loadFormats() {
    try {
      const response = await api.archives.getFormats();
      formats = response.formats || [];
    } catch (e) {
      console.error("Failed to load formats:", e);
    }
  }

  function openCreateModal(files = []) {
    createForm = {
      files: files.length > 0 ? files : [],
      archiveName: "",
      format: "zip",
      compressionLevel: 6,
      password: "",
      destination: currentPath,
    };
    showCreateModal = true;
  }

  function openExtractModal(archive) {
    extractForm = {
      archivePath: archive.path,
      destination: currentPath || archive.path.replace(/\.[^.]+$/, ""),
      password: "",
      flatten: false,
    };
    showExtractModal = true;
  }

  async function createArchive() {
    if (!createForm.archiveName || createForm.files.length === 0) {
      error = "Archive name and files are required";
      return;
    }

    creating = true;
    error = "";
    try {
      const result = await api.archives.create(
        createForm.files,
        createForm.archiveName,
        createForm.format,
        {
          compressionLevel: createForm.compressionLevel,
          password: createForm.password || undefined,
          destination: createForm.destination || undefined,
        }
      );
      success = `Archive creation job queued (Job ID: ${result.job_id})`;
      showCreateModal = false;
      setTimeout(() => loadArchives(currentPath), 2000);
      setTimeout(() => (success = ""), 5000);
    } catch (e) {
      error = e.message || "Failed to create archive";
    } finally {
      creating = false;
    }
  }

  async function extractArchive() {
    extracting = true;
    error = "";
    try {
      const result = await api.archives.extract(extractForm.archivePath, {
        destination: extractForm.destination || undefined,
        password: extractForm.password || undefined,
        flatten: extractForm.flatten,
      });
      success = `Extraction job queued (Job ID: ${result.job_id})`;
      showExtractModal = false;
      setTimeout(() => (success = ""), 5000);
    } catch (e) {
      error = e.message || "Failed to extract archive";
    } finally {
      extracting = false;
    }
  }

  async function deleteArchive(archive) {
    if (!confirm(`Delete archive "${archive.name}"?`)) return;
    try {
      await api.archives.delete(archive.path);
      success = "Archive deleted successfully";
      await loadArchives(currentPath);
      setTimeout(() => (success = ""), 3000);
    } catch (e) {
      error = e.message || "Failed to delete archive";
    }
  }

  function getFormatIcon(format) {
    switch (format) {
      case "zip":
        return "file-earmark-zip";
      case "tar.gz":
      case "tgz":
        return "file-earmark-zip";
      case "tar":
        return "archive";
      case "7z":
        return "file-earmark-zip";
      default:
        return "file-earmark";
    }
  }

  function formatBytes(bytes) {
    if (!bytes) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  }

  function closeCreateModal() {
    showCreateModal = false;
  }

  function closeExtractModal() {
    showExtractModal = false;
  }

  async function openPreviewModal(archive) {
    previewData = null;
    previewLoading = true;
    showPreviewModal = true;
    try {
      previewData = await api.archives.getInfo(archive.path);
    } catch (e) {
      error = e.message || "Failed to load archive contents";
      showPreviewModal = false;
    } finally {
      previewLoading = false;
    }
  }

  function closePreviewModal() {
    showPreviewModal = false;
    previewData = null;
  }

  function getEntryIcon(entry) {
    if (entry.is_directory) return "folder-fill";
    const ext = entry.name.split(".").pop()?.toLowerCase() || "";
    const iconMap = {
      pdf: "file-earmark-pdf",
      doc: "file-earmark-word",
      docx: "file-earmark-word",
      xls: "file-earmark-excel",
      xlsx: "file-earmark-excel",
      ppt: "file-earmark-ppt",
      pptx: "file-earmark-ppt",
      jpg: "file-earmark-image",
      jpeg: "file-earmark-image",
      png: "file-earmark-image",
      gif: "file-earmark-image",
      svg: "file-earmark-image",
      mp3: "file-earmark-music",
      wav: "file-earmark-music",
      mp4: "file-earmark-play",
      avi: "file-earmark-play",
      zip: "file-earmark-zip",
      rar: "file-earmark-zip",
      "7z": "file-earmark-zip",
      tar: "file-earmark-zip",
      gz: "file-earmark-zip",
      js: "file-earmark-code",
      ts: "file-earmark-code",
      py: "file-earmark-code",
      rs: "file-earmark-code",
      html: "file-earmark-code",
      css: "file-earmark-code",
      json: "file-earmark-code",
      xml: "file-earmark-code",
      txt: "file-earmark-text",
      md: "file-earmark-text",
    };
    return iconMap[ext] || "file-earmark";
  }

  // Sort entries: directories first, then files, both alphabetically
  function getSortedEntries(entries) {
    if (!entries) return [];
    const dirs = entries
      .filter((e) => e.is_directory)
      .sort((a, b) => a.name.localeCompare(b.name));
    const files = entries
      .filter((e) => !e.is_directory)
      .sort((a, b) => a.name.localeCompare(b.name));
    return [...dirs, ...files];
  }
</script>

<div class="archives-view p-6">
  <div class="flex justify-between items-center mb-6">
    <div>
      <h1 class="text-2xl font-bold flex items-center gap-2">
        <i class="bi bi-file-earmark-zip text-primary"></i>
        Archive Management
      </h1>
      <p class="text-base-content/60 mt-1">
        Create, extract, and manage archive files
      </p>
    </div>
    <button class="btn btn-primary" onclick={() => openCreateModal()}>
      <i class="bi bi-plus-lg"></i>
      Create Archive
    </button>
  </div>

  {#if error}
    <div class="alert alert-error mb-4">
      <i class="bi bi-exclamation-circle-fill"></i>
      <span>{error}</span>
      <button
        class="btn btn-ghost btn-sm"
        onclick={() => (error = "")}
        aria-label="Dismiss"
      >
        <i class="bi bi-x"></i>
      </button>
    </div>
  {/if}

  {#if success}
    <div class="alert alert-success mb-4">
      <i class="bi bi-check-circle-fill"></i>
      <span>{success}</span>
    </div>
  {/if}

  <!-- Breadcrumb -->
  {#if currentPath}
    <div class="breadcrumbs text-sm mb-4">
      <ul>
        <li>
          <button onclick={() => loadArchives("")} class="link link-hover">
            <i class="bi bi-house"></i> Root
          </button>
        </li>
        {#each currentPath.split("/").filter(Boolean) as segment, i}
          <li>
            <button
              onclick={() =>
                loadArchives(
                  currentPath
                    .split("/")
                    .slice(0, i + 1)
                    .join("/")
                )}
              class="link link-hover"
            >
              {segment}
            </button>
          </li>
        {/each}
      </ul>
    </div>
  {/if}

  {#if loading}
    <div class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
  {:else if archives.length === 0}
    <div class="card bg-base-200">
      <div class="card-body text-center py-12">
        <i class="bi bi-archive text-5xl text-base-content/30 mb-4"></i>
        <h3 class="text-lg font-medium">No Archives Found</h3>
        <p class="text-base-content/60">No archive files in this directory</p>
        <button
          class="btn btn-primary btn-sm mt-4"
          onclick={() => openCreateModal()}
        >
          Create Your First Archive
        </button>
      </div>
    </div>
  {:else}
    <div class="card bg-base-200">
      <div class="card-body">
        <div class="overflow-x-auto">
          <table class="table">
            <thead>
              <tr>
                <th>Name</th>
                <th>Format</th>
                <th>Size</th>
                <th>Modified</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              {#each archives as archive}
                <tr class="hover">
                  <td>
                    <div class="flex items-center gap-2">
                      <i
                        class="bi bi-{getFormatIcon(
                          archive.format
                        )} text-lg text-warning"
                      ></i>
                      <span class="font-medium">{archive.name}</span>
                    </div>
                  </td>
                  <td>
                    <span class="badge badge-ghost"
                      >{archive.format.toUpperCase()}</span
                    >
                  </td>
                  <td>{formatBytes(archive.size)}</td>
                  <td>
                    {#if archive.modified}
                      {new Date(archive.modified).toLocaleDateString()}
                    {:else}
                      -
                    {/if}
                  </td>
                  <td>
                    <div class="flex gap-1">
                      <button
                        class="btn btn-ghost btn-xs"
                        onclick={() => openPreviewModal(archive)}
                        aria-label="Preview archive contents"
                      >
                        <i class="bi bi-eye"></i>
                      </button>
                      <button
                        class="btn btn-ghost btn-xs"
                        onclick={() => openExtractModal(archive)}
                        aria-label="Extract archive"
                      >
                        <i class="bi bi-box-arrow-up"></i>
                      </button>
                      <button
                        class="btn btn-ghost btn-xs text-error"
                        onclick={() => deleteArchive(archive)}
                        aria-label="Delete archive"
                      >
                        <i class="bi bi-trash"></i>
                      </button>
                    </div>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  {/if}

  <!-- Supported Formats -->
  <div class="card bg-base-200 mt-6">
    <div class="card-body">
      <h3 class="card-title text-sm mb-4">
        <i class="bi bi-info-circle"></i>
        Supported Formats
      </h3>
      <div class="flex flex-wrap gap-2">
        {#each formats as format}
          <div class="tooltip" data-tip={format.name}>
            <span class="badge badge-outline gap-1">
              <i class="bi bi-{format.supports_password ? 'lock' : 'unlock'}"
              ></i>
              {format.extension}
            </span>
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>

<!-- Create Archive Modal -->
{#if showCreateModal}
  <div class="modal modal-open">
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={closeCreateModal}
      onkeydown={(e) => e.key === "Escape" && closeCreateModal()}
    ></div>
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">
        <i class="bi bi-plus-lg text-primary"></i>
        Create Archive
      </h3>

      <div class="space-y-4">
        <div class="form-control">
          <UIInput
  label="Archive Name *"
  bind:value={createForm.archiveName}
  placeholder="my-archive"
/>
        </div>

        <div class="form-control">
          <UISelect
  label="Format"
  bind:value={createForm.format}
/>
        </div>

        <div class="form-control">
          <label class="label" for="compression-level">
            <span class="label-text"
              >Compression Level: {createForm.compressionLevel}</span
            >
          </label>
          <input
            id="compression-level"
            type="range"
            class="range range-primary"
            min="1"
            max="9"
            bind:value={createForm.compressionLevel}
          />
          <div class="flex justify-between text-xs text-base-content/60 px-1">
            <span>Fast</span>
            <span>Balanced</span>
            <span>Best</span>
          </div>
        </div>

        {#if formats.find((f) => f.id === createForm.format)?.supports_password}
          <div class="form-control">
            <UIInput
  label="Password (optional)"
  type="password"
  bind:value={createForm.password}
  placeholder="Leave empty for no password"
/>
          </div>
        {/if}

        <div class="form-control">
          <UIInput
  label="Destination (optional)"
  bind:value={createForm.destination}
  placeholder="Current directory"
/>
        </div>

        <div class="form-control">
          <label class="label" for="archive-files">
            <span class="label-text"
              >Files to Archive (comma-separated paths)</span
            >
          </label>
          <textarea
            id="archive-files"
            class="textarea textarea-bordered w-full h-24"
            bind:value={createForm.files}
            placeholder="path/to/file1.txt, path/to/folder"
          ></textarea>
          <div class="label">
            <span class="label-text-alt text-base-content/60">
              Enter file paths separated by commas
            </span>
          </div>
        </div>
      </div>

      <div class="modal-action">
        <button class="btn btn-ghost" onclick={closeCreateModal}>Cancel</button>
        <button
          class="btn btn-primary"
          onclick={createArchive}
          disabled={creating}
        >
          {#if creating}
            <span class="loading loading-spinner loading-sm"></span>
          {/if}
          Create Archive
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Extract Archive Modal -->
{#if showExtractModal}
  <div class="modal modal-open">
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={closeExtractModal}
      onkeydown={(e) => e.key === "Escape" && closeExtractModal()}
    ></div>
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">
        <i class="bi bi-box-arrow-up text-primary"></i>
        Extract Archive
      </h3>

      <div class="space-y-4">
        <div class="form-control">
          <div class="label">
            <span class="label-text">Archive</span>
          </div>
          <div class="flex items-center gap-2 p-3 bg-base-300 rounded-lg">
            <i class="bi bi-file-earmark-zip text-warning"></i>
            <span class="font-mono text-sm">{extractForm.archivePath}</span>
          </div>
        </div>

        <div class="form-control">
          <UIInput
  label="Extract To"
  bind:value={extractForm.destination}
  placeholder="Destination folder"
/>
        </div>

        <div class="form-control">
          <UIInput
  label="Password (if protected)"
  type="password"
  bind:value={extractForm.password}
  placeholder="Leave empty if not password-protected"
/>
        </div>

        <div class="form-control">
          <label class="label cursor-pointer justify-start gap-3">
            <input
              type="checkbox"
              class="checkbox checkbox-primary"
              bind:checked={extractForm.flatten}
            />
            <span class="label-text">Flatten directory structure</span>
          </label>
        </div>
      </div>

      <div class="modal-action">
        <button class="btn btn-ghost" onclick={closeExtractModal}>Cancel</button
        >
        <button
          class="btn btn-primary"
          onclick={extractArchive}
          disabled={extracting}
        >
          {#if extracting}
            <span class="loading loading-spinner loading-sm"></span>
          {/if}
          Extract
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Preview Archive Modal -->
{#if showPreviewModal}
  <div class="modal modal-open">
    <div
      class="modal-backdrop"
      role="button"
      tabindex="-1"
      onclick={closePreviewModal}
      onkeydown={(e) => e.key === "Escape" && closePreviewModal()}
    ></div>
    <div class="modal-box max-w-4xl">
      <div class="flex justify-between items-center mb-4">
        <h3 class="font-bold text-lg flex items-center gap-2">
          <i class="bi bi-file-earmark-zip text-warning"></i>
          Archive Contents
        </h3>
        <button
          class="btn btn-ghost btn-sm btn-circle"
          onclick={closePreviewModal}
          aria-label="Close"
        >
          <i class="bi bi-x-lg"></i>
        </button>
      </div>

      {#if previewLoading}
        <div class="flex justify-center py-12">
          <span class="loading loading-spinner loading-lg"></span>
        </div>
      {:else if previewData}
        <!-- Archive Info Header -->
        <div class="stats bg-base-200 w-full mb-4">
          <div class="stat">
            <div class="stat-title">Files</div>
            <div class="stat-value text-primary">{previewData.file_count}</div>
          </div>
          <div class="stat">
            <div class="stat-title">Uncompressed</div>
            <div class="stat-value text-sm">
              {formatBytes(previewData.size_bytes)}
            </div>
          </div>
          <div class="stat">
            <div class="stat-title">Compressed</div>
            <div class="stat-value text-sm">
              {formatBytes(previewData.compressed_size)}
            </div>
          </div>
          <div class="stat">
            <div class="stat-title">Ratio</div>
            <div class="stat-value text-sm text-success">
              {previewData.size_bytes > 0
                ? Math.round(
                    (1 - previewData.compressed_size / previewData.size_bytes) *
                      100
                  ) + "%"
                : "N/A"}
            </div>
          </div>
        </div>

        <!-- Archive Contents List -->
        {#if previewData.entries && previewData.entries.length > 0}
          <div
            class="max-h-96 overflow-y-auto border border-base-300 rounded-lg"
          >
            <table class="table table-sm table-zebra">
              <thead class="sticky top-0 bg-base-200">
                <tr>
                  <th>Name</th>
                  <th class="w-24 text-right">Size</th>
                  <th class="w-24 text-right">Compressed</th>
                  <th class="w-32">Modified</th>
                </tr>
              </thead>
              <tbody>
                {#each getSortedEntries(previewData.entries) as entry}
                  <tr class={entry.is_directory ? "opacity-60" : ""}>
                    <td>
                      <div class="flex items-center gap-2">
                        <i
                          class="bi bi-{getEntryIcon(entry)} {entry.is_directory
                            ? 'text-warning'
                            : 'opacity-70'}"
                        ></i>
                        <span
                          class="font-mono text-sm truncate max-w-md"
                          title={entry.path}
                        >
                          {entry.path}
                        </span>
                      </div>
                    </td>
                    <td class="text-right font-mono text-xs">
                      {entry.is_directory ? "-" : formatBytes(entry.size)}
                    </td>
                    <td class="text-right font-mono text-xs">
                      {entry.is_directory
                        ? "-"
                        : formatBytes(entry.compressed_size)}
                    </td>
                    <td class="text-xs">
                      {#if entry.modified}
                        {new Date(entry.modified).toLocaleString()}
                      {:else}
                        -
                      {/if}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {:else}
          <div class="text-center py-8 text-base-content/60">
            <i class="bi bi-folder2-open text-4xl mb-2"></i>
            <p>No entries found in archive</p>
          </div>
        {/if}
      {/if}

      <div class="modal-action">
        <button class="btn btn-ghost" onclick={closePreviewModal}>Close</button>
        {#if previewData}
          <button
            class="btn btn-primary"
            onclick={() => {
              closePreviewModal();
              openExtractModal({ path: previewData.path });
            }}
          >
            <i class="bi bi-box-arrow-up"></i>
            Extract
          </button>
        {/if}
      </div>
    </div>
  </div>
{/if}
