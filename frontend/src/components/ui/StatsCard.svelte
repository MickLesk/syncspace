<script>
  import { colors, effects, transitions } from "../../lib/DesignSystem.js";

  let {
    icon = "bar-chart",
    label = "Total Files",
    value = "0",
    sublabel = "vs last period",
    trend = null,
    trendValue = "",
    variant = "indigo",
    loading = false,
    interactive = false,
    onclick = null,
    class: className = "",
  } = $props();

  const variants = {
    indigo: {
      gradient: `linear-gradient(135deg, ${colors.gradient.start}, ${colors.gradient.mid})`,
      border: "border-white/30 dark:border-slate-800/70",
      iconBg: "bg-white/20 text-white",
    },
    emerald: {
      gradient: "linear-gradient(135deg, #34d399, #059669)",
      border: "border-emerald-200/40 dark:border-emerald-900/60",
      iconBg: "bg-emerald-400/20 text-emerald-100",
    },
    amber: {
      gradient: "linear-gradient(135deg, #fbbf24, #f97316)",
      border: "border-amber-200/50 dark:border-amber-900/60",
      iconBg: "bg-amber-300/20 text-amber-50",
    },
    rose: {
      gradient: "linear-gradient(135deg, #fb7185, #ec4899)",
      border: "border-rose-200/60 dark:border-rose-900/60",
      iconBg: "bg-rose-400/20 text-rose-50",
    },
  };

  const trendThemes = {
    up: { text: "text-emerald-400", icon: "bi-arrow-up-right" },
    down: { text: "text-rose-400", icon: "bi-arrow-down-right" },
    neutral: { text: "text-slate-300", icon: "bi-dash-lg" },
  };

  const theme = $derived(variants[variant] || variants.indigo);
  const trendTheme = $derived(
    trend ? trendThemes[trend] || trendThemes.neutral : null
  );

  function handleKeydown(event) {
    if (!onclick) return;
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      onclick(event);
    }
  }
</script>

<div
  class={`relative overflow-hidden rounded-3xl border bg-white/80 dark:bg-slate-950/50 text-slate-900 dark:text-slate-50 shadow-[var(--ss-shadow)] transition-all duration-200 ${interactive || onclick ? "cursor-pointer hover:-translate-y-1 hover:shadow-2xl" : ""} ${className}`}
  style={`backdrop-filter:blur(${effects.blur});transition-timing-function:${transitions.easing};`}
  role={onclick ? "button" : undefined}
  tabindex={onclick ? 0 : undefined}
  on:click|stopPropagation={onclick}
  on:keydown={handleKeydown}
>
  <div
    class="absolute inset-0 opacity-70"
    style={`background:${theme.gradient};`}
  ></div>
  <div
    class="absolute inset-0 bg-gradient-to-br from-white/30 via-transparent to-transparent dark:from-slate-900/40"
  ></div>

  <div class="relative p-6 flex flex-col gap-6">
    <div class="flex items-start justify-between gap-4">
      <div class="flex items-center gap-4">
        <span
          class={`h-14 w-14 rounded-2xl flex items-center justify-center text-2xl backdrop-blur ${theme.iconBg}`}
        >
          {#if loading}
            <span class="loader"></span>
          {:else}
            <i class={`bi bi-${icon}`}></i>
          {/if}
        </span>
        <div>
          <p
            class="text-xs font-semibold uppercase tracking-[0.25em] text-slate-600/80 dark:text-slate-300/80"
          >
            {label}
          </p>
          <p
            class="text-4xl font-semibold tracking-tight text-slate-900 dark:text-white"
          >
            {#if loading}
              <span class="value-skeleton"></span>
            {:else}
              {value}
            {/if}
          </p>
        </div>
      </div>

      {#if trendTheme && trendValue && !loading}
        <div
          class={`flex flex-col items-end text-sm font-semibold ${trendTheme.text}`}
        >
          <span class="inline-flex items-center gap-1">
            <i class={`bi ${trendTheme.icon}`}></i>
            {trendValue}
          </span>
          <span class="text-[11px] uppercase tracking-[0.35em] text-white/70"
            >Trend</span
          >
        </div>
      {/if}
    </div>

    <div
      class="flex flex-col gap-3 text-sm text-slate-100/80 dark:text-slate-200/70"
    >
      {#if loading}
        <span class="line-skeleton w-2/3"></span>
        <span class="line-skeleton w-1/2"></span>
      {:else}
        <p>{sublabel}</p>
        <slot />
      {/if}
    </div>

    <slot name="sparkline" />
  </div>
</div>

<style>
  :global(:root) {
    --ss-shadow: 0 20px 45px rgba(15, 23, 42, 0.15);
  }

  .loader {
    width: 28px;
    height: 28px;
    border-radius: 999px;
    border: 3px solid rgba(255, 255, 255, 0.4);
    border-top-color: transparent;
    animation: spin 0.9s linear infinite;
  }

  .value-skeleton {
    display: inline-block;
    width: 120px;
    height: 32px;
    border-radius: 999px;
    background: linear-gradient(
      90deg,
      rgba(255, 255, 255, 0.4),
      rgba(255, 255, 255, 0.2),
      rgba(255, 255, 255, 0.4)
    );
    background-size: 200% 100%;
    animation: shimmer 1.5s ease-in-out infinite;
  }

  .line-skeleton {
    display: inline-block;
    height: 10px;
    border-radius: 999px;
    background: linear-gradient(
      90deg,
      rgba(255, 255, 255, 0.35),
      rgba(255, 255, 255, 0.15),
      rgba(255, 255, 255, 0.35)
    );
    background-size: 200% 100%;
    animation: shimmer 1.5s ease-in-out infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  @keyframes shimmer {
    0% {
      background-position: 200% 0;
    }
    100% {
      background-position: -200% 0;
    }
  }
</style>
