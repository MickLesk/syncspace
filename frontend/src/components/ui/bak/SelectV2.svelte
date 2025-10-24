<!--
  SelectV2.svelte - Modern Select Component using DaisyUI
  
  A flexible select dropdown with search, multiple selection, and custom options.
  
  Props:
  - value: string | string[] - Selected value(s)
  - options: Array<{value: string, label: string, disabled?: boolean}> - Select options
  - multiple: boolean - Enable multiple selection (default: false)
  - searchable: boolean - Enable search filter (default: false)
  - placeholder: string - Placeholder text
  - label: string - Input label
  - error: string - Error message
  - success: string - Success message
  - helpText: string - Help text
  - disabled: boolean - Disable select
  - required: boolean - Required field
  - size: "xs" | "sm" | "md" | "lg" (default: "md")
  - glass: boolean - Glass effect (default: false)
  
  Events:
  - on:change - Fired when selection changes
  
  Usage:
  <SelectV2 
    bind:value={selectedCountry}
    options={countries}
    label="Country"
    searchable
  />
-->

<script>
  export let value = "";
  export let options = [];
  export let multiple = false;
  export let searchable = false;
  export let placeholder = "Select an option...";
  export let label = "";
  export let error = "";
  export let success = "";
  export let helpText = "";
  export let disabled = false;
  export let required = false;
  export let size = "md";
  export let glass = false;

  let searchQuery = "";
  let isOpen = false;

  $: hasError = !!error;
  $: hasSuccess = !!success && !hasError;

  $: sizeClasses = {
    xs: "select-xs",
    sm: "select-sm",
    md: "select-md",
    lg: "select-lg",
  };

  $: computedClass = [
    "select",
    "select-bordered",
    "w-full",
    sizeClasses[size],
    hasError ? "select-error" : "",
    hasSuccess ? "select-success" : "",
    glass ? "glass-select" : "",
  ]
    .filter(Boolean)
    .join(" ");

  $: filteredOptions = searchable
    ? options.filter((opt) =>
        opt.label.toLowerCase().includes(searchQuery.toLowerCase())
      )
    : options;

  function handleChange(e) {
    const newValue = e.target.value;
    value = newValue;
    e.target.dispatchEvent(
      new CustomEvent("selectchange", {
        detail: { value: newValue },
        bubbles: true,
      })
    );
  }

  function handleMultipleSelect(optionValue) {
    const currentValues = Array.isArray(value) ? value : [];
    if (currentValues.includes(optionValue)) {
      value = currentValues.filter((v) => v !== optionValue);
    } else {
      value = [...currentValues, optionValue];
    }
  }

  function getDisplayValue() {
    if (multiple && Array.isArray(value)) {
      const selectedOptions = options.filter((opt) =>
        value.includes(opt.value)
      );
      return selectedOptions.map((opt) => opt.label).join(", ") || placeholder;
    }
    const selected = options.find((opt) => opt.value === value);
    return selected?.label || placeholder;
  }
</script>

<div class="form-control">
  {#if label}
    <label class="label">
      <span class="label-text">
        {label}
        {#if required}<span class="text-error">*</span>{/if}
      </span>
    </label>
  {/if}

  {#if searchable || multiple}
    <!-- Custom Dropdown for Searchable/Multiple -->
    <div class="dropdown" class:dropdown-open={isOpen}>
      <button
        type="button"
        class={computedClass}
        on:click={() => (isOpen = !isOpen)}
        {disabled}
      >
        <span class="flex-1 text-left">{getDisplayValue()}</span>
        <i class="bi bi-chevron-down ml-2"></i>
      </button>

      {#if isOpen}
        <div
          class="dropdown-content z-10 menu p-2 shadow bg-base-100 rounded-box w-full mt-1"
        >
          {#if searchable}
            <div class="p-2 sticky top-0 bg-base-100">
              <input
                type="text"
                placeholder="Search..."
                class="input input-bordered input-sm w-full"
                bind:value={searchQuery}
                on:click|stopPropagation
              />
            </div>
          {/if}

          <ul class="max-h-60 overflow-y-auto">
            {#each filteredOptions as option}
              <li>
                <button
                  type="button"
                  class="flex items-center gap-2"
                  class:active={multiple
                    ? Array.isArray(value) && value.includes(option.value)
                    : value === option.value}
                  disabled={option.disabled}
                  on:click={() => {
                    if (multiple) {
                      handleMultipleSelect(option.value);
                    } else {
                      value = option.value;
                      isOpen = false;
                    }
                  }}
                >
                  {#if multiple}
                    <input
                      type="checkbox"
                      class="checkbox checkbox-sm"
                      checked={Array.isArray(value) &&
                        value.includes(option.value)}
                      on:click|stopPropagation
                    />
                  {/if}
                  {option.label}
                </button>
              </li>
            {/each}
          </ul>
        </div>
      {/if}
    </div>
  {:else}
    <!-- Native Select -->
    <select
      class={computedClass}
      bind:value
      on:change={handleChange}
      {disabled}
      {required}
    >
      {#if placeholder}
        <option value="" disabled selected={!value}>{placeholder}</option>
      {/if}
      {#each options as option}
        <option value={option.value} disabled={option.disabled}>
          {option.label}
        </option>
      {/each}
    </select>
  {/if}

  {#if error}
    <label class="label">
      <span class="label-text-alt text-error">{error}</span>
    </label>
  {:else if success}
    <label class="label">
      <span class="label-text-alt text-success">{success}</span>
    </label>
  {:else if helpText}
    <label class="label">
      <span class="label-text-alt">{helpText}</span>
    </label>
  {/if}
</div>

<style>
  .glass-select {
    background: var(--glass-background);
    backdrop-filter: blur(var(--glass-blur));
    border: 1px solid var(--glass-border);
  }

  .dropdown {
    position: relative;
    width: 100%;
  }

  .dropdown-content {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    z-index: var(--z-dropdown);
    animation: slideDown var(--duration-200) var(--ease-standard);
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .menu li button.active {
    background: var(--color-primary);
    color: var(--color-on-primary);
  }
</style>
