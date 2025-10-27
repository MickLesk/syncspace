<script>
  import { onMount } from "svelte";
  import { showToast } from "../stores/toast.js";

  let user = $state({
    username: "admin",
    email: "admin@syncspace.com",
    displayName: "Administrator",
    bio: "System administrator and cloud storage enthusiast",
    avatar: null,
    coverPhoto: null,
    joinedDate: new Date("2024-01-15"),
    stats: { files: 1247, storage: 15.4 * 1024 ** 3, shares: 23, uploads: 892 },
  });

  let recentFiles = $state([
    {
      id: 1,
      name: "Document.pdf",
      size: 2.4 * 1024 ** 2,
      modified: new Date(Date.now() - 30 * 60 * 1000),
    },
    {
      id: 2,
      name: "Image.png",
      size: 5.2 * 1024 ** 2,
      modified: new Date(Date.now() - 2 * 60 * 60 * 1000),
    },
    {
      id: 3,
      name: "Video.mp4",
      size: 124 * 1024 ** 2,
      modified: new Date(Date.now() - 24 * 60 * 60 * 1000),
    },
  ]);

  let editMode = $state(false);

  function formatBytes(bytes) {
    return `${(bytes / 1024 ** 3).toFixed(1)} GB`;
  }

  function formatDate(date) {
    return date.toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  }

  function getInitials(name) {
    return name
      .split(" ")
      .map((n) => n[0])
      .join("")
      .toUpperCase()
      .slice(0, 2);
  }

  function handleAvatarUpload(e) {
    const file = e.target.files[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (e) => {
        user.avatar = e.target.result;
      };
      reader.readAsDataURL(file);
      showToast("Avatar updated", "success");
    }
  }

  function saveProfile() {
    editMode = false;
    showToast("Profile saved", "success");
  }
</script>

