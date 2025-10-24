<!--
  CardV2.svelte - Modern Card Component using DaisyUI
  
  A flexible card component with header, body, and actions sections.
  Supports glass effect, hover states, and various layouts.
  
  Props:
  - variant: "default" | "bordered" | "compact" | "side" (default: "default")
  - glass: boolean - Enable glassmorphism effect (default: false)
  - hoverable: boolean - Add hover lift effect (default: false)
  - clickable: boolean - Add pointer cursor and interaction states (default: false)
  - imageTop: string - URL for top image
  - imageSide: string - URL for side image (requires variant="side")
  - title: string - Card title (alternative to title slot)
  - description: string - Card description (alternative to default slot)
  
  Slots:
  - header: Custom header content
  - title: Card title content
  - default: Main card body content
  - actions: Action buttons area
  - image: Custom image content
  
  Events:
  - on:click - Fired when card is clicked (if clickable)
  
  Usage:
  <CardV2>
    <svelte:fragment slot="title">Card Title</svelte:fragment>
    <p>Card content goes here</p>
    <svelte:fragment slot="actions">
      <ButtonV2 variant="primary">Action</ButtonV2>
    </svelte:fragment>
  </CardV2>
-->

<script>
  export let variant = "default"; // default, bordered, compact, side
  export let glass = false;
  export let hoverable = false;
  export let clickable = false;
  export let imageTop = "";
  export let imageSide = "";
  export let title = "";
  export let description = "";

  $: variantClasses = {
    default: "card-normal",
    bordered: "card-bordered",
    compact: "card-compact",
    side: "card-side",
  };

  $: computedClass = [
    "card",
    "bg-base-100",
    variantClasses[variant],
    glass ? "glass-card" : "",
    hoverable ? "hoverable" : "",
    clickable ? "clickable" : "",
    imageSide && variant === "side" ? "card-side" : "",
  ]
    .filter(Boolean)
    .join(" ");

  function handleClick(e) {
    if (clickable) {
      // Dispatch click event
      e.target.dispatchEvent(
        new CustomEvent("cardclick", {
          detail: { variant, title },
          bubbles: true,
        })
      );
    }
  }
</script>

<div
  class={computedClass}
  on:click={handleClick}
  on:keydown={(e) => {
    if (clickable && (e.key === "Enter" || e.key === " ")) {
      e.preventDefault();
      handleClick(e);
    }
  }}
  role={clickable ? "button" : undefined}
  tabindex={clickable ? 0 : undefined}
>
  <!-- Image Top -->
  {#if imageTop && !imageSide}
    <figure class="card-image-top">
      {#if $$slots.image}
        <slot name="image" />
      {:else}
        <img src={imageTop} alt={title || "Card image"} />
      {/if}
    </figure>
  {/if}

  <!-- Image Side (for side variant) -->
  {#if imageSide && variant === "side"}
    <figure class="card-image-side">
      <img src={imageSide} alt={title || "Card image"} />
    </figure>
  {/if}

  <div class="card-body">
    <!-- Custom Header Slot -->
    {#if $$slots.header}
      <div class="card-header">
        <slot name="header" />
      </div>
    {/if}

    <!-- Title -->
    {#if $$slots.title || title}
      <h2 class="card-title">
        {#if $$slots.title}
          <slot name="title" />
        {:else}
          {title}
        {/if}
      </h2>
    {/if}

    <!-- Body Content -->
    {#if $$slots.default || description}
      <div class="card-content">
        {#if $$slots.default}
          <slot />
        {:else if description}
          <p>{description}</p>
        {/if}
      </div>
    {/if}

    <!-- Actions -->
    {#if $$slots.actions}
      <div class="card-actions">
        <slot name="actions" />
      </div>
    {/if}
  </div>
</div>

<style>
  /* Glass Effect */
  .glass-card {
    background: var(--glass-background);
    backdrop-filter: blur(var(--glass-blur));
    border: 1px solid var(--glass-border);
    box-shadow: var(--glass-shadow);
  }

  .glass-card::before {
    content: "";
    position: absolute;
    inset: 0;
    border-radius: inherit;
    padding: 1px;
    background: linear-gradient(
      135deg,
      rgba(255, 255, 255, 0.2),
      rgba(255, 255, 255, 0.05)
    );
    -webkit-mask:
      linear-gradient(#fff 0 0) content-box,
      linear-gradient(#fff 0 0);
    -webkit-mask-composite: xor;
    mask-composite: exclude;
    pointer-events: none;
  }

  /* Hoverable Effect */
  .hoverable {
    transition: all var(--duration-200) var(--ease-standard);
  }

  .hoverable:hover {
    transform: translateY(-4px);
    box-shadow: var(--shadow-xl);
  }

  /* Clickable State */
  .clickable {
    cursor: pointer;
    user-select: none;
  }

  .clickable:active {
    transform: scale(0.98);
  }

  .clickable:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
  }

  /* Card Header */
  .card-header {
    margin-bottom: var(--spacing-2);
  }

  /* Card Title Enhancement */
  .card-title {
    display: flex;
    align-items: center;
    gap: var(--spacing-2);
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-on-surface);
    margin-bottom: var(--spacing-3);
  }

  /* Card Content */
  .card-content {
    flex: 1;
    color: var(--color-on-surface-variant);
    font-size: var(--font-size-base);
    line-height: var(--line-height-relaxed);
  }

  /* Card Actions */
  .card-actions {
    display: flex;
    gap: var(--spacing-2);
    margin-top: var(--spacing-4);
    justify-content: flex-end;
    flex-wrap: wrap;
  }

  /* Image Styling */
  .card-image-top,
  .card-image-side {
    margin: 0;
    overflow: hidden;
  }

  .card-image-top img,
  .card-image-side img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    transition: transform var(--duration-300) var(--ease-standard);
  }

  .hoverable .card-image-top img:hover,
  .hoverable .card-image-side img:hover {
    transform: scale(1.05);
  }

  /* Side Variant Image Size */
  .card-side .card-image-side {
    max-width: 200px;
  }

  /* Compact Variant Adjustments */
  :global(.card-compact) .card-title {
    font-size: var(--font-size-lg);
    margin-bottom: var(--spacing-2);
  }

  :global(.card-compact) .card-actions {
    margin-top: var(--spacing-2);
  }

  /* Responsive */
  @media (max-width: 768px) {
    .card-side {
      flex-direction: column;
    }

    .card-side .card-image-side {
      max-width: 100%;
    }

    .card-actions {
      justify-content: stretch;
    }

    .card-actions > :global(*) {
      flex: 1;
    }
  }
</style>
