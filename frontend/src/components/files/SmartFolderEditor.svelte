<script>
  import { currentLang } from '../../stores/ui.js';
  import { t } from '../../i18n.js';
  import { success, error as errorToast } from '../../stores/toast.js';
  import api from '../../lib/api.js';
  import Modal from '../ui/Modal.svelte';
  import ColorPicker from '../ui/ColorPicker.svelte';

  const tr = (key, ...args) => t($currentLang, key, ...args);

  // Props
  let { isOpen = $bindable(false), folder = null, onClose = () => {}, onSave = () => {} } = $props();

  // State
  let name = $state(folder?.name || '');
  let description = $state(folder?.description || '');
  let icon = $state(folder?.icon || 'bi-folder-fill');
  let color = $state(folder?.color || '#3B82F6');
  let logic = $state(folder?.logic || 'AND');
  let sortBy = $state(folder?.sort_by || 'name');
  let sortOrder = $state(folder?.sort_order || 'asc');
  let conditions = $state(folder?.conditions || []);
  let isSaving = $state(false);

  // New condition being added
  let newCondition = $state({
    field: 'file_type',
    operator: 'equals',
    value: '',
  });

  const fieldOptions = [
    { value: 'file_type', label: tr('smartFolders.fields.fileType'), operators: ['equals', 'contains'] },
    { value: 'size', label: tr('smartFolders.fields.size'), operators: ['greater_than', 'less_than', 'equals'] },
    { value: 'date', label: tr('smartFolders.fields.date'), operators: ['after', 'before', 'today', 'this_week', 'this_month'] },
    { value: 'tags', label: tr('smartFolders.fields.tags'), operators: ['contains_any', 'contains_all'] },
    { value: 'name_pattern', label: tr('smartFolders.fields.namePattern'), operators: ['contains', 'starts_with', 'ends_with', 'matches_regex'] },
  ];

  const fileTypeOptions = [
    { value: 'document', label: 'Documents' },
    { value: 'image', label: 'Images' },
    { value: 'video', label: 'Videos' },
    { value: 'audio', label: 'Audio' },
    { value: 'archive', label: 'Archives' },
    { value: 'code', label: 'Code' },
    { value: 'data', label: 'Data' },
  ];

  const tagOptions = [
    { value: 'important', label: 'Important' },
    { value: 'review', label: 'Review' },
    { value: 'urgent', label: 'Urgent' },
    { value: 'archived', label: 'Archived' },
  ];

  const currentField = $derived(fieldOptions.find((f) => f.value === newCondition.field));
  const operatorOptions = $derived(
    currentField?.operators.map((op) => ({
      value: op,
      label: tr(`smartFolders.operators.${op}`),
    })) || []
  );

  function addCondition() {
    if (!newCondition.value) {
      errorToast(tr('smartFolders.valueRequired'));
      return;
    }

    conditions = [...conditions, { ...newCondition }];
    newCondition = {
      field: 'file_type',
      operator: 'equals',
      value: '',
    };
  }

  function removeCondition(index) {
    conditions = conditions.filter((_, i) => i !== index);
  }

  async function handleSave() {
    if (!name.trim()) {
      errorToast(tr('smartFolders.nameRequired'));
      return;
    }

    if (conditions.length === 0) {
      errorToast(tr('smartFolders.conditionsRequired'));
      return;
    }

    isSaving = true;
    try {
      const payload = {
        name,
        description,
        icon,
        color,
        logic,
        sort_by: sortBy,
        sort_order: sortOrder,
        conditions,
      };

      if (folder) {
        // Update existing
        const response = await api.smartFolders.update(folder.id, payload);
        if (response.ok) {
          success(tr('smartFolders.updated'));
        } else {
          errorToast(tr('smartFolders.updateError'));
        }
      } else {
        // Create new
        const response = await api.smartFolders.create(payload);
        if (response.ok) {
          success(tr('smartFolders.created'));
        } else {
          errorToast(tr('smartFolders.createError'));
        }
      }

      onSave?.();
      isOpen = false;
    } finally {
      isSaving = false;
    }
  }
