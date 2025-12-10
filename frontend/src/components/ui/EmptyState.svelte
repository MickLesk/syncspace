<script>
  /**
   * EmptyState Component - Pure Tailwind v4
   * Wiederverwendbarer Component für leere Views
   *
   * @component
   * @example
   * <EmptyState
   *   icon="bi-star-fill"
   *   title="No Favorites"
   *   description="Mark files as favorite to see them here"
   *   actionText="Browse Files"
   *   onAction={() => navigateToFiles()}
   * />
   */

  import Icon from "./Icon.svelte";
  import ModernButton from "./ModernButton.svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let {
    icon = "",
    isBootstrapIcon = true,
    title = "",
    description = "",
    actionText = "",
    onAction = null,
    size = "medium",
    content,
    actions,
  } = $props();

  // Derived default title
  const displayTitle = $derived(title || tr("noItemsFound"));

  // Size-based classes
  const sizeClasses = $derived(
    {
      small: {
        container: "py-10 px-6 gap-3",
        icon: "text-5xl",
        title: "text-lg",
        description: "text-sm max-w-sm",
      },
      medium: {
        container: "py-20 px-8 gap-4",
        icon: "text-7xl",
        title: "text-2xl",
        description: "text-base max-w-md",
      },
      large: {
        container: "py-28 px-10 gap-6",
        icon: "text-8xl",
        title: "text-3xl",
        description: "text-lg max-w-lg",
      },
    }[size]
  );
</script>

<div
  class="text-center flex flex-col items-center text-gray-500 dark:text-gray-400 {sizeClasses.container}"
>
  {#if icon}
    <div
      class="mb-2 opacity-50 hover:opacity-70 transition-all duration-300 hover:scale-105 {sizeClasses.icon}"
    >
      {#if isBootstrapIcon}
        <i class="bi {icon}" aria-hidden="true"></i>
      {:else}
        <span>{icon}</span>
      {/if}
    </div>
  {/if}

  <h3
    class="font-semibold text-gray-900 dark:text-gray-100 m-0 {sizeClasses.title}"
  >
    {displayTitle}
  </h3>

  {#if description}
    <p
      class="text-gray-600 dark:text-gray-400 m-0 leading-relaxed {sizeClasses.description}"
    >
      {description}
    </p>
  {/if}

  {@render content?.()}

  {#if actionText && onAction}
    <div class="mt-2">
      <ModernButton onclick={onAction} variant="outline" size="md">
        {actionText}
      </ModernButton>
    </div>
  {/if}

  {@render actions?.()}
</div>
