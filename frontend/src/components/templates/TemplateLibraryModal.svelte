<script>
  import { onMount } from "svelte";
  import api from "../../lib/api.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  let { show = $bindable(false), onTemplateUsed = () => {} } = $props();

  let categories = $state([]);
  let templates = $state([]);
  let filteredTemplates = $state([]);
  let selectedCategory = $state("all");
  let searchQuery = $state("");
  let loading = $state(false);
  let error = $state("");
  let selectedTemplate = $state(null);
  let showVariableForm = $state(false);
  let variableValues = $state({});
  let filename = $state("");
  let destinationPath = $state("/");

  onMount(async () => {
    await loadCategories();
    await loadTemplates();
  });

  async function loadCategories() {
    try {
      const response = await api.templates.listCategories();
      categories = response;
    } catch (err) {
      console.error("Error loading categories:", err);
    }
  }

  async function loadTemplates() {
    loading = true;
    error = "";
    try {
      const response = await api.templates.listTemplates();
      templates = response;
      filterTemplates();
    } catch (err) {
      error = t($currentLang, "templates.loadError");
      console.error("Error loading templates:", err);
    } finally {
      loading = false;
    }
  }

  function filterTemplates() {
    let result = templates;

    // Filter by category
    if (selectedCategory !== "all") {
      result = result.filter((t) => t.category === selectedCategory);
    }

    // Filter by search query
    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase();
      result = result.filter(
        (t) =>
          t.name.toLowerCase().includes(query) ||
          (t.description && t.description.toLowerCase().includes(query))
      );
    }

    filteredTemplates = result;
  }

  $effect(() => {
    selectedCategory;
    searchQuery;
    filterTemplates();
  });

  function selectTemplate(template) {
    selectedTemplate = template;
    showVariableForm = true;

    // Parse variables and initialize values
    if (template.variables) {
      try {
        const vars = JSON.parse(template.variables);
        variableValues = vars.reduce((acc, varName) => {
          acc[varName] = "";
          return acc;
        }, {});
      } catch (e) {
        variableValues = {};
      }
    } else {
      variableValues = {};
    }

    // Set default filename
    filename = template.file_extension
      ? `new_file.${template.file_extension}`
      : "new_file.txt";
  }

  async function createFromTemplate() {
    if (!filename.trim()) {
      error = t($currentLang, "templates.filenameRequired");
      return;
    }

    loading = true;
    error = "";
    try {
      const result = await api.templates.useTemplate(selectedTemplate.id, {
        filename: filename.trim(),
        destination_path: destinationPath,
        variables: variableValues,
      });

      onTemplateUsed(result.path);
      closeModal();
    } catch (err) {
      error = t($currentLang, "templates.createError");
      console.error("Error creating from template:", err);
    } finally {
      loading = false;
    }
  }

  async function toggleFavorite(template) {
    try {
      const result = await api.templates.toggleFavorite(template.id);
      template.is_favorite = result.is_favorite;
      templates = templates; // Trigger reactivity
    } catch (err) {
      console.error("Error toggling favorite:", err);
    }
  }

  function closeModal() {
    show = false;
    showVariableForm = false;
    selectedTemplate = null;
    variableValues = {};
    filename = "";
    error = "";
  }

  function getCategoryName(categoryId) {
    const category = categories.find((c) => c.id === categoryId);
    return category ? category.name : categoryId;
  }

  function getCategoryIcon(categoryId) {
    const category = categories.find((c) => c.id === categoryId);
    return category?.icon || "file-earmark";
  }
</script>

