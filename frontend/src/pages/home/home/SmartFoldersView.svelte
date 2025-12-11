<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../stores/ui";
  import { t } from "../../i18n.js";
  import PageWrapper from "../../components/PageWrapper.svelte";
  import api from "../../lib/api";
  import { success, error as errorToast } from "../../stores/toast";

  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // State
  let smartFolders = $state([]);
  let loading = $state(true);
  let showCreateModal = $state(false);
  let editingFolder = $state(null);

  // Form state
  let formName = $state("");
  let formDescription = $state("");
  let formIcon = $state("bi-folder-fill");
  let formColor = $state("#22c55e");
  let formLogic = $state("AND");
  let formConditions = $state([]);
  let formSortBy = $state("name");
  let formSortOrder = $state("asc");

  // Available condition fields
  const conditionFields = [
    { value: "file_type", label: "File Type" },
    { value: "size", label: "File Size" },
    { value: "date", label: "Modified Date" },
    { value: "name_pattern", label: "File Name" },
    { value: "tags", label: "Tags" },
  ];

  // Operators per field
  const operatorsByField = {
    file_type: [
      { value: "equals", label: "is" },
      { value: "contains", label: "contains" },
    ],
    size: [
      { value: "greater_than", label: "greater than" },
      { value: "less_than", label: "less than" },
      { value: "equals", label: "equals" },
    ],
    date: [
      { value: "after", label: "after" },
      { value: "before", label: "before" },
      { value: "last_n_days", label: "in last X days" },
    ],
    name_pattern: [
      { value: "contains", label: "contains" },
      { value: "starts_with", label: "starts with" },
      { value: "ends_with", label: "ends with" },
      { value: "matches_regex", label: "matches regex" },
    ],
    tags: [
      { value: "contains", label: "has tag" },
      { value: "not_contains", label: "doesn't have tag" },
    ],
  };

  // File type presets
  const fileTypePresets = [
    { value: "image", label: "Images (jpg, png, gif, etc.)" },
    { value: "video", label: "Videos (mp4, webm, etc.)" },
    { value: "audio", label: "Audio (mp3, wav, etc.)" },
    { value: "document", label: "Documents (pdf, doc, etc.)" },
    { value: "code", label: "Code files (js, py, rs, etc.)" },
    { value: "archive", label: "Archives (zip, rar, etc.)" },
  ];

  // Size presets (in bytes)
  const sizePresets = [
    { value: "1048576", label: "1 MB" },
    { value: "10485760", label: "10 MB" },
    { value: "104857600", label: "100 MB" },
    { value: "1073741824", label: "1 GB" },
  ];

  // Icons
  const iconOptions = [
    "bi-folder-fill",
    "bi-folder2-open",
    "bi-image",
    "bi-film",
    "bi-music-note-beamed",
    "bi-file-earmark-pdf",
    "bi-file-earmark-code",
    "bi-archive",
    "bi-star-fill",
    "bi-clock-history",
    "bi-cloud",
    "bi-people",
  ];

  // Colors
  const colorOptions = [
    "#22c55e", // green
    "#3b82f6", // blue
    "#f59e0b", // amber
    "#ef4444", // red
    "#8b5cf6", // purple
    "#ec4899", // pink
    "#06b6d4", // cyan
    "#84cc16", // lime
  ];

  onMount(async () => {
    await loadSmartFolders();
  });

  async function loadSmartFolders() {
    loading = true;
    try {
      smartFolders = await api.smartFolders.list();
    } catch (err) {
      console.error("[SmartFolders] Error loading:", err);
      errorToast(
        tr("smartFolders.loadError") || "Failed to load smart folders"
      );
    } finally {
      loading = false;
    }
  }

  function addCondition() {
    formConditions = [
      ...formConditions,
      { field: "file_type", operator: "equals", value: "" },
    ];
  }

  function removeCondition(index) {
    formConditions = formConditions.filter((_, i) => i !== index);
  }

  function updateConditionField(index, field) {
    const newConditions = [...formConditions];
    newConditions[index].field = field;
    // Reset operator to first available for new field
    newConditions[index].operator = operatorsByField[field][0].value;
    newConditions[index].value = "";
    formConditions = newConditions;
  }

  function resetForm() {
    formName = "";
    formDescription = "";
    formIcon = "bi-folder-fill";
    formColor = "#22c55e";
    formLogic = "AND";
    formConditions = [];
    formSortBy = "name";
    formSortOrder = "asc";
    editingFolder = null;
  }

  function openCreateModal() {
    resetForm();
    addCondition(); // Start with one condition
    showCreateModal = true;
  }

  function openEditModal(folder) {
    editingFolder = folder;
    formName = folder.name;
    formDescription = folder.description || "";
    formIcon = folder.icon || "bi-folder-fill";
    formColor = folder.color || "#22c55e";
    formLogic = folder.logic;
    formSortBy = folder.sort_by || "name";
    formSortOrder = folder.sort_order || "asc";
    // Load conditions from backend (would need to fetch full details)
    formConditions = [];
    addCondition();
    showCreateModal = true;
  }

  function closeModal() {
    showCreateModal = false;
    resetForm();
  }

  async function saveSmartFolder() {
    if (!formName.trim()) {
      errorToast(tr("smartFolders.nameRequired") || "Name is required");
      return;
    }

    if (formConditions.length === 0) {
      errorToast(
        tr("smartFolders.conditionsRequired") ||
          "At least one condition is required"
      );
      return;
    }

    // Validate conditions have values
    for (const cond of formConditions) {
      if (!cond.value.trim()) {
        errorToast(
          tr("smartFolders.conditionValueRequired") ||
            "All conditions must have values"
        );
        return;
      }
    }

    const payload = {
      name: formName,
      description: formDescription || null,
      icon: formIcon,
      color: formColor,
      logic: formLogic,
      conditions: formConditions,
      sort_by: formSortBy,
      sort_order: formSortOrder,
    };

    try {
      if (editingFolder) {
        await api.smartFolders.update(editingFolder.id, payload);
        success(tr("smartFolders.updated") || "Smart folder updated");
      } else {
        await api.smartFolders.create(payload);
        success(tr("smartFolders.created") || "Smart folder created");
      }
      closeModal();
      await loadSmartFolders();
    } catch (err) {
      console.error("[SmartFolders] Error saving:", err);
      errorToast(tr("smartFolders.saveError") || "Failed to save smart folder");
    }
  }

  async function deleteSmartFolder(folder) {
    if (
      !confirm(tr("smartFolders.deleteConfirm") || `Delete "${folder.name}"?`)
    ) {
      return;
    }

    try {
      await api.smartFolders.delete(folder.id);
      success(tr("smartFolders.deleted") || "Smart folder deleted");
      await loadSmartFolders();
    } catch (err) {
      console.error("[SmartFolders] Error deleting:", err);
      errorToast(
        tr("smartFolders.deleteError") || "Failed to delete smart folder"
      );
    }
  }

  async function previewSmartFolder(folder) {
    try {
      const result = await api.smartFolders.preview(folder.id);
      alert(`Found ${result.total_count} matching files`);
    } catch (err) {
      console.error("[SmartFolders] Error previewing:", err);
      errorToast(tr("smartFolders.previewError") || "Failed to preview");
    }
  }
