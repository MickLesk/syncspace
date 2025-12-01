<script>
  import { onMount } from "svelte";

  let theme = "dark";
  let files = [
    {
      id: 1,
      name: "Bericht-2024.pdf",
      size: "2.1 MB",
      date: "vor 2h",
      modified: "vor 2h",
      shared: false,
    },
    {
      id: 2,
      name: "Daten-Q4.xlsx",
      size: "542 KB",
      date: "vor 5h",
      modified: "vor 5h",
      shared: true,
    },
    {
      id: 3,
      name: "Präsentation.png",
      size: "1.8 MB",
      date: "vor 1d",
      modified: "vor 1d",
      shared: false,
    },
    {
      id: 4,
      name: "Projekte",
      size: "—",
      date: "vor 3d",
      modified: "vor 3d",
      shared: false,
    },
  ];

  let searchQuery = "";
  let selectedRows = new Set();
  let showDeleteConfirm = false;
  let sidebarOpen = true;

  $: filteredFiles = files.filter((f) =>
    f.name.toLowerCase().includes(searchQuery.toLowerCase())
  );

  function toggleSelectAll() {
    if (selectedRows.size === filteredFiles.length) {
      selectedRows = new Set();
    } else {
      selectedRows = new Set(filteredFiles.map((f) => f.id));
    }
  }

  function toggleSelect(id) {
    if (selectedRows.has(id)) {
      selectedRows.delete(id);
    } else {
      selectedRows.add(id);
    }
    selectedRows = selectedRows;
  }

  function deleteSelected() {
    files = files.filter((f) => !selectedRows.has(f.id));
    selectedRows = new Set();
    showDeleteConfirm = false;
  }

  function toggleTheme() {
    theme = theme === "dark" ? "light" : "dark";
  }

  onMount(() => {
    const saved = localStorage.getItem("mockup-v3-theme") || "dark";
    theme = saved;
  });

  $: {
    localStorage.setItem("mockup-v3-theme", theme);
  }
</script>

<div
  class="min-h-screen flex {theme === 'dark'
    ? 'bg-slate-950 text-white'
    : 'bg-white text-slate-900'}"
