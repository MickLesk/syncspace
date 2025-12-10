<script>
  /**
   * Modern Page Wrapper v2 - Unified Design System
   * Harmonized wrapper for all pages with dark/light mode support
   */
  let {
    children,
    fullHeight = false,
    padding = true,
    gradient = true,
    maxWidth = "7xl", // sm, md, lg, xl, 2xl, 3xl, 4xl, 5xl, 6xl, 7xl, full
  } = $props();

  const maxWidthClasses = {
    sm: "max-w-screen-sm",
    md: "max-w-screen-md",
    lg: "max-w-screen-lg",
    xl: "max-w-screen-xl",
    "2xl": "max-w-screen-2xl",
    "3xl": "max-w-7xl",
    "4xl": "max-w-[1600px]",
    "5xl": "max-w-[1800px]",
    "6xl": "max-w-[2000px]",
    "7xl": "max-w-[2200px]",
    full: "max-w-full",
  };
</script>

<div
  class="page-wrapper relative min-h-[calc(100vh-4rem)] bg-transparent transition-colors duration-200"
  class:min-h-screen={fullHeight}
  class:flex={fullHeight}
  class:items-center={fullHeight}
  class:justify-center={fullHeight}
  class:p-0={!padding}
  class:p-4={padding}
  class:md:p-6={padding}
  class:lg:p-8={padding}
>
  <!-- Animated Background Blobs -->
  {#if gradient}
    <div class="blob blob-1"></div>
    <div class="blob blob-2"></div>
    <div class="blob blob-3"></div>
  {/if}

  <!-- Page Content -->
  <div class="page-inner relative z-10 mx-auto {maxWidthClasses[maxWidth]}">
    {@render children()}
  </div>
</div>
