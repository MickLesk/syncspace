<script><script>

  import { onMount } from 'svelte';  import { onMount } from "svelte";

  import { showToast } from '../stores/ui.js';  import { activity } from "../stores/activity";

  import { error as errorToast } from "../stores/toast";

  let activities = $state([]);

  let loading = $state(true);  let groupedActivities = {};

  let filterType = $state('all'); // 'all', 'upload', 'download', 'delete', 'share'  let selectedFilter = "all";

  let selectedActivities = $state(new Set());  let searchQuery = "";



  // Mock data for demonstration  $: filteredActivities = filterActivities(

  const mockActivities = [    $activity,

    {    selectedFilter,

      id: 1,    searchQuery

      type: 'upload',  );

      user: { name: 'John Doe', avatar: null },  $: groupedActivities = groupByDate(filteredActivities);

      filename: 'Project Proposal.pdf',  $: todayCount = activity.getToday().length;

      timestamp: new Date(Date.now() - 30 * 60 * 1000).toISOString(),

      details: '2.4 MB uploaded to /Documents'  const activityTypes = [

    },    { value: "all", label: "All", icon: "list-ul" },

    {    { value: "upload", label: "Uploads", icon: "upload" },

      id: 2,    { value: "download", label: "Downloads", icon: "download" },

      type: 'download',    { value: "delete", label: "Deletes", icon: "trash" },

      user: { name: 'Jane Smith', avatar: null },    { value: "rename", label: "Renames", icon: "pencil" },

      filename: 'Annual Report.xlsx',    { value: "move", label: "Moves", icon: "arrow-right" },

      timestamp: new Date(Date.now() - 2 * 60 * 60 * 1000).toISOString(),  ];

      details: 'Downloaded from /Reports'

    },  const typeConfig = {

    {    upload: { label: "Uploaded", color: "success", icon: "upload" },

      id: 3,    download: { label: "Downloaded", color: "info", icon: "download" },

      type: 'share',    delete: { label: "Deleted", color: "error", icon: "trash" },

      user: { name: 'John Doe', avatar: null },    rename: { label: "Renamed", color: "warning", icon: "pencil" },

      filename: 'Vacation Photos.zip',    create: { label: "Created", color: "primary", icon: "plus-circle" },

      timestamp: new Date(Date.now() - 5 * 60 * 60 * 1000).toISOString(),    move: { label: "Moved", color: "secondary", icon: "arrow-right" },

      details: 'Shared with 3 people'    share: { label: "Shared", color: "accent", icon: "share" },

    },    favorite: { label: "Favorited", color: "warning", icon: "star-fill" },

    {    unfavorite: { label: "Unfavorited", color: "neutral", icon: "star" },

      id: 4,  };

      type: 'delete',

      user: { name: 'Admin', avatar: null },  onMount(async () => {

      filename: 'Old Backup.tar.gz',    await activity.load({ limit: 100 });

      timestamp: new Date(Date.now() - 24 * 60 * 60 * 1000).toISOString(),    const oldKey = "syncspace_activity";

      details: 'Moved to trash'    if (localStorage.getItem(oldKey)) {

    },      localStorage.removeItem(oldKey);

    {    }

      id: 5,  });

      type: 'rename',

      user: { name: 'Jane Smith', avatar: null },  function filterActivities(activities, filter, search) {

      filename: 'Meeting Notes.docx',    let filtered = activities;

      timestamp: new Date(Date.now() - 2 * 24 * 60 * 60 * 1000).toISOString(),

      details: 'Renamed from "Notes.docx"'    if (filter !== "all") {

    }      filtered = filtered.filter((a) => a.type === filter);

  ];    }



  onMount(() => {    if (search.trim()) {

    // Simulate API call      const query = search.toLowerCase();

    setTimeout(() => {      filtered = filtered.filter(

      activities = mockActivities;        (a) =>

      loading = false;          a.filename.toLowerCase().includes(query) ||

    }, 500);          a.path.toLowerCase().includes(query) ||

  });          (a.details && a.details.toLowerCase().includes(query))

      );

  function getActivityIcon(type) {    }

    const icons = {

      upload: 'bi-upload',    return filtered;

      download: 'bi-download',  }

      delete: 'bi-trash-fill',

      share: 'bi-share-fill',  function groupByDate(activities) {

      rename: 'bi-pencil-fill',    const groups = {};

      move: 'bi-folder-fill',    const now = new Date();

      create: 'bi-plus-circle-fill'    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());

    };    const yesterday = new Date(today);

    return icons[type] || 'bi-file-earmark-fill';    yesterday.setDate(yesterday.getDate() - 1);

  }

    for (const act of activities) {

  function getActivityColor(type) {      const date = new Date(act.timestamp);

    const colors = {      const dateKey = new Date(

      upload: 'success',        date.getFullYear(),

      download: 'info',        date.getMonth(),

      delete: 'error',        date.getDate()

      share: 'primary',      );

      rename: 'warning',

      move: 'secondary',      let label;

      create: 'success'      if (dateKey.getTime() === today.getTime()) {

    };        label = "Today";

    return colors[type] || 'base-content';      } else if (dateKey.getTime() === yesterday.getTime()) {

  }        label = "Yesterday";

      } else {

  function getRelativeTime(timestamp) {        label = formatDate(date);

    const now = new Date();      }

    const then = new Date(timestamp);

    const diffMs = now - then;      if (!groups[label]) {

    const diffMins = Math.floor(diffMs / 60000);        groups[label] = [];

    const diffHours = Math.floor(diffMs / 3600000);      }

    const diffDays = Math.floor(diffMs / 86400000);      groups[label].push(act);

    }

    if (diffMins < 1) return 'Just now';

    if (diffMins < 60) return `${diffMins} minute${diffMins > 1 ? 's' : ''} ago`;    return groups;

    if (diffHours < 24) return `${diffHours} hour${diffHours > 1 ? 's' : ''} ago`;  }

    if (diffDays === 1) return 'Yesterday';

    if (diffDays < 7) return `${diffDays} days ago`;  function formatDate(date) {

    return then.toLocaleDateString();    return date.toLocaleDateString("en-US", {

  }      weekday: "short",

      month: "short",

  function groupActivitiesByDate(activities) {      day: "numeric",

    const groups = {};      year:

    activities.forEach(activity => {        date.getFullYear() !== new Date().getFullYear() ? "numeric" : undefined,

      const date = new Date(activity.timestamp);    });

      const today = new Date();  }

      const yesterday = new Date(today);

      yesterday.setDate(yesterday.getDate() - 1);  function formatTime(timestamp) {

    return new Date(timestamp).toLocaleTimeString("en-US", {

      let key;      hour: "2-digit",

      if (date.toDateString() === today.toDateString()) {      minute: "2-digit",

        key = 'Today';    });

      } else if (date.toDateString() === yesterday.toDateString()) {  }

        key = 'Yesterday';

      } else {  function getRelativeTime(timestamp) {

        key = date.toLocaleDateString('en-US', { weekday: 'long', month: 'short', day: 'numeric' });    const now = Date.now();

      }    const diff = now - timestamp;



      if (!groups[key]) groups[key] = [];    const minutes = Math.floor(diff / 60000);

      groups[key].push(activity);    const hours = Math.floor(diff / 3600000);

    });    const days = Math.floor(diff / 86400000);

    return groups;

  }    if (minutes < 1) return "Just now";

    if (minutes < 60) return `${minutes}m ago`;

  function getUserInitials(name) {    if (hours < 24) return `${hours}h ago`;

    return name    if (days < 7) return `${days}d ago`;

      .split(' ')    return formatDate(new Date(timestamp));

      .map(n => n[0])  }

      .join('')

      .toUpperCase()  function handleClearAll() {

      .slice(0, 2);    if (confirm("Clear all activity history?")) {

  }      activity.clear();

    }

  function clearActivities() {  }

    if (confirm('Clear all activity history?')) {</script>

      activities = [];

      showToast('Activity history cleared', 'success');<div class="activity-view">

    }  <!-- Stats -->

  }  {#if $activity.length > 0}

    <div class="stats stats-vertical lg:stats-horizontal shadow mb-6 w-full">

  let filteredActivities = $derived(      <div class="stat">

    filterType === 'all'         <div class="stat-figure text-primary">

      ? activities           <i class="bi bi-activity text-4xl"></i>

      : activities.filter(a => a.type === filterType)        </div>

  );        <div class="stat-title">Total Events</div>

        <div class="stat-value text-primary">{$activity.length}</div>

  let groupedActivities = $derived(groupActivitiesByDate(filteredActivities));        <div class="stat-desc">All time</div>

</script>      </div>



<!-- Header -->      <div class="stat">

<div class="activity-header">        <div class="stat-figure text-success">

  <div class="header-top">          <i class="bi bi-calendar-check text-4xl"></i>

    <div>        </div>

      <h1 class="text-3xl font-bold">        <div class="stat-title">Today</div>

        <i class="bi bi-clock-history"></i> Activity        <div class="stat-value text-success">{todayCount}</div>

      </h1>        <div class="stat-desc">Recent activity</div>

      <p class="text-base-content/60 mt-1">      </div>

        {filteredActivities.length} event{filteredActivities.length !== 1 ? 's' : ''} 

        {filterType !== 'all' ? `• ${filterType}` : ''}      <div class="stat">

      </p>        <div class="stat-actions">

    </div>          <button

    <button class="btn btn-ghost gap-2" onclick={clearActivities}>            class="btn btn-sm btn-error gap-2"

      <i class="bi bi-trash"></i>            on:click={handleClearAll}

      Clear History            disabled={$activity.length === 0}

    </button>          >

  </div>            <i class="bi bi-trash-fill"></i>

            Clear All

  <!-- Filter Tabs -->          </button>

  <div class="filter-tabs">        </div>

    <button       </div>

      class="filter-btn"     </div>

      class:active={filterType === 'all'}  {/if}

      onclick={() => filterType = 'all'}>

      <i class="bi bi-list-ul"></i>  <!-- Filters & Search -->

      All  <div class="card bg-base-100 border border-base-300 shadow-sm mb-6">

    </button>    <div class="card-body p-4">

    <button       <div class="flex flex-col md:flex-row gap-4">

      class="filter-btn"         <!-- Filter Tabs -->

      class:active={filterType === 'upload'}        <div role="tablist" class="tabs tabs-boxed flex-1">

      onclick={() => filterType = 'upload'}>          {#each activityTypes as type}

      <i class="bi bi-upload"></i>            <button

      Uploads              role="tab"

    </button>              class="tab gap-2 {selectedFilter === type.value

    <button                 ? 'tab-active'

      class="filter-btn"                 : ''}"

      class:active={filterType === 'download'}              on:click={() => (selectedFilter = type.value)}

      onclick={() => filterType = 'download'}>            >

      <i class="bi bi-download"></i>              <i class="bi bi-{type.icon}"></i>

      Downloads              {type.label}

    </button>            </button>

    <button           {/each}

      class="filter-btn"         </div>

      class:active={filterType === 'share'}

      onclick={() => filterType = 'share'}>        <!-- Search -->

      <i class="bi bi-share-fill"></i>        <div class="form-control">

      Shares          <div class="input-group">

    </button>            <input

    <button               type="text"

      class="filter-btn"               placeholder="Search activities..."

      class:active={filterType === 'delete'}              class="input input-bordered"

      onclick={() => filterType = 'delete'}>              bind:value={searchQuery}

      <i class="bi bi-trash"></i>            />

      Deletes            <button class="btn btn-square">

    </button>              <i class="bi bi-search"></i>

  </div>            </button>

</div>          </div>

        </div>

<!-- Loading State -->      </div>

{#if loading}    </div>

  <div class="loading-container">  </div>

    <span class="loading loading-spinner loading-lg"></span>

    <p class="text-base-content/60 mt-4">Loading activity...</p>  <!-- Timeline -->

  </div>  {#if Object.keys(groupedActivities).length === 0}

{:else if filteredActivities.length === 0}    <div class="hero min-h-[400px]">

  <!-- Empty State -->      <div class="hero-content text-center">

  <div class="empty-state">        <div class="max-w-md">

    <i class="bi bi-clock-history empty-icon"></i>          <i class="bi bi-clock-history text-7xl text-base-300 mb-4"></i>

    <h3 class="text-xl font-semibold mt-4">No Activity</h3>          <h1 class="text-3xl font-bold">No Activity Yet</h1>

    <p class="text-base-content/60 mt-2">          <p class="py-6">File operations will appear here</p>

      {filterType === 'all'         </div>

        ? 'No activity recorded yet'       </div>

        : `No ${filterType} activity found`}    </div>

    </p>  {:else}

  </div>    {#each Object.entries(groupedActivities) as [dateLabel, activities]}

{:else}      <div class="mb-8">

  <!-- Activity Timeline -->        <!-- Date Badge -->

  <div class="activity-container">        <div class="flex items-center gap-3 mb-4">

    {#each Object.entries(groupedActivities) as [date, dateActivities]}          <div class="badge badge-primary badge-lg font-bold">{dateLabel}</div>

      <div class="activity-group">          <div class="flex-1 h-px bg-base-300"></div>

        <h2 class="group-date">{date}</h2>        </div>

        

        <div class="activity-timeline">        <!-- Timeline -->

          {#each dateActivities as activity (activity.id)}        <ul class="timeline timeline-vertical">

            <div class="activity-item">          {#each activities as act, i}

              <!-- Timeline Marker -->            {@const config = typeConfig[act.type] || typeConfig.create}

              <div class="timeline-marker">            <li>

                <div class="timeline-icon {getActivityColor(activity.type)}">              {#if i > 0}

                  <i class="{getActivityIcon(activity.type)}"></i>                <hr class="bg-{config.color}" />

                </div>              {/if}

                <div class="timeline-line"></div>              <div class="timeline-start timeline-box">

              </div>                <div class="text-xs opacity-70">

                  {formatTime(act.timestamp)}

              <!-- Activity Content -->                </div>

              <div class="activity-content">              </div>

                <div class="activity-card">              <div class="timeline-middle">

                  <!-- User Avatar -->                <div class="avatar placeholder">

                  <div class="user-avatar">                  <div

                    {#if activity.user.avatar}                    class="bg-{config.color} text-{config.color}-content rounded-full w-10"

                      <img src={activity.user.avatar} alt={activity.user.name} />                  >

                    {:else}                    <i class="bi bi-{config.icon} text-xl"></i>

                      <div class="avatar-placeholder">                  </div>

                        {getUserInitials(activity.user.name)}                </div>

                      </div>              </div>

                    {/if}              <div class="timeline-end">

                  </div>                <div

                  class="card bg-base-100 border border-base-300 shadow-sm hover:shadow-md transition-shadow"

                  <!-- Activity Details -->                >

                  <div class="activity-details">                  <div class="card-body p-4">

                    <div class="activity-header">                    <div class="flex items-start justify-between gap-2">

                      <span class="user-name">{activity.user.name}</span>                      <div class="flex-1">

                      <span class="activity-action">{activity.type}ed</span>                        <div

                      <span class="filename">{activity.filename}</span>                          class="badge badge-{config.color} badge-sm mb-2 gap-1"

                    </div>                        >

                    <p class="activity-description">{activity.details}</p>                          <i class="bi bi-{config.icon}"></i>

                    <div class="activity-footer">                          {config.label}

                      <span class="timestamp">                        </div>

                        <i class="bi bi-clock"></i>                        <h3 class="font-bold text-base">{act.filename}</h3>

                        {getRelativeTime(activity.timestamp)}                        {#if act.path}

                      </span>                          <p class="text-xs font-mono opacity-70 mt-1">

                      <div class="activity-actions">                            {act.path}

                        <button class="btn btn-ghost btn-xs gap-1" title="View details">                          </p>

                          <i class="bi bi-eye"></i>                        {/if}

                        </button>                        {#if act.details}

                        <button class="btn btn-ghost btn-xs gap-1" title="Share">                          <p class="text-sm opacity-80 mt-2">{act.details}</p>

                          <i class="bi bi-share"></i>                        {/if}

                        </button>                        <div class="text-xs opacity-60 mt-2 italic">

                      </div>                          {getRelativeTime(act.timestamp)}

                    </div>                        </div>

                  </div>                      </div>

                </div>                    </div>

              </div>                  </div>

            </div>                </div>

          {/each}              </div>

        </div>              {#if i < activities.length - 1}

      </div>                <hr class="bg-{config.color}" />

    {/each}              {/if}

  </div>            </li>

{/if}          {/each}

        </ul>

<style>      </div>

  .activity-header {    {/each}

    background: hsl(var(--b1));  {/if}

    border-bottom: 1px solid hsl(var(--bc) / 0.1);</div>

    padding: 2rem;

    margin: -2rem -2rem 2rem -2rem;<style>

  }  .activity-view {

    padding: 1.5rem;

  .header-top {    min-height: calc(100vh - 200px);

    display: flex;  }

    justify-content: space-between;

    align-items: flex-start;  /* Timeline customization */

    margin-bottom: 1.5rem;  :global(.timeline-vertical li) {

  }    margin-bottom: 2rem;

  }

  .filter-tabs {

    display: flex;  :global(.timeline-box) {

    gap: 0.5rem;    min-width: 80px;

    flex-wrap: wrap;  }

  }

  /* Badge colors */

  .filter-btn {  :global(.badge-success) {

    display: flex;    background-color: oklch(var(--su));

    align-items: center;    color: oklch(var(--suc));

    gap: 0.5rem;  }

    padding: 0.5rem 1rem;

    border-radius: var(--rounded-btn);  :global(.badge-info) {

    border: 1px solid hsl(var(--bc) / 0.1);    background-color: oklch(var(--in));

    background: hsl(var(--b1));    color: oklch(var(--inc));

    color: hsl(var(--bc) / 0.7);  }

    font-size: 0.875rem;

    font-weight: 500;  :global(.badge-error) {

    cursor: pointer;    background-color: oklch(var(--er));

    transition: all 0.2s;    color: oklch(var(--erc));

  }  }



  .filter-btn:hover {  :global(.badge-warning) {

    background: hsl(var(--b2));    background-color: oklch(var(--wa));

    border-color: hsl(var(--bc) / 0.2);    color: oklch(var(--wac));

  }  }



  .filter-btn.active {  :global(.badge-secondary) {

    background: hsl(var(--p));    background-color: oklch(var(--s));

    color: hsl(var(--pc));    color: oklch(var(--sc));

    border-color: hsl(var(--p));  }

  }

  :global(.badge-accent) {

  .loading-container {    background-color: oklch(var(--a));

    display: flex;    color: oklch(var(--ac));

    flex-direction: column;  }

    align-items: center;

    justify-content: center;  /* Avatar colors */

    min-height: 400px;  :global(.bg-success) {

  }    background-color: oklch(var(--su));

  }

  .empty-state {

    display: flex;  :global(.bg-info) {

    flex-direction: column;    background-color: oklch(var(--in));

    align-items: center;  }

    justify-content: center;

    min-height: 400px;  :global(.bg-error) {

    text-align: center;    background-color: oklch(var(--er));

  }  }



  .empty-icon {  :global(.bg-warning) {

    font-size: 4rem;    background-color: oklch(var(--wa));

    color: hsl(var(--bc) / 0.2);  }

  }

  :global(.bg-secondary) {

  .activity-container {    background-color: oklch(var(--s));

    padding: 2rem;  }

  }

  :global(.bg-accent) {

  .activity-group {    background-color: oklch(var(--a));

    margin-bottom: 3rem;  }

  }

  :global(.text-success-content) {

  .group-date {    color: oklch(var(--suc));

    font-size: 1.125rem;  }

    font-weight: 700;

    color: hsl(var(--bc) / 0.8);  :global(.text-info-content) {

    margin-bottom: 1.5rem;    color: oklch(var(--inc));

    padding-left: 3.5rem;  }

  }

  :global(.text-error-content) {

  .activity-timeline {    color: oklch(var(--erc));

    position: relative;  }

  }

  :global(.text-warning-content) {

  .activity-item {    color: oklch(var(--wac));

    display: flex;  }

    gap: 1rem;

    margin-bottom: 1.5rem;  :global(.text-secondary-content) {

    animation: slideEnter 0.4s cubic-bezier(0.4, 0, 0.2, 1);    color: oklch(var(--sc));

  }  }



  .timeline-marker {  :global(.text-accent-content) {

    display: flex;    color: oklch(var(--ac));

    flex-direction: column;  }

    align-items: center;

    flex-shrink: 0;  /* Timeline lines */

  }  :global(.timeline hr.bg-success) {

    background-color: oklch(var(--su));

  .timeline-icon {  }

    width: 2.5rem;

    height: 2.5rem;  :global(.timeline hr.bg-info) {

    border-radius: 50%;    background-color: oklch(var(--in));

    display: flex;  }

    align-items: center;

    justify-content: center;  :global(.timeline hr.bg-error) {

    font-size: 1rem;    background-color: oklch(var(--er));

    position: relative;  }

    z-index: 2;

    box-shadow: 0 0 0 4px hsl(var(--b1));  :global(.timeline hr.bg-warning) {

  }    background-color: oklch(var(--wa));

  }

  .timeline-icon.success {

    background: hsl(var(--su));  :global(.timeline hr.bg-secondary) {

    color: hsl(var(--suc));    background-color: oklch(var(--s));

  }  }



  .timeline-icon.info {  :global(.timeline hr.bg-accent) {

    background: hsl(var(--in));    background-color: oklch(var(--a));

    color: hsl(var(--inc));  }

  }</style>


  .timeline-icon.error {
    background: hsl(var(--er));
    color: hsl(var(--erc));
  }

  .timeline-icon.warning {
    background: hsl(var(--wa));
    color: hsl(var(--wac));
  }

  .timeline-icon.primary {
    background: hsl(var(--p));
    color: hsl(var(--pc));
  }

  .timeline-icon.secondary {
    background: hsl(var(--s));
    color: hsl(var(--sc));
  }

  .timeline-line {
    width: 2px;
    flex: 1;
    background: linear-gradient(
      to bottom,
      hsl(var(--bc) / 0.2) 0%,
      hsl(var(--bc) / 0.05) 100%
    );
    margin-top: 0.5rem;
  }

  .activity-item:last-child .timeline-line {
    display: none;
  }

  .activity-content {
    flex: 1;
    min-width: 0;
  }

  .activity-card {
    display: flex;
    gap: 1rem;
    padding: 1.25rem;
    background: hsl(var(--b1));
    border: 1px solid hsl(var(--bc) / 0.1);
    border-radius: var(--rounded-box);
    transition: all 0.3s;
  }

  .activity-card:hover {
    border-color: hsl(var(--bc) / 0.2);
    box-shadow: 0 4px 12px hsl(var(--bc) / 0.08);
    transform: translateY(-2px);
  }

  .user-avatar {
    width: 2.5rem;
    height: 2.5rem;
    border-radius: 50%;
    overflow: hidden;
    flex-shrink: 0;
  }

  .user-avatar img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .avatar-placeholder {
    width: 100%;
    height: 100%;
    background: hsl(var(--p) / 0.2);
    color: hsl(var(--p));
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 0.875rem;
  }

  .activity-details {
    flex: 1;
    min-width: 0;
  }

  .activity-header {
    display: flex;
    align-items: baseline;
    gap: 0.375rem;
    flex-wrap: wrap;
    margin-bottom: 0.5rem;
  }

  .user-name {
    font-weight: 600;
    color: hsl(var(--bc));
  }

  .activity-action {
    color: hsl(var(--bc) / 0.6);
    font-size: 0.875rem;
  }

  .filename {
    font-weight: 500;
    color: hsl(var(--p));
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .activity-description {
    font-size: 0.875rem;
    color: hsl(var(--bc) / 0.7);
    margin-bottom: 0.75rem;
  }

  .activity-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .timestamp {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    font-size: 0.75rem;
    color: hsl(var(--bc) / 0.5);
  }

  .activity-actions {
    display: flex;
    gap: 0.25rem;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .activity-header {
      padding: 1.5rem;
      margin: -1.5rem -1.5rem 1.5rem -1.5rem;
    }

    .header-top {
      flex-direction: column;
      gap: 1rem;
    }

    .activity-container {
      padding: 1rem;
    }

    .group-date {
      padding-left: 2.5rem;
      font-size: 1rem;
    }

    .timeline-icon {
      width: 2rem;
      height: 2rem;
      font-size: 0.875rem;
    }

    .activity-card {
      padding: 1rem;
    }

    .user-avatar {
      width: 2rem;
      height: 2rem;
    }

    .activity-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.25rem;
    }
  }
</style>