</script>

<Modal {isOpen} onClose={() (isOpen = false)} size="lg">
  <div class="space-y-4">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <h2 class="text-xl font-bold text-gray-900 dark:text-white flex items-center gap-2">
        <i class="bi {icon} text-lg" style="color: {color}"></i>
        {folder ? tr('smartFolders.editFolder') : tr('smartFolders.newFolder')}
      </h2>
    </div>

    <!-- Basic Info -->
    <div class="space-y-3 p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
      <div>
        <div class="block text-sm font-medium">{tr('name')} *</div>
        <input
          bind:value={name}
          type="text"
          placeholder="My Smart Folder"
          class="w-full px-3 py-2 bg-white dark:bg-gray-600 border border-gray-200 dark:border-gray-500 rounded-lg text-gray-900 dark:text-white focus:outline-none focus:border-blue-500"
        />
      </div>

      <div>
        <div class="block text-sm font-medium">{tr('description')} *</div>
        <textarea
          bind:value={description}
          placeholder="Optional description..."
          rows="2"
          class="w-full px-3 py-2 bg-white dark:bg-gray-600 border border-gray-200 dark:border-gray-500 rounded-lg text-gray-900 dark:text-white focus:outline-none focus:border-blue-500"
        ></textarea>
      </div>

      <div class="grid grid-cols-3 gap-3">
        <div>
          <div class="block text-sm font-medium">{tr('icon')} *</div>
          <input
            bind:value={icon}
            type="text"
            placeholder="bi-folder-fill"
            class="w-full px-2 py-2 bg-white dark:bg-gray-600 border border-gray-200 dark:border-gray-500 rounded-lg text-gray-900 dark:text-white focus:outline-none focus:border-blue-500 text-sm"
          />
        </div>

        <div>
          <div class="block text-sm font-medium">{tr('color')} *</div>
          <div class="flex gap-2">
            <input
              bind:value={color}
              type="color"
              class="w-10 h-10 rounded cursor-pointer"
            />
            <input
              bind:value={color}
              type="text"
              class="flex-1 px-2 py-2 bg-white dark:bg-gray-600 border border-gray-200 dark:border-gray-500 rounded-lg text-gray-900 dark:text-white focus:outline-none focus:border-blue-500 text-sm"
            />
          </div>
        </div>

        <div>
          <div class="block text-sm font-medium">{tr('smartFolders.logic')} *</div>
          <select
            bind:value={logic}
            class="w-full px-3 py-2 bg-white dark:bg-gray-600 border border-gray-200 dark:border-gray-500 rounded-lg text-gray-900 dark:text-white focus:outline-none focus:border-blue-500"
          >
            <option value="AND">AND (all conditions)</option>
            <option value="OR">OR (any condition)</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Conditions Builder -->
    <div class="space-y-3">
      <h3 class="text-sm font-medium text-gray-900 dark:text-white">{tr('smartFolders.conditions')}</h3>

      <!-- Add Condition Form -->
      <div class="p-3 bg-blue-50 dark:bg-blue-900/30 rounded-lg border border-blue-200 dark:border-blue-800">
        <div class="grid grid-cols-3 gap-2 mb-2">
          <select
            bind:value={newCondition.field}
            class="px-2 py-1 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded text-sm text-gray-900 dark:text-white focus:outline-none"
          >
            {#each fieldOptions as option}
              <option value={option.value}>{option.label}</option>
            {/each}
          </select>

          <select
            bind:value={newCondition.operator}
            class="px-2 py-1 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded text-sm text-gray-900 dark:text-white focus:outline-none"
          >
            {#each operatorOptions as option}
              <option value={option.value}>{option.label}</option>
            {/each}
          </select>

          <input
            bind:value={newCondition.value}
            type="text"
            placeholder="Value..."
            class="px-2 py-1 bg-white dark:bg-gray-600 border border-gray-300 dark:border-gray-500 rounded text-sm text-gray-900 dark:text-white focus:outline-none"
          />
        </div>

        <button
          onclick={addCondition}
          class="w-full px-3 py-1 bg-blue-500 text-white text-sm rounded hover:bg-blue-600 transition flex items-center justify-center gap-1"
        >
          <i class="bi bi-plus" aria-hidden="true"></i>
          {tr('smartFolders.addCondition')}
        </button>
      </div>

      <!-- Existing Conditions -->
      {#if conditions.length > 0}
        <div class="space-y-2">
          {#each conditions as condition, idx}
            <div class="flex items-center justify-between p-2 bg-gray-100 dark:bg-gray-700 rounded text-sm">
              <span class="text-gray-700 dark:text-gray-300">
                <strong>{fieldOptions.find((f) => f.value === condition.field)?.label}</strong>
                {tr(`smartFolders.operators.${condition.operator}`)}
                <span class="font-mono text-blue-600 dark:text-blue-400">"{condition.value}"</span>
              </span>

              <button aria-label="Delete" onclick={() => removeCondition(idx)} class="p-1 text-red-500 hover:bg-red-100 dark:hover:bg-red-900/30 rounded transition"><i class="bi bi-trash" aria-hidden="true"></i></button>
            </div>

            {#if idx < conditions.length - 1}
              <div class="flex items-center justify-center">
                <span class="px-2 py-0 text-xs font-bold text-gray-500 dark:text-gray-400 bg-gray-100 dark:bg-gray-700 rounded">
                  {logic}
                </span>
              </div>
            {/if}
          {/each}
        </div>
      {/if}
    </div>

    <!-- Sorting Options -->
    <div class="space-y-3 p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
      <h3 class="text-sm font-medium text-gray-900 dark:text-white">{tr('smartFolders.sortingOptions')}</h3>

      <div class="grid grid-cols-2 gap-3">
        <div>
          <div class="block text-sm text-gray-700 dark:text-gray-300 mb-1">{tr('smartFolders.sortBy')}</div>
          <select
            bind:value={sortBy}
            class="w-full px-2 py-1 bg-white dark:bg-gray-600 border border-gray-200 dark:border-gray-500 rounded text-sm text-gray-900 dark:text-white focus:outline-none"
          >
            <option value="name">{tr('name')}</option>
            <option value="modified">{tr('modified')}</option>
            <option value="size">{tr('size')}</option>
            <option value="type">{tr('type')}</option>
          </select>
        </div>

        <div>
          <div class="block text-sm text-gray-700 dark:text-gray-300 mb-1">{tr('order')}</div>
          <select
            bind:value={sortOrder}
            class="w-full px-2 py-1 bg-white dark:bg-gray-600 border border-gray-200 dark:border-gray-500 rounded text-sm text-gray-900 dark:text-white focus:outline-none"
          >
            <option value="asc">{tr('ascending')}</option>
            <option value="desc">{tr('descending')}</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Actions -->
    <div class="flex gap-2 justify-end pt-4">
      <button
        onclick={() => (isOpen = false)}
        class="px-4 py-2 text-sm bg-gray-300 dark:bg-gray-600 text-gray-900 dark:text-white rounded-lg hover:bg-gray-400 dark:hover:bg-gray-500 transition"
      >
        {tr('cancel')}
      </button>

      <button
        onclick={handleSave}
        disabled={isSaving}
        class="px-4 py-2 text-sm bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 transition flex items-center gap-2"
      >
        {#if isSaving}
          <i class="bi bi-hourglass-split animate-spin" aria-hidden="true"></i>
          {tr('saving')}
        {:else}
          <i class="bi bi-check2" aria-hidden="true"></i>
          {tr('save')}
        {/if}
      </button>
    </div>
  </div>
</Modal>