{#if show}
  <div
    class="modal modal-open"
    role="dialog"
    aria-labelledby="template-modal-title"
  >
    <div class="modal-box max-w-5xl">
      <h2 id="template-modal-title" class="text-2xl font-bold mb-4">
        {t($currentLang, "templates.title")}
      </h2>

      {#if !showVariableForm}
        <!-- Template Browser -->
        <div class="mb-4 flex gap-4">
          <!-- Category Filter -->
          <select
            bind:value={selectedCategory}
            class="select select-bordered flex-1"
          >
            <option value="all"
              >{t($currentLang, "templates.allCategories")}</option
            >
            {#each categories as category}
              <option value={category.id}>{category.name}</option>
            {/each}
          </select>

          <!-- Search -->
          <input
            type="text"
            bind:value={searchQuery}
            placeholder={t($currentLang, "templates.searchPlaceholder")}
            class="input input-bordered flex-1"
          />
        </div>

        {#if error}
          <div class="alert alert-error mb-4">
            <i class="bi bi-exclamation-triangle"></i>
            <span>{error}</span>
          </div>
        {/if}

        {#if loading}
          <div class="flex justify-center py-8">
            <span class="loading loading-spinner loading-lg"></span>
          </div>
        {:else if filteredTemplates.length === 0}
          <div class="text-center py-8 text-gray-500">
            <i class="bi bi-inbox text-4xl mb-2"></i>
            <p>{t($currentLang, "templates.noTemplates")}</p>
          </div>
        {:else}
          <div
            class="grid grid-cols-1 md:grid-cols-2 gap-4 max-h-[60vh] overflow-y-auto"
          >
            {#each filteredTemplates as template}
              <div
                class="card bg-base-200 cursor-pointer hover:bg-base-300 transition"
              >
                <div class="card-body p-4">
                  <div class="flex justify-between items-start">
                    <div class="flex items-start gap-3 flex-1">
                      <i
                        class="bi bi-{getCategoryIcon(
                          template.category
                        )} text-2xl text-primary"
                      ></i>
                      <div class="flex-1">
                        <h3 class="font-semibold text-lg">{template.name}</h3>
                        {#if template.description}
                          <p class="text-sm text-gray-600 mt-1">
                            {template.description}
                          </p>
                        {/if}
                        <div class="flex gap-2 mt-2">
                          <span class="badge badge-sm"
                            >{getCategoryName(template.category)}</span
                          >
                          {#if template.file_extension}
                            <span class="badge badge-sm badge-outline"
                              >.{template.file_extension}</span
                            >
                          {/if}
                          {#if template.usage_count > 0}
                            <span class="badge badge-sm badge-ghost">
                              <i class="bi bi-download mr-1"></i>
                              {template.usage_count}
                            </span>
                          {/if}
                        </div>
                      </div>
                    </div>
                    <button
                      onclick={() => toggleFavorite(template)}
                      class="btn btn-ghost btn-sm btn-circle"
                      aria-label={template.is_favorite
                        ? "Remove from favorites"
                        : "Add to favorites"}
                    >
                      <i
                        class="bi bi-star{template.is_favorite
                          ? '-fill text-yellow-500'
                          : ''}"
                      ></i>
                    </button>
                  </div>
                  <div class="card-actions justify-end mt-2">
                    <button
                      onclick={() => selectTemplate(template)}
                      class="btn btn-primary btn-sm"
                    >
                      {t($currentLang, "templates.useTemplate")}
                    </button>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      {:else}
        <!-- Variable Form -->
        <div class="mb-4">
          <button
            onclick={() => {
              showVariableForm = false;
              selectedTemplate = null;
            }}
            class="btn btn-ghost btn-sm"
          >
            <i class="bi bi-arrow-left"></i>
            {t($currentLang, "common.back")}
          </button>
        </div>

        <div class="bg-base-200 p-4 rounded-lg mb-4">
          <h3 class="font-semibold text-lg mb-2">{selectedTemplate.name}</h3>
          {#if selectedTemplate.description}
            <p class="text-sm text-gray-600">{selectedTemplate.description}</p>
          {/if}
        </div>

        {#if error}
          <div class="alert alert-error mb-4">
            <i class="bi bi-exclamation-triangle"></i>
            <span>{error}</span>
          </div>
        {/if}

        <div class="space-y-4">
          <!-- Filename -->
          <div class="form-control">
            <label class="label" for="template-filename">
              <span class="label-text"
                >{t($currentLang, "templates.filename")}</span
              >
            </label>
            <input
              id="template-filename"
              type="text"
              bind:value={filename}
              class="input input-bordered"
              required
            />
          </div>

          <!-- Destination Path -->
          <div class="form-control">
            <label class="label" for="template-destination">
              <span class="label-text"
                >{t($currentLang, "templates.destinationPath")}</span
              >
            </label>
            <input
              id="template-destination"
              type="text"
              bind:value={destinationPath}
              class="input input-bordered"
            />
          </div>

          <!-- Variables -->
          {#if Object.keys(variableValues).length > 0}
            <div class="divider">{t($currentLang, "templates.variables")}</div>
            {#each Object.keys(variableValues) as varName}
              <div class="form-control">
                <label class="label" for="var-{varName}">
                  <span class="label-text font-mono"
                    >{"{{" + varName + "}}"}</span
                  >
                </label>
                <input
                  id="var-{varName}"
                  type="text"
                  bind:value={variableValues[varName]}
                  class="input input-bordered"
                  placeholder={t($currentLang, "templates.enterValue")}
                />
              </div>
            {/each}
          {/if}

          <!-- Auto-filled Variables Info -->
          <div class="alert alert-info">
            <i class="bi bi-info-circle"></i>
            <div class="text-sm">
              <p>{t($currentLang, "templates.autoFilledInfo")}</p>
              <ul class="list-disc list-inside mt-1 font-mono text-xs">
                <li>
                  {"{{user}}"} - {t($currentLang, "templates.currentUser")}
                </li>
                <li>
                  {"{{date}}"} - {t($currentLang, "templates.currentDate")}
                </li>
                <li>
                  {"{{time}}"} - {t($currentLang, "templates.currentTime")}
                </li>
              </ul>
            </div>
          </div>
        </div>
      {/if}

      <!-- Actions -->
      <div class="modal-action">
        <button onclick={closeModal} class="btn" disabled={loading}>
          {t($currentLang, "common.cancel")}
        </button>
        {#if showVariableForm}
          <button
            onclick={createFromTemplate}
            class="btn btn-primary"
            disabled={loading || !filename.trim()}
          >
            {#if loading}
              <span class="loading loading-spinner"></span>
            {/if}
            {t($currentLang, "templates.create")}
          </button>
        {/if}
      </div>
    </div>
    <div class="modal-backdrop" onclick={closeModal}></div>
  </div>
{/if}