<div class="profile-view">
  <!-- Cover Photo -->
  <div class="cover-photo">
    {#if user.coverPhoto}
      <img src={user.coverPhoto} alt="Cover" />
    {:else}
      <div class="cover-placeholder"></div>
    {/if}
    <button
      class="px-3 py-1.5 text-sm hover:bg-black/10 dark:hover:bg-white/10 text-white rounded-lg transition-colors flex items-center gap-2 absolute top-4 right-4"
      aria-label="Change cover photo"
      ><i class="bi bi-camera"></i> Change Cover</button
    >
  </div>

  <!-- Profile Header -->
  <div class="profile-header">
    <div class="profile-avatar-container">
      <div class="profile-avatar">
        {#if user.avatar}
          <img src={user.avatar} alt={user.displayName} />
        {:else}
          <div class="avatar-placeholder">{getInitials(user.displayName)}</div>
        {/if}
        <label class="avatar-upload-btn">
          <i class="bi bi-camera-fill"></i>
          <input
            type="file"
            accept="image/*"
            onchange={handleAvatarUpload}
            hidden
          />
        </label>
      </div>
    </div>

    <div class="profile-info">
      {#if editMode}
        <input
          type="text"
          bind:value={user.displayName}
          class="w-full max-w-xs px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 mb-2"
        />
        <textarea
          bind:value={user.bio}
          class="w-full max-w-md px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500"
          rows="2"
        ></textarea>
      {:else}
        <h1 class="profile-name">{user.displayName}</h1>
        <p class="profile-username">@{user.username}</p>
        <p class="profile-bio">{user.bio}</p>
      {/if}
      <div class="profile-meta">
        <span><i class="bi bi-envelope"></i> {user.email}</span>
        <span
          ><i class="bi bi-calendar"></i> Joined {formatDate(
            user.joinedDate
          )}</span
        >
      </div>
    </div>

    <div class="profile-actions">
      {#if editMode}
        <button
          class="px-4 py-2 bg-blue-600 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors flex items-center gap-2"
          onclick={saveProfile}><i class="bi bi-check-lg"></i> Save</button
        >
        <button
          class="px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-200 rounded-lg transition-colors"
          onclick={() => (editMode = false)}>Cancel</button
        >
      {:else}
        <button
          class="px-4 py-2 bg-blue-600 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors flex items-center gap-2"
          onclick={() => (editMode = true)}
          ><i class="bi bi-pencil"></i> Edit Profile</button
        >
      {/if}
    </div>
  </div>

  <!-- Stats Overview -->
  <div class="stats-container">
    <div class="stat-card">
      <i class="bi bi-files stat-icon"></i>
      <div class="stat-value">{user.stats.files.toLocaleString()}</div>
      <div class="stat-label">Files</div>
    </div>
    <div class="stat-card">
      <i class="bi bi-hdd stat-icon"></i>
      <div class="stat-value">{formatBytes(user.stats.storage)}</div>
      <div class="stat-label">Storage Used</div>
    </div>
    <div class="stat-card">
      <i class="bi bi-share stat-icon"></i>
      <div class="stat-value">{user.stats.shares}</div>
      <div class="stat-label">Shares</div>
    </div>
    <div class="stat-card">
      <i class="bi bi-upload stat-icon"></i>
      <div class="stat-value">{user.stats.uploads}</div>
      <div class="stat-label">Uploads This Month</div>
    </div>
  </div>

  <!-- Recent Files -->
  <div class="content-section">
    <h2 class="section-title">Recent Files</h2>
    <div class="files-list">
      {#each recentFiles as file (file.id)}
        <div class="file-item">
          <i class="bi bi-file-earmark-fill text-2xl text-primary"></i>
          <div class="file-info">
            <div class="file-name">{file.name}</div>
            <div class="file-meta">
              {formatBytes(file.size)} • {formatDate(file.modified)}
            </div>
          </div>
          <button
            class="px-2 py-1 text-sm hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-200 rounded transition-colors"
            aria-label="File options"
            ><i class="bi bi-three-dots-vertical"></i></button
          >
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .profile-view {
    padding: 0;
  }
  .cover-photo {
    position: relative;
    height: 300px;
    background: linear-gradient(135deg, hsl(var(--p)) 0%, hsl(var(--s)) 100%);
  }
  .cover-photo img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .cover-placeholder {
    width: 100%;
    height: 100%;
    background: linear-gradient(
      135deg,
      hsl(var(--p) / 0.8) 0%,
      hsl(var(--s) / 0.8) 100%
    );
  }
  .cover-upload-btn {
    position: absolute;
    top: 1rem;
    right: 1rem;
    backdrop-filter: blur(8px);
    background: hsl(var(--b1) / 0.8);
  }
  .profile-header {
    position: relative;
    background: hsl(var(--b1));
    padding: 0 2rem 2rem 2rem;
    margin-top: -80px;
  }
  .profile-avatar-container {
    display: flex;
    justify-content: center;
    margin-bottom: 1.5rem;
  }
  .profile-avatar {
    position: relative;
    width: 160px;
    height: 160px;
    border-radius: 50%;
    border: 4px solid hsl(var(--b1));
    overflow: hidden;
  }
  .profile-avatar img,
  .avatar-placeholder {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .avatar-placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    background: hsl(var(--p) / 0.2);
    color: hsl(var(--p));
    font-size: 3rem;
    font-weight: 700;
  }
  .avatar-upload-btn {
    position: absolute;
    bottom: 0;
    right: 0;
    width: 3rem;
    height: 3rem;
    border-radius: 50%;
    background: hsl(var(--p));
    color: hsl(var(--pc));
    border: 3px solid hsl(var(--b1));
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s;
  }
  .avatar-upload-btn:hover {
    transform: scale(1.1);
  }
  .profile-info {
    text-align: center;
    margin-bottom: 1.5rem;
  }
  .profile-name {
    font-size: 2rem;
    font-weight: 700;
    margin-bottom: 0.25rem;
  }
  .profile-username {
    color: hsl(var(--bc) / 0.6);
    margin-bottom: 0.5rem;
  }
  .profile-bio {
    color: hsl(var(--bc) / 0.8);
    margin-bottom: 1rem;
    max-width: 600px;
    margin-left: auto;
    margin-right: auto;
  }
  .profile-meta {
    display: flex;
    gap: 1.5rem;
    justify-content: center;
    color: hsl(var(--bc) / 0.6);
    font-size: 0.875rem;
  }
  .profile-meta i {
    margin-right: 0.25rem;
  }
  .profile-actions {
    display: flex;
    justify-content: center;
    gap: 0.75rem;
  }
  .stats-container {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1.5rem;
    padding: 2rem;
  }
  .stat-card {
    background: hsl(var(--b1));
    border: 1px solid hsl(var(--bc) / 0.1);
    border-radius: var(--rounded-box);
    padding: 1.5rem;
    text-align: center;
    transition: all 0.3s;
  }
  .stat-card:hover {
    border-color: hsl(var(--p) / 0.3);
    transform: translateY(-4px);
  }
  .stat-icon {
    font-size: 2rem;
    color: hsl(var(--p));
    margin-bottom: 0.75rem;
  }
  .stat-value {
    font-size: 2rem;
    font-weight: 700;
    margin-bottom: 0.25rem;
  }
  .stat-label {
    color: hsl(var(--bc) / 0.6);
    font-size: 0.875rem;
  }
  .content-section {
    padding: 0 2rem 2rem 2rem;
  }
  .section-title {
    font-size: 1.5rem;
    font-weight: 700;
    margin-bottom: 1rem;
  }
  .files-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .file-item {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background: hsl(var(--b1));
    border: 1px solid hsl(var(--bc) / 0.1);
    border-radius: var(--rounded-box);
    transition: all 0.2s;
  }
  .file-item:hover {
    border-color: hsl(var(--bc) / 0.2);
    background: hsl(var(--b2));
  }
  .file-info {
    flex: 1;
  }
  .file-name {
    font-weight: 600;
  }
  .file-meta {
    font-size: 0.875rem;
    color: hsl(var(--bc) / 0.6);
  }
  @media (max-width: 768px) {
    .cover-photo {
      height: 200px;
    }
    .profile-header {
      padding: 0 1rem 1.5rem 1rem;
      margin-top: -60px;
    }
    .profile-avatar {
      width: 120px;
      height: 120px;
    }
    .profile-name {
      font-size: 1.5rem;
    }
    .profile-meta {
      flex-direction: column;
      gap: 0.5rem;
    }
    .stats-container {
      grid-template-columns: repeat(2, 1fr);
      padding: 1rem;
    }
    .content-section {
      padding: 0 1rem 1rem 1rem;
    }
  }
</style>
