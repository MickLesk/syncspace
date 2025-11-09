<script>
  /**
   * Responsive Image Component
   * Automatically generates WebP/AVIF formats with multiple sizes
   * Includes lazy loading, blur-up effect, and performance monitoring
   * 
   * Usage:
   * <ResponsiveImage
   *   src="/images/photo.jpg"
   *   alt="My photo"
   *   sizes="(max-width: 768px) 100vw, 50vw"
   *   class="w-full h-auto"
   * />
   */

  import { onMount } from 'svelte';
  import { getImageUrls, monitorImageLoad, generateBlurHash } from '../../lib/imageOptimization.js';

  let {
    src = '',
    alt = 'Image',
    title = '',
    sizes = '(max-width: 768px) 100vw, 50vw',
    class: className = '',
    loading = 'lazy',
    decoding = 'async',
    width = undefined,
    height = undefined,
    onload = null,
    onerror = null,
  } = $props();

  let imageUrls = $state({});
  let blurHash = $state('');
  let isLoaded = $state(false);
  let hasError = $state(false);
  let imgElement = $state(null);

  onMount(async () => {
    // Generate image URLs for multiple formats
    imageUrls = getImageUrls(src, { width: width || 1280, height: height || 720 });

    // Generate blur hash placeholder
    if (loading === 'lazy') {
      try {
        blurHash = await generateBlurHash(src, 30);
      } catch (error) {
        console.warn('Failed to generate blur hash:', error);
      }
    }
  });

  function handleLoad(event) {
    isLoaded = true;
    hasError = false;

    if (onload) {
      onload(event);
    }

    // Monitor performance
    if (imgElement) {
      monitorImageLoad(imgElement).then((metrics) => {
        if (metrics.success) {
          console.debug(`[Image] Loaded in ${metrics.duration.toFixed(0)}ms: ${src}`);
        }
      });
    }
  }

  function handleError(event) {
    hasError = true;
    console.error(`[Image] Failed to load: ${src}`);

    if (onerror) {
      onerror(event);
    }
  }
</script>

<picture class="relative inline-block overflow-hidden {className}" style="line-height: 0;">
  <!-- Blur hash placeholder -->
  {#if blurHash && !isLoaded && !hasError}
    <div
      class="absolute inset-0 bg-cover bg-center blur-2xl scale-110"
      style="background-image: url('{blurHash}'); z-index: 1;"
    />
  {/if}

  <!-- AVIF format (smallest, newest) -->
  {#if imageUrls.avif}
    <source
      type="image/avif"
      srcset={imageUrls.avif}
      {sizes}
      media="screen"
    />
  {/if}

  <!-- WebP format (smaller, modern) -->
  {#if imageUrls.webp}
    <source
      type="image/webp"
      srcset={imageUrls.webp}
      {sizes}
      media="screen"
    />
  {/if}

  <!-- Original format (fallback) -->
  <img
    bind:this={imgElement}
    {src}
    {alt}
    {title}
    {sizes}
    srcset={imageUrls.original || src}
    {loading}
    {decoding}
    {width}
    {height}
    onload={handleLoad}
    onerror={handleError}
    class="max-w-full h-auto block transition-opacity duration-300"
    class:loaded={isLoaded}
    class:error={hasError}
    style="opacity: {isLoaded ? 1 : 0.9}; position: relative; z-index: 2;"
  />

  <!-- Error fallback -->
  {#if hasError}
    <div
      class="absolute inset-0 flex items-center justify-center bg-gray-200 dark:bg-gray-700 text-gray-500 dark:text-gray-400 text-sm"
      style="z-index: 3;"
    >
      <i class="bi bi-image text-2xl"></i>
    </div>
  {/if}
</picture>

<style>
  picture {
    display: block;
  }

  img {
    display: block;
    max-width: 100%;
    height: auto;
  }

  img.loaded {
    opacity: 1;
  }

  img.error {
    opacity: 0.5;
  }

  :global(picture) {
    line-height: 0; /* Remove baseline gap */
  }
</style>
