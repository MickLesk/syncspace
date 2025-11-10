<script lang="ts">
  import { colorMap, colorHover, sizeMap, transitions, type ButtonVariant, type ButtonSize } from '../shared/index.js';

  interface Props {
    variant?: ButtonVariant;
    size?: ButtonSize;
    disabled?: boolean;
    loading?: boolean;
    fullWidth?: boolean;
    icon?: boolean;
    href?: string;
    onclick?: () => void;
    class?: string;
  }

  let {
    variant = 'primary',
    size = 'md',
    disabled = false,
    loading = false,
    fullWidth = false,
    icon = false,
    href,
    onclick,
    class: customClass = '',
  }: Props = $props();

  const baseClasses = `
    font-semibold rounded-lg inline-flex items-center justify-center gap-2
    ${transitions.normal} focus:outline-none focus:ring-2 focus:ring-offset-2
    disabled:opacity-50 disabled:cursor-not-allowed
    ${sizeMap[size]}
  `;

  const variantClasses = {
    primary: `bg-gradient-to-r ${colorMap.primary} text-white ${colorHover.primary} focus:ring-blue-300`,
    secondary: `bg-gray-200 text-gray-900 ${colorHover.secondary} focus:ring-gray-300`,
    danger: `bg-gradient-to-r ${colorMap.danger} text-white ${colorHover.danger} focus:ring-red-300`,
    success: `bg-gradient-to-r ${colorMap.success} text-white ${colorHover.success} focus:ring-green-300`,
    warning: `bg-gradient-to-r ${colorMap.warning} text-white ${colorHover.warning} focus:ring-yellow-300`,
    ghost: `bg-transparent text-gray-700 hover:bg-gray-100 focus:ring-gray-300`,
    outline: `border-2 border-gray-300 text-gray-700 hover:bg-gray-50 focus:ring-gray-300`,
  };

  const classes = `${baseClasses} ${variantClasses[variant]} ${fullWidth ? 'w-full' : ''} ${customClass}`;
</script>

{#if href}
  <a {href} class={classes} class:icon {disabled}>
    {#if loading}
      <span class="animate-spin inline-block w-4 h-4 border-2 border-current border-t-transparent rounded-full"></span>
    {/if}
    <slot />
  </a>
{:else}
  <button {onclick} {disabled} {class: classes} class:icon {disabled}>
    {#if loading}
      <span class="animate-spin inline-block w-4 h-4 border-2 border-current border-t-transparent rounded-full"></span>
    {/if}
    <slot />
  </button>
{/if}

<style>
  :global(.icon) {
    @apply p-2;
  }
</style>
