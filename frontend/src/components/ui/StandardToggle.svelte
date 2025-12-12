<!-- StandardToggle.svelte - Einheitliche Toggle/Switch-Komponente -->
<script>
  let {
    checked = $bindable(false),
    label = "",
    description = "",
    disabled = false,
    size = "md", // sm, md, lg
    class: className = "",
    ...restProps
  } = $props();

  const sizeClasses = {
    sm: "toggle-sm",
    md: "",
    lg: "toggle-lg",
  };
</script>

<div class="form-control {className}">
  <label class="label cursor-pointer justify-start gap-4">
    <input
      type="checkbox"
      bind:checked
      {disabled}
      class="toggle toggle-primary {sizeClasses[size]}"
      {...restProps}
    />
    <div class="flex-1">
      {#if label}
        <span class="label-text text-white font-medium">{label}</span>
      {/if}
      {#if description}
        <div class="text-sm text-white/60 mt-1">{description}</div>
      {/if}
    </div>
  </label>
</div>
