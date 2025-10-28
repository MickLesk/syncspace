<script>
  import { createEventDispatcher } from "svelte";
  import Icon from "./Icon.svelte";

  export let visible = false;

  const dispatch = createEventDispatcher();

  // Filter state
  let fileTypes = {
    images: false,
    documents: false,
    videos: false,
    audio: false,
    archives: false,
    other: false,
  };

  let sizeRange = { min: 0, max: Infinity };
  let customSizeMin = "";
  let customSizeMax = "";
  let sizeUnit = "MB"; // KB, MB, GB

  let dateRange = { from: "", to: "" };

  /**
   * Apply filters and emit event
   */
  function applyFilters() {
    const filters = {
      fileTypes: Object.keys(fileTypes).filter((type) => fileTypes[type]),
      sizeRange: {
        min: parseSize(customSizeMin, sizeUnit),
        max: parseSize(customSizeMax, sizeUnit),
      },
      dateRange: {
        from: dateRange.from ? new Date(dateRange.from) : null,
        to: dateRange.to ? new Date(dateRange.to) : null,
      },
    };

    dispatch("apply", filters);
    visible = false;
  }

  /**
   * Reset all filters
   */
  function resetFilters() {
    fileTypes = {
      images: false,
      documents: false,
      videos: false,
      audio: false,
      archives: false,
      other: false,
    };
    customSizeMin = "";
    customSizeMax = "";
    sizeUnit = "MB";
    dateRange = { from: "", to: "" };

    dispatch("reset");
  }

  /**
   * Parse size string to bytes
   */
  function parseSize(value, unit) {
    if (!value || value === "") return unit === "min" ? 0 : Infinity;

    const num = parseFloat(value);
    if (isNaN(num)) return unit === "min" ? 0 : Infinity;

    const multipliers = {
      KB: 1024,
      MB: 1024 * 1024,
      GB: 1024 * 1024 * 1024,
    };

    return num * (multipliers[unit] || 1);
  }

  /**
   * Close on backdrop click
   */
  function handleBackdropClick(e) {
    if (e.target === e.currentTarget) {
      visible = false;
    }
  }
</script>

