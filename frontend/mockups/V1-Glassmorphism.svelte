<script>
  import { onMount } from "svelte";

  let theme = "dark";
  let files = [
    {
      id: 1,
      name: "Dokumentation.pdf",
      size: "1.2 MB",
      type: "pdf",
      date: "2 Stunden",
      favorite: false,
    },
    {
      id: 2,
      name: "Daten 2024.xlsx",
      size: "542 KB",
      type: "xlsx",
      date: "5 Stunden",
      favorite: false,
    },
    {
      id: 3,
      name: "Projekte",
      size: "—",
      type: "folder",
      date: "3 Tage",
      favorite: true,
    },
    {
      id: 4,
      name: "Screenshare.png",
      size: "3.8 MB",
      type: "png",
      date: "1 Tag",
      favorite: false,
    },
  ];

  let searchQuery = "";
  let showUploadModal = false;
  let selectedFiles = new Set();
  let dragOver = false;

  $: filteredFiles = files.filter((f) =>
    f.name.toLowerCase().includes(searchQuery.toLowerCase())
  );

  function toggleFavorite(id) {
    files = files.map((f) =>
      f.id === id ? { ...f, favorite: !f.favorite } : f
    );
  }

  function toggleSelect(id) {
    if (selectedFiles.has(id)) {
      selectedFiles.delete(id);
    } else {
      selectedFiles.add(id);
    }
    selectedFiles = selectedFiles;
  }

  function deleteSelected() {
    files = files.filter((f) => !selectedFiles.has(f.id));
    selectedFiles = new Set();
  }

  function handleDrop(e) {
    e.preventDefault();
    dragOver = false;
    // Simuliere Upload
    const newFiles = ["Neue-Datei-1.txt", "Neue-Datei-2.docx"];
    newFiles.forEach((name, idx) => {
      files = [
        ...files,
        {
          id: Math.max(...files.map((f) => f.id)) + 1,
          name,
          size: Math.random() > 0.5 ? "2.5 MB" : "1.2 MB",
          type: name.split(".")[1],
          date: "gerade eben",
          favorite: false,
        },
      ];
    });
  }

  function getIcon(type) {
    const icons = {
      pdf: "bi-file-pdf",
      xlsx: "bi-table",
      png: "bi-image",
      docx: "bi-file-text",
      txt: "bi-file-earmark",
      folder: "bi-folder",
    };
    return icons[type] || "bi-file-earmark";
  }

  function toggleTheme() {
    theme = theme === "dark" ? "light" : "dark";
  }

  onMount(() => {
    const saved = localStorage.getItem("mockup-theme") || "dark";
    theme = saved;
    document.documentElement.setAttribute("data-theme", saved);
  });

  $: {
    localStorage.setItem("mockup-theme", theme);
    document.documentElement.setAttribute("data-theme", theme);
  }
</script>

<div
  class="min-h-screen transition-colors duration-300"
  class:dark={theme === "dark"}
  class:light={theme === "light"}
