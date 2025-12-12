<script>
  import { onMount } from "svelte";
  import UIInput from "../../components/ui/UIInput.svelte";
  import UISelect from "../../components/ui/UISelect.svelte";
  import UIToggle from "../../components/ui/UIToggle.svelte";
  import UICheckbox from "../../components/ui/UICheckbox.svelte";
  import { themeStore } from "../../stores/themes.js";
  import { authStore } from "../../stores/auth.js";
  import UICard from "../../components/ui/UICard.svelte";
  import UITabs from "../../components/ui/UITabs.svelte";
  import UIButton from "../../components/ui/UIButton.svelte";
  import UIModal from "../../components/ui/UIModal.svelte";

  let themes = [];
  let activeTheme = null;
  let presets = [];
  let isLoading = false;
  let error = null;

  let showCreateModal = false;
  let showEditModal = false;
  let editingTheme = null;

  let currentTab = "presets";
  const tabs = [
    { id: "presets", label: "Theme-Presets", icon: "palette" },
    { id: "custom", label: "Meine Themes", icon: "brush" },
    { id: "editor", label: "Theme-Editor", icon: "sliders" },
  ];

  // Theme-Editor State
  let editorTheme = {
    theme_name: "Neues Theme",
    primary_color: "#3b82f6",
    secondary_color: "#1e40af",
    accent_color: "#06b6d4",
    background_color: "#ffffff",
    surface_color: "#f8fafc",
    text_color: "#1f2937",
    density: "medium",
    font_size: "medium",
    border_radius: 8,
    shadow_intensity: 5,
    glass_effect: true,
    animations: true,
    high_contrast: false,
  };

  let fileInput;

  // Reactive statements
  $: ({ themes, activeTheme, presets, isLoading, error } = $themeStore);

  onMount(async () => {
    await themeStore.loadThemes();
  });

  // Preset categories
  const presetCategories = [
    { id: "standard", label: "Standard", icon: "house" },
    { id: "nature", label: "Natur", icon: "tree" },
    { id: "warm", label: "Warm", icon: "sun" },
    { id: "cool", label: "Kühl", icon: "snowflake" },
    { id: "vibrant", label: "Lebendig", icon: "rainbow" },
    { id: "dark", label: "Dunkel", icon: "moon" },
    {
      id: "accessibility",
      label: "Barrierefreiheit",
      icon: "universal-access-circle",
    },
    { id: "retro", label: "Retro", icon: "cassette" },
    { id: "classic", label: "Klassisch", icon: "gem" },
    { id: "pastel", label: "Pastell", icon: "heart" },
    { id: "urban", label: "Urban", icon: "buildings" },
  ];

  // Functions
  async function activateTheme(themeId) {
    try {
      await themeStore.activateTheme(themeId);
    } catch (err) {
      console.error("Failed to activate theme:", err);
    }
  }

  async function createFromPreset(preset) {
    try {
      const newTheme = await themeStore.createFromPreset(preset);
      await activateTheme(newTheme.id);
      currentTab = "custom";
    } catch (err) {
      console.error("Failed to create theme from preset:", err);
    }
  }

  async function deleteTheme(themeId) {
    if (confirm("Theme wirklich löschen?")) {
      try {
        await themeStore.deleteTheme(themeId);
      } catch (err) {
        console.error("Failed to delete theme:", err);
      }
    }
  }

  function openEditModal(theme) {
    editingTheme = { ...theme };
    showEditModal = true;
  }

  async function saveEditedTheme() {
    try {
      await themeStore.updateTheme(editingTheme.id, editingTheme);
      showEditModal = false;
      editingTheme = null;
    } catch (err) {
      console.error("Failed to update theme:", err);
    }
  }

  async function createCustomTheme() {
    try {
      const newTheme = await themeStore.createTheme({
        ...editorTheme,
        is_custom: true,
      });
      await activateTheme(newTheme.id);
      currentTab = "custom";
    } catch (err) {
      console.error("Failed to create custom theme:", err);
    }
  }

  async function exportTheme(themeId) {
    try {
      await themeStore.exportTheme(themeId);
    } catch (err) {
      console.error("Failed to export theme:", err);
    }
  }

  function handleImportFile() {
    fileInput.click();
  }

  async function importTheme(event) {
    const file = event.target.files[0];
    if (!file) return;

    try {
      const importedTheme = await themeStore.importTheme(file);
      currentTab = "custom";
    } catch (err) {
      console.error("Failed to import theme:", err);
    }
  }

  function getPresetsByCategory(category) {
    return presets.filter((preset) => preset.category === category);
  }
