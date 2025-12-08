<script>
  import { onMount, onDestroy } from "svelte";
  import { currentLang } from "../stores/ui";
  import { t } from "../i18n.js";
  import PageWrapper from "../components/PageWrapper.svelte";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Dashboard state
  let storageUsed = $state(0);
  let storageTotal = $state(100);
  let storagePercent = $state(0);
  let recentUploads = $state([]);
  let recentShares = $state([]);
  let loading = $state(true);

  // Enhanced stats
  let totalFiles = $state(0);
  let totalUsers = $state(0);
  let activeUsersToday = $state(0);
  let totalSharesCount = $state(0);
  let pendingJobs = $state(0);
  let activeSessions = $state(0);
  let uploadTrend = $state(0);
  let storageTrend = $state(0);
  
  // Auto-refresh
  let refreshInterval = null;
  let lastRefresh = $state(new Date());

  onMount(async () => {
    await loadDashboardData();
    
    // Auto-refresh every 5 minutes
    refreshInterval = setInterval(async () => {
      await loadDashboardData();
      lastRefresh = new Date();
    }, 5 * 60 * 1000);
  });
  
  onDestroy(() => {
    if (refreshInterval) {
      clearInterval(refreshInterval);
    }
  });

  async function loadDashboardData() {
    loading = true;
    try {
      const token = localStorage.getItem("authToken");
      const headers = { Authorization: `Bearer ${token}` };

      const [statsRes, activityRes, sharesRes] = await Promise.all([
        fetch("http://localhost:8080/api/dashboard/stats", { headers }),
        fetch("http://localhost:8080/api/activity?limit=10&action=upload", {
          headers,
        }),
        fetch("http://localhost:8080/api/shares", { headers }).catch(() => ({
          ok: false,
        })),
      ]);

      // Process storage stats
      if (statsRes.ok) {
        const data = await statsRes.json();
        if (data.overview) {
          const usedGB = data.overview.total_storage_bytes / 1024 ** 3;
          storageUsed = usedGB;
          storageTotal = 100; // 100 GB limit
          storagePercent = Math.min((usedGB / storageTotal) * 100, 100);

          // Enhanced stats from backend
          totalFiles = data.overview.total_files || 0;
          totalUsers = data.overview.total_users || 0;
          activeUsersToday = data.overview.active_users_today || 0;
          totalSharesCount = data.overview.total_shares || 0;
          pendingJobs = data.overview.pending_jobs || 0;
          activeSessions = data.overview.active_sessions || 0;
        }

        // Get trends
        if (data.trends) {
          uploadTrend = data.trends.upload_trend || 0;
          storageTrend = data.trends.storage_trend || 0;
        }
      }

      // Process recent uploads
      if (activityRes.ok) {
        const activities = await activityRes.json();
        recentUploads = (activities || []).slice(0, 5).map((a) => ({
          id: a.id,
          name: extractFileName(a.file_path || a.file_name),
          size: formatBytes(a.metadata?.size || 0),
          date: formatDate(a.created_at),
          updatedAt: formatDate(a.created_at),
        }));
      }

      // Process shares
      if (sharesRes.ok) {
        const shares = await sharesRes.json();
        recentShares = (shares || []).slice(0, 5).map((s) => ({
          id: s.id || s.share_id,
          name: s.name || extractFileName(s.file_path),
          createdAt: formatDate(s.created_at),
          expiresAt: s.expires_at ? formatDate(s.expires_at) : "Nie",
          status:
            s.expires_at && new Date(s.expires_at) < new Date()
              ? "expired"
              : "active",
          isPublic: !s.password,
          files: s.file_count || 1,
          recipients: s.recipient_count || 0,
        }));
      }
    } catch (err) {
      console.error("Failed to load dashboard:", err);
    } finally {
      loading = false;
    }
  }

  function extractFileName(path) {
    if (!path) return "Unbekannt";
    return path.split("/").pop() || path;
  }

  function formatBytes(bytes) {
    if (!bytes || bytes === 0) return "-";
    if (bytes < 1024) return bytes + " B";
    if (bytes < 1024 ** 2) return (bytes / 1024).toFixed(1) + " KB";
    if (bytes < 1024 ** 3) return (bytes / 1024 ** 2).toFixed(1) + " MB";
    return (bytes / 1024 ** 3).toFixed(2) + " GB";
  }

  function formatDate(dateStr) {
    if (!dateStr) return "-";
    const date = new Date(dateStr);
    return date.toLocaleDateString("de-DE", {
      day: "2-digit",
      month: "2-digit",
      year: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }
</script>

<PageWrapper title="Dashboard" showSidebar={true}>
  <div class="dashboard">
    <!-- Header -->
    <div class="dashboard-header">
      <div class="header-left">
        <h1 class="dashboard-title">
          <i class="bi bi-grid-1x2-fill"></i>
          Dashboard
        </h1>
        <span class="last-refresh">
          <i class="bi bi-clock"></i>
          {lastRefresh.toLocaleTimeString()}
        </span>
      </div>
      <button 
        class="refresh-btn" 
        onclick={async () => { await loadDashboardData(); lastRefresh = new Date(); }}
        disabled={loading}
        title="Refresh Dashboard"
      >
        <i class="bi bi-arrow-clockwise {loading ? 'spinning' : ''}"></i>
      </button>
    </div>

    {#if loading}
      <!-- Skeleton Loading -->
      <div class="stats-grid">
        {#each Array(4) as _}
          <div class="skeleton h-28 w-full rounded-xl"></div>
        {/each}
      </div>
      <div class="skeleton h-32 w-full rounded-xl mt-4"></div>
      <div class="quick-actions">
        {#each Array(4) as _}
          <div class="skeleton h-24 w-full rounded-xl"></div>
        {/each}
      </div>
    {:else}
      <!-- Stats Cards Grid -->
      <div class="stats-grid">
        <!-- Total Files -->
        <div class="stat-card">
          <div class="stat-icon files-stat-icon">
            <i class="bi bi-files"></i>
          </div>
          <div class="stat-content">
            <p class="stat-label">Total Files</p>
            <p class="stat-value">{totalFiles.toLocaleString()}</p>
            {#if uploadTrend !== 0}
              <p
                class="stat-trend"
                class:positive={uploadTrend > 0}
                class:negative={uploadTrend < 0}
              >
                <i
                  class="bi {uploadTrend > 0 ? 'bi-arrow-up' : 'bi-arrow-down'}"
                ></i>
                {Math.abs(uploadTrend)}% this week
              </p>
            {/if}
          </div>
        </div>

        <!-- Active Shares -->
        <div class="stat-card">
          <div class="stat-icon shares-stat-icon">
            <i class="bi bi-share"></i>
          </div>
          <div class="stat-content">
            <p class="stat-label">Active Shares</p>
            <p class="stat-value">{totalSharesCount.toLocaleString()}</p>
            <p class="stat-trend neutral">
              <i class="bi bi-link-45deg"></i>
              {recentShares.length} recent
            </p>
          </div>
        </div>

        <!-- Active Users -->
        <div class="stat-card">
          <div class="stat-icon users-stat-icon">
            <i class="bi bi-people"></i>
          </div>
          <div class="stat-content">
            <p class="stat-label">Active Today</p>
            <p class="stat-value">{activeUsersToday}</p>
            <p class="stat-trend neutral">
              <i class="bi bi-person-check"></i>
              of {totalUsers} users
            </p>
          </div>
        </div>

        <!-- Sessions -->
        <div class="stat-card">
          <div class="stat-icon sessions-stat-icon">
            <i class="bi bi-display"></i>
          </div>
          <div class="stat-content">
            <p class="stat-label">Active Sessions</p>
            <p class="stat-value">{activeSessions}</p>
            {#if pendingJobs > 0}
              <p class="stat-trend neutral">
                <i class="bi bi-hourglass-split"></i>
                {pendingJobs} jobs pending
              </p>
            {/if}
          </div>
        </div>
      </div>

      <!-- Storage Usage Card -->
      <div class="card storage-card">
        <div class="card-header">
          <div class="card-icon storage-icon">
            <i class="bi bi-hdd-stack"></i>
          </div>
          <h2>Storage Usage</h2>
          <span class="storage-total">Total: {storageTotal.toFixed(2)} GB</span>
        </div>
        <div class="storage-bar-container">
          <div class="storage-bar" style="width: {storagePercent}%"></div>
        </div>
        <div class="storage-info">
          <span>{storageUsed.toFixed(2)} GB used</span>
          <span>{(storageTotal - storageUsed).toFixed(2)} GB available</span>
        </div>
      </div>

      <!-- Quick Actions -->
      <div class="quick-actions">
        <a href="#/files" class="action-card">
          <div class="action-icon files-icon">
            <i class="bi bi-folder2-open"></i>
          </div>
          <div class="action-text">
            <h3>My Files</h3>
            <p>Access and manage your uploaded files</p>
          </div>
        </a>
        <a href="#/shared" class="action-card">
          <div class="action-icon shares-icon">
            <i class="bi bi-share"></i>
          </div>
          <div class="action-text">
            <h3>My Shares</h3>
            <p>View and manage your shared files</p>
          </div>
        </a>
        <a href="#/search" class="action-card">
          <div class="action-icon search-icon">
            <i class="bi bi-search"></i>
          </div>
          <div class="action-text">
            <h3>Search Files</h3>
            <p>Find files with full-text search</p>
          </div>
        </a>
        <a href="#/activity" class="action-card">
          <div class="action-icon activity-icon">
            <i class="bi bi-activity"></i>
          </div>
          <div class="action-text">
            <h3>Activity Log</h3>
            <p>View recent file activity</p>
          </div>
        </a>
      </div>

      <!-- Recent Uploads Table -->
      <div class="card table-card">
        <div class="card-header-row">
          <h2>
            <i class="bi bi-cloud-arrow-up"></i>
            Recent Uploads
          </h2>
          <div class="header-actions">
            <a href="#/files" class="btn-outline">
              <i class="bi bi-folder2"></i>
              View All
            </a>
            <a href="#/files" class="btn-primary">
              <i class="bi bi-upload"></i>
              Upload
            </a>
          </div>
        </div>

        {#if recentUploads.length === 0}
          <div class="empty-state">
            <i class="bi bi-cloud-arrow-up"></i>
            <p>No recent uploads</p>
          </div>
        {:else}
          <div class="table-container">
            <table>
              <thead>
                <tr>
                  <th></th>
                  <th>NAME</th>
                  <th>DESCRIPTION</th>
                  <th>SIZE</th>
                  <th>CREATED AT</th>
                  <th>UPDATED AT</th>
                  <th>ACTIONS</th>
                </tr>
              </thead>
              <tbody>
                {#each recentUploads as file (file.id)}
                  <tr>
                    <td class="checkbox-cell"><input type="checkbox" /></td>
                    <td class="name-cell">
                      <i class="bi bi-file-earmark file-icon"></i>
                      {file.name}
                    </td>
                    <td class="desc-cell">-</td>
                    <td>{file.size}</td>
                    <td>{file.date}</td>
                    <td>{file.updatedAt}</td>
                    <td class="actions-cell">
                      <button class="action-dot" aria-label="File actions menu"
                        ><i class="bi bi-three-dots-vertical" aria-hidden="true"
                        ></i></button
                      >
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>

      <!-- Recent Shares Table -->
      <div class="card table-card">
        <div class="card-header-row">
          <h2>
            <i class="bi bi-share"></i>
            Recent Shares
          </h2>
          <div class="header-actions">
            <a href="#/shared" class="btn-outline">
              <i class="bi bi-share"></i>
              View All
            </a>
            <a href="#/shared" class="btn-primary">
              <i class="bi bi-plus-lg"></i>
              Create Share
            </a>
          </div>
        </div>

        {#if recentShares.length === 0}
          <div class="empty-state">
            <i class="bi bi-share"></i>
            <p>No shares yet</p>
          </div>
        {:else}
          <div class="table-container">
            <table>
              <thead>
                <tr>
                  <th></th>
                  <th>NAME</th>
                  <th>DESCRIPTION</th>
                  <th>CREATED AT</th>
                  <th>EXPIRES AT</th>
                  <th>STATUS</th>
                  <th>SECURITY</th>
                  <th>FILES</th>
                  <th>RECIPIENTS</th>
                  <th>ACTIONS</th>
                </tr>
              </thead>
              <tbody>
                {#each recentShares as share (share.id)}
                  <tr>
                    <td class="checkbox-cell"><input type="checkbox" /></td>
                    <td class="name-cell">{share.name}</td>
                    <td class="desc-cell">-</td>
                    <td>{share.createdAt}</td>
                    <td>{share.expiresAt}</td>
                    <td>
                      <span
                        class="status-badge"
                        class:expired={share.status === "expired"}
                      >
                        {share.status === "expired"
                          ? "Expired"
                          : "Never Expires"}
                      </span>
                    </td>
                    <td>
                      <span
                        class="security-badge"
                        class:protected={!share.isPublic}
                      >
                        <i class="bi {share.isPublic ? 'bi-globe' : 'bi-lock'}"
                        ></i>
                        {share.isPublic ? "Public" : "Protected"}
                      </span>
                    </td>
                    <td>{share.files} files</td>
                    <td>{share.recipients} recipients</td>
                    <td class="actions-cell">
                      <button class="action-dot" aria-label="Share actions menu"
                        ><i class="bi bi-three-dots-vertical" aria-hidden="true"
                        ></i></button
                      >
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div class="dashboard-footer">
        <span>Powered by <span class="font-semibold">SyncSpace</span></span>
        <span>v1.0-beta</span>
      </div>
    {/if}
  </div>
</PageWrapper>

<style>
  .dashboard {
    max-width: 1200px;
    margin: 0 auto;
    padding: 1.5rem;
  }

  .dashboard-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .dashboard-title {
    font-size: 1.5rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #111827;
  }

  :global(.dark) .dashboard-title {
    color: #f9fafb;
  }

  .last-refresh {
    font-size: 0.75rem;
    color: #6b7280;
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  :global(.dark) .last-refresh {
    color: #9ca3af;
  }

  .refresh-btn {
    padding: 0.5rem 0.75rem;
    border-radius: 0.5rem;
    background: #f3f4f6;
    border: 1px solid #e5e7eb;
    color: #374151;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .refresh-btn:hover:not(:disabled) {
    background: #e5e7eb;
    border-color: #d1d5db;
  }

  .refresh-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  :global(.dark) .refresh-btn {
    background: #374151;
    border-color: #4b5563;
    color: #f9fafb;
  }

  :global(.dark) .refresh-btn:hover:not(:disabled) {
    background: #4b5563;
  }

  .spinning {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  /* Skeleton classes are from global animations.css */

  /* Stats Grid */
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  @media (max-width: 1024px) {
    .stats-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  @media (max-width: 640px) {
    .stats-grid {
      grid-template-columns: 1fr;
    }
  }

  .stat-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    padding: 1.25rem;
    display: flex;
    align-items: flex-start;
    gap: 1rem;
    transition: all 0.2s;
  }

  :global(.dark) .stat-card {
    background: #1f2937;
    border-color: #374151;
  }

  .stat-card:hover {
    border-color: #22c55e;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .stat-icon {
    width: 48px;
    height: 48px;
    border-radius: 0.75rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
    flex-shrink: 0;
  }

  .files-stat-icon {
    background: #dbeafe;
    color: #3b82f6;
  }

  .shares-stat-icon {
    background: #dcfce7;
    color: #22c55e;
  }

  .users-stat-icon {
    background: #fef3c7;
    color: #f59e0b;
  }

  .sessions-stat-icon {
    background: #e0e7ff;
    color: #6366f1;
  }

  :global(.dark) .files-stat-icon {
    background: rgba(59, 130, 246, 0.2);
  }

  :global(.dark) .shares-stat-icon {
    background: rgba(34, 197, 94, 0.2);
  }

  :global(.dark) .users-stat-icon {
    background: rgba(245, 158, 11, 0.2);
  }

  :global(.dark) .sessions-stat-icon {
    background: rgba(99, 102, 241, 0.2);
  }

  .stat-content {
    min-width: 0;
  }

  .stat-label {
    font-size: 0.75rem;
    font-weight: 500;
    color: #6b7280;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 0.25rem;
  }

  :global(.dark) .stat-label {
    color: #9ca3af;
  }

  .stat-value {
    font-size: 1.75rem;
    font-weight: 700;
    color: #111827;
    line-height: 1.2;
    margin-bottom: 0.25rem;
  }

  :global(.dark) .stat-value {
    color: #f9fafb;
  }

  .stat-trend {
    font-size: 0.75rem;
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .stat-trend.positive {
    color: #22c55e;
  }

  .stat-trend.negative {
    color: #ef4444;
  }

  .stat-trend.neutral {
    color: #6b7280;
  }

  :global(.dark) .stat-trend.neutral {
    color: #9ca3af;
  }

  /* Cards */
  .card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    margin-bottom: 1.5rem;
  }

  :global(.dark) .card {
    background: #1f2937;
    border-color: #374151;
  }

  /* Storage Card */
  .storage-card {
    padding: 1.25rem;
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }

  .card-header h2 {
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
    flex: 1;
  }

  :global(.dark) .card-header h2 {
    color: #f9fafb;
  }

  .storage-total {
    font-size: 0.875rem;
    color: #6b7280;
  }

  :global(.dark) .storage-total {
    color: #9ca3af;
  }

  .card-icon {
    width: 36px;
    height: 36px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
  }

  .storage-icon {
    background: #dcfce7;
    color: #22c55e;
  }

  :global(.dark) .storage-icon {
    background: rgba(34, 197, 94, 0.2);
  }

  .storage-bar-container {
    height: 8px;
    background: #e5e7eb;
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 0.75rem;
  }

  :global(.dark) .storage-bar-container {
    background: #374151;
  }

  .storage-bar {
    height: 100%;
    background: linear-gradient(90deg, #22c55e, #16a34a);
    border-radius: 4px;
    transition: width 0.5s ease;
  }

  .storage-info {
    display: flex;
    justify-content: space-between;
    font-size: 0.875rem;
    color: #6b7280;
  }

  :global(.dark) .storage-info {
    color: #9ca3af;
  }

  /* Quick Actions */
  .quick-actions {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  @media (max-width: 1024px) {
    .quick-actions {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  @media (max-width: 640px) {
    .quick-actions {
      grid-template-columns: 1fr;
    }
  }

  .action-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1.25rem;
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.75rem;
    text-decoration: none;
    transition: all 0.2s;
  }

  :global(.dark) .action-card {
    background: #1f2937;
    border-color: #374151;
  }

  .action-card:hover {
    border-color: #22c55e;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .action-icon {
    width: 48px;
    height: 48px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
  }

  .files-icon {
    background: #dbeafe;
    color: #3b82f6;
  }

  .shares-icon {
    background: #dcfce7;
    color: #22c55e;
  }

  .search-icon {
    background: #fef3c7;
    color: #f59e0b;
  }

  .activity-icon {
    background: #e0e7ff;
    color: #6366f1;
  }

  :global(.dark) .files-icon {
    background: rgba(59, 130, 246, 0.2);
  }

  :global(.dark) .shares-icon {
    background: rgba(34, 197, 94, 0.2);
  }

  :global(.dark) .search-icon {
    background: rgba(245, 158, 11, 0.2);
  }

  :global(.dark) .activity-icon {
    background: rgba(99, 102, 241, 0.2);
  }

  .action-text h3 {
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
    margin-bottom: 0.25rem;
  }

  :global(.dark) .action-text h3 {
    color: #f9fafb;
  }

  .action-text p {
    font-size: 0.875rem;
    color: #6b7280;
  }

  :global(.dark) .action-text p {
    color: #9ca3af;
  }

  /* Table Card */
  .table-card {
    overflow: hidden;
  }

  .card-header-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.25rem;
    border-bottom: 1px solid #e5e7eb;
  }

  :global(.dark) .card-header-row {
    border-color: #374151;
  }

  .card-header-row h2 {
    font-size: 1rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #111827;
  }

  :global(.dark) .card-header-row h2 {
    color: #f9fafb;
  }

  .header-actions {
    display: flex;
    gap: 0.5rem;
  }

  .btn-outline,
  .btn-primary {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.5rem 0.875rem;
    font-size: 0.875rem;
    font-weight: 500;
    border-radius: 0.5rem;
    text-decoration: none;
    transition: all 0.2s;
  }

  .btn-outline {
    background: transparent;
    border: 1px solid #d1d5db;
    color: #374151;
  }

  :global(.dark) .btn-outline {
    border-color: #4b5563;
    color: #d1d5db;
  }

  .btn-outline:hover {
    background: #f3f4f6;
  }

  :global(.dark) .btn-outline:hover {
    background: #374151;
  }

  .btn-primary {
    background: #22c55e;
    border: 1px solid #22c55e;
    color: white;
  }

  .btn-primary:hover {
    background: #16a34a;
  }

  /* Table */
  .table-container {
    overflow-x: auto;
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  thead {
    background: #f9fafb;
  }

  :global(.dark) thead {
    background: #111827;
  }

  th {
    padding: 0.75rem 1rem;
    text-align: left;
    font-size: 0.75rem;
    font-weight: 600;
    color: #6b7280;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  :global(.dark) th {
    color: #9ca3af;
  }

  td {
    padding: 0.875rem 1rem;
    font-size: 0.875rem;
    color: #374151;
    border-top: 1px solid #e5e7eb;
  }

  :global(.dark) td {
    color: #d1d5db;
    border-color: #374151;
  }

  tr:hover td {
    background: #f9fafb;
  }

  :global(.dark) tr:hover td {
    background: #111827;
  }

  .checkbox-cell {
    width: 40px;
  }

  .checkbox-cell input {
    width: 16px;
    height: 16px;
    border-radius: 4px;
    cursor: pointer;
  }

  .name-cell {
    font-weight: 500;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .file-icon {
    color: #22c55e;
  }

  .desc-cell {
    color: #9ca3af;
  }

  .actions-cell {
    width: 50px;
  }

  .action-dot {
    background: none;
    border: none;
    padding: 0.25rem;
    cursor: pointer;
    color: #6b7280;
    border-radius: 0.25rem;
  }

  .action-dot:hover {
    background: #e5e7eb;
  }

  :global(.dark) .action-dot:hover {
    background: #374151;
  }

  /* Status Badges */
  .status-badge {
    display: inline-flex;
    align-items: center;
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
    font-weight: 500;
    border-radius: 9999px;
    background: #dcfce7;
    color: #16a34a;
  }

  .status-badge.expired {
    background: #fee2e2;
    color: #dc2626;
  }

  :global(.dark) .status-badge {
    background: rgba(34, 197, 94, 0.2);
  }

  :global(.dark) .status-badge.expired {
    background: rgba(220, 38, 38, 0.2);
  }

  .security-badge {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
    font-weight: 500;
    border-radius: 9999px;
    background: #dcfce7;
    color: #16a34a;
  }

  .security-badge.protected {
    background: #fef3c7;
    color: #d97706;
  }

  :global(.dark) .security-badge {
    background: rgba(34, 197, 94, 0.2);
  }

  :global(.dark) .security-badge.protected {
    background: rgba(245, 158, 11, 0.2);
  }

  /* Empty State */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem;
    color: #9ca3af;
  }

  .empty-state i {
    font-size: 2.5rem;
    margin-bottom: 0.75rem;
    opacity: 0.5;
  }

  .empty-state p {
    font-size: 0.875rem;
  }

  /* Footer */
  .dashboard-footer {
    display: flex;
    justify-content: center;
    gap: 0.5rem;
    padding: 2rem 0;
    font-size: 0.75rem;
    color: #9ca3af;
  }
</style>
