<script>
  import { onMount } from "svelte";

  let theme = "dark";
  let files = [
    {
      id: 1,
      name: "Bericht 2024.pdf",
      size: "2.1 MB",
      type: "pdf",
      date: "vor 2h",
      category: "Dokumente",
      favorite: true,
    },
    {
      id: 2,
      name: "Daten.xlsx",
      size: "542 KB",
      type: "xlsx",
      date: "vor 5h",
      category: "Daten",
      favorite: false,
    },
    {
      id: 3,
      name: "Banner.png",
      size: "1.8 MB",
      type: "png",
      date: "vor 1d",
      category: "Bilder",
      favorite: false,
    },
    {
      id: 4,
      name: "Video-Tutorial.mp4",
      size: "45 MB",
      type: "mp4",
      date: "vor 3d",
      category: "Videos",
      favorite: true,
    },
  ];

  let searchQuery = "";
  let activeTab = "all";
  let sortBy = "date";
  let viewMode = "list";
  let contextMenu = { show: false, x: 0, y: 0, file: null };

  $: filteredFiles = files
    .filter((f) => {
      if (activeTab !== "all" && f.category.toLowerCase() !== activeTab)
        return false;
      return f.name.toLowerCase().includes(searchQuery.toLowerCase());
    })
    .sort((a, b) => {
      if (sortBy === "name") return a.name.localeCompare(b.name);
      if (sortBy === "size") return parseFloat(b.size) - parseFloat(a.size);
      return 0;
    });

  $: stats = {
    total: files.length,
    storage: "2.4 GB",
    syncs: "5",
    activity: "12",
  };

  function showContextMenu(e, file) {
    e.preventDefault();
    contextMenu = { show: true, x: e.clientX, y: e.clientY, file };
  }

  function closeContextMenu() {
    contextMenu = { show: false, x: 0, y: 0, file: null };
  }

  function toggleFavorite(id) {
    files = files.map((f) =>
      f.id === id ? { ...f, favorite: !f.favorite } : f
    );
    closeContextMenu();
  }

  function getGradientClass(index) {
    const gradients = [
      "from-blue-500 to-blue-700",
      "from-purple-500 to-purple-700",
      "from-green-500 to-green-700",
      "from-pink-500 to-pink-700",
    ];
    return gradients[index % gradients.length];
  }

  function toggleTheme() {
    theme = theme === "dark" ? "light" : "dark";
  }

  onMount(() => {
    const saved = localStorage.getItem("mockup-v2-theme") || "dark";
    theme = saved;
  });

  $: {
    localStorage.setItem("mockup-v2-theme", theme);
    document.documentElement.setAttribute("data-theme", theme);
  }
</script>

<div
  class="min-h-screen {theme === 'dark'
    ? 'bg-gradient-to-br from-slate-900 via-slate-950 to-slate-900 text-white'
    : 'bg-gradient-to-br from-slate-50 via-white to-slate-50 text-slate-900'}"
