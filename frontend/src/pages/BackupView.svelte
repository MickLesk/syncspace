<script><script>

  import { onMount } from "svelte";  import { onMount } from "svelte";

  import { success, error as errorToast } from "../stores/toast";  import { success, error as errorToast } from "../stores/toast";

  import api from "../lib/api";  import api from "../lib/api";

  import BackupScheduleManager from "../components/BackupScheduleManager.svelte";  import BackupScheduleManager from "../components/BackupScheduleManager.svelte";

  import BackupVerificationPanel from "../components/BackupVerificationPanel.svelte";  import BackupVerificationPanel from "../components/BackupVerificationPanel.svelte";



  let activeTab = $state('backups'); // 'backups', 'schedules', 'export'  let activeTab = $state('export'); // 'export', 'backups', 'schedules'

  let backups = $state([]);  let backups = $state([]);

  let selectedBackup = $state(null);  let selectedBackup = $state(null);

  let loadingBackups = $state(false);  let loadingBackups = $state(false);

  

  onMount(() => {  let JSZip = null;

    if (activeTab === 'backups') {  let loadingZipLib = false;

      loadBackups();  let exporting = false;

    }  let exportProgress = { current: 0, total: 0 };

  });  let importing = false;

  let importProgress = { current: 0, total: 0 };

  async function loadBackups() {  let lastBackupDate = null;

    loadingBackups = true;  let importFileInput;

    try {

      const response = await fetch('/api/backups', {  $: {

        headers: {    const stored = localStorage.getItem("last_backup_date");

          'Authorization': `Bearer ${localStorage.getItem('authToken')}`    if (stored) {

        }      lastBackupDate = new Date(parseInt(stored));

      });    }

  }

      if (!response.ok) throw new Error('Failed to load backups');  

  onMount(() => {

      backups = await response.json();    if (activeTab === 'backups') {

    } catch (err) {      loadBackups();

      errorToast('Failed to load backups');    }

      console.error(err);  });

    } finally {  

      loadingBackups = false;  async function loadBackups() {

    }    loadingBackups = true;

  }    try {

      const response = await fetch('/api/backups', {

  async function createBackup(type = 'full') {        headers: {

    try {          'Authorization': `Bearer ${localStorage.getItem('authToken')}`

      const response = await fetch('/api/backups/create', {        }

        method: 'POST',      });

        headers: {      

          'Content-Type': 'application/json',      if (!response.ok) throw new Error('Failed to load backups');

          'Authorization': `Bearer ${localStorage.getItem('authToken')}`      

        },      backups = await response.json();

        body: JSON.stringify({    } catch (err) {

          backup_type: type,      errorToast('Failed to load backups');

          include_versions: true      console.error(err);

        })    } finally {

      });      loadingBackups = false;

    }

      if (!response.ok) throw new Error('Failed to create backup');  }

  

      success('Backup started! Check back in a few moments.');  async function createBackup(type = 'full') {

      setTimeout(() => loadBackups(), 2000);    try {

    } catch (err) {      const response = await fetch('/api/backups/create', {

      errorToast('Failed to create backup');        method: 'POST',

      console.error(err);        headers: {

    }          'Content-Type': 'application/json',

  }          'Authorization': `Bearer ${localStorage.getItem('authToken')}`

        },

  async function deleteBackup(backupId) {        body: JSON.stringify({

    if (!confirm('Are you sure you want to delete this backup?')) return;          backup_type: type,

          include_versions: true

    try {        })

      const response = await fetch(`/api/backups/${backupId}`, {      });

        method: 'DELETE',      

        headers: {      if (!response.ok) throw new Error('Failed to create backup');

          'Authorization': `Bearer ${localStorage.getItem('authToken')}`      

        }      success('Backup started! Check back in a few moments.');

      });      setTimeout(() => loadBackups(), 2000);

    } catch (err) {

      if (!response.ok) throw new Error('Failed to delete backup');      errorToast('Failed to create backup');

      console.error(err);

      success('Backup deleted successfully');    }

      selectedBackup = null;  }

      await loadBackups();  

    } catch (err) {  async function deleteBackup(backupId) {

      errorToast('Failed to delete backup');    if (!confirm('Are you sure you want to delete this backup?')) return;

      console.error(err);    

    }    try {

  }      const response = await fetch(`/api/backups/${backupId}`, {

        method: 'DELETE',

  function formatBytes(bytes) {        headers: {

    if (!bytes || bytes === 0) return "0 B";          'Authorization': `Bearer ${localStorage.getItem('authToken')}`

    const k = 1024;        }

    const sizes = ["B", "KB", "MB", "GB"];      });

    const i = Math.floor(Math.log(bytes) / Math.log(k));      

    return (bytes / Math.pow(k, i)).toFixed(2) + " " + sizes[i];      if (!response.ok) throw new Error('Failed to delete backup');

  }      

      success('Backup deleted successfully');

  function formatDate(dateString) {      await loadBackups();

    if (!dateString) return "Never";    } catch (err) {

    return new Date(dateString).toLocaleString();      errorToast('Failed to delete backup');

  }      console.error(err);

    }

  function getStatusBadge(status) {  }

    const badges = {  

      'completed': 'badge-success',  function formatBytes(bytes) {

      'in_progress': 'badge-warning',    if (!bytes || bytes === 0) return "0 B";

      'failed': 'badge-error'    const k = 1024;

    };    const sizes = ["B", "KB", "MB", "GB"];

    return badges[status] || 'badge-ghost';    const i = Math.floor(Math.log(bytes) / Math.log(k));

  }    return (bytes / Math.pow(k, i)).toFixed(2) + " " + sizes[i];

  }

  $effect(() => {  

    if (activeTab === 'backups' && backups.length === 0) {  function formatDate(dateString) {

      loadBackups();    if (!dateString) return "Never";

    }    return new Date(dateString).toLocaleString();

  });  }

