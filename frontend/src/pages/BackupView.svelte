<script>
  import { files, currentPath } from "../stores/ui";
  import { success, error } from "../stores/toast";
  import api from "../lib/api";
  import Icon from "../components/ui/Icon.svelte";
  import Button from "../components/ui/Button.svelte";
  import ProgressBar from "../components/ui/ProgressBar.svelte";
  import InfoCard from "../components/ui/InfoCard.svelte";
  import PageHeader from "../components/ui/PageHeader.svelte";

  // Note: Using dynamic import for JSZip since it's external library
  let JSZip = null;
  let loadingZipLib = false;

  let exporting = false;
  let exportProgress = { current: 0, total: 0 };
  let importing = false;
  let importProgress = { current: 0, total: 0 };
  let lastBackupDate = null;

  // Load last backup info from localStorage
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
      // @ts-ignore - Dynamic import of external CDN library without type definitions
      const module = await import("https://cdn.jsdelivr.net/npm/jszip@3.10.1/dist/jszip.min.js");
      // @ts-ignore - window.JSZip available after script load
      JSZip = module.default || window.JSZip;
      loadingZipLib = false;
      return true;
    } catch (e) {
      console.error("Failed to load JSZip:", e);
      error("Failed to load ZIP library");
      loadingZipLib = false;
      return false;
    }
  }

  async function exportAllFiles() {
    if (exporting) return;

    // Load JSZip library
    if (!(await loadJSZipLib())) {
      error("ZIP library not available");
      return;
    }

    exporting = true;
    exportProgress = { current: 0, total: 0 };

    try {
      // Get all files recursively
      const allFiles = await getAllFilesRecursive("/");

      if (allFiles.length === 0) {
        error("No files to export");
        exporting = false;
        return;
      }

      exportProgress.total = allFiles.length;
      success(`Exporting ${allFiles.length} files...`);

      const zip = new JSZip();

      // Add files to ZIP
      for (const fileInfo of allFiles) {
        try {
          const response = await api.downloadFile(fileInfo.fullPath);
          const blob = await response.blob();

          // Add to ZIP with full path
          zip.file(fileInfo.fullPath, blob);

          exportProgress.current++;
        } catch (e) {
          console.error(`Failed to fetch ${fileInfo.fullPath}:`, e);
        }
      }

      // Generate ZIP
      success("Generating backup archive...");
      const zipBlob = await zip.generateAsync(
        {
          type: "blob",
          compression: "DEFLATE",
          compressionOptions: { level: 6 },
        },
        (metadata) => {
          // Progress callback
          console.log(`ZIP progress: ${metadata.percent.toFixed(2)}%`);
        }
      );

      // Download ZIP
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

      // Save backup date
      localStorage.setItem("last_backup_date", Date.now().toString());
      lastBackupDate = new Date();

      success(`Backup created: ${filename} (${formatBytes(zipBlob.size)})`);
    } catch (e) {
      console.error("Export failed:", e);
      error("Failed to create backup");
    } finally {
      exporting = false;
    }
  }

  async function exportCurrentFolder() {
    if (exporting) return;

    if (!(await loadJSZipLib())) {
      error("ZIP library not available");
      return;
    }

    exporting = true;
    exportProgress = { current: 0, total: 0 };

    try {
      const currentFiles = $files.filter((f) => !f.is_dir);

      if (currentFiles.length === 0) {
        error("No files to export in current folder");
        exporting = false;
        return;
      }

      exportProgress.total = currentFiles.length;
      success(`Exporting ${currentFiles.length} files from current folder...`);

      const zip = new JSZip();

      for (const file of currentFiles) {
        try {
          const response = await api.downloadFile(file.name);
          const blob = await response.blob();
          zip.file(file.name, blob);
          exportProgress.current++;
        } catch (e) {
          console.error(`Failed to fetch ${file.name}:`, e);
        }
      }

      const zipBlob = await zip.generateAsync({
        type: "blob",
        compression: "DEFLATE",
      });

      const folderName = $currentPath.replace(/\//g, "_") || "root";
      const timestamp = new Date()
        .toISOString()
        .replace(/[:.]/g, "-")
        .slice(0, -5);
      const filename = `syncspace-${folderName}-${timestamp}.zip`;

      const url = URL.createObjectURL(zipBlob);
      const a = document.createElement("a");
      a.href = url;
      a.download = filename;
      a.click();
      URL.revokeObjectURL(url);

      success(`Exported: ${filename}`);
    } catch (e) {
      console.error("Export failed:", e);
      error("Failed to export folder");
    } finally {
      exporting = false;
    }
  }

  async function handleImportZip(event) {
    const file = event.target.files?.[0];
    if (!file) return;

    if (!(await loadJSZipLib())) {
      error("ZIP library not available");
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

          // Upload to current path
          await api.uploadFile(filePath, fileData);

          importProgress.current++;
        } catch (e) {
          console.error(`Failed to import ${filePath}:`, e);
        }
      }

      success(`Imported ${importProgress.current} files successfully!`);

      // Reload files
      window.location.reload();
    } catch (e) {
      console.error("Import failed:", e);
      error("Failed to import backup");
    } finally {
      importing = false;
      event.target.value = ""; // Reset input
    }
  }

  async function getAllFilesRecursive(path) {
    const result = [];

    try {
      const response = await api.listFiles(path);
      const fileList = await response.json();

      for (const item of fileList) {
        if (item.is_dir) {
          // Recursively fetch subdirectory
          const subPath = path + item.name + "/";
          const subFiles = await getAllFilesRecursive(subPath);
          result.push(...subFiles);
        } else {
          result.push({
            name: item.name,
            fullPath: (path + item.name).replace(/^\//, ""), // Remove leading slash
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

  function getProgressPercent(progress) {
    if (progress.total === 0) return 0;
    return Math.round((progress.current / progress.total) * 100);
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
  <!-- Crystal Glass Header -->
  <PageHeader 
    title="Backup & Export"
    subtitle="Create backups or export your files"
    icon="cloud-download"
    gradient="green"
  />

  <div class="backup-cards">
    <!-- Last Backup Info -->
    <InfoCard 
      variant={lastBackupDate ? "glass" : "bordered"}
      title="Last Backup"
      description={formatDate(lastBackupDate)}
    >
      <div slot="content">
        {#if lastBackupDate}
          <p class="info-hint">{getTimeSinceBackup()}</p>
        {:else}
          <p class="info-hint">No backup created yet</p>
        {/if}
      </div>
    </InfoCard>

    <!-- Export All -->
    <div class="action-card">
      <div class="card-icon">📦</div>
      <h3 class="card-title">Export All Files</h3>
      <p class="card-description">
        Download all files and folders as a single ZIP archive.
      </p>
      <Button
        variant="primary"
        fullWidth
        onClick={exportAllFiles}
        disabled={exporting || importing}
      >
        {#if exporting && exportProgress.total > 0}
          <Icon name="hourglass-split" size={16} />
          Exporting... {exportProgress.current}/{exportProgress.total}
        {:else if exporting}
          <Icon name="hourglass-split" size={16} />
          Preparing...
        {:else}
          <Icon name="download" size={16} />
          Export All
        {/if}
      </Button>
      {#if exporting && exportProgress.total > 0}
        <ProgressBar 
          value={(exportProgress.current / exportProgress.total) * 100} 
          variant="primary"
          showLabel={true}
        />
      {/if}
    </div>

    <!-- Export Current Folder -->
    <div class="action-card">
      <div class="card-icon">📁</div>
      <h3 class="card-title">Export Current Folder</h3>
      <p class="card-description">
        Export only files in the current folder (non-recursive).
      </p>
      <Button
        variant="secondary"
        fullWidth
        onClick={exportCurrentFolder}
        disabled={exporting || importing}
      >
        <Icon name="folder-open" size={16} />
        Export Folder
      </Button>
    </div>

    <!-- Import Backup -->
    <div class="action-card">
      <div class="card-icon">📥</div>
      <h3 class="card-title">Import Backup</h3>
      <p class="card-description">Restore files from a ZIP backup archive.</p>
      <label class="btn-import-label">
        <input
          type="file"
          accept=".zip"
          onchange={handleImportZip}
          disabled={exporting || importing}
          style="display: none;"
        />
        <Button variant="secondary" fullWidth disabled={exporting || importing}>
          {#if importing}
            <Icon name="hourglass-split" size={16} />
            Importing... {importProgress.current}/{importProgress.total}
          {:else}
            <Icon name="upload" size={16} />
            Import ZIP
          {/if}
        </Button>
      </label>
      {#if importing && importProgress.total > 0}
        <ProgressBar 
          value={(importProgress.current / importProgress.total) * 100} 
          variant="success"
          showLabel={true}
        />
      {/if}
    </div>
  </div>

  {#if exporting || importing}
    <div class="progress-card">
      <div class="progress-header">
        <span class="progress-label">
          {#if exporting}
            Exporting files...
          {:else}
            Importing files...
          {/if}
        </span>
        <span class="progress-text">
          {exporting ? exportProgress.current : importProgress.current} /
          {exporting ? exportProgress.total : importProgress.total}
        </span>
      </div>
      <div class="progress-bar">
        <div
          class="progress-fill"
          style="width: {getProgressPercent(
            exporting ? exportProgress : importProgress
          )}%"
        ></div>
      </div>
    </div>
  {/if}

  <div class="info-section">
    <h3 class="section-title">ℹ️ Information</h3>
    <ul class="info-list">
      <li>Backups are created as ZIP archives for easy portability</li>
      <li>Export All includes all folders and subfolders recursively</li>
      <li>Export Folder only includes files in the current directory</li>
      <li>
        Import will upload all files from the ZIP to your current location
      </li>
      <li>Large backups may take several minutes to complete</li>
      <li>Backups are not encrypted - store them securely</li>
    </ul>
  </div>
</div>

<style>
  .backup-view {
    padding: 0;
    max-width: 1200px;
    margin: 0 auto;
  }

  .info-hint {
    margin: 0;
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .action-card {
    background: var(--md-sys-color-surface-container-low);
    padding: 24px;
    border-radius: 20px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
    transition: all 0.2s ease;
  }

  .action-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
  }

  .card-icon {
    font-size: 48px;
    margin-bottom: 16px;
  }

  .card-title {
    margin: 0 0 8px 0;
    font-size: 18px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
  }

  .card-description {
    margin: 0 0 20px 0;
    font-size: 14px;
    line-height: 1.5;
    color: var(--md-sys-color-on-surface-variant);
  }

  .btn-import-label {
    display: block;
    cursor: pointer;
  }

  .progress-card {
    background: var(--md-sys-color-surface-container-low);
    padding: 24px;
    border-radius: 16px;
    margin-bottom: 24px;
  }

  .progress-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .progress-label {
    font-size: 14px;
    font-weight: 500;
    color: var(--md-sys-color-primary);
  }

  .progress-text {
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .progress-bar {
    width: 100%;
    height: 8px;
    background: var(--md-sys-color-surface-container-high);
    border-radius: 4px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(
      90deg,
      var(--md-sys-color-primary),
      var(--md-sys-color-secondary)
    );
    border-radius: 4px;
    transition: width 0.3s ease;
  }

  .info-section {
    background: var(--md-sys-color-surface-container-lowest);
    padding: 24px;
    border-radius: 16px;
    border-left: 4px solid var(--md-sys-color-primary);
    margin: 0 32px;
  }

  .section-title {
    margin: 0 0 16px 0;
    font-size: 16px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
  }

  .info-list {
    margin: 0;
    padding-left: 24px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .info-list li {
    margin-bottom: 8px;
    font-size: 14px;
    line-height: 1.5;
  }

  /* Mobile Responsive */
  @media (max-width: 768px) {
    .backup-view {
      padding: 16px;
    }

    .backup-cards {
      grid-template-columns: 1fr;
    }
  }
</style>
