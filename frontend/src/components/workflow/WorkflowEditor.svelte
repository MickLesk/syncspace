<script>
  import { workflow } from "../../lib/api.js";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  let { rule, triggerTypes, actionTypes, onSave, onCancel } = $props();

  let formData = $state({
    name: rule?.name || "",
    display_name: rule?.display_name || "",
    description: rule?.description || "",
    trigger_type: rule?.trigger_type || "",
    trigger_config: rule ? JSON.parse(rule.trigger_config) : {},
    condition_config: rule ? JSON.parse(rule.condition_config) : {},
    action_type: rule?.action_type || "",
    action_config: rule ? JSON.parse(rule.action_config) : {},
    is_active: rule?.is_active ?? true,
    priority: rule?.priority || 100,
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

<div class="modal modal-open">
  <div class="modal-box max-w-4xl">
    <h3 class="font-bold text-2xl mb-4">
      {rule
        ? t($currentLang, "workflow.editRule")
        : t($currentLang, "workflow.createRule")}
    </h3>

    <form onsubmit={handleSubmit}>
      <!-- Basic Info -->
      <div class="space-y-4 mb-6">
        <div>
          <label class="label">
            <span class="label-text font-semibold"
              >{t($currentLang, "workflow.ruleName")}</span
            >
          </label>
          <input
            type="text"
            bind:value={formData.display_name}
            placeholder={t($currentLang, "workflow.ruleNamePlaceholder")}
            class="input input-bordered w-full"
            required
          />
        </div>

        <div>
          <label class="label">
            <span class="label-text font-semibold"
              >{t($currentLang, "workflow.description")}</span
            >
          </label>
          <textarea
            bind:value={formData.description}
            placeholder={t($currentLang, "workflow.descriptionPlaceholder")}
            class="textarea textarea-bordered w-full"
            rows="2"
          ></textarea>
        </div>

        <div class="flex gap-4">
          <div class="flex-1">
            <label class="label">
              <span class="label-text font-semibold"
                >{t($currentLang, "workflow.priority")}</span
              >
            </label>
            <input
              type="number"
              bind:value={formData.priority}
              class="input input-bordered w-full"
              min="1"
              max="1000"
            />
            <label class="label">
              <span class="label-text-alt"
                >{t($currentLang, "workflow.priorityHint")}</span
              >
            </label>
          </div>

          <div class="flex items-center">
            <label class="label cursor-pointer">
              <input
                type="checkbox"
                bind:checked={formData.is_active}
                class="checkbox checkbox-primary mr-2"
              />
              <span class="label-text font-semibold"
                >{t($currentLang, "workflow.activeRule")}</span
              >
            </label>
          </div>
        </div>
      </div>

      <!-- IFTTT-Style Workflow Builder -->
      <div class="space-y-6">
        <!-- Trigger Section -->
        <div class="bg-primary/10 p-4 rounded-lg border-2 border-primary/30">
          <div class="flex items-center gap-2 mb-3">
            <i class="bi bi-lightning-charge text-primary text-xl"></i>
            <h4 class="font-bold text-lg">
              {t($currentLang, "workflow.whenThis")}
            </h4>
          </div>

          <div>
            <label class="label">
              <span class="label-text"
                >{t($currentLang, "workflow.triggerType")}</span
              >
            </label>
            <select
              bind:value={formData.trigger_type}
              class="select select-bordered w-full"
              required
            >
              <option value=""
                >{t($currentLang, "workflow.selectTrigger")}</option
              >
              {#each triggerTypes as triggerType}
                <option value={triggerType.value}>{triggerType.label}</option>
              {/each}
            </select>
            {#if formData.trigger_type}
              <label class="label">
                <span class="label-text-alt text-primary"
                  >{getTriggerDescription(formData.trigger_type)}</span
                >
              </label>
            {/if}
          </div>

          {#if formData.trigger_type && getTriggerConfigFields().length > 0}
            <div class="mt-4 space-y-3">
              {#each getTriggerConfigFields() as field}
                <div>
                  <label class="label">
                    <span class="label-text">{field.label}</span>
                  </label>
                  {#if field.type === "checkbox"}
                    <input
                      type="checkbox"
                      bind:checked={formData.trigger_config[field.key]}
                      class="checkbox checkbox-primary"
                    />
                  {:else}
                    <input
                      type={field.type}
                      bind:value={formData.trigger_config[field.key]}
                      placeholder={field.placeholder}
                      class="input input-bordered w-full input-sm"
                    />
                  {/if}
                </div>
              {/each}
            </div>
          {/if}
        </div>

        <!-- Condition Section (Optional) -->
        <div class="bg-warning/10 p-4 rounded-lg border-2 border-warning/30">
          <div class="flex items-center gap-2 mb-3">
            <i class="bi bi-filter text-warning text-xl"></i>
            <h4 class="font-bold text-lg">
              {t($currentLang, "workflow.ifThis")}
            </h4>
            <span class="badge badge-sm badge-ghost"
              >{t($currentLang, "workflow.optional")}</span
            >
          </div>

          <div class="space-y-3">
            {#each getConditionConfigFields() as field}
              <div>
                <label class="label">
                  <span class="label-text">{field.label}</span>
                </label>
                <input
                  type={field.type}
                  bind:value={formData.condition_config[field.key]}
                  placeholder={field.placeholder}
                  class="input input-bordered w-full input-sm"
                />
              </div>
            {/each}
          </div>
        </div>

        <!-- Action Section -->
        <div
          class="bg-secondary/10 p-4 rounded-lg border-2 border-secondary/30"
        >
          <div class="flex items-center gap-2 mb-3">
            <i class="bi bi-gear text-secondary text-xl"></i>
            <h4 class="font-bold text-lg">
              {t($currentLang, "workflow.thenThat")}
            </h4>
          </div>

          <div>
            <label class="label">
              <span class="label-text"
                >{t($currentLang, "workflow.actionType")}</span
              >
            </label>
            <select
              bind:value={formData.action_type}
              class="select select-bordered w-full"
              required
            >
              <option value=""
                >{t($currentLang, "workflow.selectAction")}</option
              >
              {#each actionTypes as actionType}
                <option value={actionType.value}>{actionType.label}</option>
              {/each}
            </select>
            {#if formData.action_type}
              <label class="label">
                <span class="label-text-alt text-secondary"
                  >{getActionDescription(formData.action_type)}</span
                >
              </label>
            {/if}
          </div>

          {#if formData.action_type && getActionConfigFields().length > 0}
            <div class="mt-4 space-y-3">
              {#each getActionConfigFields() as field}
                <div>
                  <label class="label">
                    <span class="label-text">{field.label}</span>
                  </label>
                  {#if field.type === "checkbox"}
                    <input
                      type="checkbox"
                      bind:checked={formData.action_config[field.key]}
                      class="checkbox checkbox-secondary"
                    />
                  {:else if field.type === "select"}
                    <select
                      bind:value={formData.action_config[field.key]}
                      class="select select-bordered w-full select-sm"
                    >
                      {#each field.options as option}
                        <option value={option}>{option}</option>
                      {/each}
                    </select>
                  {:else if field.type === "textarea"}
                    <textarea
                      bind:value={formData.action_config[field.key]}
                      placeholder={field.placeholder}
                      class="textarea textarea-bordered w-full textarea-sm"
                      rows="3"
                    ></textarea>
                  {:else}
                    <input
                      type={field.type}
                      bind:value={formData.action_config[field.key]}
                      placeholder={field.placeholder}
                      class="input input-bordered w-full input-sm"
                    />
                  {/if}
                </div>
              {/each}
            </div>
          {/if}
        </div>
      </div>

      {#if error}
        <div class="alert alert-error mt-4">
          <i class="bi bi-exclamation-triangle"></i>
          <span>{error}</span>
        </div>
      {/if}

      <!-- Actions -->
      <div class="modal-action">
        <button
          type="button"
          onclick={onCancel}
          class="btn btn-ghost"
          disabled={loading}
        >
          {t($currentLang, "workflow.cancel")}
        </button>
        <button type="submit" class="btn btn-primary" disabled={loading}>
          {#if loading}
            <span class="loading loading-spinner loading-sm"></span>
          {/if}
          {rule
            ? t($currentLang, "workflow.saveRule")
            : t($currentLang, "workflow.createRule")}
        </button>
      </div>
    </form>
  </div>
</div>

<style>
  .label {
    padding: 0.25rem 0;
  }
</style>
