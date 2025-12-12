<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../../stores/ui.js";
  import t from "../../../i18n.js";
  import { tags as tagsApi } from "../../../lib/api.js";
  import { success, error as errorToast } from "../../../stores/toast.js";

  // Standard UI Components
  import StandardGlassCard from "../../../components/ui/StandardGlassCard.svelte";
  import StandardButton from "../../../components/ui/StandardButton.svelte";
  import StandardModal from "../../../components/ui/StandardModal.svelte";
  import StandardTabs from "../../../components/ui/StandardTabs.svelte";

  // State with Svelte 5 runes
  let loading = $state(true);
  let tags = $state([]);
  let error = $state(null);
  let selectedTags = $state(new Set());
  let activeTab = $state("cloud");

  // Tag detail modal
  let showTagModal = $state(false);
  let selectedTag = $state(null);
  let tagFiles = $state([]);
  let loadingTagFiles = $state(false);

  // Create tag modal
  let showCreateTagModal = $state(false);
  let newTagName = $state("");
  let newTagColor = $state("#10B981");
  let creatingTag = $state(false);

  // Cloud configuration
  let minFontSize = $state(16);
  let maxFontSize = $state(42);
  let colorMode = $state("category");

  // Filter & sort
  let sortBy = $state("usage");
  let minUsage = $state(1);
  let searchQuery = $state("");

  const colorCategories = [
    "#10B981", // emerald
    "#3B82F6", // blue
    "#F59E0B", // amber
    "#8B5CF6", // violet
    "#EC4899", // pink
    "#06B6D4", // cyan
    "#EF4444", // red
    "#84CC16", // lime
    "#F97316", // orange
    "#6366F1", // indigo
  ];

  // Tabs configuration
  const tabs = [
    {
      id: "cloud",
      icon: "cloud",
      label: t("tags.cloudView") || "Tag Cloud",
      badge: () => tags.length || null,
    },
    {
      id: "list",
      icon: "list-ul",
      label: t("tags.listView") || "Liste",
      badge: null,
    },
    {
      id: "manage",
      icon: "gear",
      label: t("tags.management") || "Verwaltung",
      badge: () => selectedTags.size || null,
    },
  ];

  // Computed values
  const filteredTags = $derived(
    tags
      .filter((tag) => {
        const matchesSearch =
          !searchQuery ||
          tag.name.toLowerCase().includes(searchQuery.toLowerCase());
        const matchesUsage = tag.usage_count >= minUsage;
        return matchesSearch && matchesUsage;
      })
      .sort((a, b) => {
        if (sortBy === "usage") {
          return b.usage_count - a.usage_count;
        } else if (sortBy === "name") {
          return a.name.localeCompare(b.name);
        } else if (sortBy === "recent") {
          return (
            new Date(b.updated_at || b.created_at) -
            new Date(a.updated_at || a.created_at)
          );
        }
        return 0;
      })
  );

  const maxUsage = $derived(Math.max(...tags.map((tag) => tag.usage_count), 1));

  onMount(async () => {
    await loadTags();
  });

  async function loadTags() {
    loading = true;
    error = null;

    try {
      const response = await tagsApi.list();
      tags = response || [];
    } catch (err) {
      console.error("Failed to load tags:", err);
      error = t("tags.loadError") || "Fehler beim Laden der Tags";
    } finally {
      loading = false;
    }
  }

  async function createTag() {
    if (!newTagName.trim()) return;

    creatingTag = true;
    try {
      const tagData = {
        name: newTagName.trim(),
        color: newTagColor,
        description: "",
      };

      await tagsApi.create(tagData);
      await loadTags();

      // Reset form
      newTagName = "";
      newTagColor = "#10B981";
      showCreateTagModal = false;

      success(t("tags.createSuccess") || "Tag erfolgreich erstellt");
    } catch (err) {
      console.error("Failed to create tag:", err);
      errorToast(t("tags.createError") || "Fehler beim Erstellen des Tags");
    } finally {
      creatingTag = false;
    }
  }

  async function deleteTag(tagId) {
    if (!confirm(t("tags.confirmDelete") || "Tag wirklich löschen?")) return;

    try {
      await tagsApi.delete(tagId);
      await loadTags();
      success(t("tags.deleteSuccess") || "Tag erfolgreich gelöscht");
    } catch (err) {
      console.error("Failed to delete tag:", err);
      errorToast(t("tags.deleteError") || "Fehler beim Löschen des Tags");
    }
  }

  async function viewTagFiles(tag) {
    selectedTag = tag;
    loadingTagFiles = true;
    showTagModal = true;

    try {
      const response = await tagsApi.getFiles(tag.id);
      tagFiles = response || [];
    } catch (err) {
      console.error("Failed to load tag files:", err);
      tagFiles = [];
    } finally {
      loadingTagFiles = false;
    }
  }

  function toggleTagSelection(tagId) {
    if (selectedTags.has(tagId)) {
      selectedTags.delete(tagId);
    } else {
      selectedTags.add(tagId);
    }
    selectedTags = selectedTags; // Trigger reactivity
  }

  function getTagFontSize(usage) {
    const ratio = usage / maxUsage;
    return minFontSize + (maxFontSize - minFontSize) * ratio;
  }

  function getTagColor(tag, index) {
    switch (colorMode) {
      case "category":
        return tag.color || colorCategories[index % colorCategories.length];
      case "usage":
        const intensity = Math.min(tag.usage_count / maxUsage, 1);
        return `hsl(${210 + intensity * 60}, 70%, ${60 - intensity * 20}%)`;
      case "random":
        return colorCategories[
          Math.floor(Math.random() * colorCategories.length)
        ];
      default:
        return tag.color || "#10B981";
    }
  }

  async function deleteBulkTags() {
    if (selectedTags.size === 0) return;

    const count = selectedTags.size;
    if (
      !confirm(
        t("tags.confirmBulkDelete", count) || `${count} Tags wirklich löschen?`
      )
    )
      return;

    try {
      await Promise.all(
        Array.from(selectedTags).map((id) => tagsApi.delete(id))
      );
      selectedTags.clear();
      await loadTags();
      success(t("tags.bulkDeleteSuccess") || "Tags erfolgreich gelöscht");
    } catch (err) {
      console.error("Failed to delete tags:", err);
      errorToast(t("tags.bulkDeleteError") || "Fehler beim Löschen der Tags");
    }
  }

  function handleTabChange(event) {
    activeTab = event.detail.tabId;
  }

  // Modal actions with reactive disabled state
  const createTagModalActions = $derived([
    {
      label: t("common.cancel") || "Abbrechen",
      variant: "default",
      onClick: () => (showCreateTagModal = false),
      disabled: creatingTag,
    },
    {
      label: t("tags.createTag") || "Tag erstellen",
      icon: "plus",
      variant: "primary",
      onClick: createTag,
      disabled: creatingTag || !newTagName.trim(),
    },
  ]);

  const tagModalActions = [
    {
      label: t("common.close") || "Schließen",
      variant: "default",
      onClick: () => (showTagModal = false),
    },
  ];