>
  <style>
    :global(:root) {
      --glass-light: rgba(255, 255, 255, 0.1);
      --glass-border: rgba(255, 255, 255, 0.2);
    }

    :global(html[data-theme="light"]) {
      --glass-light: rgba(0, 0, 0, 0.05);
      --glass-border: rgba(0, 0, 0, 0.1);
    }

    :global(.glass) {
      background: var(--glass-light);
      backdrop-filter: blur(12px);
      border: 1px solid var(--glass-border);
    }
  </style>

  <!-- Navigation -->
  <nav
    class="glass sticky top-0 z-50 border-b {theme === 'dark'
      ? 'border-white/10 bg-slate-950/40'
      : 'border-slate-200 bg-white/40'}"
  >
    <div class="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div
          class="w-10 h-10 rounded-lg bg-gradient-to-br from-blue-400 to-purple-600 flex items-center justify-center"
        >
          <i class="bi bi-cloud-check text-white"></i>
        </div>
        <span
          class="text-xl font-bold bg-gradient-to-r from-blue-400 to-purple-600 bg-clip-text text-transparent"
          >SyncSpace V1</span
        >
      </div>
      <button
        on:click={toggleTheme}
        class="p-2 rounded-lg hover:bg-white/10 transition"
      >
        <i
          class="bi {theme === 'dark' ? 'bi-moon-stars' : 'bi-brightness-high'}"
        ></i>
      </button>
    </div>
  </nav>

  <!-- Main Content -->
  <div class="max-w-7xl mx-auto px-6 py-8">
    <!-- Header -->
    <div class="mb-8">
      <h1 class="text-4xl font-bold mb-2">Dateien</h1>
      <p class="text-slate-400">
        {files.length} Dateien • {(Math.random() * 10).toFixed(1)} GB verwendet
      </p>
    </div>

    <!-- Search & Actions -->
    <div class="flex gap-4 mb-8">
      <div class="flex-1 glass px-4 py-3 rounded-lg flex items-center gap-2">
        <i class="bi bi-search"></i>
        <input
          type="text"
          placeholder="Dateien durchsuchen..."
          bind:value={searchQuery}
          class="bg-transparent flex-1 outline-none"
        />
      </div>
      <button
        on:click={() => (showUploadModal = true)}
        class="glass px-6 py-3 rounded-lg hover:bg-white/15 transition flex items-center gap-2"
      >
        <i class="bi bi-plus-lg"></i> Hochladen
      </button>
      {#if selectedFiles.size > 0}
        <button
          on:click={deleteSelected}
          class="px-6 py-3 rounded-lg bg-red-500/20 hover:bg-red-500/30 text-red-400 transition flex items-center gap-2"
        >
          <i class="bi bi-trash"></i> Löschen ({selectedFiles.size})
        </button>
      {/if}
    </div>

    <!-- Drag & Drop Zone -->
    <div
      on:dragover|preventDefault={() => (dragOver = true)}
      on:dragleave={() => (dragOver = false)}
      on:drop={handleDrop}
      class="glass p-12 rounded-2xl mb-8 border-2 border-dashed transition {dragOver
        ? 'border-blue-400 bg-blue-500/10'
        : 'border-white/20'}"
    >
      <div class="text-center">
        <i class="bi bi-cloud-arrow-up text-4xl text-blue-400 mb-4 block"></i>
        <p class="text-lg font-semibold mb-2">Dateien hierher ziehen</p>
        <p class="text-slate-400 text-sm">
          oder <button
            class="text-blue-400 hover:underline"
            on:click={() => (showUploadModal = true)}>Dateien wählen</button
          >
        </p>
      </div>
    </div>

    <!-- Files Grid -->
    <div class="grid grid-cols-1 gap-6">
      {#each filteredFiles as file (file.id)}
        <div
          class="glass p-6 rounded-xl hover:bg-white/15 transition group flex items-center gap-6"
        >
          <input
            type="checkbox"
            checked={selectedFiles.has(file.id)}
            on:change={() => toggleSelect(file.id)}
            class="w-5 h-5 cursor-pointer"
          />

          <div
            class="w-12 h-12 rounded-lg bg-gradient-to-br from-blue-500/30 to-purple-600/20 flex items-center justify-center flex-shrink-0"
          >
            <i
              class="bi {getIcon(file.type)} text-lg {file.type === 'pdf'
                ? 'text-red-400'
                : 'text-blue-400'}"
            ></i>
          </div>

          <div class="flex-1 min-w-0">
            <p class="font-semibold truncate">{file.name}</p>
            <p class="text-sm text-slate-400">{file.size} • {file.date}</p>
          </div>

          <button
            on:click={() => toggleFavorite(file.id)}
            class="p-2 rounded-lg hover:bg-white/10 transition"
          >
            <i
              class="bi {file.favorite
                ? 'bi-star-fill'
                : 'bi-star'} {file.favorite ? 'text-yellow-400' : ''}"
            ></i>
          </button>

          <button
            class="p-2 rounded-lg hover:bg-white/10 transition opacity-0 group-hover:opacity-100 transition"
          >
            <i class="bi bi-three-dots-vertical"></i>
          </button>
        </div>
      {/each}
    </div>

    {#if filteredFiles.length === 0}
      <div class="text-center py-12">
        <i class="bi bi-inbox text-4xl text-slate-400 mb-4 block"></i>
        <p class="text-slate-400">Keine Dateien gefunden</p>
      </div>
    {/if}
  </div>

  <!-- Upload Modal -->
  {#if showUploadModal}
    <div
      class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    >
      <div class="glass p-8 rounded-2xl max-w-md w-full">
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-2xl font-bold">Datei hochladen</h2>
          <button
            on:click={() => (showUploadModal = false)}
            class="text-slate-400 hover:text-white"
          >
            <i class="bi bi-x text-2xl"></i>
          </button>
        </div>

        <div
          class="border-2 border-dashed border-white/20 rounded-lg p-8 text-center mb-6"
        >
          <i class="bi bi-cloud-arrow-up text-3xl text-blue-400 mb-3 block"></i>
          <p class="text-sm text-slate-400 mb-4">
            Datei auswählen oder hierher ziehen
          </p>
          <button
            class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition"
          >
            Durchsuchen
          </button>
        </div>

        <div class="flex gap-3">
          <button
            on:click={() => (showUploadModal = false)}
            class="flex-1 px-4 py-2 glass rounded-lg hover:bg-white/10 transition"
          >
            Abbrechen
          </button>
          <button
            class="flex-1 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition"
          >
            Hochladen
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>