>
  <!-- Navigation -->
  <nav
    class="border-b {theme === 'dark'
      ? 'border-white/10 bg-black/40 backdrop-blur-xl'
      : 'border-slate-200 bg-white/40 backdrop-blur-xl'}"
  >
    <div class="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div
          class="w-10 h-10 rounded-xl bg-gradient-to-br from-blue-400 to-purple-600 flex items-center justify-center"
        >
          <i class="bi bi-cloud-check text-white"></i>
        </div>
        <span
          class="text-xl font-bold bg-gradient-to-r from-blue-400 via-purple-500 to-pink-500 bg-clip-text text-transparent"
          >SyncSpace V2</span
        >
      </div>
      <button
        on:click={toggleTheme}
        class="p-2 rounded-lg hover:bg-white/10 transition"
      >
        <i class="bi {theme === 'dark' ? 'bi-brightness-high' : 'bi-moon'}"></i>
      </button>
    </div>
  </nav>

  <div class="max-w-7xl mx-auto px-6 py-8">
    <!-- Header -->
    <div class="mb-8">
      <h1 class="text-4xl font-bold mb-2">Dashboard</h1>
      <p class="text-slate-400">Übersicht deiner Dateien und Aktivitäten</p>
    </div>

    <!-- Stats Grid -->
    <div class="grid grid-cols-4 gap-6 mb-12">
      <div
        class="p-6 rounded-2xl text-white bg-gradient-to-br from-blue-500 to-blue-700 hover:shadow-lg hover:shadow-blue-500/20 transition transform hover:-translate-y-1"
      >
        <div class="flex items-center justify-between mb-4">
          <span class="text-white/80">Speicher</span>
          <i class="bi bi-pie-chart-fill text-2xl text-white/60"></i>
        </div>
        <h3 class="text-3xl font-bold">2.4 GB</h3>
        <p class="text-white/70 text-sm mt-2">von 100 GB (2.4%)</p>
      </div>

      <div
        class="p-6 rounded-2xl text-white bg-gradient-to-br from-purple-500 to-purple-700 hover:shadow-lg hover:shadow-purple-500/20 transition transform hover:-translate-y-1"
      >
        <div class="flex items-center justify-between mb-4">
          <span class="text-white/80">Dateien</span>
          <i class="bi bi-file-earmark-fill text-2xl text-white/60"></i>
        </div>
        <h3 class="text-3xl font-bold">{stats.total}</h3>
        <p class="text-white/70 text-sm mt-2">aktive Dateien</p>
      </div>

      <div
        class="p-6 rounded-2xl text-white bg-gradient-to-br from-green-500 to-green-700 hover:shadow-lg hover:shadow-green-500/20 transition transform hover:-translate-y-1"
      >
        <div class="flex items-center justify-between mb-4">
          <span class="text-white/80">Syncs</span>
          <i class="bi bi-arrow-repeat text-2xl text-white/60"></i>
        </div>
        <h3 class="text-3xl font-bold">{stats.syncs}</h3>
        <p class="text-white/70 text-sm mt-2">alle aktiv</p>
      </div>

      <div
        class="p-6 rounded-2xl text-white bg-gradient-to-br from-pink-500 to-pink-700 hover:shadow-lg hover:shadow-pink-500/20 transition transform hover:-translate-y-1"
      >
        <div class="flex items-center justify-between mb-4">
          <span class="text-white/80">Aktivität</span>
          <i class="bi bi-activity text-2xl text-white/60"></i>
        </div>
        <h3 class="text-3xl font-bold">{stats.activity}</h3>
        <p class="text-white/70 text-sm mt-2">heute</p>
      </div>
    </div>

    <!-- Search & Filter -->
    <div class="flex gap-4 mb-8">
      <div
        class="flex-1 flex items-center gap-2 px-4 py-3 rounded-lg border {theme ===
        'dark'
          ? 'border-white/20 bg-white/5'
          : 'border-slate-200 bg-slate-50'}"
      >
        <i class="bi bi-search"></i>
        <input
          type="text"
          placeholder="Dateien durchsuchen..."
          bind:value={searchQuery}
          class="bg-transparent flex-1 outline-none"
        />
      </div>

      <select
        bind:value={sortBy}
        class="px-4 py-3 rounded-lg border {theme === 'dark'
          ? 'border-white/20 bg-white/5'
          : 'border-slate-200 bg-slate-50'} outline-none"
      >
        <option value="date">Datum</option>
        <option value="name">Name</option>
        <option value="size">Größe</option>
      </select>

      <div class="flex gap-2">
        <button
          on:click={() => (viewMode = "list")}
          class="p-3 rounded-lg {viewMode === 'list'
            ? 'bg-gradient-to-r from-blue-500 to-purple-600 text-white'
            : 'border ' +
              (theme === 'dark' ? 'border-white/20' : 'border-slate-200')}"
        >
          <i class="bi bi-list-ul"></i>
        </button>
        <button
          on:click={() => (viewMode = "grid")}
          class="p-3 rounded-lg {viewMode === 'grid'
            ? 'bg-gradient-to-r from-blue-500 to-purple-600 text-white'
            : 'border ' +
              (theme === 'dark' ? 'border-white/20' : 'border-slate-200')}"
        >
          <i class="bi bi-grid-3x3-gap"></i>
        </button>
      </div>
    </div>

    <!-- Tabs -->
    <div
      class="flex gap-2 mb-8 pb-4 border-b {theme === 'dark'
        ? 'border-white/10'
        : 'border-slate-200'}"
    >
      {#each ["all", "Dokumente", "Bilder", "Videos"] as tab}
        <button
          on:click={() => (activeTab = tab.toLowerCase())}
          class="px-4 py-2 rounded-lg transition {activeTab ===
          tab.toLowerCase()
            ? 'bg-gradient-to-r from-blue-500 to-purple-600 text-white'
            : 'text-slate-400 hover:text-white'}"
        >
          {tab === "all" ? "Alle" : tab}
        </button>
      {/each}
    </div>

    <!-- Files -->
    {#if viewMode === "list"}
      <div class="space-y-3">
        {#each filteredFiles as file, idx (file.id)}
          <div
            class="p-4 rounded-lg border flex items-center gap-4 hover:border-white/30 transition cursor-pointer group {theme ===
            'dark'
              ? 'border-white/10 bg-white/5'
              : 'border-slate-200 bg-slate-100'}"
            on:contextmenu|preventDefault={(e) => showContextMenu(e, file)}
          >
            <div
              class="w-12 h-12 rounded-lg bg-gradient-to-br {getGradientClass(
                idx
              )} flex items-center justify-center flex-shrink-0 text-white text-xl"
            >
              <i class="bi bi-file-earmark"></i>
            </div>

            <div class="flex-1">
              <p class="font-semibold">{file.name}</p>
              <p class="text-sm text-slate-400">{file.size} • {file.date}</p>
            </div>

            <button
              on:click={() => toggleFavorite(file.id)}
              class="opacity-0 group-hover:opacity-100 transition"
            >
              <i
                class="bi {file.favorite
                  ? 'bi-star-fill text-yellow-400'
                  : 'bi-star'}"
              ></i>
            </button>

            <button class="opacity-0 group-hover:opacity-100 transition">
              <i class="bi bi-three-dots-vertical"></i>
            </button>
          </div>
        {/each}
      </div>
    {:else}
      <div class="grid grid-cols-4 gap-6">
        {#each filteredFiles as file, idx (file.id)}
          <div
            class="p-6 rounded-xl border {theme === 'dark'
              ? 'border-white/10 bg-white/5 hover:bg-white/10'
              : 'border-slate-200 bg-slate-100 hover:bg-slate-200'} transition cursor-pointer group"
            on:contextmenu|preventDefault={(e) => showContextMenu(e, file)}
          >
            <div
              class="w-full h-32 rounded-lg bg-gradient-to-br {getGradientClass(
                idx
              )} flex items-center justify-center text-white text-4xl mb-4 group-hover:scale-105 transition"
            >
              <i class="bi bi-file-earmark"></i>
            </div>
            <p class="font-semibold truncate">{file.name}</p>
            <p class="text-sm text-slate-400">{file.size}</p>
          </div>
        {/each}
      </div>
    {/if}

    {#if filteredFiles.length === 0}
      <div class="text-center py-12">
        <i
          class="bi bi-inbox text-4xl {theme === 'dark'
            ? 'text-slate-600'
            : 'text-slate-300'} mb-4 block"
        ></i>
        <p class="text-slate-400">Keine Dateien gefunden</p>
      </div>
    {/if}
  </div>

  <!-- Context Menu -->
  {#if contextMenu.show}
    <div
      class="fixed bg-white/10 backdrop-blur-lg rounded-lg shadow-xl border border-white/20 py-2 z-50"
      style="left: {contextMenu.x}px; top: {contextMenu.y}px;"
      on:mouseleave={closeContextMenu}
    >
      <button
        class="w-full px-4 py-2 text-left hover:bg-white/10 transition flex items-center gap-3"
      >
        <i class="bi bi-eye"></i> Vorschau
      </button>
      <button
        on:click={() => toggleFavorite(contextMenu.file.id)}
        class="w-full px-4 py-2 text-left hover:bg-white/10 transition flex items-center gap-3"
      >
        <i class="bi {contextMenu.file.favorite ? 'bi-star-fill' : 'bi-star'}"
        ></i>
        {contextMenu.file.favorite ? "Unfav" : "Favorit"}
      </button>
      <button
        class="w-full px-4 py-2 text-left hover:bg-white/10 transition flex items-center gap-3"
      >
        <i class="bi bi-download"></i> Download
      </button>
      <hr class="border-white/10 my-2" />
      <button
        class="w-full px-4 py-2 text-left hover:bg-red-500/20 text-red-400 transition flex items-center gap-3"
      >
        <i class="bi bi-trash"></i> Löschen
      </button>
    </div>
  {/if}
</div>
