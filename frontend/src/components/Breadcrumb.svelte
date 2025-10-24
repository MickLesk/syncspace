<script>
  import { createEventDispatcher } from "svelte";
  import { currentLang } from "../stores/ui.js";
  import { t } from "../i18n.js";
  import { success } from "../stores/toast";

  export let path = "/";
  export let maxVisibleSegments = 3;

  const dispatch = createEventDispatcher();

  $: segments = path.split("/").filter(Boolean);
  $: showDropdown = segments.length > maxVisibleSegments;
  $: visibleSegments = showDropdown
    ? [segments[0], ...segments.slice(-(maxVisibleSegments - 1))]
    : segments;
  $: hiddenSegments = showDropdown
    ? segments.slice(1, -(maxVisibleSegments - 1))
    : [];

  let dropdownOpen = false;

  function navigateToHome() {
    dispatch("navigate", { path: "/" });
    dropdownOpen = false;
  }

  function navigateToSegment(index) {
    const newPath = "/" + segments.slice(0, index + 1).join("/") + "/";
    dispatch("navigate", { path: newPath });
    dropdownOpen = false;
  }

  function navigateUp() {
    if (segments.length === 0) return;
    const newPath = "/" + segments.slice(0, -1).join("/") + "/";
    dispatch("navigate", { path: newPath });
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
  }

  function handleClickOutside(event) {
    if (!event.target.closest(".dropdown")) {
      dropdownOpen = false;
    }
  }
</script>

<svelte:window on:click={handleClickOutside} />

<div class="breadcrumb-nav">
  <!-- Up Button -->
  {#if segments.length > 0}
    <button
      class="btn btn-sm btn-circle btn-ghost"
      on:click={navigateUp}
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
        <button class="breadcrumb-item" on:click={navigateToHome}>
          <i class="bi bi-house-fill"></i>
          <span class="ml-1">{t($currentLang, "home")}</span>
        </button>
      </li>

      {#if segments.length > 0}
        <!-- First segment (always visible if exists) -->
        {#if showDropdown}
          <li>
            <button
              class="breadcrumb-item"
              on:click={() => navigateToSegment(0)}
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
                on:click|stopPropagation={() => (dropdownOpen = !dropdownOpen)}
              >
                <i class="bi bi-three-dots"></i>
              </button>
              {#if dropdownOpen}
                <ul
                  tabindex="0"
                  class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52 z-50"
                  on:click|stopPropagation
                >
                  {#each hiddenSegments as segment, i}
                    <li>
                      <button on:click={() => navigateToSegment(i + 1)}>
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
              <button
                class="breadcrumb-item"
                class:breadcrumb-active={i === segments.length - 1}
                on:click={() => navigateToSegment(i)}
              >
                {segment}
              </button>
            </li>
          {/each}
        {:else}
          <!-- Last segments when using dropdown -->
          {#each segments.slice(-(maxVisibleSegments - 1)) as segment, i}
            <li>
              <button
                class="breadcrumb-item"
                class:breadcrumb-active={i === maxVisibleSegments - 2}
                on:click={() =>
                  navigateToSegment(
                    segments.length - (maxVisibleSegments - 1) + i
                  )}
              >
                {segment}
              </button>
            </li>
          {/each}
        {/if}
      {/if}
    </ul>
  </div>

  <!-- Copy Path Button -->
  <button
    class="btn btn-sm btn-ghost gap-2"
    on:click={copyPath}
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
</style>
