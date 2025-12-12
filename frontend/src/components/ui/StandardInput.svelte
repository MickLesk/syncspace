<!-- StandardInput.svelte - Einheitliche Input-Komponente -->
<script>
  let {
    type = "text",
    placeholder = "",
    label = "",
    value = $bindable(""),
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

  <input
    {type}
    {placeholder}
    {disabled}
    {required}
    bind:value
    class="input input-bordered w-full bg-white/5 border-white/20 text-white placeholder-white/40
           focus:border-white/40 focus:bg-white/10 transition-all
           {error ? 'border-red-400 focus:border-red-400' : ''}"
    {...restProps}
  />

  {#if error}
    <label class="label">
      <span class="label-text-alt text-red-400">{error}</span>
    </label>
  {/if}
</div>