</script>

<div class="theme-customization-view">
  <div class="header mb-6">
    <h1 class="text-3xl font-bold text-white mb-2">
      <i class="bi bi-palette me-3"></i>
      Theme-Anpassung
    </h1>
    <p class="text-white/80">
      Personalisiere das Aussehen von SyncSpace mit vorgefertigten Themes oder
      erstelle deine eigenen.
    </p>
  </div>

  <UITabs bind:currentTab {tabs} />

  {#if error}
    <div class="alert alert-error mb-4">
      <i class="bi bi-exclamation-triangle"></i>
      <span>{error}</span>
      <UIButton
        size="sm"
        variant="ghost"
        onclick={() => themeStore.clearError()}
      >
        <i class="bi bi-x"></i>
      </UIButton>
    </div>
  {/if}

  <!-- Preset Tab -->
  {#if currentTab === "presets"}
    <UICard>
      <div class="space-y-8">
        {#each presetCategories as category}
          {#if getPresetsByCategory(category.id).length > 0}
            <div>
              <h3
                class="text-xl font-semibold text-white mb-4 flex items-center"
              >
                <i class="bi bi-{category.icon} me-2"></i>
                {category.label}
              </h3>
              <div
                class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4"
              >
                {#each getPresetsByCategory(category.id) as preset}
                  <div class="theme-preset-card">
                    <div class="colors-preview">
                      <div
                        class="color-swatch"
                        style="background: {preset.primary_color}"
                      ></div>
                      <div
                        class="color-swatch"
                        style="background: {preset.secondary_color}"
                      ></div>
                      <div
                        class="color-swatch"
                        style="background: {preset.accent_color}"
                      ></div>
                    </div>
                    <div class="theme-info">
                      <h4 class="text-lg font-semibold text-white">
                        {preset.display_name}
                      </h4>
                      <p class="text-sm text-white/70">{preset.description}</p>
                    </div>
                    <div class="theme-actions">
                      <UIButton
                        variant="primary"
                        size="sm"
                        onclick={() => createFromPreset(preset)}
                        disabled={isLoading}
                      >
                        <i class="bi bi-download me-1"></i>
                        Verwenden
                      </UIButton>
                    </div>
                  </div>
                {/each}
              </div>
            </div>
          {/if}
        {/each}
      </div>
    </UICard>
  {/if}

  <!-- Custom Themes Tab -->
  {#if currentTab === "custom"}
    <UICard>
      <div class="flex justify-between items-center mb-6">
        <h2 class="text-xl font-semibold text-white">Meine Themes</h2>
        <div class="flex gap-2">
          <UIButton variant="secondary" onclick={handleImportFile}>
            <i class="bi bi-upload me-1"></i>
            Theme importieren
          </UIButton>
        </div>
      </div>

      {#if themes.length === 0}
        <div class="text-center py-12">
          <i class="bi bi-palette text-6xl text-white/30 mb-4"></i>
          <p class="text-white/70 mb-4">
            Noch keine benutzerdefinierten Themes erstellt.
          </p>
          <UIButton variant="primary" onclick={() => (currentTab = "editor")}>
            Erstes Theme erstellen
          </UIButton>
        </div>
      {:else}
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {#each themes as theme}
            <div
              class="theme-custom-card"
              class:active={activeTheme?.id === theme.id}
            >
              <div class="colors-preview">
                <div
                  class="color-swatch"
                  style="background: {theme.primary_color}"
                ></div>
                <div
                  class="color-swatch"
                  style="background: {theme.secondary_color}"
                ></div>
                <div
                  class="color-swatch"
                  style="background: {theme.accent_color}"
                ></div>
              </div>

              <div class="theme-info">
                <h4 class="text-lg font-semibold text-white">
                  {theme.theme_name}
                </h4>
                <p class="text-sm text-white/70">
                  {theme.is_custom ? "Benutzerdefiniert" : "Preset"}
                  • Erstellt am {new Date(theme.created_at).toLocaleDateString(
                    "de-DE"
                  )}
                </p>
                {#if activeTheme?.id === theme.id}
                  <span class="active-badge">
                    <i class="bi bi-check-circle me-1"></i>
                    Aktiv
                  </span>
                {/if}
              </div>

              <div class="theme-actions">
                <div class="flex gap-2">
                  {#if activeTheme?.id !== theme.id}
                    <UIButton
                      variant="primary"
                      size="sm"
                      onclick={() => activateTheme(theme.id)}
                      disabled={isLoading}
                    >
                      <i class="bi bi-check me-1"></i>
                      Aktivieren
                    </UIButton>
                  {/if}

                  <UIButton
                    variant="ghost"
                    size="sm"
                    onclick={() => openEditModal(theme)}
                  >
                    <i class="bi bi-pencil"></i>
                  </UIButton>

                  <UIButton
                    variant="ghost"
                    size="sm"
                    onclick={() => exportTheme(theme.id)}
                  >
                    <i class="bi bi-download"></i>
                  </UIButton>

                  {#if theme.is_custom}
                    <UIButton
                      variant="ghost"
                      size="sm"
                      onclick={() => deleteTheme(theme.id)}
                      class="text-red-400 hover:text-red-300"
                    >
                      <i class="bi bi-trash"></i>
                    </UIButton>
                  {/if}
                </div>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </UICard>
  {/if}

  <!-- Theme Editor Tab -->
  {#if currentTab === "editor"}
    <UICard>
      <h2 class="text-xl font-semibold text-white mb-6">Theme-Editor</h2>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <!-- Editor Controls -->
        <div class="space-y-6">
          <!-- Basic Info -->
          <div>
            <label class="block text-sm font-medium text-white mb-2"
              >Theme-Name</label
            >
            <input
              type="text"
              bind:value={editorTheme.theme_name}
              class="input-field w-full"
              placeholder="Mein Custom Theme"
            />
          </div>

          <!-- Colors -->
          <div>
            <h3 class="text-lg font-medium text-white mb-4">Farben</h3>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm text-white/80 mb-2"
                  >Primärfarbe</label
                >
                <input
                  type="color"
                  bind:value={editorTheme.primary_color}
                  class="color-input"
                />
              </div>
              <div>
                <label class="block text-sm text-white/80 mb-2"
                  >Sekundärfarbe</label
                >
                <input
                  type="color"
                  bind:value={editorTheme.secondary_color}
                  class="color-input"
                />
              </div>
              <div>
                <label class="block text-sm text-white/80 mb-2"
                  >Akzentfarbe</label
                >
                <input
                  type="color"
                  bind:value={editorTheme.accent_color}
                  class="color-input"
                />
              </div>
              <div>
                <label class="block text-sm text-white/80 mb-2"
                  >Hintergrund</label
                >
                <input
                  type="color"
                  bind:value={editorTheme.background_color}
                  class="color-input"
                />
              </div>
            </div>
          </div>

          <!-- Layout Settings -->
          <div>
            <h3 class="text-lg font-medium text-white mb-4">Layout</h3>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm text-white/80 mb-2">Dichte</label>
                <select bind:value={editorTheme.density} class="select-field">
                  <option value="compact">Kompakt</option>
                  <option value="medium">Normal</option>
                  <option value="comfortable">Komfortabel</option>
                </select>
              </div>
              <div>
                <label class="block text-sm text-white/80 mb-2"
                  >Schriftgröße</label
                >
                <select bind:value={editorTheme.font_size} class="select-field">
                  <option value="small">Klein</option>
                  <option value="medium">Normal</option>
                  <option value="large">Groß</option>
                  <option value="xlarge">Sehr groß</option>
                </select>
              </div>
              <div>
                <label class="block text-sm text-white/80 mb-2"
                  >Eckenradius</label
                >
                <input
                  type="range"
                  min="0"
                  max="20"
                  bind:value={editorTheme.border_radius}
                  class="range-input"
                />
                <span class="text-xs text-white/60"
                  >{editorTheme.border_radius}px</span
                >
              </div>
              <div>
                <label class="block text-sm text-white/80 mb-2"
                  >Schatten-Intensität</label
                >
                <input
                  type="range"
                  min="0"
                  max="10"
                  bind:value={editorTheme.shadow_intensity}
                  class="range-input"
                />
                <span class="text-xs text-white/60"
                  >{editorTheme.shadow_intensity}</span
                >
              </div>
            </div>
          </div>

          <!-- Effects -->
          <div>
            <h3 class="text-lg font-medium text-white mb-4">Effekte</h3>
            <div class="space-y-3">
              <label class="flex items-center gap-3">
                <input
                  type="checkbox"
                  bind:checked={editorTheme.glass_effect}
                  class="checkbox"
                />
                <span class="text-white/90">Glasmorphismus-Effekt</span>
              </label>
              <label class="flex items-center gap-3">
                <input
                  type="checkbox"
                  bind:checked={editorTheme.animations}
                  class="checkbox"
                />
                <span class="text-white/90">Animationen aktivieren</span>
              </label>
              <label class="flex items-center gap-3">
                <input
                  type="checkbox"
                  bind:checked={editorTheme.high_contrast}
                  class="checkbox"
                />
                <span class="text-white/90">Hoher Kontrast</span>
              </label>
            </div>
          </div>

          <div class="pt-4">
            <UIButton
              variant="primary"
              onclick={createCustomTheme}
              disabled={isLoading}
            >
              <i class="bi bi-plus me-1"></i>
              Theme erstellen
            </UIButton>
          </div>
        </div>

        <!-- Live Preview -->
        <div>
          <h3 class="text-lg font-medium text-white mb-4">Vorschau</h3>
          <div
            class="theme-preview"
            style="
            --primary: {editorTheme.primary_color};
            --secondary: {editorTheme.secondary_color};
            --accent: {editorTheme.accent_color};
            --background: {editorTheme.background_color};
            --surface: {editorTheme.surface_color};
            --border-radius: {editorTheme.border_radius}px;
            --shadow-intensity: {editorTheme.shadow_intensity};
          "
          >
            <div class="preview-window">
              <div class="preview-header">
                <div class="preview-buttons">
                  <span style="background: #ff5f57"></span>
                  <span style="background: #febc2e"></span>
                  <span style="background: #28ca42"></span>
                </div>
                <span class="text-xs">SyncSpace Vorschau</span>
              </div>
              <div class="preview-content">
                <div class="preview-sidebar">
                  <div class="preview-nav-item active">
                    <i class="bi bi-house"></i> Dashboard
                  </div>
                  <div class="preview-nav-item">
                    <i class="bi bi-folder"></i> Dateien
                  </div>
                  <div class="preview-nav-item">
                    <i class="bi bi-search"></i> Suche
                  </div>
                </div>
                <div class="preview-main">
                  <div class="preview-card">
                    <h4>Beispiel-Karte</h4>
                    <p>So würde eine Karte mit diesem Theme aussehen.</p>
                    <button class="preview-button">Beispiel Button</button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </UICard>
  {/if}
</div>

<!-- Edit Modal -->
<UIModal bind:showModal={showEditModal} title="Theme bearbeiten">
  {#if editingTheme}
    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-white mb-2"
          >Theme-Name</label
        >
        <input
          type="text"
          bind:value={editingTheme.theme_name}
          class="input-field w-full"
        />
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div>
          <label class="block text-sm text-white/80 mb-2">Primärfarbe</label>
          <input
            type="color"
            bind:value={editingTheme.primary_color}
            class="color-input"
          />
        </div>
        <div>
          <label class="block text-sm text-white/80 mb-2">Akzentfarbe</label>
          <input
            type="color"
            bind:value={editingTheme.accent_color}
            class="color-input"
          />
        </div>
      </div>
    </div>
  {/if}

  <svelte:fragment slot="actions">
    {#if editingTheme}
      <UIButton variant="ghost" onclick={() => (showEditModal = false)}>
        Abbrechen
      </UIButton>
      <UIButton
        variant="primary"
        onclick={saveEditedTheme}
        disabled={isLoading}
      >
        Speichern
      </UIButton>
    {/if}
  </svelte:fragment>
</UIModal>

<!-- Hidden file input for import -->
<input
  type="file"
  accept=".json,.syncspace-theme"
  bind:this={fileInput}
  onchange={importTheme}
  style="display: none"
/>

<style>
  .theme-customization-view {
    max-width: 1400px;
    margin: 0 auto;
    padding: 2rem;
  }

  .theme-preset-card {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 1rem;
    transition: all 0.2s ease;
  }

  .theme-preset-card:hover {
    background: rgba(255, 255, 255, 0.08);
    border-color: rgba(255, 255, 255, 0.2);
    transform: translateY(-2px);
  }

  .theme-custom-card {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 1rem;
    transition: all 0.2s ease;
    position: relative;
  }

  .theme-custom-card:hover {
    background: rgba(255, 255, 255, 0.08);
    border-color: rgba(255, 255, 255, 0.2);
  }

  .theme-custom-card.active {
    border-color: var(--primary-color, #3b82f6);
    background: rgba(59, 130, 246, 0.1);
  }

  .colors-preview {
    display: flex;
    gap: 4px;
    margin-bottom: 0.75rem;
  }

  .color-swatch {
    width: 24px;
    height: 24px;
    border-radius: 6px;
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .theme-info h4 {
    margin-bottom: 0.25rem;
  }

  .theme-actions {
    margin-top: 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .active-badge {
    display: inline-flex;
    align-items: center;
    padding: 0.25rem 0.5rem;
    background: rgba(34, 197, 94, 0.2);
    color: rgb(34, 197, 94);
    border-radius: 6px;
    font-size: 0.75rem;
    font-weight: 500;
    margin-top: 0.5rem;
  }

  .input-field,
  .select-field {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 8px;
    padding: 0.5rem;
    color: white;
    transition: all 0.2s ease;
  }

  .input-field:focus,
  .select-field:focus {
    outline: none;
    border-color: var(--primary-color, #3b82f6);
    background: rgba(255, 255, 255, 0.1);
  }

  .color-input {
    width: 100%;
    height: 40px;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.2);
    background: transparent;
    cursor: pointer;
  }

  .range-input {
    width: 100%;
    height: 4px;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 2px;
    outline: none;
    -webkit-appearance: none;
  }

  .range-input::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 16px;
    height: 16px;
    background: var(--primary-color, #3b82f6);
    border-radius: 50%;
    cursor: pointer;
  }

  .checkbox {
    width: 16px;
    height: 16px;
    accent-color: var(--primary-color, #3b82f6);
  }

  .theme-preview {
    border-radius: 12px;
    overflow: hidden;
    border: 1px solid rgba(255, 255, 255, 0.2);
    background: rgba(255, 255, 255, 0.05);
  }

  .preview-window {
    background: var(--background);
    min-height: 300px;
  }

  .preview-header {
    background: rgba(0, 0, 0, 0.1);
    padding: 0.5rem 1rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .preview-buttons {
    display: flex;
    gap: 4px;
  }

  .preview-buttons span {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    display: block;
  }

  .preview-content {
    display: flex;
    min-height: 260px;
  }

  .preview-sidebar {
    width: 120px;
    background: rgba(0, 0, 0, 0.1);
    padding: 1rem 0.5rem;
    border-right: 1px solid rgba(255, 255, 255, 0.1);
  }

  .preview-nav-item {
    padding: 0.5rem;
    margin-bottom: 0.25rem;
    border-radius: var(--border-radius);
    font-size: 0.75rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: rgba(255, 255, 255, 0.7);
  }

  .preview-nav-item.active {
    background: var(--primary);
    color: white;
  }

  .preview-main {
    flex: 1;
    padding: 1rem;
  }

  .preview-card {
    background: var(--surface);
    border-radius: var(--border-radius);
    padding: 1rem;
    border: 1px solid rgba(255, 255, 255, 0.1);
    box-shadow: 0 4px 6px rgba(0, 0, 0, calc(var(--shadow-intensity) * 0.05));
  }

  .preview-card h4 {
    color: var(--primary);
    margin-bottom: 0.5rem;
  }

  .preview-card p {
    color: rgba(255, 255, 255, 0.8);
    font-size: 0.8rem;
    margin-bottom: 1rem;
  }

  .preview-button {
    background: var(--accent);
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: var(--border-radius);
    font-size: 0.8rem;
    cursor: pointer;
    transition: opacity 0.2s ease;
  }

  .preview-button:hover {
    opacity: 0.9;
  }

  @media (max-width: 768px) {
    .theme-customization-view {
      padding: 1rem;
    }

    .grid {
      grid-template-columns: 1fr;
    }
  }
</style>
