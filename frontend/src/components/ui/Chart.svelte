<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  // Props
  let {
    data = [],
    title = "Chart",
    type = "doughnut", // doughnut, bar
    size = "md", // sm, md, lg
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // Calculate total for percentages
  let total = $derived(data.reduce((sum, item) => sum + item.value, 0));

  // Calculate percentages and cumulative values for doughnut
  let chartData = $derived(
    data.map((item, index) => {
      const percentage = total > 0 ? (item.value / total) * 100 : 0;
      const cumulativePercentage = data
        .slice(0, index)
        .reduce((sum, d) => sum + (d.value / total) * 100, 0);

      return {
        ...item,
        percentage,
        cumulativePercentage,
      };
    })
  );

  // Size mapping
  const sizeMap = {
    sm: { chart: 120, stroke: 20, bar: 120 },
    md: { chart: 200, stroke: 30, bar: 200 },
    lg: { chart: 280, stroke: 40, bar: 300 },
  };

  let dimensions = $derived(sizeMap[size]);

  // Calculate stroke offset for each segment
  function getStrokeOffset(cumulativePercentage, circumference) {
    return circumference - (cumulativePercentage / 100) * circumference;
  }

  function getStrokeDasharray(percentage, circumference) {
    return `${(percentage / 100) * circumference} ${circumference}`;
  }
</script>

<div class="chart-container">
  {#if type === "doughnut"}
    <div
      class="doughnut-chart"
      style="width: {dimensions.chart}px; height: {dimensions.chart}px;"
    >
      <svg viewBox="0 0 100 100" class="doughnut-svg">
        {#each chartData as item, i}
          {@const radius = 40}
          {@const circumference = 2 * Math.PI * radius}
          {@const offset = getStrokeOffset(
            item.cumulativePercentage,
            circumference
          )}
          {@const dasharray = getStrokeDasharray(
            item.percentage,
            circumference
          )}

          <circle
            class="doughnut-segment"
            cx="50"
            cy="50"
            r={radius}
            fill="none"
            stroke={item.color || `hsl(var(--p))`}
            stroke-width={dimensions.stroke}
            stroke-dasharray={dasharray}
            stroke-dashoffset={offset}
            transform="rotate(-90 50 50)"
            style="--delay: {i * 0.1}s"
          />
        {/each}
      </svg>

      <!-- Center Label -->
      <div class="doughnut-center">
        <div class="doughnut-total">
          {#if data.length > 0}
            <div class="total-label">{title}</div>
            <div class="total-value">
              {data.reduce((sum, d) => sum + d.value, 0).toLocaleString()}
            </div>
          {/if}
        </div>
      </div>
    </div>

    <!-- Legend -->
    <div class="chart-legend">
      {#each chartData as item}
        <div class="legend-item">
          <div
            class="legend-color"
            style="background: {item.color || 'hsl(var(--p))'}"
          ></div>
          <div class="legend-content">
            <div class="legend-label">{item.label}</div>
            <div class="legend-value">
              {item.value ? item.value.toLocaleString() : "0"} ({item.percentage?.toFixed(
                1
              ) || "0"}%)
            </div>
          </div>
        </div>
      {/each}
    </div>
  {:else if type === "bar"}
    <div class="bar-chart" style="height: {dimensions.bar}px;">
      {#each chartData as item}
        <div class="bar-item">
          <div class="bar-label">{item.label}</div>
          <div class="bar-wrapper">
            <div
              class="bar-fill"
              style="width: {item.percentage}%; background: {item.color ||
                'hsl(var(--p))'}"
            >
              <span class="bar-percentage">{item.percentage.toFixed(1)}%</span>
            </div>
          </div>
          <div class="bar-value">{item.value.toLocaleString()}</div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .chart-container {
    width: 100%;
  }

  /* Doughnut Chart */
  .doughnut-chart {
    position: relative;
    margin: 0 auto 1.5rem;
  }

  .doughnut-svg {
    width: 100%;
    height: 100%;
    transform: scaleY(-1);
  }

  .doughnut-segment {
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    animation: drawSegment 1s ease-out var(--delay, 0s) both;
  }

  .doughnut-segment:hover {
    filter: brightness(1.1);
    stroke-width: 32;
  }

  @keyframes drawSegment {
    from {
      stroke-dashoffset: 251.2;
    }
  }

  .doughnut-center {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
  }

  .doughnut-total {
    animation: fadeIn 0.5s ease-out 0.5s both;
  }

  .total-label {
    font-size: 0.75rem;
    font-weight: 600;
    color: hsl(var(--bc) / 0.6);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .total-value {
    font-size: 1.5rem;
    font-weight: 700;
    color: hsl(var(--bc));
    margin-top: 0.25rem;
  }

  /* Legend */
  .chart-legend {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .legend-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.625rem 0.875rem;
    background: hsl(var(--b2) / 0.5);
    border-radius: 0.5rem;
    transition: all 0.2s ease;
  }

  .legend-item:hover {
    background: hsl(var(--b2));
    transform: translateX(4px);
  }

  .legend-color {
    width: 1rem;
    height: 1rem;
    border-radius: 0.25rem;
    flex-shrink: 0;
  }

  .legend-content {
    flex: 1;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
  }

  .legend-label {
    font-size: 0.875rem;
    font-weight: 600;
    color: hsl(var(--bc));
  }

  .legend-value {
    font-size: 0.8125rem;
    color: hsl(var(--bc) / 0.7);
    font-weight: 500;
  }

  /* Bar Chart */
  .bar-chart {
    display: flex;
    flex-direction: column;
    justify-content: space-evenly;
    gap: 1rem;
  }

  .bar-item {
    display: grid;
    grid-template-columns: 120px 1fr auto;
    align-items: center;
    gap: 1rem;
  }

  .bar-label {
    font-size: 0.875rem;
    font-weight: 600;
    color: hsl(var(--bc));
    text-align: right;
  }

  .bar-wrapper {
    height: 2rem;
    background: hsl(var(--b2));
    border-radius: 1rem;
    overflow: hidden;
    position: relative;
  }

  .bar-fill {
    height: 100%;
    border-radius: 1rem;
    transition: width 1s cubic-bezier(0.4, 0, 0.2, 1);
    animation: expandBar 1s cubic-bezier(0.4, 0, 0.2, 1);
    display: flex;
    align-items: center;
    justify-content: flex-end;
    padding-right: 0.75rem;
    position: relative;
    overflow: hidden;
  }

  .bar-fill::before {
    content: "";
    position: absolute;
    inset: 0;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2));
    animation: shimmer 2s infinite;
  }

  @keyframes expandBar {
    from {
      width: 0;
    }
  }

  @keyframes shimmer {
    0% {
      transform: translateX(-100%);
    }
    100% {
      transform: translateX(100%);
    }
  }

  .bar-percentage {
    font-size: 0.75rem;
    font-weight: 700;
    color: white;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
    position: relative;
    z-index: 1;
  }

  .bar-value {
    font-size: 0.875rem;
    font-weight: 600;
    color: hsl(var(--bc) / 0.7);
    min-width: 80px;
    text-align: right;
  }

  @media (max-width: 768px) {
    .bar-item {
      grid-template-columns: 1fr;
      gap: 0.5rem;
    }

    .bar-label {
      text-align: left;
    }

    .bar-value {
      text-align: left;
    }
  }
</style>