</script>

  async function loadJSZipLib() {

<div class="backup-view p-6">    if (JSZip) return true;

  <div class="mb-6">    if (loadingZipLib) return false;

    <h1 class="text-3xl font-bold mb-2">Backup & Restore</h1>

    <p class="text-base-content/70">Manage backups, schedules, and restore points</p>    loadingZipLib = true;

  </div>    try {

      // @ts-ignore - Dynamic CDN import

  <!-- Tabs -->      const module = await import(

  <div class="tabs tabs-boxed mb-6">        "https://cdn.jsdelivr.net/npm/jszip@3.10.1/dist/jszip.min.js"

    <button      );

      class="tab {activeTab === 'backups' ? 'tab-active' : ''}"      // @ts-ignore - JSZip from CDN

      onclick={() => activeTab = 'backups'}>      JSZip = module.default || window.JSZip;

      <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">      loadingZipLib = false;

        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4"/>      return true;

      </svg>    } catch (e) {

      Backups      errorToast("Failed to load ZIP library");

    </button>      loadingZipLib = false;

      return false;

    <button    }

      class="tab {activeTab === 'schedules' ? 'tab-active' : ''}"  }

      onclick={() => activeTab = 'schedules'}>  

      <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">  function getTimeSinceBackup() {

        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>    if (!lastBackupDate) return null;

      </svg>

      Schedules    const now = new Date();

    </button>    const diffMs = now.getTime() - lastBackupDate.getTime();

    const diffDays = Math.floor(diffMs / 86400000);

    <button    const diffHours = Math.floor(diffMs / 3600000);

      class="tab {activeTab === 'export' ? 'tab-active' : ''}"

      onclick={() => activeTab = 'export'}>    if (diffDays > 0) return `${diffDays} day${diffDays > 1 ? "s" : ""} ago`;

      <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">    if (diffHours > 0)

        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"/>      return `${diffHours} hour${diffHours > 1 ? "s" : ""} ago`;

      </svg>    return "Less than an hour ago";

      Export/Import  }

    </button>

  </div>  async function exportAllFiles() {

    if (exporting) return;

  <!-- Tab Content -->

  {#if activeTab === 'backups'}    if (!(await loadJSZipLib())) {

    <div class="backups-tab">      errorToast("ZIP library not available");

      <div class="flex justify-between items-center mb-4">      return;

        <h2 class="text-xl font-bold">Backup History</h2>    }

        <div class="flex gap-2">

          <button class="btn btn-sm btn-primary" onclick={() => createBackup('full')}>    exporting = true;

            <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">    exportProgress = { current: 0, total: 0 };

              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>

            </svg>    try {

            Full Backup      const allFiles = await getAllFilesRecursive("/");

          </button>

          <button class="btn btn-sm btn-outline" onclick={() => createBackup('database')}>      if (allFiles.length === 0) {

            Database        errorToast("No files to export");

          </button>        exporting = false;

          <button class="btn btn-sm btn-outline" onclick={() => createBackup('files')}>        return;

            Files      }

          </button>

        </div>      exportProgress.total = allFiles.length;

      </div>      success(`Exporting ${allFiles.length} files...`);



      {#if loadingBackups}      const zip = new JSZip();

        <div class="flex justify-center py-12">

          <span class="loading loading-spinner loading-lg"></span>      for (const fileInfo of allFiles) {

        </div>        try {

      {:else if backups.length === 0}          const blob = await api.files.download(fileInfo.fullPath);

        <div class="text-center py-12">          zip.file(fileInfo.fullPath, blob);

          <svg class="w-16 h-16 mx-auto mb-4 text-base-content/30" fill="none" stroke="currentColor" viewBox="0 0 24 24">          exportProgress.current++;

            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4"/>        } catch (e) {

          </svg>          console.error(`Failed to fetch ${fileInfo.fullPath}:`, e);

          <p class="text-base-content/70 mb-4">No backups found</p>        }

          <button class="btn btn-primary" onclick={() => createBackup('full')}>      }

            Create your first backup

          </button>      success("Generating backup archive...");

        </div>      const zipBlob = await zip.generateAsync({

      {:else}        type: "blob",

        <div class="grid gap-4">        compression: "DEFLATE",

          {#each backups as backup}        compressionOptions: { level: 6 },

            <div class="card bg-base-200 shadow-md">      });

              <div class="card-body">

                <div class="flex justify-between items-start">      const timestamp = new Date()

                  <div class="flex-1">        .toISOString()

                    <h3 class="card-title text-lg mb-2">        .replace(/[:.]/g, "-")

                      <span class="badge badge-sm">{backup.backup_type}</span>        .slice(0, -5);

                      {backup.id}      const filename = `syncspace-backup-${timestamp}.zip`;

                      <div class="badge badge-sm {getStatusBadge(backup.status)}">

                        {backup.status}      const url = URL.createObjectURL(zipBlob);

                      </div>      const a = document.createElement("a");

                    </h3>      a.href = url;

      a.download = filename;

                    <div class="grid grid-cols-3 gap-3 text-sm">      a.click();

                      <div>      URL.revokeObjectURL(url);

                        <span class="text-base-content/70">Size:</span>

                        <span class="ml-2 font-semibold">{formatBytes(backup.size_bytes)}</span>      localStorage.setItem("last_backup_date", Date.now().toString());

                      </div>      lastBackupDate = new Date();

                      <div>

                        <span class="text-base-content/70">Files:</span>      success(`Backup created: ${filename} (${formatBytes(zipBlob.size)})`);

                        <span class="ml-2 font-semibold">{backup.file_count || 0}</span>    } catch (e) {

                      </div>      errorToast("Failed to create backup");

                      <div>    } finally {

                        <span class="text-base-content/70">Created:</span>      exporting = false;

                        <span class="ml-2">{formatDate(backup.created_at)}</span>    }

                      </div>  }

                    </div>

  async function handleImportZip(event) {

                    {#if backup.error_message}    const file = event.target.files?.[0];

                      <div class="alert alert-error mt-2">    if (!file) return;

                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">

                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>    if (!(await loadJSZipLib())) {

                        </svg>      errorToast("ZIP library not available");

                        <span>{backup.error_message}</span>      return;

                      </div>    }

                    {/if}

                  </div>    importing = true;

    importProgress = { current: 0, total: 0 };

                  <div class="flex gap-2">

                    {#if backup.status === 'completed'}    try {

                      <button      success("Reading ZIP file...");

                        class="btn btn-sm btn-primary"      const zip = await JSZip.loadAsync(file);

                        onclick={() => selectedBackup = backup}>

                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">      const filePaths = Object.keys(zip.files).filter(

                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>        (path) => !zip.files[path].dir

                        </svg>      );

                        Verify      importProgress.total = filePaths.length;

                      </button>

                    {/if}      success(`Importing ${filePaths.length} files...`);



                    <button      for (const filePath of filePaths) {

                      class="btn btn-sm btn-ghost text-error"        try {

                      onclick={() => deleteBackup(backup.id)}>          const fileData = await zip.files[filePath].async("blob");

                      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">          const cleanPath = filePath.replace(/^\/+/, "");

                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>          await api.files.upload(cleanPath, fileData);

                      </svg>          importProgress.current++;

                    </button>        } catch (e) {

                  </div>          console.error(`Failed to import ${filePath}:`, e);

                </div>        }

      }

                {#if selectedBackup?.id === backup.id}

                  <div class="mt-4 pt-4 border-t border-base-300">      success(`Imported ${importProgress.current} files successfully!`);

                    <BackupVerificationPanel backupId={backup.id} />      window.location.reload();

                  </div>    } catch (e) {

                {/if}      errorToast("Failed to import backup");

              </div>    } finally {

            </div>      importing = false;

          {/each}      event.target.value = "";

        </div>    }

      {/if}  }

    </div>

  {:else if activeTab === 'schedules'}  async function getAllFilesRecursive(path) {

    <BackupScheduleManager />    const result = [];

  {:else if activeTab === 'export'}

    <div class="export-tab">    try {

      <div class="alert alert-info mb-6">      const backendPath = path.replace(/^\/+|\/+$/g, "");

        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">      const fileList = await api.files.list(backendPath);

          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>

        </svg>      for (const item of fileList) {

        <div>        if (item.is_dir) {

          <h3 class="font-bold">Export/Import Feature</h3>          const subPath = path + item.name + "/";

          <div class="text-sm">This feature allows manual ZIP export/import of files. Check the old backup view for ZIP functionality.</div>          const subFiles = await getAllFilesRecursive(subPath);

        </div>          result.push(...subFiles);

      </div>        } else {

          result.push({

      <div class="card bg-base-200">            name: item.name,

        <div class="card-body">            fullPath: (path + item.name).replace(/^\//, ""),

          <h3 class="card-title">Coming Soon</h3>            size: item.size,

          <p>Manual export and import functionality will be integrated here. For now, use the backend API or schedules for automated backups.</p>          });

        </div>        }

      </div>      }

    </div>    } catch (e) {

  {/if}      console.error(`Failed to list ${path}:`, e);

</div>    }



<style>    return result;

  .backup-view {  }

    max-width: 1400px;  

    margin: 0 auto;  function formatDateOld(date) {

  }    if (!date) return "Never";

</style>    return date.toLocaleDateString() + " at " + date.toLocaleTimeString();

  }

  async function loadJSZipLib() {
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
