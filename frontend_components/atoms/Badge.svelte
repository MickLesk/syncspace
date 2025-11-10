<script lang="ts">
  import { colorMap, colorText, colorBorder, type BadgeVariant } from '../shared/index.js';

  interface Props {
    variant?: BadgeVariant;
    outline?: boolean;
    size?: 'sm' | 'md' | 'lg';
    icon?: boolean;
  }

  let {
    variant = 'primary',
    outline = false,
    size = 'md',
    icon = false,
  }: Props = $props();

  const sizeClasses = {
    sm: 'px-2 py-1 text-xs',
    md: 'px-3 py-1.5 text-sm',
    lg: 'px-4 py-2 text-base',
  };

  const variantClasses = {
    primary: outline
      ? `border-2 ${colorBorder.primary} ${colorText.primary} bg-transparent`
      : `bg-gradient-to-r ${colorMap.primary} text-white`,
    secondary: outline
      ? `border-2 ${colorBorder.secondary} ${colorText.secondary} bg-transparent`
      : `bg-gradient-to-r ${colorMap.secondary} text-white`,
    success: outline
      ? `border-2 ${colorBorder.success} ${colorText.success} bg-transparent`
      : `bg-gradient-to-r ${colorMap.success} text-white`,
    danger: outline
      ? `border-2 ${colorBorder.danger} ${colorText.danger} bg-transparent`
      : `bg-gradient-to-r ${colorMap.danger} text-white`,
    warning: outline
      ? `border-2 ${colorBorder.warning} ${colorText.warning} bg-transparent`
      : `bg-gradient-to-r ${colorMap.warning} text-white`,
    info: outline
      ? `border-2 ${colorBorder.info} ${colorText.info} bg-transparent`
      : `bg-gradient-to-r ${colorMap.info} text-white`,
  };

  const classes = `rounded-full font-semibold inline-flex items-center gap-2 ${sizeClasses[size]} ${variantClasses[variant]} ${icon ? 'p-1.5' : ''}`;
</script>

<span class={classes}>
  <slot />
</span>
