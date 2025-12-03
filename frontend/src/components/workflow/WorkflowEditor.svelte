<script>
  import { workflow } from "../../lib/api.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";
  import Modal from "../ui/Modal.svelte";

  let { rule, triggerTypes, actionTypes, onSave, onCancel } = $props();

  let formData = $state({
    name: "",
    display_name: "",
    description: "",
    trigger_type: "",
    trigger_config: {},
    condition_config: {},
    action_type: "",
    action_config: {},
    is_active: true,
    priority: 100,
  });

  // Update form data when rule prop changes
  $effect(() => {
    if (rule) {
      formData.name = rule.name || "";
      formData.display_name = rule.display_name || "";
      formData.description = rule.description || "";
      formData.trigger_type = rule.trigger_type || "";
      formData.trigger_config = JSON.parse(rule.trigger_config || "{}");
      formData.condition_config = JSON.parse(rule.condition_config || "{}");
      formData.action_type = rule.action_type || "";
      formData.action_config = JSON.parse(rule.action_config || "{}");
      formData.is_active = rule.is_active ?? true;
      formData.priority = rule.priority || 100;
    }
  });

  let loading = $state(false);
  let error = $state("");

  async function handleSubmit(e) {
    e.preventDefault();
    // Validation
    if (
      !formData.display_name ||
      !formData.trigger_type ||
      !formData.action_type
    ) {
      error = t($currentLang, "workflow.validationError");
      return;
    }

    loading = true;
    error = "";

    try {
      const payload = {
        name:
          formData.name ||
          formData.display_name.toLowerCase().replace(/\s+/g, "_"),
        display_name: formData.display_name,
        description: formData.description || null,
        trigger_type: formData.trigger_type,
        trigger_config: formData.trigger_config,
        condition_config: formData.condition_config,
        action_type: formData.action_type,
        action_config: formData.action_config,
        is_active: formData.is_active,
        priority: formData.priority,
      };

      if (rule) {
        await workflow.updateRule(rule.id, payload);
      } else {
        await workflow.createRule(payload);
      }

      onSave();
    } catch (err) {
      console.error("Failed to save workflow rule:", err);
      error = t($currentLang, "workflow.saveError");
    } finally {
      loading = false;
    }
  }

  function getTriggerDescription(value) {
    const type = triggerTypes.find((t) => t.value === value);
    return type ? type.description : "";
  }

  function getActionDescription(value) {
    const type = actionTypes.find((a) => a.value === value);
    return type ? type.description : "";
  }

  // Dynamic config fields based on trigger/action type
  function getTriggerConfigFields() {
    switch (formData.trigger_type) {
      case "file_upload":
      case "file_delete":
      case "file_move":
      case "file_rename":
        return [
          {
            key: "path_pattern",
            label: t($currentLang, "workflow.pathPattern"),
            type: "text",
            placeholder: "**/*",
            default: "**/*",
          },
          {
            key: "file_types",
            label: t($currentLang, "workflow.fileTypes"),
            type: "text",
            placeholder: "jpg,png,pdf",
            default: "",
          },
        ];
      case "scheduled":
        return [
          {
            key: "cron_expression",
            label: t($currentLang, "workflow.cronExpression"),
            type: "text",
            placeholder: "0 0 * * *",
            default: "0 0 * * *",
          },
        ];
      case "webhook":
        return [
          {
            key: "webhook_path",
            label: t($currentLang, "workflow.webhookPath"),
            type: "text",
            placeholder: "/webhook/custom",
            default: "",
          },
        ];
      default:
        return [];
    }
  }

  function getActionConfigFields() {
    switch (formData.action_type) {
      case "convert_file":
        return [
          {
            key: "target_format",
            label: t($currentLang, "workflow.targetFormat"),
            type: "text",
            placeholder: "pdf",
            default: "",
          },
          {
            key: "keep_original",
            label: t($currentLang, "workflow.keepOriginal"),
            type: "checkbox",
            default: false,
          },
        ];
      case "compress_file":
        return [
          {
            key: "quality",
            label: t($currentLang, "workflow.quality"),
            type: "number",
            placeholder: "85",
            default: 85,
          },
          {
            key: "keep_original",
            label: t($currentLang, "workflow.keepOriginal"),
            type: "checkbox",
            default: false,
          },
        ];
      case "move_file":
      case "copy_file":
        return [
          {
            key: "target_path",
            label: t($currentLang, "workflow.targetPath"),
            type: "text",
            placeholder: "/destination",
            default: "",
          },
          {
            key: "create_folders",
            label: t($currentLang, "workflow.createFolders"),
            type: "checkbox",
            default: true,
          },
        ];
      case "add_tag":
        return [
          {
            key: "tag_name",
            label: t($currentLang, "workflow.tagName"),
            type: "text",
            placeholder: "important",
            default: "",
          },
        ];
      case "send_notification":
        return [
          {
            key: "title",
            label: t($currentLang, "workflow.notificationTitle"),
            type: "text",
            placeholder: "Notification",
            default: "",
          },
          {
            key: "message",
            label: t($currentLang, "workflow.notificationMessage"),
            type: "text",
            placeholder: "Message",
            default: "",
          },
        ];
      case "send_webhook":
        return [
          {
            key: "webhook_url",
            label: t($currentLang, "workflow.webhookUrl"),
            type: "text",
            placeholder: "https://...",
            default: "",
          },
          {
            key: "method",
            label: t($currentLang, "workflow.httpMethod"),
            type: "select",
            options: ["POST", "GET", "PUT"],
            default: "POST",
          },
        ];
      case "send_email":
        return [
          {
            key: "to_email",
            label: t($currentLang, "workflow.toEmail"),
            type: "text",
            placeholder: "user@example.com",
            default: "",
          },
          {
            key: "subject",
            label: t($currentLang, "workflow.emailSubject"),
            type: "text",
            placeholder: "Subject",
            default: "",
          },
          {
            key: "body",
            label: t($currentLang, "workflow.emailBody"),
            type: "textarea",
            placeholder: "Email body",
            default: "",
          },
        ];
      default:
        return [];
    }
  }

  function getConditionConfigFields() {
    return [
      {
        key: "file_size_min",
        label: t($currentLang, "workflow.minFileSize"),
        type: "number",
        placeholder: "0",
        default: 0,
      },
      {
        key: "file_size_max",
        label: t($currentLang, "workflow.maxFileSize"),
        type: "number",
        placeholder: "0 (unlimited)",
        default: 0,
      },
      {
        key: "mime_types",
        label: t($currentLang, "workflow.mimeTypes"),
        type: "text",
        placeholder: "image/jpeg,application/pdf",
        default: "",
      },
    ];
  }

  // Initialize config with defaults
  function initializeConfig(config, fields) {
    fields.forEach((field) => {
      if (!(field.key in config)) {
        config[field.key] = field.default;
      }
    });
  }

  $effect(() => {
    // Auto-initialize configs when trigger/action type changes
    const triggerFields = getTriggerConfigFields();
    const actionFields = getActionConfigFields();
    const conditionFields = getConditionConfigFields();

    initializeConfig(formData.trigger_config, triggerFields);
    initializeConfig(formData.action_config, actionFields);
    initializeConfig(formData.condition_config, conditionFields);
  });
