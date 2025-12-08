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
  <div class="smart-folders-view">
    <!-- Header -->
    <div class="view-header">
      <div class="header-left">
        <h1 class="view-title">
          <i class="bi bi-lightning-fill text-amber-500"></i>
          {tr("smartFolders.title") || "Smart Folders"}
        </h1>
        <p class="view-subtitle">
          {tr("smartFolders.subtitle") ||
            "Create dynamic folders based on rules"}
        </p>
      </div>
      <button class="btn-primary" onclick={openCreateModal}>
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
      <div class="empty-state">
        <div class="empty-icon">
          <i class="bi bi-lightning"></i>
        </div>
        <h3>{tr("smartFolders.empty") || "No Smart Folders Yet"}</h3>
        <p>
          {tr("smartFolders.emptyHint") ||
            "Create your first smart folder to automatically organize files based on rules."}
        </p>
        <button class="btn-primary mt-4" onclick={openCreateModal}>
          <i class="bi bi-plus-lg"></i>
          {tr("smartFolders.create") || "Create Smart Folder"}
        </button>
      </div>
    {:else}
      <!-- Smart Folders Grid -->
      <div class="folders-grid">
        {#each smartFolders as folder (folder.id)}
          <div
            class="folder-card"
            style="--folder-color: {folder.color || '#22c55e'}"
          >
            <div class="folder-header">
              <div
                class="folder-icon"
                style="background-color: {folder.color ||
                  '#22c55e'}20; color: {folder.color || '#22c55e'}"
              >
                <i class="bi {folder.icon || 'bi-folder-fill'}"></i>
              </div>
              <div class="folder-info">
                <h3>{folder.name}</h3>
                {#if folder.description}
                  <p class="folder-desc">{folder.description}</p>
                {/if}
              </div>
            </div>

            <div class="folder-meta">
              <span class="meta-item">
                <i class="bi bi-funnel"></i>
                {folder.conditions_count}
                {folder.conditions_count === 1 ? "rule" : "rules"}
              </span>
              <span class="meta-item">
                <i
                  class="bi bi-{folder.logic === 'AND' ? 'intersect' : 'union'}"
                ></i>
                {folder.logic}
              </span>
              <span class="meta-item" class:inactive={!folder.is_active}>
                <i
                  class="bi bi-{folder.is_active ? 'check-circle' : 'x-circle'}"
                ></i>
                {folder.is_active ? "Active" : "Inactive"}
              </span>
            </div>

            <div class="folder-actions">
              <button
                class="action-btn"
                onclick={() => previewSmartFolder(folder)}
                title="Preview"
              >
                <i class="bi bi-eye"></i>
              </button>
              <button
                class="action-btn"
                onclick={() => openEditModal(folder)}
                title="Edit"
              >
                <i class="bi bi-pencil"></i>
              </button>
              <button
                class="action-btn danger"
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
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div class="modal-backdrop" role="dialog" aria-modal="true" aria-labelledby="modal-title" tabindex="-1" onclick={closeModal} onkeydown={(e) => e.key === 'Escape' && closeModal()}>
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div class="modal-content" onclick={(e) => e.stopPropagation()}>
        <div class="modal-header">
          <h2 id="modal-title">
            <i class="bi bi-lightning-fill text-amber-500"></i>
            {editingFolder
              ? tr("smartFolders.edit") || "Edit Smart Folder"
              : tr("smartFolders.create") || "Create Smart Folder"}
          </h2>
          <button class="close-btn" onclick={closeModal} aria-label="Close modal">
            <i class="bi bi-x-lg" aria-hidden="true"></i>
          </button>
        </div>

        <div class="modal-body">
          <!-- Basic Info -->
          <div class="form-section">
            <h3>{tr("smartFolders.basicInfo") || "Basic Information"}</h3>

            <div class="form-group">
              <label for="name">{tr("smartFolders.name") || "Name"} *</label>
              <input
                type="text"
                id="name"
                bind:value={formName}
                placeholder={tr("smartFolders.namePlaceholder") ||
                  "e.g., Large Images"}
              />
            </div>

            <div class="form-group">
              <label for="description"
                >{tr("smartFolders.description") || "Description"}</label
              >
              <input
                type="text"
                id="description"
                bind:value={formDescription}
                placeholder={tr("smartFolders.descriptionPlaceholder") ||
                  "Optional description"}
              />
            </div>

            <div class="form-row">
              <div class="form-group">
                <span class="label-text">{tr("smartFolders.icon") || "Icon"}</span>
                <div class="icon-picker" role="radiogroup" aria-label="Select icon">
                  {#each iconOptions as icon}
                    <button
                      type="button"
                      class="icon-option"
                      class:selected={formIcon === icon}
                      onclick={() => (formIcon = icon)}
                      aria-label="Select {icon} icon"
                      aria-pressed={formIcon === icon}
                    >
                      <i class="bi {icon}" aria-hidden="true"></i>
                    </button>
                  {/each}
                </div>
              </div>

              <div class="form-group">
                <span class="label-text">{tr("smartFolders.color") || "Color"}</span>
                <div class="color-picker" role="radiogroup" aria-label="Select color">
                  {#each colorOptions as color}
                    <button
                      type="button"
                      class="color-option"
                      class:selected={formColor === color}
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
          <div class="form-section">
            <div class="section-header">
              <h3>{tr("smartFolders.conditions") || "Conditions"}</h3>
              <div class="logic-toggle">
                <button
                  type="button"
                  class:active={formLogic === "AND"}
                  onclick={() => (formLogic = "AND")}
                >
                  AND
                </button>
                <button
                  type="button"
                  class:active={formLogic === "OR"}
                  onclick={() => (formLogic = "OR")}
                >
                  OR
                </button>
              </div>
            </div>

            <div class="conditions-list">
              {#each formConditions as condition, index}
                <div class="condition-row">
                  <select
                    value={condition.field}
                    onchange={(e) =>
                      updateConditionField(index, e.target.value)}
                  >
                    {#each conditionFields as field}
                      <option value={field.value}>{field.label}</option>
                    {/each}
                  </select>

                  <select bind:value={condition.operator}>
                    {#each operatorsByField[condition.field] || [] as op}
                      <option value={op.value}>{op.label}</option>
                    {/each}
                  </select>

                  {#if condition.field === "file_type"}
                    <select bind:value={condition.value}>
                      <option value="">Select type...</option>
                      {#each fileTypePresets as preset}
                        <option value={preset.value}>{preset.label}</option>
                      {/each}
                    </select>
                  {:else if condition.field === "size"}
                    <select bind:value={condition.value}>
                      <option value="">Select size...</option>
                      {#each sizePresets as preset}
                        <option value={preset.value}>{preset.label}</option>
                      {/each}
                    </select>
                  {:else if condition.field === "date" && condition.operator === "last_n_days"}
                    <input
                      type="number"
                      bind:value={condition.value}
                      placeholder="Number of days"
                      min="1"
                    />
                  {:else if condition.field === "date"}
                    <input type="date" bind:value={condition.value} />
                  {:else}
                    <input
                      type="text"
                      bind:value={condition.value}
                      placeholder="Enter value..."
                    />
                  {/if}

                  <button
                    type="button"
                    class="remove-btn"
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
                class="add-condition-btn"
                onclick={addCondition}
              >
                <i class="bi bi-plus"></i>
                {tr("smartFolders.addCondition") || "Add Condition"}
              </button>
            </div>
          </div>

          <!-- Sorting -->
          <div class="form-section">
            <h3>{tr("smartFolders.sorting") || "Sorting"}</h3>
            <div class="form-row">
              <div class="form-group">
                <label for="sortBy"
                  >{tr("smartFolders.sortBy") || "Sort by"}</label
                >
                <select id="sortBy" bind:value={formSortBy}>
                  <option value="name">Name</option>
                  <option value="modified">Modified Date</option>
                  <option value="size">Size</option>
                  <option value="type">Type</option>
                </select>
              </div>
              <div class="form-group">
                <label for="sortOrder"
                  >{tr("smartFolders.sortOrder") || "Order"}</label
                >
                <select id="sortOrder" bind:value={formSortOrder}>
                  <option value="asc">Ascending</option>
                  <option value="desc">Descending</option>
                </select>
              </div>
            </div>
          </div>
        </div>

        <div class="modal-footer">
          <button class="btn-secondary" onclick={closeModal}>
            {tr("cancel") || "Cancel"}
          </button>
          <button class="btn-primary" onclick={saveSmartFolder}>
            <i class="bi bi-check-lg"></i>
            {editingFolder ? tr("save") || "Save" : tr("create") || "Create"}
          </button>
        </div>
      </div>
    </div>
  {/if}
</PageWrapper>

<style>
  .smart-folders-view {
    max-width: 1200px;
    margin: 0 auto;
    padding: 1.5rem;
  }

  .view-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 2rem;
  }

  .header-left {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .view-title {
    font-size: 1.5rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #111827;
  }

  :global(.dark) .view-title {
    color: #f9fafb;
  }

  .view-subtitle {
    color: #6b7280;
    font-size: 0.875rem;
  }

  :global(.dark) .view-subtitle {
    color: #9ca3af;
  }

  .btn-primary {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.25rem;
    background: #22c55e;
    color: white;
    border: none;
    border-radius: 0.5rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-primary:hover {
    background: #16a34a;
  }

  .btn-secondary {
    padding: 0.75rem 1.25rem;
    background: #f3f4f6;
    color: #374151;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  :global(.dark) .btn-secondary {
    background: #374151;
    color: #f9fafb;
    border-color: #4b5563;
  }

  .btn-secondary:hover {
    background: #e5e7eb;
  }

  :global(.dark) .btn-secondary:hover {
    background: #4b5563;
  }

  /* Empty State */
  .empty-state {
    text-align: center;
    padding: 4rem 2rem;
    background: white;
    border-radius: 1rem;
    border: 2px dashed #e5e7eb;
  }

  :global(.dark) .empty-state {
    background: #1f2937;
    border-color: #374151;
  }

  .empty-icon {
    width: 80px;
    height: 80px;
    margin: 0 auto 1.5rem;
    background: #fef3c7;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 2.5rem;
    color: #f59e0b;
  }

  .empty-state h3 {
    font-size: 1.25rem;
    font-weight: 600;
    color: #111827;
    margin-bottom: 0.5rem;
  }

  :global(.dark) .empty-state h3 {
    color: #f9fafb;
  }

  .empty-state p {
    color: #6b7280;
    max-width: 400px;
    margin: 0 auto;
  }

  :global(.dark) .empty-state p {
    color: #9ca3af;
  }

  /* Folders Grid */
  .folders-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 1.5rem;
  }

  .folder-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 1rem;
    padding: 1.5rem;
    transition: all 0.2s;
    border-left: 4px solid var(--folder-color);
  }

  :global(.dark) .folder-card {
    background: #1f2937;
    border-color: #374151;
  }

  .folder-card:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .folder-header {
    display: flex;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .folder-icon {
    width: 48px;
    height: 48px;
    border-radius: 0.75rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    flex-shrink: 0;
  }

  .folder-info h3 {
    font-weight: 600;
    color: #111827;
    margin-bottom: 0.25rem;
  }

  :global(.dark) .folder-info h3 {
    color: #f9fafb;
  }

  .folder-desc {
    font-size: 0.875rem;
    color: #6b7280;
  }

  :global(.dark) .folder-desc {
    color: #9ca3af;
  }

  .folder-meta {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    margin-bottom: 1rem;
    padding-top: 1rem;
    border-top: 1px solid #e5e7eb;
  }

  :global(.dark) .folder-meta {
    border-color: #374151;
  }

  .meta-item {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    font-size: 0.75rem;
    color: #6b7280;
    background: #f3f4f6;
    padding: 0.25rem 0.625rem;
    border-radius: 9999px;
  }

  :global(.dark) .meta-item {
    background: #374151;
    color: #9ca3af;
  }

  .meta-item.inactive {
    color: #ef4444;
  }

  .folder-actions {
    display: flex;
    gap: 0.5rem;
    justify-content: flex-end;
  }

  .action-btn {
    width: 36px;
    height: 36px;
    border-radius: 0.5rem;
    border: 1px solid #e5e7eb;
    background: white;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  :global(.dark) .action-btn {
    background: #374151;
    border-color: #4b5563;
    color: #9ca3af;
  }

  .action-btn:hover {
    background: #f3f4f6;
    color: #111827;
  }

  :global(.dark) .action-btn:hover {
    background: #4b5563;
    color: #f9fafb;
  }

  .action-btn.danger:hover {
    background: #fef2f2;
    color: #ef4444;
    border-color: #fecaca;
  }

  :global(.dark) .action-btn.danger:hover {
    background: rgba(239, 68, 68, 0.2);
    border-color: #ef4444;
  }

  /* Modal */
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 50;
    padding: 1rem;
  }

  .modal-content {
    background: white;
    border-radius: 1rem;
    width: 100%;
    max-width: 640px;
    max-height: 90vh;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  :global(.dark) .modal-content {
    background: #1f2937;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid #e5e7eb;
  }

  :global(.dark) .modal-header {
    border-color: #374151;
  }

  .modal-header h2 {
    font-size: 1.25rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #111827;
  }

  :global(.dark) .modal-header h2 {
    color: #f9fafb;
  }

  .close-btn {
    background: none;
    border: none;
    color: #6b7280;
    cursor: pointer;
    padding: 0.5rem;
    border-radius: 0.375rem;
    transition: all 0.2s;
  }

  .close-btn:hover {
    background: #f3f4f6;
    color: #111827;
  }

  :global(.dark) .close-btn:hover {
    background: #374151;
    color: #f9fafb;
  }

  .modal-body {
    padding: 1.5rem;
    overflow-y: auto;
    flex: 1;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding: 1.5rem;
    border-top: 1px solid #e5e7eb;
  }

  :global(.dark) .modal-footer {
    border-color: #374151;
  }

  /* Form */
  .form-section {
    margin-bottom: 1.5rem;
  }

  .form-section h3 {
    font-size: 0.875rem;
    font-weight: 600;
    color: #374151;
    margin-bottom: 1rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  :global(.dark) .form-section h3 {
    color: #9ca3af;
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .section-header h3 {
    margin-bottom: 0;
  }

  .form-group {
    margin-bottom: 1rem;
  }

  .form-group label {
    display: block;
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
    margin-bottom: 0.5rem;
  }

  :global(.dark) .form-group label {
    color: #d1d5db;
  }

  .form-group input,
  .form-group select {
    width: 100%;
    padding: 0.625rem 0.875rem;
    border: 1px solid #d1d5db;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    background: white;
    color: #111827;
  }

  :global(.dark) .form-group input,
  :global(.dark) .form-group select {
    background: #374151;
    border-color: #4b5563;
    color: #f9fafb;
  }

  .form-group input:focus,
  .form-group select:focus {
    outline: none;
    border-color: #22c55e;
    box-shadow: 0 0 0 3px rgba(34, 197, 94, 0.1);
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  /* Icon & Color Picker */
  .icon-picker,
  .color-picker {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .icon-option {
    width: 40px;
    height: 40px;
    border: 2px solid #e5e7eb;
    border-radius: 0.5rem;
    background: white;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
    color: #6b7280;
  }

  :global(.dark) .icon-option {
    background: #374151;
    border-color: #4b5563;
  }

  .icon-option:hover {
    border-color: #22c55e;
    color: #22c55e;
  }

  .icon-option.selected {
    border-color: #22c55e;
    background: #dcfce7;
    color: #22c55e;
  }

  :global(.dark) .icon-option.selected {
    background: rgba(34, 197, 94, 0.2);
  }

  .color-option {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    border: 3px solid transparent;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .color-option:hover {
    transform: scale(1.1);
  }

  .color-option.selected {
    border-color: #111827;
  }

  :global(.dark) .color-option.selected {
    border-color: #f9fafb;
  }

  /* Logic Toggle */
  .logic-toggle {
    display: flex;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    overflow: hidden;
  }

  :global(.dark) .logic-toggle {
    border-color: #4b5563;
  }

  .logic-toggle button {
    padding: 0.375rem 0.75rem;
    font-size: 0.75rem;
    font-weight: 600;
    border: none;
    background: white;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.2s;
  }

  :global(.dark) .logic-toggle button {
    background: #374151;
    color: #9ca3af;
  }

  .logic-toggle button.active {
    background: #22c55e;
    color: white;
  }

  /* Conditions */
  .conditions-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .condition-row {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr auto;
    gap: 0.5rem;
    align-items: center;
  }

  .condition-row select,
  .condition-row input {
    padding: 0.5rem 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    background: white;
    color: #111827;
  }

  :global(.dark) .condition-row select,
  :global(.dark) .condition-row input {
    background: #374151;
    border-color: #4b5563;
    color: #f9fafb;
  }

  .remove-btn {
    width: 36px;
    height: 36px;
    border-radius: 0.375rem;
    border: 1px solid #e5e7eb;
    background: white;
    color: #ef4444;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  :global(.dark) .remove-btn {
    background: #374151;
    border-color: #4b5563;
  }

  .remove-btn:hover:not(:disabled) {
    background: #fef2f2;
  }

  .remove-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .add-condition-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.75rem;
    border: 2px dashed #d1d5db;
    border-radius: 0.5rem;
    background: transparent;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 0.875rem;
  }

  :global(.dark) .add-condition-btn {
    border-color: #4b5563;
    color: #9ca3af;
  }

  .add-condition-btn:hover {
    border-color: #22c55e;
    color: #22c55e;
    background: #f0fdf4;
  }

  :global(.dark) .add-condition-btn:hover {
    background: rgba(34, 197, 94, 0.1);
  }
</style>
