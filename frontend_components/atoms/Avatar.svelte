<script lang="ts">
  import {
    shape,
    shapeExpressive,
    springs,
    elevation,
  } from "../shared/index.js";

  type ShapeVariant = "normal" | "extra-rounded" | "squircle" | "mixed";
  type AvatarSize = "xs" | "sm" | "md" | "lg" | "xl" | "2xl";

  interface Props {
    src?: string;
    alt?: string;
    initials?: string;

    // 🆕 M3 EXPRESSIVE FEATURES
    size?: AvatarSize;
    shapeStyle?: ShapeVariant; // Decorative shapes!
    gradient?: boolean; // Gradient border
    elevationLevel?: 0 | 1 | 2 | 3;

    class?: string;
  }

  let {
    src = "",
    alt = "",
    initials = "",
    size = "md",
    shapeStyle = "normal",
    gradient = false,
    elevationLevel = 1,
    class: customClass = "",
  }: Props = $props();

  // 🆕 M3 EXPRESSIVE: Expanded size scale
  const sizeClasses = {
    xs: "w-6 h-6 text-xs",
    sm: "w-8 h-8 text-sm",
    md: "w-10 h-10 text-base",
    lg: "w-12 h-12 text-lg",
    xl: "w-16 h-16 text-xl",
    "2xl": "w-20 h-20 text-2xl",
  };

  // 🆕 M3 EXPRESSIVE: Shape variety (decorative!)
  const shapeClasses = {
    normal: shape.full,
    "extra-rounded": shapeExpressive["extra-extra-large"],
    squircle: shapeExpressive["squircle-xl"],
    mixed: shapeExpressive["mixed-rounded"],
  };

  const gradientBorder = gradient
    ? "ring-2 ring-gradient-to-r from-blue-500 to-purple-500 ring-offset-2"
    : "";
</script>

<div
  class={`
    ${sizeClasses[size]}
    ${shapeClasses[shapeStyle]}
    ${elevation[elevationLevel]}
    ${gradientBorder}
    bg-gradient-to-br from-blue-500 to-purple-500
    flex items-center justify-center
    overflow-hidden
    transition-all duration-300 ease-[${springs.spatial.bouncy}]
    hover:scale-110
    hover:rotate-3
    active:scale-95
    active:rotate-0
    ${customClass}
  `}
>
  {#if src}
    <img {src} {alt} class="w-full h-full object-cover" />
  {:else if initials}
    <span class="font-bold text-white uppercase">
      {initials}
    </span>
  {:else}
    <i class="bi bi-person-fill text-white text-2xl"></i>
  {/if}
</div>
