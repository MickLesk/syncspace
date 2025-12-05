<script>
  import { onMount } from "svelte";
  import {
    customTheme,
    PRESET_THEMES,
    currentPresetName,
  } from "$stores/customTheme.js";
  import { t } from "$lib/i18n.js";

  let showColorPicker = null;
  let showAdvanced = false;
  let importFile;

  onMount(() => {
    customTheme.load();
  });

  function handleColorChange(property, color) {
    customTheme.updateProperty(property, color);
  }

  function handlePresetClick(presetKey) {
    customTheme.applyPreset(presetKey);
  }

  function handleDensityChange(density) {
    customTheme.updateProperty("density", density);
  }

  function handleFontSizeChange(size) {
    customTheme.updateProperty("fontSize", size);
  }

  function handleBorderRadiusChange(radius) {
    customTheme.updateProperty("borderRadius", radius);
  }

  function handleShadowIntensityChange(intensity) {
    customTheme.updateProperty("shadowIntensity", intensity);
  }

  function handleImport() {
    if (importFile?.files?.[0]) {
      customTheme.import(importFile.files[0]);
      importFile.value = "";
    }
  }

  function toggleColorPicker(property) {
    showColorPicker = showColorPicker === property ? null : property;
  }
</script>

<div
  class="p-6 bg-white dark:bg-gray-900 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700"
