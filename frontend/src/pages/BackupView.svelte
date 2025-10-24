<script>
  import { onMount } from "svelte";
  import { success, error as errorToast } from "../stores/toast";
  import api from "../lib/api";

  let JSZip = null;
  let loadingZipLib = false;
  let exporting = false;
  let exportProgress = { current: 0, total: 0 };
  let importing = false;
  let importProgress = { current: 0, total: 0 };
  let lastBackupDate = null;
  let importFileInput;

  $: {
    const stored = localStorage.getItem("last_backup_date");
    if (stored) {
      lastBackupDate = new Date(parseInt(stored));
    }
  }

  async function loadJSZipLib() {
    if (JSZip) return true;
    if (loadingZipLib) return false;

    loadingZipLib = true;
    try {
      // @ts-ignore - Dynamic CDN import
      const module = await import(
        "https://cdn.jsdelivr.net/npm/jszip@3.10.1/dist/jszip.min.js"
      );
      // @ts-ignore - JSZip from CDN
      JSZip = module.default || window.JSZip;
      loadingZipLib = false;
      return true;
    } catch (e) {
      errorToast("Failed to load ZIP library");
      loadingZipLib = false;
      return false;
    }
  }

  async function exportAllFiles() {
    if (exporting) return;

    if (!(await loadJSZipLib())) {
      errorToast("ZIP library not available");
      return;
    }

    exporting = true;
    exportProgress = { current: 0, total: 0 };

    try {
      const allFiles = await getAllFilesRecursive("/");

      if (allFiles.length === 0) {
        errorToast("No files to export");
        exporting = false;
        return;
      }

      exportProgress.total = allFiles.length;
      success(`Exporting ${allFiles.length} files...`);

      const zip = new JSZip();

      for (const fileInfo of allFiles) {
        try {
          const blob = await api.files.download(fileInfo.fullPath);
          zip.file(fileInfo.fullPath, blob);
          exportProgress.current++;
        } catch (e) {
          console.error(`Failed to fetch ${fileInfo.fullPath}:`, e);
        }
      }

      success("Generating backup archive...");
      const zipBlob = await zip.generateAsync({
        type: "blob",
        compression: "DEFLATE",
        compressionOptions: { level: 6 },
      });

      const timestamp = new Date()
        .toISOString()
        .replace(/[:.]/g, "-")
        .slice(0, -5);
      const filename = `syncspace-backup-${timestamp}.zip`;

      const url = URL.createObjectURL(zipBlob);
      const a = document.createElement("a");
      a.href = url;
      a.download = filename;
      a.click();
      URL.revokeObjectURL(url);

      localStorage.setItem("last_backup_date", Date.now().toString());
      lastBackupDate = new Date();

      success(`Backup created: ${filename} (${formatBytes(zipBlob.size)})`);
    } catch (e) {
      errorToast("Failed to create backup");
    } finally {
      exporting = false;
    }
  }

  async function handleImportZip(event) {
    const file = event.target.files?.[0];
    if (!file) return;

    if (!(await loadJSZipLib())) {
      errorToast("ZIP library not available");
      return;
    }

    importing = true;
    importProgress = { current: 0, total: 0 };

    try {
      success("Reading ZIP file...");
      const zip = await JSZip.loadAsync(file);

      const filePaths = Object.keys(zip.files).filter(
        (path) => !zip.files[path].dir
      );
      importProgress.total = filePaths.length;

      success(`Importing ${filePaths.length} files...`);

      for (const filePath of filePaths) {
        try {
          const fileData = await zip.files[filePath].async("blob");
          const cleanPath = filePath.replace(/^\/+/, "");
          await api.files.upload(cleanPath, fileData);
          importProgress.current++;
        } catch (e) {
          console.error(`Failed to import ${filePath}:`, e);
        }
      }

      success(`Imported ${importProgress.current} files successfully!`);
      window.location.reload();
    } catch (e) {
      errorToast("Failed to import backup");
    } finally {
      importing = false;
      event.target.value = "";
    }
  }

  async function getAllFilesRecursive(path) {
    const result = [];

    try {
      const backendPath = path.replace(/^\/+|\/+$/g, "");
      const fileList = await api.files.list(backendPath);

      for (const item of fileList) {
        if (item.is_dir) {
          const subPath = path + item.name + "/";
          const subFiles = await getAllFilesRecursive(subPath);
          result.push(...subFiles);
        } else {
          result.push({
            name: item.name,
            fullPath: (path + item.name).replace(/^\//, ""),
            size: item.size,
          });
        }
      }
    } catch (e) {
      console.error(`Failed to list ${path}:`, e);
    }

    return result;
  }

  function formatBytes(bytes) {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return (bytes / Math.pow(k, i)).toFixed(2) + " " + sizes[i];
  }

  function formatDate(date) {
    if (!date) return "Never";
    return date.toLocaleDateString() + " at " + date.toLocaleTimeString();
  }

  function getTimeSinceBackup() {
    if (!lastBackupDate) return null;

    const now = new Date();
    const diffMs = now.getTime() - lastBackupDate.getTime();
    const diffDays = Math.floor(diffMs / 86400000);
    const diffHours = Math.floor(diffMs / 3600000);

    if (diffDays > 0) return `${diffDays} day${diffDays > 1 ? "s" : ""} ago`;
    if (diffHours > 0)
      return `${diffHours} hour${diffHours > 1 ? "s" : ""} ago`;
    return "Less than an hour ago";
  }
</script>

<div class="backup-view">
  <!-- Last Backup Info -->
  {#if lastBackupDate}
    <div class="alert alert-success mb-6">
      <i class="bi bi-check-circle-fill text-2xl"></i>
      <div>
        <h3 class="font-bold">Last Backup</h3>
        <div class="text-xs">
          {formatDate(lastBackupDate)} • {getTimeSinceBackup()}
        </div>
      </div>
    </div>
  {/if}

  <!-- Action Cards -->
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
    <!-- Export All -->
    <div
      class="card bg-base-100 border border-base-300 shadow-sm hover:shadow-md transition-shadow"
    >
      <div class="card-body items-center text-center">
        <div class="text-6xl mb-4">
          <i class="bi bi-box-arrow-down text-primary"></i>
        </div>
        <h2 class="card-title">Export All Files</h2>
        <p class="text-sm opacity-70 mb-4">
          Download everything as ZIP archive
        </p>
        <button
          class="btn btn-primary w-full gap-2"
          on:click={exportAllFiles}
          disabled={exporting || importing}
        >
          {#if exporting && exportProgress.total > 0}
            <span class="loading loading-spinner"></span>
            {exportProgress.current}/{exportProgress.total}
          {:else if exporting}
            <span class="loading loading-spinner"></span>
            Preparing...
          {:else}
            <i class="bi bi-download"></i>
            Export All
          {/if}
        </button>
        {#if exporting && exportProgress.total > 0}
          <progress
            class="progress progress-primary w-full mt-2"
            value={exportProgress.current}
            max={exportProgress.total}
          ></progress>
        {/if}
      </div>
    </div>

    <!-- Import Backup -->
    <div
      class="card bg-base-100 border border-base-300 shadow-sm hover:shadow-md transition-shadow"
    >
      <div class="card-body items-center text-center">
        <div class="text-6xl mb-4">
          <i class="bi bi-box-arrow-in-up text-success"></i>
        </div>
        <h2 class="card-title">Import Backup</h2>
        <p class="text-sm opacity-70 mb-4">Restore files from ZIP archive</p>
        <input
          type="file"
          accept=".zip"
          bind:this={importFileInput}
          on:change={handleImportZip}
          class="hidden"
        />
        <button
          class="btn btn-success w-full gap-2"
          on:click={() => importFileInput?.click()}
          disabled={exporting || importing}
        >
          {#if importing}
            <span class="loading loading-spinner"></span>
            {importProgress.current}/{importProgress.total}
          {:else}
            <i class="bi bi-upload"></i>
            Import ZIP
          {/if}
        </button>
        {#if importing && importProgress.total > 0}
          <progress
            class="progress progress-success w-full mt-2"
            value={importProgress.current}
            max={importProgress.total}
          ></progress>
        {/if}
      </div>
    </div>

    <!-- Info Card -->
    <div
      class="card bg-info text-info-content border border-base-300 shadow-sm"
    >
      <div class="card-body">
        <h2 class="card-title">
          <i class="bi bi-info-circle-fill"></i>
          How it works
        </h2>
        <ul class="text-sm space-y-2">
          <li>✓ All files exported as ZIP</li>
          <li>✓ Folder structure preserved</li>
          <li>✓ Compression enabled</li>
          <li>✓ Safe & portable</li>
        </ul>
      </div>
    </div>
  </div>

  <!-- Important Notes -->
  <div class="card bg-warning text-warning-content">
    <div class="card-body">
      <h2 class="card-title">
        <i class="bi bi-exclamation-triangle-fill"></i>
        Important Notes
      </h2>
      <ul class="list-disc list-inside space-y-1 text-sm">
        <li>Backups are not encrypted - store securely</li>
        <li>Large backups may take several minutes</li>
        <li>Import will upload all files from ZIP</li>
        <li>Existing files may be overwritten</li>
        <li>JSZip library loaded on demand</li>
      </ul>
    </div>
  </div>
</div>

<style>
  .backup-view {
    padding: 1.5rem;
    min-height: calc(100vh - 200px);
  }
</style>
