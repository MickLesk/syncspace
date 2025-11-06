<script>
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  let {
    label = "",
    icon = "",
    selected = false,
    variant = "filter", // filter, tag, removable
    size = "medium", // small, medium, large
    onRemove = undefined,
    onClick = () => {},
  } = $props();

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));
</script>

{#if variant === "removable" && onRemove}
  <div class="chip {variant} {size}" class:selected>
    {#if icon}
      <i class="bi {icon}"></i>
    {/if}

    <span class="chip-label">{label}</span>

    <button
      class="remove-btn"
      onclick={(e) => {
        e.stopPropagation();
        onRemove?.();
      }}
      type="button"
      aria-label="Remove"
    >
      <i class="bi bi-x"></i>
    </button>
  </div>
{:else}
  <button
    class="chip {variant} {size}"
    class:selected
    onclick={onClick}
    type="button"
  >
    {#if icon}
      <i class="bi {icon}"></i>
    {/if}

    <span class="chip-label">{label}</span>
  </button>
{/if}

<style>
  .chip {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    border: 1.5px solid rgba(15, 23, 42, 0.1);
    background: rgba(255, 255, 255, 0.5);
    backdrop-filter: blur(8px);
    border-radius: 10px;
    font-family: "Inter", sans-serif;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    white-space: nowrap;
    letter-spacing: -0.1px;
  }

  button.chip {
    border: 1.5px solid rgba(15, 23, 42, 0.1);
  }

  div.chip {
    border: 1.5px solid rgba(15, 23, 42, 0.1);
  }

  .chip:hover {
    background: rgba(255, 255, 255, 0.8);
    border-color: rgba(99, 102, 241, 0.3);
    transform: translateY(-1px);
  }

  .chip.selected {
    background: linear-gradient(135deg, #6366f1, #8b5cf6);
    border-color: transparent;
    color: white;
    box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
  }

  .chip i {
    font-size: 14px;
  }

  .chip-label {
    line-height: 1;
  }

  .remove-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 18px;
    height: 18px;
    border: none;
    background: rgba(0, 0, 0, 0.15);
    border-radius: 50%;
    cursor: pointer;
    transition: all 0.2s;
    padding: 0;
    margin-left: 2px;
  }

  .remove-btn:hover {
    background: rgba(0, 0, 0, 0.25);
    transform: scale(1.1);
  }

  .remove-btn i {
    font-size: 12px;
  }

  /* Sizes */
  .small {
    height: 24px;
    padding: 0 10px;
    font-size: 11px;
  }

  .small i {
    font-size: 12px;
  }

  .small .remove-btn {
    width: 16px;
    height: 16px;
  }

  .medium {
    height: 32px;
    padding: 0 14px;
    font-size: 13px;
  }

  .large {
    height: 40px;
    padding: 0 18px;
    font-size: 14px;
  }

  .large i {
    font-size: 16px;
  }

  .large .remove-btn {
    width: 20px;
    height: 20px;
  }

  /* Variants */
  .tag {
    cursor: default;
    background: linear-gradient(
      135deg,
      rgba(99, 102, 241, 0.08),
      rgba(139, 92, 246, 0.08)
    );
    border-color: rgba(99, 102, 241, 0.15);
    color: #6366f1;
  }

  .tag:hover {
    transform: none;
  }

  .removable {
    padding-right: 8px;
  }

  /* Dark Mode */
  :global(.dark) .chip {
    background: rgba(255, 255, 255, 0.06);
    border-color: rgba(255, 255, 255, 0.1);
  }

  :global(.dark) .chip:hover {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(129, 140, 248, 0.3);
  }

  :global(.dark) .chip.selected {
    background: linear-gradient(135deg, #6366f1, #8b5cf6);
  }

  :global(.dark) .tag {
    background: rgba(99, 102, 241, 0.15);
    border-color: rgba(99, 102, 241, 0.25);
    color: #818cf8;
  }

  :global(.dark) .remove-btn {
    background: rgba(255, 255, 255, 0.15);
  }

  :global(.dark) .remove-btn:hover {
    background: rgba(255, 255, 255, 0.25);
  }
</style>
