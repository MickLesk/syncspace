<script>
  /**
   * Modern Page Wrapper with Glassmorphism Background
   * Wrapper für alle internen Pages (nach dem Login)
   */
  import { currentView } from "../stores/ui.js";

  let {
    children,
    fullHeight = false,
    padding = true,
    gradient = true,
  } = $props();
</script>

<div
  class="modern-page-wrapper"
  class:full-height={fullHeight}
  class:no-padding={!padding}
  class:with-gradient={gradient}
>
  <!-- Subtle Background Pattern -->
  {#if gradient}
    <div class="bg-pattern"></div>
  {/if}

  <!-- Page Content -->
  <div class="page-inner">
    {@render children()}
  </div>
</div>

<style>
  .modern-page-wrapper {
    position: relative;
    min-height: calc(100vh - 4rem);
    padding: 2rem;
    background: rgb(249 250 251);
  }

  @media (prefers-color-scheme: dark) {
    .modern-page-wrapper {
      background: rgb(17 24 39);
    }
  }

  .with-gradient {
    background: linear-gradient(
      135deg,
      rgb(249 250 251) 0%,
      rgb(243 244 246) 50%,
      rgb(249 250 251) 100%
    );
  }

  @media (prefers-color-scheme: dark) {
    .with-gradient {
      background: linear-gradient(
        135deg,
        rgb(17 24 39) 0%,
        rgb(31 41 55) 50%,
        rgb(17 24 39) 100%
      );
    }
  }

  .full-height {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .no-padding {
    padding: 0;
  }

  /* Subtle Pattern Overlay */
  .bg-pattern {
    position: absolute;
    inset: 0;
    opacity: 0.03;
    background-image: radial-gradient(
        circle at 25px 25px,
        rgb(59 130 246) 2%,
        transparent 0%
      ),
      radial-gradient(circle at 75px 75px, rgb(147 51 234) 2%, transparent 0%);
    background-size: 100px 100px;
    pointer-events: none;
  }

  .page-inner {
    position: relative;
    z-index: 1;
    max-width: 1600px;
    margin: 0 auto;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .modern-page-wrapper {
      padding: 1rem;
    }
  }
</style>

