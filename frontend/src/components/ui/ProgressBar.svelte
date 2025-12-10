<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  let {
    value = 0,
    max = 100,
    variant = "primary", // primary, success, warning, error, glass
    size = "medium", // small, medium, large
    showLabel = true,
    animated = true,
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
  const percentage = $derived(Math.min((value / max) * 100, 100));

  const trackHeight = $derived(
    size === "small" ? "h-1.5" : size === "large" ? "h-3" : "h-2"
  );

  const labelSize = $derived(
    size === "small" ? "text-xs" : size === "large" ? "text-sm" : "text-[13px]"
  );

  const percentageSize = $derived(
    size === "small"
      ? "text-[11px]"
      : size === "large"
        ? "text-[13px]"
        : "text-xs"
  );

  const variantClasses = $derived(
    variant === "success"
      ? "bg-gradient-to-r from-green-500 to-teal-500 shadow-[0_0_12px_rgba(16,185,129,0.4)]"
      : variant === "warning"
        ? "bg-gradient-to-r from-amber-500 to-orange-500 shadow-[0_0_12px_rgba(245,158,11,0.4)]"
        : variant === "error"
          ? "bg-gradient-to-r from-red-500 to-red-600 shadow-[0_0_12px_rgba(239,68,68,0.4)]"
          : variant === "glass"
            ? "bg-indigo-500/50 backdrop-blur-sm shadow-[0_0_12px_rgba(99,102,241,0.3)]"
            : "bg-gradient-to-r from-indigo-500 to-purple-500 shadow-[0_0_12px_rgba(99,102,241,0.4)]"
  );
</script>

<div class="w-full">
  {#if showLabel}
    <div class="flex justify-between items-center mb-2">
      <span
        class="{labelSize} font-semibold text-gray-900 dark:text-white tracking-tight"
      >
        {value} / {max}
      </span>
      <span class="{percentageSize} font-bold text-gray-500 dark:text-gray-400">
        {Math.round(percentage)}%
      </span>
    </div>
  {/if}

  <div
    class="w-full {trackHeight} bg-gray-900/5 dark:bg-white/10 rounded-lg overflow-hidden shadow-inner"
  >
    <div
      class="h-full rounded-lg transition-[width] duration-500 ease-out relative overflow-hidden {variantClasses} {animated
        ? 'shimmer'
        : ''}"
      style="width: {percentage}%"
    ></div>
  </div>
</div>
