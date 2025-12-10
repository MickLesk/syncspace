<script>
  import { createEventDispatcher } from "svelte";
  import { currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
  const dispatch = createEventDispatcher();

  let { path = "", onDrop = null } = $props(); // Svelte 5 runes syntax

  let dropTarget = $state(null); // Track which breadcrumb is a drop target

  // Utility: split path into segments and decode URL encoding
  function getPathSegments(path) {
    if (!path || path === "/") return [];
    return path
      .replace(/^\/+|\/+$/g, "")
      .split("/")
      .map((segment) => decodeURIComponent(segment)); // Decode %20 etc.
  }

  // Svelte 5: Use $derived instead of $:
  const segments = $derived(getPathSegments(path || ""));

  // Emit navigation event
  function handleNavigate(idx) {
    if (idx === -1) {
      // Home clicked
      dispatch("navigate", { path: "/" });
    } else {
      const newPath = "/" + segments.slice(0, idx + 1).join("/") + "/";
      dispatch("navigate", { path: newPath });
    }
  }

  // Build path for a specific segment (for drag & drop)
  function getSegmentPath(idx) {
    if (idx === -1) return "/";
    return "/" + segments.slice(0, idx + 1).join("/") + "/";
  }

  // Handle drag over
  function handleDragOver(e, idx) {
    e.preventDefault();
    e.dataTransfer.dropEffect = "move";
    dropTarget = idx;
  }

  // Handle drag leave
  function handleDragLeave(idx) {
    if (dropTarget === idx) {
      dropTarget = null;
    }
  }

  // Handle drop
  function handleDrop(e, idx) {
    e.preventDefault();
    dropTarget = null;

    if (!onDrop) return;

    try {
      const draggedFileData = e.dataTransfer.getData("text/plain");
      const draggedFile = JSON.parse(draggedFileData);
      const destinationPath = getSegmentPath(idx);

      onDrop(draggedFile, destinationPath);
    } catch (err) {
      console.error("[Breadcrumbs] Error handling drop:", err);
    }
  }
</script>

<nav
  class="flex items-center gap-1 text-sm overflow-x-auto py-2 px-1 select-none scrollbar-modern"
  aria-label="Breadcrumb"
>
  <!-- Home Button -->
  <button
    type="button"
    class="flex items-center gap-2 px-3 py-1.5 rounded-lg font-medium transition-all whitespace-nowrap text-green-500 dark:text-green-400 border-2 border-transparent hover:bg-green-500/10 hover:border-green-500/20 {dropTarget === -1 ? 'bg-green-500/15 border-green-500 border-dashed animate-pulse' : ''}"
    onclick={() => handleNavigate(-1)}
    ondragover={(e) => handleDragOver(e, -1)}
    ondragleave={() => handleDragLeave(-1)}
    ondrop={(e) => handleDrop(e, -1)}
    aria-label="Home"
  >
    <i class="bi bi-house-door-fill text-lg" aria-hidden="true"></i>
    <span>Home</span>
  </button>

  <!-- Path Segments -->
  {#each segments as segment, i}
    <!-- Separator -->
    <i
      class="bi bi-chevron-right text-gray-400 dark:text-gray-500 text-xs flex-shrink-0"
      aria-hidden="true"
    ></i>

    <!-- Segment Button -->
    <button
      type="button"
      class="flex items-center gap-2 px-3 py-1.5 rounded-lg font-medium transition-all truncate max-w-[200px] whitespace-nowrap border-2 border-transparent {dropTarget === i ? 'bg-green-500/15 border-green-500 border-dashed animate-pulse' : ''} {i === segments.length - 1 ? 'bg-green-500/10 text-green-600 dark:text-green-300 font-semibold' : 'text-green-500 dark:text-green-400 hover:bg-green-500/10 hover:border-green-500/20'}"
      onclick={() => handleNavigate(i)}
      ondragover={(e) => handleDragOver(e, i)}
      ondragleave={() => handleDragLeave(i)}
      ondrop={(e) => handleDrop(e, i)}
      aria-label={segment}
      title={segment}
    >
      <i class="bi bi-folder-fill text-base" aria-hidden="true"></i>
      <span class="truncate">{segment}</span>
    </button>
  {/each}
</nav>

