<script>
  import { createEventDispatcher } from "svelte";
  import { currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  import { success } from "../stores/toast";

  let { path = "/", maxVisibleSegments = 3 } = $props();

  const dispatch = createEventDispatcher();

  let segments = $derived(path.split("/").filter(Boolean));
  let showDropdown = $derived(segments.length > maxVisibleSegments);
  let visibleSegments = $derived(
    showDropdown
      ? [segments[0], ...segments.slice(-(maxVisibleSegments - 1))]
      : segments
  );
  let hiddenSegments = $derived(
    showDropdown ? segments.slice(1, -(maxVisibleSegments - 1)) : []
  );

  let dropdownOpen = $state(false);
  let activeActionMenu = $state(null); // Track which breadcrumb has action menu open

  function navigateToHome() {
    dispatch("navigate", { path: "/" });
    dropdownOpen = false;
    activeActionMenu = null;
  }

  function navigateToSegment(index) {
    const newPath = "/" + segments.slice(0, index + 1).join("/") + "/";
    dispatch("navigate", { path: newPath });
    dropdownOpen = false;
    activeActionMenu = null;
  }

  function navigateUp() {
    if (segments.length === 0) return;
    const newPath = "/" + segments.slice(0, -1).join("/") + "/";
    dispatch("navigate", { path: newPath });
    activeActionMenu = null;
  }

  function copyPath() {
    navigator.clipboard
      .writeText(path)
      .then(() => {
        success(t($currentLang, "pathCopied"));
      })
      .catch(() => {
        // Fallback für ältere Browser
        const input = document.createElement("input");
        input.value = path;
        document.body.appendChild(input);
        input.select();
        document.execCommand("copy");
        document.body.removeChild(input);
        success(t($currentLang, "pathCopied"));
      });
    activeActionMenu = null;
  }

  function copySegmentPath(index) {
    const segmentPath = "/" + segments.slice(0, index + 1).join("/") + "/";
    navigator.clipboard
      .writeText(segmentPath)
      .then(() => {
        success(t($currentLang, "pathCopied"));
      })
      .catch(() => {
        const input = document.createElement("input");
        input.value = segmentPath;
        document.body.appendChild(input);
        input.select();
        document.execCommand("copy");
        document.body.removeChild(input);
        success(t($currentLang, "pathCopied"));
      });
    activeActionMenu = null;
  }

  function openInNewTab(index) {
    const segmentPath = "/" + segments.slice(0, index + 1).join("/") + "/";
    // Emit event for parent to handle
    dispatch("openInNewTab", { path: segmentPath });
    activeActionMenu = null;
  }

  function addToFavorites(index) {
    const segmentPath = "/" + segments.slice(0, index + 1).join("/") + "/";
    dispatch("addToFavorites", { path: segmentPath });
    success("Added to favorites");
    activeActionMenu = null;
  }

  function toggleActionMenu(index) {
    activeActionMenu = activeActionMenu === index ? null : index;
  }

  function handleClickOutside(event) {
    if (
      !event.target.closest(".dropdown") &&
      !event.target.closest(".breadcrumb-action-menu")
    ) {
      dropdownOpen = false;
      activeActionMenu = null;
    }
  }
</script>

<svelte:window onclick={handleClickOutside} />

<div class="breadcrumb-nav">
  <!-- Up Button -->
  {#if segments.length > 0}
    <button
      class="btn btn-sm btn-circle btn-ghost"
      onclick={navigateUp}
      title={t($currentLang, "goUpOneLevel")}
      aria-label={t($currentLang, "goUpOneLevel")}
    >
      <i class="bi bi-arrow-up"></i>
    </button>
  {/if}

  <!-- Breadcrumbs -->
  <div class="breadcrumbs text-sm flex-1">
    <ul>
      <!-- Home -->
      <li>
        <div class="breadcrumb-item-wrapper">
          <button class="breadcrumb-item" onclick={navigateToHome}>
            <i class="bi bi-house-fill"></i>
            <span class="ml-1">{t($currentLang, "home")}</span>
          </button>
          <div class="breadcrumb-actions">
            <button
              class="breadcrumb-action-btn"
              onclick={(e) => {
                e.stopPropagation();
                toggleActionMenu("home");
              }}
              title="Quick actions"
            >
              <i class="bi bi-three-dots-vertical"></i>
            </button>
            {#if activeActionMenu === "home"}
              <div class="breadcrumb-action-menu">
                <button onclick={() => copySegmentPath(-1)}>
                  <i class="bi bi-clipboard"></i>
                  <span>Copy Path</span>
                  <kbd class="kbd kbd-xs">Ctrl+C</kbd>
                </button>
                <button onclick={() => addToFavorites(-1)}>
                  <i class="bi bi-star"></i>
                  <span>Add to Favorites</span>
                  <kbd class="kbd kbd-xs">F</kbd>
                </button>
              </div>
            {/if}
          </div>
        </div>
      </li>

      {#if segments.length > 0}
        <!-- First segment (always visible if exists) -->
        {#if showDropdown}
          <li>
            <button
              class="breadcrumb-item"
              onclick={() => navigateToSegment(0)}
            >
              {segments[0]}
            </button>
          </li>
        {/if}

        <!-- Dropdown for hidden segments -->
        {#if showDropdown && hiddenSegments.length > 0}
          <li>
            <div class="dropdown dropdown-hover">
              <button
                tabindex="0"
                class="breadcrumb-item dropdown-trigger"
                onclick={(e) => {
                  e.stopPropagation();
                  dropdownOpen = !dropdownOpen;
                }}
                aria-label="Show hidden path segments"
              >
                <i class="bi bi-three-dots"></i>
              </button>
              {#if dropdownOpen}
                <ul
                  role="menu"
                  class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52 z-50"
                  onclick={(e) => e.stopPropagation()}
                  onkeydown={(e) => {
                    if (e.key === 'Escape') dropdownOpen = false;
                  }}
                >
                  {#each hiddenSegments as segment, i}
                    <li>
                      <button onclick={() => navigateToSegment(i + 1)}>
                        <i class="bi bi-folder-fill text-warning"></i>
                        {segment}
                      </button>
                    </li>
                  {/each}
                </ul>
              {/if}
            </div>
          </li>
        {/if}

        <!-- Visible segments -->
        {#if !showDropdown}
          {#each segments as segment, i}
            <li>
              <div class="breadcrumb-item-wrapper">
                <button
                  class="breadcrumb-item"
                  class:breadcrumb-active={i === segments.length - 1}
                  onclick={() => navigateToSegment(i)}
                >
                  {segment}
                </button>
                <div class="breadcrumb-actions">
                  <button
                    class="breadcrumb-action-btn"
                    onclick={(e) => {
                      e.stopPropagation();
                      toggleActionMenu(i);
                    }}
                    title="Quick actions"
                  >
                    <i class="bi bi-three-dots-vertical"></i>
                  </button>
                  {#if activeActionMenu === i}
                    <div class="breadcrumb-action-menu">
                      <button onclick={() => copySegmentPath(i)}>
                        <i class="bi bi-clipboard"></i>
                        <span>Copy Path</span>
                        <kbd class="kbd kbd-xs">Ctrl+C</kbd>
                      </button>
                      <button onclick={() => openInNewTab(i)}>
                        <i class="bi bi-box-arrow-up-right"></i>
                        <span>Open in New Tab</span>
                        <kbd class="kbd kbd-xs">Ctrl+T</kbd>
                      </button>
                      <button onclick={() => addToFavorites(i)}>
                        <i class="bi bi-star"></i>
                        <span>Add to Favorites</span>
                        <kbd class="kbd kbd-xs">F</kbd>
                      </button>
                    </div>
                  {/if}
                </div>
              </div>
            </li>
          {/each}
        {:else}
          <!-- Last segments when using dropdown -->
          {#each segments.slice(-(maxVisibleSegments - 1)) as segment, i}
            <li>
              <div class="breadcrumb-item-wrapper">
                <button
                  class="breadcrumb-item"
                  class:breadcrumb-active={i === maxVisibleSegments - 2}
                  onclick={() =>
                    navigateToSegment(
                      segments.length - (maxVisibleSegments - 1) + i
                    )}
                >
                  {segment}
                </button>
                <div class="breadcrumb-actions">
                  <button
                    class="breadcrumb-action-btn"
                    onclick={(e) => {
                      e.stopPropagation();
                      toggleActionMenu(`last-${i}`);
                    }}
                    title="Quick actions"
                  >
                    <i class="bi bi-three-dots-vertical"></i>
                  </button>
                  {#if activeActionMenu === `last-${i}`}
                    <div class="breadcrumb-action-menu">
                      <button
                        onclick={() =>
                          copySegmentPath(
                            segments.length - (maxVisibleSegments - 1) + i
                          )}
                      >
                        <i class="bi bi-clipboard"></i>
                        <span>Copy Path</span>
                        <kbd class="kbd kbd-xs">Ctrl+C</kbd>
                      </button>
                      <button
                        onclick={() =>
                          openInNewTab(
                            segments.length - (maxVisibleSegments - 1) + i
                          )}
                      >
                        <i class="bi bi-box-arrow-up-right"></i>
                        <span>Open in New Tab</span>
                        <kbd class="kbd kbd-xs">Ctrl+T</kbd>
                      </button>
                      <button
                        onclick={() =>
                          addToFavorites(
                            segments.length - (maxVisibleSegments - 1) + i
                          )}
                      >
                        <i class="bi bi-star"></i>
                        <span>Add to Favorites</span>
                        <kbd class="kbd kbd-xs">F</kbd>
                      </button>
                    </div>
                  {/if}
                </div>
              </div>
            </li>
          {/each}
        {/if}
      {/if}
    </ul>
  </div>

  <!-- Copy Path Button -->
  <button
    class="btn btn-sm btn-ghost gap-2"
    onclick={copyPath}
    title={t($currentLang, "copyFullPath")}
    aria-label={t($currentLang, "copyFullPath")}
  >
    <i class="bi bi-clipboard"></i>
    <span class="hidden sm:inline">{t($currentLang, "copy")}</span>
  </button>
</div>

<style>
  .breadcrumb-nav {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    background: var(--fallback-b1, oklch(var(--b1) / 1));
    border: 1px solid var(--fallback-bc, oklch(var(--bc) / 0.2));
    border-radius: var(--rounded-box, 1rem);
    margin-bottom: 1.5rem;
  }

  .breadcrumb-item {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.5rem;
    border-radius: var(--rounded-btn, 0.5rem);
    transition: all 0.2s ease;
    background: transparent;
    border: none;
    cursor: pointer;
    color: var(--fallback-bc, oklch(var(--bc) / 0.6));
    font-size: 0.875rem;
  }

  .breadcrumb-item:hover {
    background: var(--fallback-bc, oklch(var(--bc) / 0.1));
    color: var(--fallback-bc, oklch(var(--bc) / 1));
  }

  .breadcrumb-active {
    font-weight: 600;
    color: var(--fallback-p, oklch(var(--p) / 1));
  }

  .dropdown-trigger {
    padding: 0.25rem 0.5rem;
  }

  .dropdown-content {
    margin-top: 0.5rem;
    border: 1px solid var(--fallback-bc, oklch(var(--bc) / 0.2));
  }

  @media (max-width: 640px) {
    .breadcrumb-nav {
      flex-wrap: wrap;
      gap: 0.25rem;
    }
  }

  /* Breadcrumb item wrapper for actions */
  .breadcrumb-item-wrapper {
    position: relative;
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
  }

  .breadcrumb-item-wrapper:hover .breadcrumb-actions {
    opacity: 1;
  }

  .breadcrumb-actions {
    opacity: 0;
    transition: opacity 0.2s ease;
  }

  .breadcrumb-action-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    padding: 0;
    border: none;
    background: transparent;
    color: hsl(var(--bc) / 0.4);
    border-radius: 0.25rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .breadcrumb-action-btn:hover {
    background: hsl(var(--bc) / 0.1);
    color: hsl(var(--bc) / 0.8);
  }

  .breadcrumb-action-btn i {
    font-size: 0.875rem;
  }

  .breadcrumb-action-menu {
    position: absolute;
    top: calc(100% + 0.5rem);
    left: 0;
    min-width: 16rem;
    background: hsl(var(--b1));
    border: 2px solid hsl(var(--bc) / 0.1);
    border-radius: 0.75rem;
    box-shadow:
      0 10px 25px -5px rgba(0, 0, 0, 0.1),
      0 8px 10px -6px rgba(0, 0, 0, 0.1);
    padding: 0.5rem;
    z-index: 1000;
    animation: slideDown 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .breadcrumb-action-menu button {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    padding: 0.625rem 0.875rem;
    border: none;
    background: transparent;
    color: hsl(var(--bc));
    border-radius: 0.5rem;
    cursor: pointer;
    transition: all 0.2s ease;
    font-size: 0.875rem;
    text-align: left;
  }

  .breadcrumb-action-menu button:hover {
    background: hsl(var(--p) / 0.1);
    color: hsl(var(--p));
  }

  .breadcrumb-action-menu button i {
    font-size: 1rem;
    flex-shrink: 0;
  }

  .breadcrumb-action-menu button span {
    flex: 1;
  }

  .breadcrumb-action-menu button .kbd {
    margin-left: auto;
  }
</style>