</script>

<Modal
  visible={true}
  title={rule
    ? t($currentLang, "workflow.editRule")
    : t($currentLang, "workflow.createRule")}
  icon="diagram-3"
  size="xl"
  onclose={onCancel}
>
  {#snippet children()}
    <form onsubmit={handleSubmit} class="space-y-6">
      <!-- Basic Info -->
      <div class="space-y-4">
        <div>
          <label
            class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
            for="rule-name"
          >
            {t($currentLang, "workflow.ruleName")}
          </label>
          <input
            id="rule-name"
            type="text"
            bind:value={formData.display_name}
            placeholder={t($currentLang, "workflow.ruleNamePlaceholder")}
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            required
          />
        </div>

        <div>
          <label
            class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
            for="rule-desc"
          >
            {t($currentLang, "workflow.description")}
          </label>
          <textarea
            id="rule-desc"
            bind:value={formData.description}
            placeholder={t($currentLang, "workflow.descriptionPlaceholder")}
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            rows="2"
          ></textarea>
        </div>

        <div class="flex gap-4 items-end">
          <div class="flex-1">
            <label
              class="block text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300"
              for="rule-priority"
            >
              {t($currentLang, "workflow.priority")}
            </label>
            <input
              id="rule-priority"
              type="number"
              bind:value={formData.priority}
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              min="1"
              max="1000"
            />
            <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
              {t($currentLang, "workflow.priorityHint")}
            </p>
          </div>

          <label class="flex items-center gap-2 cursor-pointer pb-2">
            <input
              type="checkbox"
              bind:checked={formData.is_active}
              class="w-4 h-4 rounded border-gray-300 text-blue-500 focus:ring-blue-500"
            />
            <span class="font-semibold text-gray-700 dark:text-gray-300">
              {t($currentLang, "workflow.activeRule")}
            </span>
          </label>
        </div>
      </div>

      <!-- IFTTT-Style Workflow Builder -->
      <!-- Trigger Section -->
      <div
        class="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg border-2 border-blue-200 dark:border-blue-700"
      >
        <div class="flex items-center gap-2 mb-3">
          <i
            class="bi bi-lightning-charge text-blue-500 text-xl"
            aria-hidden="true"
          ></i>
          <h4 class="font-bold text-lg text-blue-700 dark:text-blue-300">
            {t($currentLang, "workflow.whenThis")}
          </h4>
        </div>

        <div>
          <label
            class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
            for="trigger-type"
          >
            {t($currentLang, "workflow.triggerType")}
          </label>
          <select
            id="trigger-type"
            bind:value={formData.trigger_type}
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            required
          >
            <option value="">{t($currentLang, "workflow.selectTrigger")}</option
            >
            {#each triggerTypes as triggerType}
              <option value={triggerType.value}>{triggerType.label}</option>
            {/each}
          </select>
          {#if formData.trigger_type}
            <p class="mt-1 text-sm text-blue-600 dark:text-blue-400">
              {getTriggerDescription(formData.trigger_type)}
            </p>
          {/if}
        </div>

        {#if formData.trigger_type && getTriggerConfigFields().length > 0}
          <div class="mt-4 space-y-3">
            {#each getTriggerConfigFields() as field}
              <div>
                <label
                  class="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300"
                  for="trigger-{field.key}"
                >
                  {field.label}
                </label>
                {#if field.type === "checkbox"}
                  <input
                    id="trigger-{field.key}"
                    type="checkbox"
                    bind:checked={formData.trigger_config[field.key]}
                    class="w-4 h-4 rounded border-gray-300 text-blue-500 focus:ring-blue-500"
                  />
                {:else}
                  <input
                    id="trigger-{field.key}"
                    type={field.type}
                    bind:value={formData.trigger_config[field.key]}
                    placeholder={field.placeholder}
                    class="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
                  />
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Condition Section (Optional) -->
      <div
        class="bg-yellow-50 dark:bg-yellow-900/20 p-4 rounded-lg border-2 border-yellow-200 dark:border-yellow-700"
      >
        <div class="flex items-center gap-2 mb-3">
          <i class="bi bi-filter text-yellow-600 text-xl" aria-hidden="true"
          ></i>
          <h4 class="font-bold text-lg text-yellow-700 dark:text-yellow-300">
            {t($currentLang, "workflow.ifThis")}
          </h4>
          <span
            class="px-2 py-0.5 text-xs bg-gray-200 dark:bg-gray-700 rounded text-gray-600 dark:text-gray-300"
          >
            {t($currentLang, "workflow.optional")}
          </span>
        </div>

        <div class="space-y-3">
          {#each getConditionConfigFields() as field}
            <div>
              <label
                class="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300"
                for="condition-{field.key}"
              >
                {field.label}
              </label>
              <input
                id="condition-{field.key}"
                type={field.type}
                bind:value={formData.condition_config[field.key]}
                placeholder={field.placeholder}
                class="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
              />
            </div>
          {/each}
        </div>
      </div>

      <!-- Action Section -->
      <div
        class="bg-purple-50 dark:bg-purple-900/20 p-4 rounded-lg border-2 border-purple-200 dark:border-purple-700"
      >
        <div class="flex items-center gap-2 mb-3">
          <i class="bi bi-gear text-purple-500 text-xl" aria-hidden="true"></i>
          <h4 class="font-bold text-lg text-purple-700 dark:text-purple-300">
            {t($currentLang, "workflow.thenThat")}
          </h4>
        </div>

        <div>
          <label
            class="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300"
            for="action-type"
          >
            {t($currentLang, "workflow.actionType")}
          </label>
          <select
            id="action-type"
            bind:value={formData.action_type}
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-purple-500 focus:border-transparent"
            required
          >
            <option value="">{t($currentLang, "workflow.selectAction")}</option>
            {#each actionTypes as actionType}
              <option value={actionType.value}>{actionType.label}</option>
            {/each}
          </select>
          {#if formData.action_type}
            <p class="mt-1 text-sm text-purple-600 dark:text-purple-400">
              {getActionDescription(formData.action_type)}
            </p>
          {/if}
        </div>

        {#if formData.action_type && getActionConfigFields().length > 0}
          <div class="mt-4 space-y-3">
            {#each getActionConfigFields() as field}
              <div>
                <label
                  class="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300"
                  for="action-{field.key}"
                >
                  {field.label}
                </label>
                {#if field.type === "checkbox"}
                  <input
                    id="action-{field.key}"
                    type="checkbox"
                    bind:checked={formData.action_config[field.key]}
                    class="w-4 h-4 rounded border-gray-300 text-purple-500 focus:ring-purple-500"
                  />
                {:else if field.type === "select"}
                  <select
                    id="action-{field.key}"
                    bind:value={formData.action_config[field.key]}
                    class="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
                  >
                    {#each field.options as option}
                      <option value={option}>{option}</option>
                    {/each}
                  </select>
                {:else if field.type === "textarea"}
                  <textarea
                    id="action-{field.key}"
                    bind:value={formData.action_config[field.key]}
                    placeholder={field.placeholder}
                    class="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
                    rows="3"
                  ></textarea>
                {:else}
                  <input
                    id="action-{field.key}"
                    type={field.type}
                    bind:value={formData.action_config[field.key]}
                    placeholder={field.placeholder}
                    class="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
                  />
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      </div>

      {#if error}
        <div
          class="bg-red-50 dark:bg-red-900/30 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-300 px-4 py-3 rounded-lg"
        >
          <i class="bi bi-exclamation-triangle mr-2" aria-hidden="true"></i>
          <span>{error}</span>
        </div>
      {/if}
    </form>
  {/snippet}

  {#snippet actions()}
    <button
      type="button"
      onclick={onCancel}
      class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors"
      disabled={loading}
    >
      {t($currentLang, "workflow.cancel")}
    </button>
    <button
      type="submit"
      class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg font-medium transition-colors disabled:opacity-50"
      disabled={loading}
      onclick={handleSubmit}
    >
      {#if loading}
        <span class="loading loading-spinner loading-sm mr-2"></span>
      {/if}
      {rule
        ? t($currentLang, "workflow.saveRule")
        : t($currentLang, "workflow.createRule")}
    </button>
  {/snippet}
</Modal>
