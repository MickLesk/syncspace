<script>
  /**
   * Modern Card Component - HyperUI Style
   * @param {boolean} hoverable - Adds hover effects
   * @param {boolean} bordered - Adds border
   * @param {string} padding - 'none' | 'sm' | 'md' | 'lg'
   */

  let {
    hoverable = false,
    bordered = true,
    padding = "md",
    class: className = "",
    children,
  } = $props();

  const paddingClasses = {
    none: "",
    sm: "p-4",
    md: "p-6",
    lg: "p-8",
  };
</script>

<div
  class="bg-white dark:bg-gray-900 rounded-xl {bordered
    ? 'border border-gray-200 dark:border-gray-800'
    : ''} {hoverable
    ? 'hover:shadow-lg transition-shadow duration-200'
    : 'shadow-sm'} {paddingClasses[padding]} {className}"
>
  {@render children?.()}
</div>
