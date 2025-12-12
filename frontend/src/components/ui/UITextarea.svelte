<!-- UITextarea.svelte - Multi-line text input component -->
<script>
  let {
    placeholder = "",
    label = "",
    value = $bindable(""),
    disabled = false,
    required = false,
    error = "",
    rows = 4,
    maxlength,
    class: className = "",
    ...restProps
  } = $props();

  const charCount = $derived(value.length);
</script>

<div class="form-control w-full {className}">
  {#if label}
    <label for="textarea-{label}" class="label">
      <span class="label-text text-white font-medium">
        {label}
        {#if required}<span class="text-red-400">*</span>{/if}
      </span>
      {#if maxlength}
        <span class="label-text-alt text-white/60">{charCount}/{maxlength}</span
        >
      {/if}
    </label>
  {/if}

  <textarea
    id="textarea-{label}"
    {placeholder}
    {disabled}
    {required}
    {rows}
    {maxlength}
    bind:value
    class="textarea textarea-bordered w-full bg-white/5 border-white/20 text-white placeholder-white/40
           focus:border-white/40 focus:bg-white/10 transition-all resize-y
           {error ? 'border-red-400 focus:border-red-400' : ''}"
    {...restProps}
  ></textarea>

  {#if error}
    <div class="label">
      <span class="label-text-alt text-red-400">{error}</span>
    </div>
  {/if}
</div>
