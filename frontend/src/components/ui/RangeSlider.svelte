<script>
  /**
   * Range Slider Component - SyncSpace Styleguide Compliant
   * Modern slider with labels, markers, and real-time value display
   */
  let {
    value = $bindable(4),
    min = 1,
    max = 8,
    step = 1,
    label = "",
    showValue = true,
    showMarkers = true,
    disabled = false,
    oninput = null,
    onchange = null,
    class: className = "",
    id = `slider-${Math.random().toString(36).substr(2, 9)}`,
  } = $props();

  const markers = $derived(
    Array.from({ length: max - min + 1 }, (_, i) => min + i * step)
  );

  function handleInput(e) {
    value = parseInt(e.target.value);
    oninput?.(e);
  }

  function handleChange(e) {
    onchange?.(e);
  }
</script>

<div class="range-slider-container {className}">
  {#if label || showValue}
    <div class="slider-header">
      {#if label}
        <label for={id} class="slider-label">
          {label}
        </label>
      {/if}
      {#if showValue}
        <span class="slider-value">{value}</span>
      {/if}
    </div>
  {/if}

  <input
    {id}
    type="range"
    {min}
    {max}
    {step}
    {disabled}
    bind:value
    oninput={handleInput}
    onchange={handleChange}
    class="slider-input"
    aria-label={label || "Range slider"}
  />

  {#if showMarkers}
    <div class="slider-markers">
      {#each markers as marker}
        <span class="slider-marker" class:active={value === marker}>
          {marker}
        </span>
      {/each}
    </div>
  {/if}
</div>

<style>
  .range-slider-container {
    width: 100%;
  }

  .slider-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.75rem;
  }

  .slider-label {
    font-size: 0.875rem;
    font-weight: 500;
    color: #4b5563;
  }

  :global(.dark) .slider-label {
    color: #d1d5db;
  }

  .slider-value {
    font-size: 1.125rem;
    font-weight: 700;
    color: #22c55e;
  }

  :global(.dark) .slider-value {
    color: #4ade80;
  }

  .slider-input {
    width: 100%;
    height: 0.5rem;
    background: #e5e7eb;
    border-radius: 0.5rem;
    outline: none;
    appearance: none;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  :global(.dark) .slider-input {
    background: #374151;
  }

  .slider-input:hover {
    background: #d1d5db;
  }

  :global(.dark) .slider-input:hover {
    background: #4b5563;
  }

  .slider-input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Webkit browsers (Chrome, Safari, Edge) */
  .slider-input::-webkit-slider-thumb {
    appearance: none;
    width: 1.25rem;
    height: 1.25rem;
    background: #22c55e;
    border-radius: 50%;
    cursor: pointer;
    transition: all 0.2s ease;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .slider-input::-webkit-slider-thumb:hover {
    background: #16a34a;
    transform: scale(1.1);
    box-shadow: 0 4px 8px rgba(34, 197, 94, 0.3);
  }

  :global(.dark) .slider-input::-webkit-slider-thumb {
    background: #4ade80;
  }

  :global(.dark) .slider-input::-webkit-slider-thumb:hover {
    background: #22c55e;
  }

  /* Firefox */
  .slider-input::-moz-range-thumb {
    width: 1.25rem;
    height: 1.25rem;
    background: #22c55e;
    border: none;
    border-radius: 50%;
    cursor: pointer;
    transition: all 0.2s ease;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .slider-input::-moz-range-thumb:hover {
    background: #16a34a;
    box-shadow: 0 4px 8px rgba(34, 197, 94, 0.3);
  }

  :global(.dark) .slider-input::-moz-range-thumb {
    background: #4ade80;
  }

  :global(.dark) .slider-input::-moz-range-thumb:hover {
    background: #22c55e;
  }

  /* Firefox track */
  .slider-input::-moz-range-track {
    width: 100%;
    height: 0.5rem;
    background: transparent;
    border-radius: 0.5rem;
  }

  .slider-markers {
    display: flex;
    justify-content: space-between;
    margin-top: 0.5rem;
    padding: 0 0.125rem;
  }

  .slider-marker {
    font-size: 0.75rem;
    font-weight: 500;
    color: #9ca3af;
    transition: all 0.2s ease;
    text-align: center;
    min-width: 1.5rem;
  }

  .slider-marker.active {
    color: #22c55e;
    font-weight: 700;
    transform: scale(1.1);
  }

  :global(.dark) .slider-marker {
    color: #6b7280;
  }

  :global(.dark) .slider-marker.active {
    color: #4ade80;
  }
</style>
