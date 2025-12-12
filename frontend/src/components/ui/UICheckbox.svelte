<!-- UICheckbox.svelte - Checkbox component -->
<script>
  let {
    checked = $bindable(false),
    label = "",
    description = "",
    disabled = false,
    error = "",
    class: className = "",
    ...restProps
  } = $props();
</script>

<div class="form-control {className}">
  <label class="label cursor-pointer justify-start gap-3">
    <input
      type="checkbox"
      bind:checked
      {disabled}
      class="checkbox checkbox-primary bg-white/5 border-white/20 checked:bg-blue-500 checked:border-blue-500"
      {...restProps}
    />

    <div class="flex-1">
      {#if label}
        <span class="label-text text-white font-medium">{label}</span>
      {/if}
      {#if description}
        <p class="text-sm text-white/60 mt-0.5">{description}</p>
      {/if}
    </div>
  </label>

  {#if error}
    <label class="label">
      <span class="label-text-alt text-red-400">{error}</span>
    </label>
  {/if}
</div>