</script>

<PageWrapper
  title={tr("smartFolders.title") || "Smart Folders"}
  showSidebar={true}
>
  <div class="max-w-6xl mx-auto p-6">
    <!-- Header -->
    <div class="flex justify-between items-start mb-8">
      <div class="flex flex-col gap-1">
        <h1 class="text-2xl font-semibold flex items-center gap-2 text-base-content">
          <i class="bi bi-lightning-fill text-amber-500"></i>
          {tr("smartFolders.title") || "Smart Folders"}
        </h1>
        <p class="text-base-content/60 text-sm">
          {tr("smartFolders.subtitle") || "Create dynamic folders based on rules"}
        </p>
      </div>
      <button class="btn btn-success gap-2" onclick={openCreateModal}>
        <i class="bi bi-plus-lg"></i>
        {tr("smartFolders.create") || "Create Smart Folder"}
      </button>
    </div>

    {#if loading}
      <!-- Skeleton Loading -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each Array(6) as _}
          <div class="skeleton h-40 w-full rounded-xl"></div>
        {/each}
      </div>
    {:else if smartFolders.length === 0}
      <!-- Empty State -->
      <div class="text-center py-16 px-8 bg-base-100 rounded-2xl border-2 border-dashed border-base-300">
        <div class="w-20 h-20 mx-auto mb-6 bg-amber-100 dark:bg-amber-500/20 rounded-full flex items-center justify-center text-4xl text-amber-500">
          <i class="bi bi-lightning"></i>
        </div>
        <h3 class="text-xl font-semibold text-base-content mb-2">{tr("smartFolders.empty") || "No Smart Folders Yet"}</h3>
        <p class="text-base-content/60 max-w-md mx-auto">
          {tr("smartFolders.emptyHint") || "Create your first smart folder to automatically organize files based on rules."}
        </p>
        <button class="btn btn-success gap-2 mt-4" onclick={openCreateModal}>
          <i class="bi bi-plus-lg"></i>
          {tr("smartFolders.create") || "Create Smart Folder"}
        </button>
      </div>
    {:else}
      <!-- Smart Folders Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
        {#each smartFolders as folder (folder.id)}
          <div
            class="bg-gradient-to-br from-base-100/80 to-base-100/40 backdrop-blur-xl shadow-[0_4px_12px_rgba(0,0,0,0.08)] hover:shadow-[0_8px_24px_rgba(34,197,94,0.15)] transition-all rounded-2xl p-6 transition-all hover:shadow-lg border-l-4"
            style="border-left-color: {folder.color || '#22c55e'}"
          >
            <div class="flex gap-4 mb-4">
              <div
                class="w-12 h-12 rounded-xl flex items-center justify-center text-2xl shrink-0"
                style="background-color: {folder.color || '#22c55e'}20; color: {folder.color || '#22c55e'}"
              >
                <i class="bi {folder.icon || 'bi-folder-fill'}"></i>
              </div>
              <div>
                <h3 class="font-semibold text-base-content">{folder.name}</h3>
                {#if folder.description}
                  <p class="text-sm text-base-content/60">{folder.description}</p>
                {/if}
              </div>
            </div>

            <div class="flex flex-wrap gap-3 mb-4 pt-4 border-t border-base-300">
              <span class="flex items-center gap-1.5 text-xs text-base-content/60 bg-base-200 px-2.5 py-1 rounded-full">
                <i class="bi bi-funnel"></i>
                {folder.conditions_count} {folder.conditions_count === 1 ? tr("rule") : tr("rules")}
              </span>
              <span class="flex items-center gap-1.5 text-xs text-base-content/60 bg-base-200 px-2.5 py-1 rounded-full">
                <i class="bi bi-{folder.logic === 'AND' ? 'intersect' : 'union'}"></i>
                {folder.logic}
              </span>
              <span class="flex items-center gap-1.5 text-xs bg-base-200 px-2.5 py-1 rounded-full {folder.is_active ? 'text-base-content/60' : 'text-error'}">
                <i class="bi bi-{folder.is_active ? 'check-circle' : 'x-circle'}"></i>
                {folder.is_active ? tr("active") : tr("inactive")}
              </span>
            </div>

            <div class="flex gap-2 justify-end">
              <button
                class="btn btn-ghost btn-sm btn-square"
                onclick={() => previewSmartFolder(folder)}
                title="Preview"
              >
                <i class="bi bi-eye"></i>
              </button>
              <button
                class="btn btn-ghost btn-sm btn-square"
                onclick={() => openEditModal(folder)}
                title="Edit"
              >
                <i class="bi bi-pencil"></i>
              </button>
              <button
                class="btn btn-ghost btn-sm btn-square hover:bg-error/10 hover:text-error"
                onclick={() => deleteSmartFolder(folder)}
                title="Delete"
              >
                <i class="bi bi-trash"></i>
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>

  <!-- Create/Edit Modal -->
  {#if showCreateModal}
    <div
      class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
      role="dialog"
      aria-modal="true"
      aria-labelledby="modal-title"
      tabindex="-1"
      onclick={closeModal}
      onkeydown={(e) => e.key === "Escape" && closeModal()}
    >
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div class="bg-base-100 rounded-2xl w-full max-w-2xl max-h-[90vh] overflow-hidden flex flex-col" onclick={(e) => e.stopPropagation()}>
        <div class="flex justify-between items-center p-6 border-b border-base-300">
          <h2 id="modal-title" class="text-xl font-semibold flex items-center gap-2 text-base-content">
            <i class="bi bi-lightning-fill text-amber-500"></i>
            {editingFolder
              ? tr("smartFolders.edit") || "Edit Smart Folder"
              : tr("smartFolders.create") || "Create Smart Folder"}
          </h2>
          <button
            class="btn btn-ghost btn-sm btn-square"
            onclick={closeModal}
            aria-label="Close modal"
          >
            <i class="bi bi-x-lg" aria-hidden="true"></i>
          </button>
        </div>

        <div class="p-6 overflow-y-auto flex-1">
          <!-- Basic Info -->
          <div class="mb-6">
            <h3 class="text-sm font-semibold text-base-content/60 mb-4 uppercase tracking-wide">{tr("smartFolders.basicInfo") || "Basic Information"}</h3>

            <div class="form-control mb-4">
              <label class="label" for="name">
                <span class="label-text">{tr("smartFolders.name") || "Name"} *</span>
              </label>
              <input
                type="text"
                id="name"
                class="input input-bordered w-full"
                bind:value={formName}
                placeholder={tr("smartFolders.namePlaceholder") || "e.g., Large Images"}
              />
            </div>

            <div class="form-control mb-4">
              <label class="label" for="description">
                <span class="label-text">{tr("smartFolders.description") || "Description"}</span>
              </label>
              <input
                type="text"
                id="description"
                class="input input-bordered w-full"
                bind:value={formDescription}
                placeholder={tr("smartFolders.descriptionPlaceholder") || "Optional description"}
              />
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div class="form-control">
                <span class="label-text mb-2 block">{tr("smartFolders.icon") || "Icon"}</span>
                <div class="flex flex-wrap gap-2" role="radiogroup" aria-label="Select icon">
                  {#each iconOptions as icon}
                    <button
                      type="button"
                      class="w-10 h-10 border-2 rounded-lg flex items-center justify-center text-xl transition-all {formIcon === icon ? 'border-success bg-success/10 text-success' : 'border-base-300 bg-base-100 text-base-content/60 hover:border-success hover:text-success'}"
                      onclick={() => (formIcon = icon)}
                      aria-label="Select {icon} icon"
                      aria-pressed={formIcon === icon}
                    >
                      <i class="bi {icon}" aria-hidden="true"></i>
                    </button>
                  {/each}
                </div>
              </div>

              <div class="form-control">
                <span class="label-text mb-2 block">{tr("smartFolders.color") || "Color"}</span>
                <div class="flex flex-wrap gap-2" role="radiogroup" aria-label="Select color">
                  {#each colorOptions as color}
                    <button
                      type="button"
                      class="w-8 h-8 rounded-full border-3 transition-all flex items-center justify-center hover:scale-110 {formColor === color ? 'border-base-content' : 'border-transparent'}"
                      style="background-color: {color}"
                      onclick={() => (formColor = color)}
                      aria-label="Select color {color}"
                      aria-pressed={formColor === color}
                    >
                      {#if formColor === color}
                        <i class="bi bi-check text-white" aria-hidden="true"></i>
                      {/if}
                    </button>
                  {/each}
                </div>
              </div>
            </div>
          </div>

          <!-- Conditions -->
          <div class="mb-6">
            <div class="flex justify-between items-center mb-4">
              <h3 class="text-sm font-semibold text-base-content/60 uppercase tracking-wide">{tr("smartFolders.conditions") || "Conditions"}</h3>
              <div class="flex border border-base-300 rounded-lg overflow-hidden">
                <button
                  type="button"
                  class="px-3 py-1.5 text-xs font-semibold transition-all {formLogic === 'AND' ? 'bg-success text-white' : 'bg-base-100 text-base-content/60'}"
                  onclick={() => (formLogic = "AND")}
                >
                  AND
                </button>
                <button
                  type="button"
                  class="px-3 py-1.5 text-xs font-semibold transition-all {formLogic === 'OR' ? 'bg-success text-white' : 'bg-base-100 text-base-content/60'}"
                  onclick={() => (formLogic = "OR")}
                >
                  OR
                </button>
              </div>
            </div>

            <div class="flex flex-col gap-3">
              {#each formConditions as condition, index}
                <div class="grid grid-cols-[1fr_1fr_1fr_auto] gap-2 items-center">
                  <select
                    class="select select-bordered select-sm"
                    value={condition.field}
                    onchange={(e) => updateConditionField(index, e.target.value)}
                  >
                    {#each conditionFields as field}
                      <option value={field.value}>{field.label}</option>
                    {/each}
                  </select>

                  <select class="select select-bordered select-sm" bind:value={condition.operator}>
                    {#each operatorsByField[condition.field] || [] as op}
                      <option value={op.value}>{op.label}</option>
                    {/each}
                  </select>

                  {#if condition.field === "file_type"}
                    <select class="select select-bordered select-sm" bind:value={condition.value}>
                      <option value="">{tr("smartFolders.selectType") || "Select type..."}</option>
                      {#each fileTypePresets as preset}
                        <option value={preset.value}>{preset.label}</option>
                      {/each}
                    </select>
                  {:else if condition.field === "size"}
                    <select class="select select-bordered select-sm" bind:value={condition.value}>
                      <option value="">{tr("smartFolders.selectSize") || "Select size..."}</option>
                      {#each sizePresets as preset}
                        <option value={preset.value}>{preset.label}</option>
                      {/each}
                    </select>
                  {:else if condition.field === "date" && condition.operator === "last_n_days"}
                    <input
                      type="number"
                      class="input input-bordered input-sm"
                      bind:value={condition.value}
                      placeholder={tr("smartFolders.numberOfDays") || "Number of days"}
                      min="1"
                    />
                  {:else if condition.field === "date"}
                    <input type="date" class="input input-bordered input-sm" bind:value={condition.value} />
                  {:else}
                    <input
                      type="text"
                      class="input input-bordered input-sm"
                      bind:value={condition.value}
                      placeholder={tr("smartFolders.enterValue") || "Enter value..."}
                    />
                  {/if}

                  <button
                    type="button"
                    class="btn btn-ghost btn-sm btn-square text-error"
                    onclick={() => removeCondition(index)}
                    disabled={formConditions.length <= 1}
                    aria-label="Remove condition"
                  >
                    <i class="bi bi-trash" aria-hidden="true"></i>
                  </button>
                </div>
              {/each}

              <button
                type="button"
                class="flex items-center justify-center gap-2 py-3 border-2 border-dashed border-base-300 rounded-lg text-base-content/60 text-sm hover:border-success hover:text-success hover:bg-success/5 transition-all"
                onclick={addCondition}
              >
                <i class="bi bi-plus"></i>
                {tr("smartFolders.addCondition") || "Add Condition"}
              </button>
            </div>
          </div>

          <!-- Sorting -->
          <div class="mb-6">
            <h3 class="text-sm font-semibold text-base-content/60 mb-4 uppercase tracking-wide">{tr("smartFolders.sorting") || "Sorting"}</h3>
            <div class="grid grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label" for="sortBy">
                  <span class="label-text">{tr("smartFolders.sortBy") || "Sort by"}</span>
                </label>
                <select id="sortBy" class="select select-bordered" bind:value={formSortBy}>
                  <option value="name">{tr("smartFolders.name") || "Name"}</option>
                  <option value="modified">{tr("smartFolders.modifiedDate") || "Modified Date"}</option>
                  <option value="size">{tr("smartFolders.fields.size") || "Size"}</option>
                  <option value="type">{tr("smartFolders.type") || "Type"}</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label" for="sortOrder">
                  <span class="label-text">{tr("smartFolders.sortOrder") || "Order"}</span>
                </label>
                <select id="sortOrder" class="select select-bordered" bind:value={formSortOrder}>
                  <option value="asc">{tr("smartFolders.ascending") || "Ascending"}</option>
                  <option value="desc">{tr("smartFolders.descending") || "Descending"}</option>
                </select>
              </div>
            </div>
          </div>
        </div>

        <div class="flex justify-end gap-3 p-6 border-t border-base-300">
          <button class="btn btn-ghost" onclick={closeModal}>
            {tr("cancel") || "Cancel"}
          </button>
          <button class="btn btn-success gap-2" onclick={saveSmartFolder}>
            <i class="bi bi-check-lg"></i>
            {editingFolder ? tr("save") || "Save" : tr("create") || "Create"}
          </button>
        </div>
      </div>
    </div>
  {/if}
</PageWrapper>

<style>
  /* Minimal styles for border-3 which isn't in Tailwind by default */
  .border-3 {
    border-width: 3px;
  }
</style>
