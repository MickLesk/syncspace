<!-- StandardSelect.svelte - Einheitliche Select-Komponente -->
<script>
  let {
    options = [],
    value = $bindable(""),
    placeholder = "Wählen...",
    label = "",
    disabled = false,
    required = false,
    error = "",
    class: className = "",
    ...restProps
  } = $props();
</script>

<div class="form-control w-full {className}">
  {#if label}
    <label class="label">
      <span class="label-text text-white font-medium">
        {label}
        {#if required}<span class="text-red-400">*</span>{/if}
      </span>
    </label>
  {/if}

  <select
    bind:value
    {disabled}
    {required}
    class="select select-bordered w-full bg-white/5 border-white/20 text-white
           focus:border-white/40 focus:bg-white/10 transition-all
           {error ? 'border-red-400 focus:border-red-400' : ''}"
    {...restProps}
  >
    {#if placeholder}
      <option value="" disabled>{placeholder}</option>
    {/if}
    {#each options as option}
      {#if typeof option === "string"}
        <option value={option}>{option}</option>
      {:else}
        <option value={option.value}>{option.label}</option>
      {/if}
    {/each}
  </select>

  {#if error}
    <label class="label">
      <span class="label-text-alt text-red-400">{error}</span>
    </label>
  {/if}
</div>
