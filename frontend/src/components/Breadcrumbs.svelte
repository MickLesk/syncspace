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
  class="flex items-center gap-1 text-sm overflow-x-auto py-2 px-1"
  aria-label="Breadcrumb"
>
  <!-- Home Button -->
  <button
    type="button"
    class="breadcrumb-item flex items-center gap-2 px-3 py-1.5 rounded-lg font-medium transition-all"
    class:drop-target={dropTarget === -1}
    onclick={() => handleNavigate(-1)}
    ondragover={(e) => handleDragOver(e, -1)}
    ondragleave={() => handleDragLeave(-1)}
    ondrop={(e) => handleDrop(e, -1)}
    aria-label="Home"
  >
    <i class="bi bi-house-door-fill text-lg"></i>
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
      class="breadcrumb-item flex items-center gap-2 px-3 py-1.5 rounded-lg font-medium transition-all truncate max-w-[200px]"
      class:drop-target={dropTarget === i}
      class:last-segment={i === segments.length - 1}
      onclick={() => handleNavigate(i)}
      ondragover={(e) => handleDragOver(e, i)}
      ondragleave={() => handleDragLeave(i)}
      ondrop={(e) => handleDrop(e, i)}
      aria-label={segment}
      title={segment}
    >
      <i class="bi bi-folder-fill text-base"></i>
      <span class="truncate">{segment}</span>
    </button>
  {/each}
</nav>

<style>
  nav {
    user-select: none;
    scrollbar-width: thin;
  }

  nav::-webkit-scrollbar {
    height: 4px;
  }

  nav::-webkit-scrollbar-thumb {
    background: rgba(156, 163, 175, 0.3);
    border-radius: 2px;
  }

  nav::-webkit-scrollbar-thumb:hover {
    background: rgba(156, 163, 175, 0.5);
  }

  .breadcrumb-item {
    background: transparent;
    color: rgb(59, 130, 246);
    border: 2px solid transparent;
    white-space: nowrap;
  }

  .breadcrumb-item:hover {
    background: rgba(59, 130, 246, 0.1);
    border-color: rgba(59, 130, 246, 0.2);
  }

  .breadcrumb-item.last-segment {
    background: rgba(59, 130, 246, 0.1);
    color: rgb(37, 99, 235);
    font-weight: 600;
  }

  .breadcrumb-item.drop-target {
    background: rgba(16, 185, 129, 0.15);
    border-color: rgb(16, 185, 129);
    border-style: dashed;
    animation: pulse 1.5s ease-in-out infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      border-color: rgb(16, 185, 129);
      background: rgba(16, 185, 129, 0.15);
    }
    50% {
      border-color: rgb(5, 150, 105);
      background: rgba(16, 185, 129, 0.25);
    }
  }

  /* Dark Mode */
  :global(.dark) .breadcrumb-item {
    color: rgb(96, 165, 250);
  }

  :global(.dark) .breadcrumb-item:hover {
    background: rgba(96, 165, 250, 0.1);
    border-color: rgba(96, 165, 250, 0.2);
  }

  :global(.dark) .breadcrumb-item.last-segment {
    background: rgba(96, 165, 250, 0.15);
    color: rgb(147, 197, 253);
  }

  :global(.dark) .breadcrumb-item.drop-target {
    background: rgba(16, 185, 129, 0.2);
    border-color: rgb(16, 185, 129);
  }
</style>
