<script lang="ts">
  import {
    shadows,
    elevation,
    shape,
    shapeExpressive,
    transitions,
    springs,
    motionPresets,
    surface,
    colors,
    typographyEmphasized,
  } from "../shared/index.js";
  import type { Snippet } from "svelte";

  type ShapeVariant =
    | "normal"
    | "extra-rounded"
    | "squircle"
    | "mixed"
    | "top-rounded";
  type MotionVariant = "smooth" | "bouncy" | "gentle";
  type SurfaceTone =
    | "base"
    | "container"
    | "containerLow"
    | "primary"
    | "secondary"
    | "tertiary";

  interface Props {
    title?: string;
    description?: string;
    footer?: string;

    // 🆕 M3 EXPRESSIVE FEATURES
    emphasized?: boolean; // Bold title typography
    shapeStyle?: ShapeVariant; // Shape variety
    motion?: MotionVariant; // Spring physics
    surfaceTone?: SurfaceTone; // Surface mixing
    elevationLevel?: 0 | 1 | 2 | 3 | 4 | 5;

    // Standard props
    hoverable?: boolean;
    bordered?: boolean;
    class?: string;
    children?: Snippet;
    footerSnippet?: Snippet;
  }

  let {
    title,
    description,
    footer,
    emphasized = false,
    shapeStyle = "normal",
    motion = "smooth",
    surfaceTone = "base",
    elevationLevel = 1,
    hoverable = false,
    bordered = true,
    class: customClass = "",
    children,
    footerSnippet,
  }: Props = $props();

  // 🆕 M3 EXPRESSIVE: Shape variety
  const shapeClasses = {
    normal: shape.lg,
    "extra-rounded": shapeExpressive["extra-large"],
    squircle: shapeExpressive["squircle-lg"],
    mixed: shapeExpressive["mixed-rounded"],
    "top-rounded": shapeExpressive["top-extra-rounded"],
  };

  // 🆕 M3 EXPRESSIVE: Spring motion
  const motionClasses = {
    smooth: `transition-all duration-300 ease-[${springs.spatial.smooth}]`,
    bouncy: `transition-all duration-400 ease-[${springs.spatial.bouncy}]`,
    gentle: `transition-all duration-500 ease-[${springs.spatial.gentle}]`,
  };

  // 🆕 M3 EXPRESSIVE: Surface tone mixing
  const surfaceClasses = {
    base: surface.base,
    container: surface.container,
    containerLow: surface.containerLow,
    primary: `${colors.primary.container}`,
    secondary: `${colors.secondary.container}`,
    tertiary: `${colors.tertiary.container}`,
  };

  // 🆕 M3 EXPRESSIVE: Typography emphasis
  const titleClasses = emphasized
    ? `${typographyEmphasized.title.large} mb-3`
    : "text-lg font-semibold text-gray-900 dark:text-gray-100 mb-2";
</script>

<div
  class={`
    p-6
    ${shapeClasses[shapeStyle]}
    ${surfaceClasses[surfaceTone]}
    ${bordered ? "border border-gray-200 dark:border-gray-700" : ""}
    ${elevation[elevationLevel]}
    ${motionClasses[motion]}
    ${hoverable ? `hover:${elevation[elevationLevel + 1]} hover:scale-[1.02] hover:-translate-y-1 cursor-pointer` : ""}
    ${customClass}
  `}
>
  {#if title}
    <h3 class={titleClasses}>
      {title}
    </h3>
  {/if}

  {#if description}
    <p class="text-gray-600 dark:text-gray-400 text-sm mb-4">
      {description}
    </p>
  {/if}

  <div class="mb-4">
    {#if children}
      {@render children()}
    {/if}
  </div>

  {#if footer}
    <div class="pt-4 border-t border-gray-200 dark:border-gray-700 mt-4">
      <p class="text-xs text-gray-500 dark:text-gray-400">{footer}</p>
    </div>
  {/if}

  {#if footerSnippet}
    <div class="pt-4 border-t border-gray-200 dark:border-gray-700 mt-4">
      {@render footerSnippet()}
    </div>
  {/if}
</div>

<style>
  /* M3 Expressive: Spring hover with transform origin */
  div:hover {
    transform-origin: center;
  }
</style>
