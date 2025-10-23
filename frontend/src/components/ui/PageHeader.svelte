<script>
  import Icon from "./Icon.svelte";

  export let title = "";
  export let subtitle = "";
  export let icon = ""; // Bootstrap icon name
  export let gradient = "purple"; // purple, blue, green, orange, red
</script>

<div class="page-header gradient-{gradient}">
  <div class="header-glow"></div>
  <div class="header-content">
    {#if icon}
      <div class="icon-circle">
        <Icon name={icon} size={28} />
      </div>
    {/if}
    <div class="header-text">
      <h1>{title}</h1>
      {#if subtitle}
        <p class="subtitle">{subtitle}</p>
      {/if}
      <slot name="stats" />
    </div>
    <slot name="actions" />
  </div>
</div>

<style>
  .page-header {
    position: relative;
    overflow: hidden;
    padding: 48px 40px;
    margin-bottom: 32px;
    border-radius: 0 0 32px 32px;
    box-shadow: 0 8px 32px rgba(103, 80, 164, 0.2);
  }

  /* Gradient Variants */
  .gradient-purple {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  }

  .gradient-blue {
    background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
  }

  .gradient-green {
    background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
  }

  .gradient-orange {
    background: linear-gradient(135deg, #fa709a 0%, #fee140 100%);
  }

  .gradient-red {
    background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
  }

  .header-glow {
    position: absolute;
    top: -50%;
    right: -20%;
    width: 500px;
    height: 500px;
    background: radial-gradient(circle, rgba(255, 255, 255, 0.15) 0%, transparent 70%);
    pointer-events: none;
    animation: float 8s ease-in-out infinite;
  }

  @keyframes float {
    0%, 100% { transform: translate(0, 0); }
    50% { transform: translate(-30px, -30px); }
  }

  .header-content {
    position: relative;
    display: flex;
    align-items: center;
    gap: 20px;
    flex-wrap: wrap;
  }

  .icon-circle {
    width: 64px;
    height: 64px;
    background: rgba(255, 255, 255, 0.2);
    backdrop-filter: blur(10px);
    border-radius: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    flex-shrink: 0;
  }

  .header-text {
    flex: 1;
    min-width: 200px;
  }

  .header-text h1 {
    margin: 0 0 4px 0;
    font-size: 32px;
    font-weight: 600;
    color: white;
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  }

  .subtitle {
    margin: 0;
    font-size: 14px;
    color: rgba(255, 255, 255, 0.9);
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  }

  /* Stats slot styling */
  .header-text :global(.stats-bar) {
    display: flex;
    gap: 16px;
    flex-wrap: wrap;
    margin-top: 8px;
  }

  .header-text :global(.stat) {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 12px;
    background: rgba(255, 255, 255, 0.2);
    backdrop-filter: blur(10px);
    border-radius: 12px;
    font-size: 13px;
    color: white;
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  }

  .header-text :global(.stat i) {
    font-size: 16px;
  }

  /* Actions slot styling */
  .header-content :global(.header-actions) {
    display: flex;
    gap: 12px;
    flex-wrap: wrap;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .page-header {
      padding: 32px 24px;
    }

    .header-content {
      flex-direction: column;
      align-items: flex-start;
    }

    .header-text h1 {
      font-size: 24px;
    }

    .icon-circle {
      width: 48px;
      height: 48px;
    }
  }
</style>
