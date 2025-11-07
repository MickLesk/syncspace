<script>
  import { createEventDispatcher } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import * as api from "../../lib/api.js";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  let {
    value = $bindable(""),
    placeholder = "",
    fullWidth = false,
    enableSuggestions = false,
    oninput = null,
    onclear = null,
    onselect = null,
  } = $props();

  const defaultPlaceholder = $derived(placeholder || tr("search"));

  const dispatch = createEventDispatcher();
  let focused = $state(false);
  let suggestions = $state([]);
  let showSuggestions = $state(false);
  let selectedIndex = $state(-1);
  let suggestDebounceTimer = $state(null);

  async function handleInput(e) {
    value = e.target.value;
    dispatch("input", value);
    if (typeof oninput === "function") oninput(value);

    // Load suggestions if enabled and value is not empty
    if (enableSuggestions && value.length >= 2) {
      // Debounce suggestions
      clearTimeout(suggestDebounceTimer);
      suggestDebounceTimer = setTimeout(async () => {
        try {
          const results = await api.search.suggest(value, 8);
          suggestions = results;
          showSuggestions = true;
          selectedIndex = -1;
        } catch (error) {
          console.error("Failed to load suggestions:", error);
          suggestions = [];
        }
      }, 300);
    } else {
      showSuggestions = false;
      suggestions = [];
    }
  }

  function handleClear() {
    value = "";
    suggestions = [];
    showSuggestions = false;
    dispatch("clear");
    if (typeof onclear === "function") onclear();
  }

  function handleFocus() {
    focused = true;
    if (enableSuggestions && value.length >= 2 && suggestions.length > 0) {
      showSuggestions = true;
    }
  }

  function handleBlur() {
    // Delay to allow clicking on suggestions
    setTimeout(() => {
      focused = false;
      showSuggestions = false;
    }, 200);
  }

  function selectSuggestion(suggestion) {
    value = suggestion.text;
    showSuggestions = false;
    suggestions = [];
    dispatch("select", suggestion);
    if (typeof onselect === "function") onselect(suggestion);
  }

  function handleKeydown(e) {
    if (!showSuggestions || suggestions.length === 0) return;

    if (e.key === "ArrowDown") {
      e.preventDefault();
      selectedIndex = Math.min(selectedIndex + 1, suggestions.length - 1);
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      selectedIndex = Math.max(selectedIndex - 1, -1);
    } else if (e.key === "Enter" && selectedIndex >= 0) {
      e.preventDefault();
      selectSuggestion(suggestions[selectedIndex]);
    } else if (e.key === "Escape") {
      showSuggestions = false;
      selectedIndex = -1;
    }
  }
</script>

<div class="search-bar-container">
  <div class="search-bar" class:focused class:full-width={fullWidth}>
    <span class="search-icon">🔍</span>
    <input
      type="text"
      class="search-input"
      placeholder={defaultPlaceholder}
      bind:value
      oninput={handleInput}
      onfocus={handleFocus}
      onblur={handleBlur}
      onkeydown={handleKeydown}
    />
    {#if value}
      <button class="clear-button" onclick={handleClear} type="button">
        ✕
      </button>
    {/if}
  </div>

  {#if showSuggestions && suggestions.length > 0}
    <div class="suggestions-dropdown">
      {#each suggestions as suggestion, index}
        <button
          class="suggestion-item"
          class:selected={index === selectedIndex}
          onclick={() => selectSuggestion(suggestion)}
          type="button"
        >
          <span class="suggestion-icon">
            {#if suggestion.file_type === "document"}📄
            {:else if suggestion.file_type === "image"}🖼️
            {:else if suggestion.file_type === "video"}🎬
            {:else if suggestion.file_type === "audio"}🎵
            {:else if suggestion.file_type === "code"}💻
            {:else if suggestion.file_type === "archive"}📦
            {:else}📁
            {/if}
          </span>
          <span class="suggestion-text">{suggestion.text}</span>
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .search-bar-container {
    position: relative;
    width: 320px;
    max-width: 100%;
  }

  .search-bar-container:has(.full-width) {
    width: 100%;
  }

  .search-bar {
    display: flex;
    align-items: center;
    gap: 12px;
    height: 48px;
    padding: 0 16px;
    background: var(--md-sys-color-surface-variant);
    border-radius: 24px;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    width: 100%;
  }

  .search-bar.full-width {
    width: 100%;
  }

  .search-bar.focused {
    background: var(--md-sys-color-surface);
    box-shadow: var(--md-elevation-2);
  }

  .search-icon {
    font-size: 20px;
    color: var(--md-sys-color-on-surface-variant);
    flex-shrink: 0;
  }

  .search-input {
    flex: 1;
    border: none;
    background: transparent;
    color: var(--md-sys-color-on-surface);
    font-family: "Roboto", sans-serif;
    font-size: 16px;
    outline: none;
  }

  .search-input::placeholder {
    color: var(--md-sys-color-on-surface-variant);
  }

  .clear-button {
    width: 24px;
    height: 24px;
    border: none;
    background: var(--md-sys-color-on-surface-variant);
    color: var(--md-sys-color-surface);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.2s;
    flex-shrink: 0;
  }

  .clear-button:hover {
    background: var(--md-sys-color-on-surface);
  }

  .suggestions-dropdown {
    position: absolute;
    top: calc(100% + 8px);
    left: 0;
    right: 0;
    background: var(--md-sys-color-surface);
    border-radius: 16px;
    box-shadow: var(--md-elevation-3);
    overflow: hidden;
    z-index: 1000;
    max-height: 400px;
    overflow-y: auto;
  }

  .suggestion-item {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    padding: 12px 16px;
    border: none;
    background: transparent;
    color: var(--md-sys-color-on-surface);
    cursor: pointer;
    transition: background 0.2s;
    text-align: left;
  }

  .suggestion-item:hover,
  .suggestion-item.selected {
    background: var(--md-sys-color-surface-variant);
  }

  .suggestion-icon {
    font-size: 20px;
    flex-shrink: 0;
  }

  .suggestion-text {
    flex: 1;
    font-size: 14px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