>
  <!-- Header -->
  <div class="mb-8">
    <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
      <i class="bi bi-palette mr-2" aria-hidden="true"></i>
      {t("settings.theme_customization")}
    </h2>
    <p class="text-sm text-gray-600 dark:text-gray-400">
      {t("settings.customize_colors_and_appearance")}
    </p>
  </div>

  <!-- Preset Themes -->
  <div class="mb-8">
    <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
      {t("theme.presets")}
    </h3>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
      {#each Object.entries(PRESET_THEMES) as [key, preset]}
        <button
          onclick={() => handlePresetClick(key)}
          class="p-4 rounded-lg border-2 transition-all hover:shadow-md {$currentPresetName ===
          key
            ? 'border-green-500 bg-green-50 dark:border-green-400 dark:bg-green-900/20'
            : 'border-gray-300 bg-gray-50 dark:border-gray-600 dark:bg-gray-800'}"
        >
          <div class="flex gap-2 mb-3 h-6">
            <div
              class="flex-1 rounded"
              style="background-color: {preset.primaryColor}"
            ></div>
            <div
              class="flex-1 rounded"
              style="background-color: {preset.secondaryColor}"
            ></div>
            <div
              class="flex-1 rounded"
              style="background-color: {preset.accentColor}"
            ></div>
          </div>
          <p class="font-medium text-gray-900 dark:text-white text-sm">
            {preset.name}
          </p>
          <p class="text-xs text-gray-500 dark:text-gray-400">
            {preset.description}
          </p>
          {#if $currentPresetName === key}
            <div
              class="mt-2 text-xs text-green-600 dark:text-green-400 flex items-center"
            >
              <i class="bi bi-check-circle mr-1" aria-hidden="true"></i>
              {t("theme.active")}
            </div>
          {/if}
        </button>
      {/each}
    </div>
  </div>

  <!-- Color Customization -->
  <div class="mb-8 pb-8 border-b border-gray-200 dark:border-gray-700">
    <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
      {t("theme.custom_colors")}
    </h3>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <!-- Primary Color -->
      <div>
        <div
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
        >
          {t("theme.primary_color")}
        </div>
        <div class="flex gap-2">
          <button
            onclick={() => toggleColorPicker("primaryColor")}
            class="w-12 h-12 rounded-lg border-2 border-gray-300 dark:border-gray-600 hover:border-gray-500 transition-colors cursor-pointer"
            style="background-color: {$customTheme.primaryColor}"
            title={t("theme.click_to_edit")}
          ></button>
          <input
            type="text"
            value={$customTheme.primaryColor}
            onchange={(e) => handleColorChange("primaryColor", e.target.value)}
            class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg dark:bg-gray-800 dark:text-white"
            placeholder="#3B82F6"
          />
        </div>
        {#if showColorPicker === "primaryColor"}
          <input
            type="color"
            value={$customTheme.primaryColor}
            onchange={(e) => handleColorChange("primaryColor", e.target.value)}
            class="mt-2"
          />
        {/if}
      </div>

      <!-- Secondary Color -->
      <div>
        <div
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
        >
          {t("theme.secondary_color")}
        </div>
        <div class="flex gap-2">
          <button
            onclick={() => toggleColorPicker("secondaryColor")}
            class="w-12 h-12 rounded-lg border-2 border-gray-300 dark:border-gray-600 hover:border-gray-500 transition-colors cursor-pointer"
            style="background-color: {$customTheme.secondaryColor}"
            title={t("theme.click_to_edit")}
          ></button>
          <input
            type="text"
            value={$customTheme.secondaryColor}
            on:change={(e) =>
              handleColorChange("secondaryColor", e.target.value)}
            class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg dark:bg-gray-800 dark:text-white"
            placeholder="#10B981"
          />
        </div>
        {#if showColorPicker === "secondaryColor"}
          <input
            type="color"
            value={$customTheme.secondaryColor}
            on:change={(e) =>
              handleColorChange("secondaryColor", e.target.value)}
            class="mt-2"
          />
        {/if}
      </div>

      <!-- Accent Color -->
      <div>
        <div
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
        >
          {t("theme.accent_color")}
        </div>
        <div class="flex gap-2">
          <button
            onclick={() => toggleColorPicker("accentColor")}
            class="w-12 h-12 rounded-lg border-2 border-gray-300 dark:border-gray-600 hover:border-gray-500 transition-colors cursor-pointer"
            style="background-color: {$customTheme.accentColor}"
            title={t("theme.click_to_edit")}
          ></button>
          <input
            type="text"
            value={$customTheme.accentColor}
            on:change={(e) => handleColorChange("accentColor", e.target.value)}
            class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg dark:bg-gray-800 dark:text-white"
            placeholder="#F59E0B"
          />
        </div>
        {#if showColorPicker === "accentColor"}
          <input
            type="color"
            value={$customTheme.accentColor}
            on:change={(e) => handleColorChange("accentColor", e.target.value)}
            class="mt-2"
          />
        {/if}
      </div>
    </div>
  </div>

  <!-- Display Settings -->
  <div class="mb-8 pb-8 border-b border-gray-200 dark:border-gray-700">
    <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
      {t("theme.display_settings")}
    </h3>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <!-- Density -->
      <div>
        <div
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3"
        >
          {t("theme.density")}
        </div>
        <div class="space-y-2">
          {#each ["compact", "comfortable", "spacious"] as option}
            <label class="flex items-center cursor-pointer">
              <input
                type="radio"
                name="density"
                value={option}
                checked={$customTheme.density === option}
                on:change={() => handleDensityChange(option)}
                class="mr-3"
              />
              <span class="text-gray-700 dark:text-gray-300 capitalize">
                {t(`theme.density_${option}`)}
              </span>
            </label>
          {/each}
        </div>
      </div>

      <!-- Font Size -->
      <div>
        <div
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3"
        >
          {t("theme.font_size")}
        </div>
        <div class="space-y-2">
          {#each ["sm", "base", "lg", "xl"] as option}
            <label class="flex items-center cursor-pointer">
              <input
                type="radio"
                name="fontSize"
                value={option}
                checked={$customTheme.fontSize === option}
                on:change={() => handleFontSizeChange(option)}
                class="mr-3"
              />
              <span class="text-gray-700 dark:text-gray-300 capitalize">
                {t(`theme.size_${option}`)}
              </span>
            </label>
          {/each}
        </div>
      </div>

      <!-- Border Radius -->
      <div>
        <div
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3"
        >
          {t("theme.border_radius")}
        </div>
        <div class="space-y-2">
          {#each ["none", "sm", "base", "lg", "xl"] as option}
            <label class="flex items-center cursor-pointer">
              <input
                type="radio"
                name="borderRadius"
                value={option}
                checked={$customTheme.borderRadius === option}
                on:change={() => handleBorderRadiusChange(option)}
                class="mr-3"
              />
              <span class="text-gray-700 dark:text-gray-300 capitalize">
                {t(`theme.radius_${option}`)}
              </span>
            </label>
          {/each}
        </div>
      </div>

      <!-- Shadow Intensity -->
      <div>
        <div
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3"
        >
          {t("theme.shadow_intensity")}
        </div>
        <div class="space-y-2">
          {#each ["light", "normal", "strong"] as option}
            <label class="flex items-center cursor-pointer">
              <input
                type="radio"
                name="shadowIntensity"
                value={option}
                checked={$customTheme.shadowIntensity === option}
                on:change={() => handleShadowIntensityChange(option)}
                class="mr-3"
              />
              <span class="text-gray-700 dark:text-gray-300 capitalize">
                {t(`theme.shadow_${option}`)}
              </span>
            </label>
          {/each}
        </div>
      </div>
    </div>
  </div>

  <!-- Import/Export -->
  <div>
    <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
      {t("theme.import_export")}
    </h3>
    <div class="flex gap-4">
      <button
        onclick={() => customTheme.export()}
        class="flex-1 px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors flex items-center justify-center"
      >
        <i class="bi bi-download mr-2" aria-hidden="true"></i>
        {t("theme.export")}
      </button>

      <label class="flex-1">
        <button
          onclick={() => importFile?.click()}
          class="w-full px-4 py-2 bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 text-gray-900 dark:text-white rounded-lg transition-colors flex items-center justify-center cursor-pointer"
        >
          <i class="bi bi-upload mr-2" aria-hidden="true"></i>
          {t("theme.import")}
        </button>
        <input
          bind:this={importFile}
          type="file"
          accept=".json"
          on:change={handleImport}
          hidden
        />
      </label>

      <button
        onclick={() => customTheme.reset()}
        class="flex-1 px-4 py-2 bg-red-100 dark:bg-red-900/20 hover:bg-red-200 dark:hover:bg-red-900/30 text-red-700 dark:text-red-400 rounded-lg transition-colors flex items-center justify-center"
      >
        <i class="bi bi-arrow-counterclockwise mr-2" aria-hidden="true"></i>
        {t("theme.reset")}
      </button>
    </div>
  </div>
</div>

<style>
  :global([data-density="compact"]) {
    --spacing-factor: 0.75;
  }

  :global([data-density="comfortable"]) {
    --spacing-factor: 1;
  }

  :global([data-density="spacious"]) {
    --spacing-factor: 1.25;
  }

  :global([data-font-size="sm"]) {
    font-size: 0.875rem;
  }

  :global([data-font-size="base"]) {
    font-size: 1rem;
  }

  :global([data-font-size="lg"]) {
    font-size: 1.125rem;
  }

  :global([data-font-size="xl"]) {
    font-size: 1.25rem;
  }
</style>