{#if visible}
  <div class="filter-overlay" onclick={handleBackdropClick}>
    <div class="filter-panel">
      <div class="filter-header">
        <h3>Advanced Filters</h3>
        <button class="btn-close" onclick={() => (visible = false)}>
          <Icon name="x" size={20} />
        </button>
      </div>

      <div class="filter-content">
        <!-- File Type Filters -->
        <div class="filter-section">
          <h4>File Types</h4>
          <div class="filter-chips">
            <label class="filter-chip" class:active={fileTypes.images}>
              <input type="checkbox" bind:checked={fileTypes.images} />
              <Icon name="image" size={16} />
              <span>Images</span>
            </label>
            <label class="filter-chip" class:active={fileTypes.documents}>
              <input type="checkbox" bind:checked={fileTypes.documents} />
              <Icon name="file-text" size={16} />
              <span>Documents</span>
            </label>
            <label class="filter-chip" class:active={fileTypes.videos}>
              <input type="checkbox" bind:checked={fileTypes.videos} />
              <Icon name="play-circle" size={16} />
              <span>Videos</span>
            </label>
            <label class="filter-chip" class:active={fileTypes.audio}>
              <input type="checkbox" bind:checked={fileTypes.audio} />
              <Icon name="music-note" size={16} />
              <span>Audio</span>
            </label>
            <label class="filter-chip" class:active={fileTypes.archives}>
              <input type="checkbox" bind:checked={fileTypes.archives} />
              <Icon name="archive" size={16} />
              <span>Archives</span>
            </label>
            <label class="filter-chip" class:active={fileTypes.other}>
              <input type="checkbox" bind:checked={fileTypes.other} />
              <Icon name="file-earmark" size={16} />
              <span>Other</span>
            </label>
          </div>
        </div>

        <!-- Size Range Filter -->
        <div class="filter-section">
          <h4>File Size</h4>
          <div class="size-inputs">
            <div class="input-group">
              <label>Min</label>
              <input
                type="number"
                bind:value={customSizeMin}
                placeholder="0"
                min="0"
              />
            </div>
            <span class="input-separator">—</span>
            <div class="input-group">
              <label>Max</label>
              <input
                type="number"
                bind:value={customSizeMax}
                placeholder="∞"
                min="0"
              />
            </div>
            <select bind:value={sizeUnit} class="unit-select">
              <option value="KB">KB</option>
              <option value="MB">MB</option>
              <option value="GB">GB</option>
            </select>
          </div>
        </div>

        <!-- Date Range Filter -->
        <div class="filter-section">
          <h4>Modified Date</h4>
          <div class="date-inputs">
            <div class="input-group">
              <label>From</label>
              <input type="date" bind:value={dateRange.from} />
            </div>
            <div class="input-group">
              <label>To</label>
              <input type="date" bind:value={dateRange.to} />
            </div>
          </div>
        </div>
      </div>

      <div class="filter-actions">
        <button class="btn-secondary" onclick={resetFilters}> Reset </button>
        <button class="btn-primary" onclick={applyFilters}>
          Apply Filters
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .filter-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.2s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .filter-panel {
    background: var(--md-sys-color-surface);
    border-radius: 28px;
    width: 90%;
    max-width: 600px;
    max-height: 80vh;
    overflow: hidden;
    box-shadow: var(--md-elevation-5);
    animation: slideUp 0.3s ease-out;
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .filter-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 24px;
    border-bottom: 1px solid var(--md-sys-color-outline-variant);
  }

  .filter-header h3 {
    margin: 0;
    font-size: 24px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
  }

  .btn-close {
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 8px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--md-sys-color-on-surface-variant);
    transition: all 0.2s;
  }

  .btn-close:hover {
    background: var(--md-sys-color-surface-variant);
  }

  .filter-content {
    padding: 24px;
    max-height: calc(80vh - 180px);
    overflow-y: auto;
  }

  .filter-section {
    margin-bottom: 32px;
  }

  .filter-section:last-child {
    margin-bottom: 0;
  }

  .filter-section h4 {
    margin: 0 0 16px 0;
    font-size: 16px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
  }

  .filter-chips {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
  }

  .filter-chip {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 16px;
    border-radius: 20px;
    border: 1px solid var(--md-sys-color-outline);
    background: transparent;
    cursor: pointer;
    transition: all 0.2s;
    user-select: none;
  }

  .filter-chip input[type="checkbox"] {
    display: none;
  }

  .filter-chip:hover {
    background: var(--md-sys-color-surface-variant);
  }

  .filter-chip.active {
    background: var(--md-sys-color-primary-container);
    border-color: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary-container);
  }

  .size-inputs,
  .date-inputs {
    display: flex;
    gap: 12px;
    align-items: flex-end;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
    flex: 1;
  }

  .input-group label {
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
    font-weight: 500;
  }

  .input-group input {
    padding: 12px 16px;
    border-radius: 12px;
    border: 1px solid var(--md-sys-color-outline);
    background: var(--md-sys-color-surface);
    color: var(--md-sys-color-on-surface);
    font-family: "Roboto", sans-serif;
    font-size: 14px;
    transition: all 0.2s;
  }

  .input-group input:focus {
    outline: none;
    border-color: var(--md-sys-color-primary);
    box-shadow: 0 0 0 3px var(--md-sys-color-primary-container);
  }

  .input-separator {
    padding-bottom: 12px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .unit-select {
    padding: 12px 16px;
    border-radius: 12px;
    border: 1px solid var(--md-sys-color-outline);
    background: var(--md-sys-color-surface);
    color: var(--md-sys-color-on-surface);
    font-family: "Roboto", sans-serif;
    font-size: 14px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .unit-select:focus {
    outline: none;
    border-color: var(--md-sys-color-primary);
  }

  .filter-actions {
    display: flex;
    gap: 12px;
    padding: 24px;
    border-top: 1px solid var(--md-sys-color-outline-variant);
    justify-content: flex-end;
  }

  .btn-secondary,
  .btn-primary {
    padding: 12px 24px;
    border-radius: 20px;
    font-family: "Roboto", sans-serif;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    border: none;
  }

  .btn-secondary {
    background: transparent;
    color: var(--md-sys-color-primary);
    border: 1px solid var(--md-sys-color-outline);
  }

  .btn-secondary:hover {
    background: var(--md-sys-color-surface-variant);
  }

  .btn-primary {
    background: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
  }

  .btn-primary:hover {
    background: var(--md-sys-color-primary);
    box-shadow: var(--md-elevation-2);
  }
</style>