>
  <!-- Sidebar -->
  <aside
    class="w-64 border-r {theme === 'dark'
      ? 'border-slate-800'
      : 'border-slate-200'} flex flex-col transition-all"
  >
    <div
      class="p-6 border-b {theme === 'dark'
        ? 'border-slate-800'
        : 'border-slate-200'}"
    >
      <div class="flex items-center gap-3">
        <div
          class="w-8 h-8 rounded bg-blue-600 flex items-center justify-center"
        >
          <i class="bi bi-cloud-check text-white text-sm"></i>
        </div>
        <span class="font-semibold">SyncSpace</span>
      </div>
    </div>

    <nav class="flex-1 p-6 space-y-1">
      <a
        href="#"
        class="flex items-center gap-3 px-4 py-2 text-sm rounded {theme ===
        'dark'
          ? 'bg-slate-800 text-white'
          : 'bg-blue-50 text-blue-700'}"
      >
        <i class="bi bi-folder w-5"></i>
        <span>Dateien</span>
      </a>
      <a
        href="#"
        class="flex items-center gap-3 px-4 py-2 text-sm rounded {theme ===
        'dark'
          ? 'text-slate-400 hover:text-white hover:bg-slate-800/50'
          : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'} transition"
      >
        <i class="bi bi-search w-5"></i>
        <span>Suchen</span>
      </a>
      <a
        href="#"
        class="flex items-center gap-3 px-4 py-2 text-sm rounded {theme ===
        'dark'
          ? 'text-slate-400 hover:text-white hover:bg-slate-800/50'
          : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'} transition"
      >
        <i class="bi bi-star w-5"></i>
        <span>Favoriten</span>
      </a>
      <a
        href="#"
        class="flex items-center gap-3 px-4 py-2 text-sm rounded {theme ===
        'dark'
          ? 'text-slate-400 hover:text-white hover:bg-slate-800/50'
          : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'} transition"
      >
        <i class="bi bi-share w-5"></i>
        <span>Geteilt</span>
      </a>
      <a
        href="#"
        class="flex items-center gap-3 px-4 py-2 text-sm rounded {theme ===
        'dark'
          ? 'text-slate-400 hover:text-white hover:bg-slate-800/50'
          : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'} transition"
      >
        <i class="bi bi-trash w-5"></i>
        <span>Papierkorb</span>
      </a>
    </nav>

    <div
      class="p-6 border-t {theme === 'dark'
        ? 'border-slate-800'
        : 'border-slate-200'}"
    >
      <button
        on:click={toggleTheme}
        class="flex items-center gap-3 px-4 py-2 text-sm rounded w-full hover:bg-slate-800/50 transition"
      >
        <i class="bi {theme === 'dark' ? 'bi-brightness-high' : 'bi-moon'} w-5"
        ></i>
        <span>{theme === "dark" ? "Light" : "Dark"} Mode</span>
      </button>
    </div>
  </aside>

  <!-- Main -->
  <main class="flex-1 flex flex-col">
    <!-- Top Bar -->
    <header
      class="border-b {theme === 'dark'
        ? 'border-slate-800 bg-slate-900'
        : 'border-slate-200 bg-slate-50'} px-8 py-4 flex items-center justify-between"
    >
      <div>
        <h1 class="text-2xl font-semibold">Dateien</h1>
      </div>
      <button
        class="px-4 py-2 text-sm rounded bg-blue-600 hover:bg-blue-700 text-white transition font-medium"
      >
        <i class="bi bi-plus-lg"></i> Hochladen
      </button>
    </header>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto p-8">
      <!-- Search -->
      <div class="mb-8 flex gap-4">
        <div
          class="flex-1 flex items-center gap-2 px-4 py-3 rounded-lg border {theme ===
          'dark'
            ? 'border-slate-700 bg-slate-800/50'
            : 'border-slate-200 bg-slate-50'}"
        >
          <i class="bi bi-search text-slate-400"></i>
          <input
            type="text"
            placeholder="Dateien durchsuchen..."
            bind:value={searchQuery}
            class="bg-transparent flex-1 outline-none"
          />
        </div>
      </div>

      <!-- Stats -->
      <div class="grid grid-cols-4 gap-6 mb-8">
        <div
          class={`p-4 rounded border ${theme === "dark" ? "border-slate-700 bg-slate-800/50" : "border-slate-200 bg-slate-50"}`}
        >
          <p
            class={`text-sm mb-1 ${theme === "dark" ? "text-slate-400" : "text-slate-600"}`}
          >
            Speicherplatz
          </p>
          <p class="text-2xl font-semibold">2.4 GB</p>
          <p
            class={`text-xs mt-1 ${theme === "dark" ? "text-slate-500" : "text-slate-600"}`}
          >
            von 100 GB
          </p>
        </div>
        <div
          class={`p-4 rounded border ${theme === "dark" ? "border-slate-700 bg-slate-800/50" : "border-slate-200 bg-slate-50"}`}
        >
          <p
            class={`text-sm mb-1 ${theme === "dark" ? "text-slate-400" : "text-slate-600"}`}
          >
            Dateien
          </p>
          <p class="text-2xl font-semibold">{files.length}</p>
          <p
            class={`text-xs mt-1 ${theme === "dark" ? "text-slate-500" : "text-slate-600"}`}
          >
            Gesamt
          </p>
        </div>
        <div
          class={`p-4 rounded border ${theme === "dark" ? "border-slate-700 bg-slate-800/50" : "border-slate-200 bg-slate-50"}`}
        >
          <p
            class={`text-sm mb-1 ${theme === "dark" ? "text-slate-400" : "text-slate-600"}`}
          >
            Syncs
          </p>
          <p class="text-2xl font-semibold">5</p>
          <p
            class={`text-xs mt-1 ${theme === "dark" ? "text-slate-500" : "text-slate-600"}`}
          >
            Aktiv
          </p>
        </div>
        <div
          class={`p-4 rounded border ${theme === "dark" ? "border-slate-700 bg-slate-800/50" : "border-slate-200 bg-slate-50"}`}
        >
          <p
            class={`text-sm mb-1 ${theme === "dark" ? "text-slate-400" : "text-slate-600"}`}
          >
            Geändert
          </p>
          <p class="text-2xl font-semibold">2h</p>
          <p
            class={`text-xs mt-1 ${theme === "dark" ? "text-slate-500" : "text-slate-600"}`}
          >
            Kürzlich
          </p>
        </div>
      </div>

      <!-- File List -->
      <div
        class={`border rounded overflow-hidden ${theme === "dark" ? "border-slate-700" : "border-slate-200"}`}
      >
        <div
          class={`px-6 py-4 border-b grid grid-cols-4 gap-4 text-sm font-semibold ${theme === "dark" ? "border-slate-700 bg-slate-800/30 text-slate-400" : "border-slate-200 bg-slate-50 text-slate-600"}`}
        >
          <div class="flex items-center gap-3">
            <input
              type="checkbox"
              checked={selectedRows.size === filteredFiles.length &&
                filteredFiles.length > 0}
              on:change={toggleSelectAll}
              class="w-4 h-4 cursor-pointer"
            />
            Name
          </div>
          <div>Größe</div>
          <div>Geändert</div>
          <div class="text-right">Aktionen</div>
        </div>

        <div
          class={`divide-y ${theme === "dark" ? "divide-slate-700" : "divide-slate-200"}`}
        >
          {#each filteredFiles as file (file.id)}
            <div
              class={`px-6 py-4 grid grid-cols-4 gap-4 items-center hover:bg-white/5 transition ${selectedRows.has(file.id) ? (theme === "dark" ? "bg-blue-500/10" : "bg-blue-50") : ""}`}
            >
              <div class="flex items-center gap-3">
                <input
                  type="checkbox"
                  checked={selectedRows.has(file.id)}
                  on:change={() => toggleSelect(file.id)}
                  class="w-4 h-4 cursor-pointer"
                />
                <i
                  class="bi bi-file-earmark {theme === 'dark'
                    ? 'text-slate-400'
                    : 'text-slate-500'}"
                ></i>
                <span class="text-sm">{file.name}</span>
              </div>
              <div
                class={`text-sm ${theme === "dark" ? "text-slate-400" : "text-slate-600"}`}
              >
                {file.size}
              </div>
              <div
                class={`text-sm ${theme === "dark" ? "text-slate-400" : "text-slate-600"}`}
              >
                {file.modified}
              </div>
              <div class="text-right flex gap-2 justify-end">
                {#if file.shared}
                  <span
                    class="text-xs px-2 py-1 rounded bg-green-500/20 text-green-400"
                    >geteilt</span
                  >
                {/if}
                <button
                  class={`p-1 rounded hover:bg-white/10 transition ${theme === "dark" ? "text-slate-400" : "text-slate-500"}`}
                >
                  <i class="bi bi-three-dots"></i>
                </button>
              </div>
            </div>
          {/each}
        </div>
      </div>

      {#if filteredFiles.length === 0}
        <div class="text-center py-12">
          <i
            class="bi bi-inbox text-4xl {theme === 'dark'
              ? 'text-slate-700'
              : 'text-slate-300'} mb-4 block"
          ></i>
          <p class={theme === "dark" ? "text-slate-500" : "text-slate-400"}>
            Keine Dateien gefunden
          </p>
        </div>
      {/if}

      {#if selectedRows.size > 0}
        <div
          class="fixed bottom-8 left-8 flex gap-3 bg-blue-600 text-white px-6 py-3 rounded-lg shadow-lg"
        >
          <span>{selectedRows.size} ausgewählt</span>
          <button
            on:click={() => (showDeleteConfirm = true)}
            class="ml-4 px-3 py-1 bg-red-500 hover:bg-red-600 rounded transition"
          >
            Löschen
          </button>
          <button
            on:click={() => (selectedRows = new Set())}
            class="px-3 py-1 bg-blue-500 hover:bg-blue-400 rounded transition"
          >
            Abbrechen
          </button>
        </div>
      {/if}
    </div>
  </main>

  <!-- Delete Confirmation -->
  {#if showDeleteConfirm}
    <div
      class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center"
    >
      <div
        class={`p-8 rounded-lg max-w-md w-full ${theme === "dark" ? "bg-slate-900 border border-slate-700" : "bg-white border border-slate-200"}`}
      >
        <h2 class="text-xl font-bold mb-4">Datei(en) löschen?</h2>
        <p
          class={`mb-6 ${theme === "dark" ? "text-slate-400" : "text-slate-600"}`}
        >
          {selectedRows.size} Datei(en) werden permanent gelöscht.
        </p>
        <div class="flex gap-3">
          <button
            on:click={() => (showDeleteConfirm = false)}
            class={`flex-1 px-4 py-2 rounded border transition ${theme === "dark" ? "border-slate-700 hover:bg-slate-800" : "border-slate-200 hover:bg-slate-50"}`}
          >
            Abbrechen
          </button>
          <button
            on:click={deleteSelected}
            class="flex-1 px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded transition"
          >
            Löschen
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>