</script>

<!-- Page Container with Standard Background -->
<div
  class="min-h-screen bg-gradient-to-br from-blue-50/50 to-indigo-100/50 dark:from-slate-900 dark:to-slate-800 p-4 sm:p-6 lg:p-8"
>
  <div class="max-w-7xl mx-auto">
    <!-- Page Header -->
    <div class="mb-6 sm:mb-8">
      <div class="flex items-center justify-between">
        <div>
          <h1
            class="text-2xl sm:text-3xl font-bold text-gray-900 dark:text-white mb-2"
          >
            <i
              class="bi bi-tags text-green-600 dark:text-green-400 mr-3"
              aria-hidden="true"
            ></i>
            {t("tags.title") || "Tag Cloud"}
          </h1>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            {t("tags.subtitle") ||
              "Visualisierung und Verwaltung von Datei-Tags"}
          </p>
        </div>

        <!-- Action Buttons -->
        <div class="flex items-center space-x-2">
          <StandardButton
            variant="default"
            icon="arrow-clockwise"
            onclick={loadTags}
            disabled={loading}
          >
            {t("common.refresh") || "Aktualisieren"}
          </StandardButton>

          <StandardButton
            variant="primary"
            icon="plus"
            onclick={() => (showCreateTagModal = true)}
          >
            {t("tags.createTag") || "Tag erstellen"}
          </StandardButton>
        </div>
      </div>
    </div>

    <!-- Stats Cards -->
    <div class="grid-4 grid-gap mb-6">
      <StandardGlassCard
        title={t("tags.totalTags") || "Gesamte Tags"}
        icon="tags"
        padding="p-4"
      >
        <div class="text-2xl font-bold text-green-600 dark:text-green-400">
          {tags.length}
        </div>
        <p class="text-caption mt-1">
          {t("tags.tagsInSystem") || "Tags im System"}
        </p>
      </StandardGlassCard>

      <StandardGlassCard
        title={t("tags.mostUsed") || "Meistverwendet"}
        icon="trophy"
        padding="p-4"
      >
        <div class="text-2xl font-bold text-yellow-600 dark:text-yellow-400">
          {maxUsage}
        </div>
        <p class="text-caption mt-1">
          {t("tags.usageCount") || "Verwendungen"}
        </p>
      </StandardGlassCard>

      <StandardGlassCard
        title={t("tags.selectedCount") || "Ausgewählt"}
        icon="check-circle"
        padding="p-4"
      >
        <div class="text-2xl font-bold text-blue-600 dark:text-blue-400">
          {selectedTags.size}
        </div>
        <p class="text-caption mt-1">
          {t("tags.selectedTags") || "Ausgewählte Tags"}
        </p>
      </StandardGlassCard>

      <StandardGlassCard
        title={t("tags.averageUsage") || "Durchschnitt"}
        icon="bar-chart"
        padding="p-4"
      >
        <div class="text-2xl font-bold text-purple-600 dark:text-purple-400">
          {tags.length
            ? Math.round(
                tags.reduce((sum, tag) => sum + tag.usage_count, 0) /
                  tags.length
              )
            : 0}
        </div>
        <p class="text-caption mt-1">
          {t("tags.avgUsage") || "Ø Verwendungen"}
        </p>
      </StandardGlassCard>
    </div>

    <!-- Main Content -->
    <StandardGlassCard {loading} {error}>
      <!-- Tabs -->
      <StandardTabs
        {tabs}
        {activeTab}
        variant="default"
        on:change={handleTabChange}
      />

      <!-- Controls -->
      <div
        class="flex flex-col lg:flex-row gap-4 items-start lg:items-center justify-between mb-6"
      >
        <!-- Search -->
        <div class="flex-1 max-w-md">
          <div class="relative">
            <input
              type="text"
              placeholder={t("tags.searchPlaceholder") || "Tags durchsuchen..."}
              bind:value={searchQuery}
              class="form-input pl-10"
            />
            <i
              class="bi bi-search absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400"
              aria-hidden="true"
            ></i>
          </div>
        </div>

        <!-- Controls -->
        <div class="flex items-center space-x-2">
          {#if activeTab === "cloud"}
            <!-- Color Mode -->
            <select bind:value={colorMode} class="form-input">
              <option value="category"
                >{t("tags.colorByCategory") || "Nach Kategorie"}</option
              >
              <option value="usage"
                >{t("tags.colorByUsage") || "Nach Verwendung"}</option
              >
              <option value="random"
                >{t("tags.colorRandom") || "Zufällig"}</option
              >
            </select>
          {/if}

          <!-- Sort -->
          <select bind:value={sortBy} class="form-input">
            <option value="usage"
              >{t("tags.sortByUsage") || "Nach Verwendung"}</option
            >
            <option value="name">{t("tags.sortByName") || "Nach Name"}</option>
            <option value="recent"
              >{t("tags.sortByRecent") || "Nach Datum"}</option
            >
          </select>
        </div>
      </div>

      <!-- Bulk Actions -->
      {#if activeTab === "manage" && selectedTags.size > 0}
        <div
          class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4 mb-6"
        >
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <i
                class="bi bi-exclamation-triangle text-red-600 dark:text-red-400"
                aria-hidden="true"
              ></i>
              <span class="text-body">
                {selectedTags.size}
                {t("tags.tagsSelected") || "Tags ausgewählt"}
              </span>
            </div>

            <StandardButton
              variant="danger"
              icon="trash"
              onclick={deleteBulkTags}
            >
              {t("tags.deleteSelected") || "Ausgewählte löschen"}
            </StandardButton>
          </div>
        </div>
      {/if}

      <!-- Content based on active tab -->
      {#if activeTab === "cloud"}
        <!-- Tag Cloud View -->
        {#if filteredTags.length > 0}
          <div
            class="tag-cloud p-6 bg-white/30 dark:bg-slate-800/30 rounded-lg min-h-[400px] flex flex-wrap items-center justify-center gap-3"
          >
            {#each filteredTags as tag, index}
              <button
                class="tag-item inline-block px-3 py-1 rounded-full transition-all duration-200 hover:scale-110 hover:shadow-lg cursor-pointer border-2 border-transparent hover:border-white/50"
                style="font-size: {getTagFontSize(
                  tag.usage_count
                )}px; color: {getTagColor(
                  tag,
                  index
                )}; background-color: {getTagColor(tag, index)}20"
                onclick={() => viewTagFiles(tag)}
                title="{tag.name} ({tag.usage_count} {t('tags.files') ||
                  'Dateien'})"
              >
                {tag.name}
                <span class="text-xs opacity-70">({tag.usage_count})</span>
              </button>
            {/each}
          </div>
        {:else}
          <div class="text-center py-12">
            <i class="bi bi-tags text-6xl text-gray-400 mb-4" aria-hidden="true"
            ></i>
            <h3 class="text-subheading mb-2">
              {t("tags.noTags") || "Keine Tags gefunden"}
            </h3>
            <p class="text-caption">
              {t("tags.createFirstTag") || "Erstellen Sie Ihren ersten Tag"}
            </p>
          </div>
        {/if}
      {:else if activeTab === "list"}
        <!-- List View -->
        {#if filteredTags.length > 0}
          <div class="space-y-2">
            {#each filteredTags as tag}
              <div class="glass-card-hover p-4">
                <div class="flex items-center justify-between">
                  <div class="flex items-center space-x-3">
                    <div
                      class="w-4 h-4 rounded-full border-2 border-white"
                      style="background-color: {tag.color || '#10B981'}"
                    ></div>
                    <div>
                      <h4 class="text-subheading">{tag.name}</h4>
                      <p class="text-caption">
                        {tag.usage_count}
                        {t("tags.usages") || "Verwendungen"} •
                        {tag.created_at
                          ? new Date(tag.created_at).toLocaleDateString()
                          : t("common.unknown") || "Unbekannt"}
                      </p>
                    </div>
                  </div>

                  <div class="flex items-center space-x-2">
                    <StandardButton
                      variant="ghost"
                      icon="eye"
                      iconPosition="only"
                      size="xs"
                      onclick={() => viewTagFiles(tag)}
                      aria-label="Tag-Dateien anzeigen"
                    />

                    <StandardButton
                      variant="ghost"
                      icon="trash"
                      iconPosition="only"
                      size="xs"
                      onclick={() => deleteTag(tag.id)}
                      aria-label="Tag löschen"
                    />
                  </div>
                </div>
              </div>
            {/each}
          </div>
        {:else}
          <div class="text-center py-12">
            <i
              class="bi bi-list-ul text-6xl text-gray-400 mb-4"
              aria-hidden="true"
            ></i>
            <h3 class="text-subheading mb-2">
              {t("tags.noTagsInList") || "Keine Tags in der Liste"}
            </h3>
            <p class="text-caption">
              {t("tags.adjustFilters") || "Passen Sie die Filter an"}
            </p>
          </div>
        {/if}
      {:else if activeTab === "manage"}
        <!-- Management View -->
        {#if filteredTags.length > 0}
          <div class="space-y-2">
            {#each filteredTags as tag}
              <div class="glass-card-hover p-4">
                <div class="flex items-center justify-between">
                  <label
                    class="flex items-center space-x-3 flex-1 cursor-pointer"
                  >
                    <input
                      type="checkbox"
                      checked={selectedTags.has(tag.id)}
                      onchange={() => toggleTagSelection(tag.id)}
                      class="form-input w-4 h-4"
                    />
                    <div
                      class="w-4 h-4 rounded-full border-2 border-white"
                      style="background-color: {tag.color || '#10B981'}"
                    ></div>
                    <div>
                      <h4 class="text-subheading">{tag.name}</h4>
                      <p class="text-caption">
                        {tag.usage_count}
                        {t("tags.files") || "Dateien"} • ID: {tag.id}
                      </p>
                    </div>
                  </label>

                  <div class="flex items-center space-x-2">
                    <StandardButton
                      variant="ghost"
                      icon="pencil"
                      iconPosition="only"
                      size="xs"
                      onclick={() => {
                        /* Edit functionality */
                      }}
                      aria-label="Tag bearbeiten"
                    />

                    <StandardButton
                      variant="ghost"
                      icon="trash"
                      iconPosition="only"
                      size="xs"
                      onclick={() => deleteTag(tag.id)}
                      aria-label="Tag löschen"
                    />
                  </div>
                </div>
              </div>
            {/each}
          </div>
        {:else}
          <div class="text-center py-12">
            <i class="bi bi-gear text-6xl text-gray-400 mb-4" aria-hidden="true"
            ></i>
            <h3 class="text-subheading mb-2">
              {t("tags.noTagsToManage") || "Keine Tags zu verwalten"}
            </h3>
            <p class="text-caption">
              {t("tags.createTagsFirst") || "Erstellen Sie zuerst einige Tags"}
            </p>
          </div>
        {/if}
      {/if}
    </StandardGlassCard>
  </div>
</div>

<!-- Create Tag Modal -->
<StandardModal
  bind:show={showCreateTagModal}
  title={t("tags.createNewTag") || "Neuen Tag erstellen"}
  size="md"
  actions={createTagModalActions}
  loading={creatingTag}
>
  <div class="space-y-4">
    <div>
      <label class="form-label">
        {t("tags.tagName") || "Tag-Name"}
        <input
          type="text"
          bind:value={newTagName}
          placeholder={t("tags.tagNamePlaceholder") || "Tag-Name eingeben..."}
          class="form-input mt-1"
          maxlength="50"
        />
      </label>
    </div>

    <div>
      <label class="form-label">
        {t("tags.tagColor") || "Tag-Farbe"}
        <div class="flex items-center space-x-3 mt-1">
          <input
            type="color"
            bind:value={newTagColor}
            class="form-input w-16 h-10"
          />
          <div class="flex space-x-2">
            {#each colorCategories.slice(0, 6) as color}
              <button
                class="w-6 h-6 rounded-full border-2 {newTagColor === color
                  ? 'border-gray-600'
                  : 'border-gray-300'}"
                style="background-color: {color}"
                onclick={() => (newTagColor = color)}
                aria-label="Farbe auswählen"
              ></button>
            {/each}
          </div>
        </div>
      </label>
    </div>
  </div>
</StandardModal>

<!-- Tag Files Modal -->
<StandardModal
  bind:show={showTagModal}
  title={selectedTag
    ? `${t("tags.filesWithTag") || "Dateien mit Tag"}: ${selectedTag.name}`
    : ""}
  size="lg"
  actions={tagModalActions}
  loading={loadingTagFiles}
>
  {#if tagFiles.length > 0}
    <div class="space-y-2 max-h-96 overflow-y-auto">
      {#each tagFiles as file}
        <div
          class="flex items-center justify-between p-3 bg-gray-50 dark:bg-slate-700/50 rounded-lg"
        >
          <div class="flex items-center space-x-3">
            <i class="bi bi-file-earmark text-gray-400" aria-hidden="true"></i>
            <div>
              <p class="text-body font-medium">{file.filename || file.name}</p>
              <p class="text-caption">{file.path || file.filePath}</p>
            </div>
          </div>
          <div class="text-right">
            <p class="text-caption">
              {file.size
                ? new Intl.NumberFormat().format(file.size) + " B"
                : ""}
            </p>
          </div>
        </div>
      {/each}
    </div>
  {:else if !loadingTagFiles}
    <div class="text-center py-8">
      <i class="bi bi-file-x text-4xl text-gray-400 mb-2" aria-hidden="true"
      ></i>
      <p class="text-caption">
        {t("tags.noFilesWithTag") || "Keine Dateien mit diesem Tag"}
      </p>
    </div>
  {/if}
</StandardModal>

<style>
  .tag-cloud {
    line-height: 1.8;
  }

  .tag-item {
    font-weight: 600;
    text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
  }
</style>
