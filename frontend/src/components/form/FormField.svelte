<script>
  import { transitions } from "../../lib/DesignSystem.js";

  let {
    label = "",
    description = "",
    hint = "",
    error = "",
    required = false,
    optionalText = "Optional",
    layout = "stacked",
    id = undefined,
    class: className = "",
  } = $props();

  const isInline = $derived(() => layout === "inline");
  const statusColor = $derived(() =>
    error ? "text-red-500" : "text-slate-500 dark:text-slate-400"
  );
</script>

<div
  class={`form-field ${
    isInline
      ? "sm:grid sm:grid-cols-[200px,minmax(0,1fr)] sm:gap-6"
      : "space-y-3"
  } ${className}`}
>
  <div class="flex flex-col gap-1">
    {#if label}
      <label
        for={id}
        class="text-sm font-medium text-slate-700 dark:text-slate-100 flex items-center gap-2"
      >
        {label}
        {#if required}
          <span class="text-xs font-semibold text-red-500">*</span>
        {:else if optionalText}
          <span class="text-xs text-slate-400">{optionalText}</span>
        {/if}
      </label>
    {/if}

    {#if description}
      <p class="text-xs text-slate-500 dark:text-slate-400">{description}</p>
    {/if}

    <slot name="label-actions" />
  </div>

  <div class="space-y-2">
    <div
      class={`group relative flex items-center gap-3 rounded-2xl border bg-white dark:bg-slate-900/70 px-4 py-3 shadow-sm transition-all duration-200 focus-within:ring-2 focus-within:ring-indigo-400 ${
        error
          ? "border-red-300 dark:border-red-700"
          : "border-slate-200 dark:border-slate-800/70 hover:border-indigo-200/70"
      }`}
      aria-invalid={Boolean(error)}
      style={`transition-timing-function:${transitions.easing};`}
    >
      <slot name="leading" />
      <div class="flex-1 min-w-0">
        <slot />
      </div>
      <slot name="trailing" />
    </div>

    {#if hint && !error}
      <p class={`text-xs ${statusColor}`}>{hint}</p>
    {/if}

    {#if error}
      <p class="text-xs text-red-500 font-medium flex items-center gap-2">
        <i class="bi bi-exclamation-diamond"></i>
        {error}
      </p>
    {/if}

    <slot name="support" />
  </div>
</div>
