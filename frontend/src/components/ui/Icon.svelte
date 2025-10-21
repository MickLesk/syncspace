<script>
  /**
   * Bootstrap Icon Component
   * Usage: <Icon name="folder" size="24" />
   * See: https://icons.getbootstrap.com/
   */
  export let name = "file-earmark"; // Bootstrap icon name
  export let size = 16; // Size in pixels
  export let color = "currentColor";
  export let className = "";
</script>

<i
  class="bi bi-{name} {className}"
  style="font-size: {size}px; color: {color};"
  aria-hidden="true"
></i>
