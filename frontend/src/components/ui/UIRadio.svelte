<!-- UIRadio.svelte - Radio button group component -->
<script>
  let {
    options = [],
    value = $bindable(""),
    label = "",
    name = "",
    disabled = false,
    error = "",
    class: className = "",
  } = $props();

  // Normalize options to always be {value, label} objects
  const normalizedOptions = $derived(
    options.map((opt) =>
      typeof opt === "string" ? { value: opt, label: opt } : opt
    )
  );
</script>

<div class="form-control w-full {className}">
  {#if label}
    <label class="label">
      <span class="label-text text-white font-medium">{label}</span>
    </label>
  {/if}

  <div class="flex flex-col gap-2">
    {#each normalizedOptions as option}
      <label
        class="label cursor-pointer justify-start gap-3 bg-white/5 border border-white/10 rounded-lg px-4 py-3 hover:bg-white/10 transition-colors"
      >
        <input
          type="radio"
          {name}
          value={option.value}
          bind:group={value}
          {disabled}
          class="radio radio-primary bg-white/5 border-white/20 checked:bg-blue-500 checked:border-blue-500"
        />
        <div class="flex-1">
          <span class="label-text text-white font-medium">{option.label}</span>
          {#if option.description}
            <p class="text-sm text-white/60 mt-0.5">{option.description}</p>
          {/if}
        </div>
      </label>
    {/each}
  </div>

  {#if error}
    <label class="label">
      <span class="label-text-alt text-red-400">{error}</span>
    </label>
  {/if}
</div>
