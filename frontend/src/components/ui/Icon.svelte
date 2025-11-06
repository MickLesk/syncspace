<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  /**
   * Bootstrap Icon Component
   * Usage: <Icon name="folder" size="24" />
   * See: https://icons.getbootstrap.com/
   */
  let {
    name = "file-earmark",
    size = 16,
    color = "currentColor",
    className = "",
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
</script>

<i
  class="bi bi-{name} {className}"
  style="font-size: {size}px; color: {color};"
  aria-hidden="true"
></i>
