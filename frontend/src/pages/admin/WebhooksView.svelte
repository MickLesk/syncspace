<script>
  /**
   * Webhook Management View
   * Create and manage webhooks for external integrations
   */
  import { onMount, onDestroy } from "svelte";
  import { currentLang } from "../../stores/ui.js";
  import { t } from "../../i18n.js";

  const API_BASE = `${window.location.protocol}//${window.location.hostname}:8080/api`;

  // State
  let webhooks = $state([]);
  let availableEvents = $state([]);
  let loading = $state(true);
  let error = $state(null);

  // Modal states
  let showCreateModal = $state(false);
  let showEditModal = $state(false);
  let showDeliveriesModal = $state(false);
  let showTestModal = $state(false);

  // Form states
  let editingWebhook = $state(null);
  let deliveries = $state([]);
  let testResult = $state(null);
  let testLoading = $state(false);

  // Create/Edit form
  let formName = $state("");
  let formUrl = $state("");
  let formEvents = $state([]);
  let formSecret = $state("");
  let formIsActive = $state(true);
  let formSaving = $state(false);

  // Translation helper
  const tr = $derived((key, ...args) => t($currentLang, key, ...args));

  // API helpers
  function getHeaders() {
    const token = localStorage.getItem("authToken");
    return {
      "Content-Type": "application/json",
      Authorization: token ? `Bearer ${token}` : "",
    };
  }

  async function fetchJSON(url, options = {}) {
    const response = await fetch(url, {
      ...options,
      headers: { ...getHeaders(), ...options.headers },
    });
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }
    return response.json();
  }

  // Load webhooks
  async function loadWebhooks() {
    try {
      loading = true;
      error = null;
      const data = await fetchJSON(`${API_BASE}/webhooks`);
      webhooks = data.webhooks || [];
    } catch (e) {
      console.error("Failed to load webhooks:", e);
      error = tr("webhooks.loadError");
    } finally {
      loading = false;
    }
  }

  // Load available events
  async function loadAvailableEvents() {
    try {
      const data = await fetchJSON(`${API_BASE}/webhooks/events`);
      availableEvents = data.events || [];
    } catch (e) {
      console.error("Failed to load webhook events:", e);
    }
  }

  // Create webhook
  async function createWebhook() {
    if (!formName.trim() || !formUrl.trim() || formEvents.length === 0) {
      return;
    }

    try {
      formSaving = true;
      await fetchJSON(`${API_BASE}/webhooks`, {
        method: "POST",
        body: JSON.stringify({
          name: formName.trim(),
          url: formUrl.trim(),
          events: formEvents,
          secret: formSecret.trim() || null,
        }),
      });
      showCreateModal = false;
      resetForm();
      await loadWebhooks();
    } catch (e) {
      console.error("Failed to create webhook:", e);
      error = tr("webhooks.createError");
    } finally {
      formSaving = false;
    }
  }

  // Update webhook
  async function updateWebhook() {
    if (!editingWebhook || !formName.trim() || !formUrl.trim()) {
      return;
    }

    try {
      formSaving = true;
      await fetchJSON(`${API_BASE}/webhooks/${editingWebhook.id}`, {
        method: "PUT",
        body: JSON.stringify({
          name: formName.trim(),
          url: formUrl.trim(),
          events: formEvents,
          secret: formSecret.trim() || null,
          is_active: formIsActive,
        }),
      });
      showEditModal = false;
      editingWebhook = null;
      resetForm();
      await loadWebhooks();
    } catch (e) {
      console.error("Failed to update webhook:", e);
      error = tr("webhooks.updateError");
    } finally {
      formSaving = false;
    }
  }

  // Delete webhook
  async function deleteWebhook(webhook) {
    if (!confirm(tr("webhooks.deleteConfirm", webhook.name))) {
      return;
    }

    try {
      await fetch(`${API_BASE}/webhooks/${webhook.id}`, {
        method: "DELETE",
        headers: getHeaders(),
      });
      await loadWebhooks();
    } catch (e) {
      console.error("Failed to delete webhook:", e);
      error = tr("webhooks.deleteError");
    }
  }

  // Toggle webhook active state
  async function toggleWebhook(webhook) {
    try {
      await fetchJSON(`${API_BASE}/webhooks/${webhook.id}`, {
        method: "PUT",
        body: JSON.stringify({
          is_active: !webhook.is_active,
        }),
      });
      await loadWebhooks();
    } catch (e) {
      console.error("Failed to toggle webhook:", e);
    }
  }

  // Test webhook
  async function testWebhook(webhook) {
    try {
      testLoading = true;
      testResult = null;
      showTestModal = true;
      editingWebhook = webhook;

      const data = await fetchJSON(`${API_BASE}/webhooks/${webhook.id}/test`, {
        method: "POST",
        body: JSON.stringify({ event_type: "test.ping" }),
      });
      testResult = data;
    } catch (e) {
      console.error("Failed to test webhook:", e);
      testResult = { success: false, error: e.message };
    } finally {
      testLoading = false;
    }
  }

  // Load deliveries
  async function loadDeliveries(webhook) {
    try {
      editingWebhook = webhook;
      showDeliveriesModal = true;
      const data = await fetchJSON(
        `${API_BASE}/webhooks/${webhook.id}/deliveries`
      );
      deliveries = data.deliveries || [];
    } catch (e) {
      console.error("Failed to load deliveries:", e);
      deliveries = [];
    }
  }

  // Reset failure count
  async function resetFailures(webhook) {
    try {
      await fetch(`${API_BASE}/webhooks/${webhook.id}/reset-failures`, {
        method: "POST",
        headers: getHeaders(),
      });
      await loadWebhooks();
    } catch (e) {
      console.error("Failed to reset failures:", e);
    }
  }

  // Form helpers
  function resetForm() {
    formName = "";
    formUrl = "";
    formEvents = [];
    formSecret = "";
    formIsActive = true;
  }

  function openCreateModal() {
    resetForm();
    showCreateModal = true;
  }

  function openEditModal(webhook) {
    editingWebhook = webhook;
    formName = webhook.name;
    formUrl = webhook.url;
    formEvents = [...webhook.events];
    formSecret = "";
    formIsActive = webhook.is_active;
    showEditModal = true;
  }

  function toggleEvent(eventId) {
    if (formEvents.includes(eventId)) {
      formEvents = formEvents.filter((e) => e !== eventId);
    } else {
      formEvents = [...formEvents, eventId];
    }
  }

  function selectAllEvents() {
    formEvents = availableEvents.map((e) => e.id);
  }

  function deselectAllEvents() {
    formEvents = [];
  }

  // Format helpers
  function formatDate(dateStr) {
    if (!dateStr) return "-";
    return new Date(dateStr).toLocaleString();
  }

  function formatDuration(ms) {
    if (ms < 1000) return `${ms}ms`;
    return `${(ms / 1000).toFixed(2)}s`;
  }

  function getStatusColor(success) {
    return success
      ? "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
      : "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200";
  }

  function getEventCategory(eventId) {
    if (eventId.startsWith("file.")) return "file";
    if (eventId.startsWith("folder.")) return "folder";
    if (eventId.startsWith("user.")) return "user";
    if (eventId.startsWith("backup.")) return "backup";
    return "other";
  }

  function getEventIcon(eventId) {
    const category = getEventCategory(eventId);
    switch (category) {
      case "file":
        return "bi-file-earmark";
      case "folder":
        return "bi-folder";
      case "user":
        return "bi-person";
      case "backup":
        return "bi-cloud-arrow-up";
      default:
        return "bi-lightning";
    }
  }

  // Lifecycle
  onMount(() => {
    loadWebhooks();
    loadAvailableEvents();
  });
</script>

<div class="p-6 max-w-7xl mx-auto">
  <!-- Header -->
  <div class="flex items-center justify-between mb-6">
    <div>
      <h1
        class="text-2xl font-bold text-gray-900 dark:text-white flex items-center gap-2"
      >
        <i class="bi bi-broadcast text-primary-600" aria-hidden="true"></i>
        {tr("webhooks.title")}
      </h1>
      <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
        {tr("webhooks.description")}
      </p>
    </div>
    <button
      aria-label="Add"
      onclick={openCreateModal}
      class="btn btn-primary flex items-center gap-2"
      ><i class="bi bi-plus-lg" aria-hidden="true"></i>
      {tr("webhooks.create")}
    </button>
  </div>

  <!-- Error message -->
  {#if error}
    <div
      class="mb-4 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-700 dark:text-red-200 flex items-center gap-2"
    >
      <i class="bi bi-exclamation-circle" aria-hidden="true"></i>
      {error}
      <button
        aria-label="Close"
        onclick={() => (error = null)}
        class="ml-auto text-red-500 hover:text-red-700"
        ><i class="bi bi-x-lg" aria-hidden="true"></i></button
      >
    </div>
  {/if}

  <!-- Loading state -->
  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div
        class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"
      ></div>
    </div>
  {:else if webhooks.length === 0}
    <!-- Empty state -->
    <div class="text-center py-12 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
      <i
        class="bi bi-broadcast text-4xl text-gray-400 dark:text-gray-500 mb-4"
        aria-hidden="true"
      ></i>
      <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-2">
        {tr("webhooks.noWebhooks")}
      </h3>
      <p class="text-gray-500 dark:text-gray-400 mb-4">
        {tr("webhooks.noWebhooksDesc")}
      </p>
      <button onclick={openCreateModal} class="btn btn-primary">
        <i class="bi bi-plus-lg mr-2" aria-hidden="true"></i>
        {tr("webhooks.createFirst")}
      </button>
    </div>
  {:else}
    <!-- Webhooks list -->
    <div class="space-y-4">
      {#each webhooks as webhook}
        <div
          class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-4 hover:shadow-md transition-shadow"
        >
          <div class="flex items-start justify-between">
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-3 mb-2">
                <h3
                  class="text-lg font-semibold text-gray-900 dark:text-white truncate"
                >
                  {webhook.name}
                </h3>
                <span
                  class="px-2 py-0.5 text-xs rounded-full {webhook.is_active
                    ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
                    : 'bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-400'}"
                >
                  {webhook.is_active
                    ? tr("webhooks.active")
                    : tr("webhooks.inactive")}
                </span>
                {#if webhook.has_secret}
                  <span
                    class="px-2 py-0.5 text-xs rounded-full bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200"
                  >
                    <i class="bi bi-shield-lock-fill mr-1" aria-hidden="true"
                    ></i>
                    {tr("webhooks.signed")}
                  </span>
                {/if}
                {#if webhook.failure_count > 0}
                  <span
                    class="px-2 py-0.5 text-xs rounded-full bg-amber-100 text-amber-800 dark:bg-amber-900 dark:text-amber-200"
                  >
                    <i
                      class="bi bi-exclamation-triangle mr-1"
                      aria-hidden="true"
                    ></i>
                    {webhook.failure_count}
                    {tr("webhooks.failures")}
                  </span>
                {/if}
              </div>

              <div
                class="text-sm text-gray-500 dark:text-gray-400 mb-2 font-mono truncate"
              >
                {webhook.url}
              </div>

              <div class="flex flex-wrap gap-1 mb-3">
                {#each webhook.events.slice(0, 5) as event}
                  <span
                    class="px-2 py-0.5 text-xs bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300 rounded"
                  >
                    <i class="{getEventIcon(event)} mr-1"></i>
                    {event}
                  </span>
                {/each}
                {#if webhook.events.length > 5}
                  <span
                    class="px-2 py-0.5 text-xs bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300 rounded"
                  >
                    +{webhook.events.length - 5}
                    {tr("webhooks.more")}
                  </span>
                {/if}
              </div>

              <div class="text-xs text-gray-400 dark:text-gray-500">
                {tr("webhooks.created")}: {formatDate(webhook.created_at)}
                {#if webhook.last_triggered_at}
                  <span class="mx-2">•</span>
                  {tr("webhooks.lastTriggered")}: {formatDate(
                    webhook.last_triggered_at
                  )}
                {/if}
              </div>
            </div>

            <div class="flex items-center gap-2 ml-4">
              <button
                aria-label="Test webhook"
                onclick={() => testWebhook(webhook)}
                class="p-2 text-gray-500 hover:text-primary-600 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
                title={tr("webhooks.test")}
                ><i class="bi bi-play-fill" aria-hidden="true"></i></button
              >
              <button
                onclick={() => loadDeliveries(webhook)}
                class="p-2 text-gray-500 hover:text-primary-600 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
                title={tr("webhooks.deliveries")}
              >
                <i class="bi bi-clock-history" aria-hidden="true"></i>
              </button>
              <button
                onclick={() => toggleWebhook(webhook)}
                class="p-2 text-gray-500 hover:text-primary-600 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
                title={webhook.is_active
                  ? tr("webhooks.disable")
                  : tr("webhooks.enable")}
              >
                <i
                  class="bi {webhook.is_active
                    ? 'bi-pause-fill'
                    : 'bi-play-fill'}"
                ></i>
              </button>
              <button
                onclick={() => openEditModal(webhook)}
                class="p-2 text-gray-500 hover:text-blue-600 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
                title={tr("webhooks.edit")}
              >
                <i class="bi bi-pencil" aria-hidden="true"></i>
              </button>
              <button
                onclick={() => deleteWebhook(webhook)}
                class="p-2 text-gray-500 hover:text-red-600 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
                title={tr("webhooks.delete")}
                aria-label="Delete"
              >
                <i class="bi bi-trash" aria-hidden="true"></i>
              </button>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<!-- Create/Edit Modal -->
{#if showCreateModal || showEditModal}
  <div
    class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    onclick={(e) => {
      if (e.target === e.currentTarget) {
        showCreateModal = false;
        showEditModal = false;
      }
    }}
    onkeydown={(e) =>
      e.key === "Escape" &&
      ((showCreateModal = false), (showEditModal = false))}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-2xl max-w-2xl w-full max-h-[90vh] overflow-hidden flex flex-col"
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700"
      >
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
          {showCreateModal
            ? tr("webhooks.createWebhook")
            : tr("webhooks.editWebhook")}
        </h2>
        <button
          aria-label="Close"
          onclick={() => {
            showCreateModal = false;
            showEditModal = false;
          }}
          class="p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 rounded-lg"
        >
          <i class="bi bi-x-lg" aria-hidden="true"></i>
        </button>
      </div>

      <!-- Body -->
      <div class="p-4 overflow-y-auto flex-1 space-y-4">
        <!-- Name -->
        <div>
          <div
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >
            {tr("webhooks.name")} *
          </div>
          <input
            type="text"
            bind:value={formName}
            placeholder={tr("webhooks.namePlaceholder")}
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent"
          />
        </div>

        <!-- URL -->
        <div>
          <div
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >
            {tr("webhooks.url")} *
          </div>
          <input
            type="url"
            bind:value={formUrl}
            placeholder="https://example.com/webhook"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white font-mono text-sm focus:ring-2 focus:ring-primary-500 focus:border-transparent"
          />
        </div>

        <!-- Secret -->
        <div>
          <div
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >
            {tr("webhooks.secret")}
            <span class="text-gray-400 font-normal ml-1"
              >({tr("webhooks.optional")})</span
            >
          </div>
          <input
            type="password"
            bind:value={formSecret}
            placeholder={showEditModal
              ? tr("webhooks.secretPlaceholderEdit")
              : tr("webhooks.secretPlaceholder")}
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white font-mono text-sm focus:ring-2 focus:ring-primary-500 focus:border-transparent"
          />
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
            {tr("webhooks.secretHelp")}
          </p>
        </div>

        <!-- Events -->
        <div>
          <div class="flex items-center justify-between mb-2">
            <div
              class="block text-sm font-medium text-gray-700 dark:text-gray-300"
            >
              {tr("webhooks.events")} *
            </div>
            <div class="flex gap-2">
              <button
                onclick={selectAllEvents}
                class="text-xs text-primary-600 hover:text-primary-700 dark:text-primary-400"
              >
                {tr("webhooks.selectAll")}
              </button>
              <span class="text-gray-300 dark:text-gray-600">|</span>
              <button
                onclick={deselectAllEvents}
                class="text-xs text-primary-600 hover:text-primary-700 dark:text-primary-400"
              >
                {tr("webhooks.deselectAll")}
              </button>
            </div>
          </div>
          <div
            class="grid grid-cols-2 gap-2 max-h-48 overflow-y-auto p-2 bg-gray-50 dark:bg-gray-900 rounded-lg"
          >
            {#each availableEvents as event}
              <label
                class="flex items-center gap-2 p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-800 cursor-pointer"
              >
                <input
                  type="checkbox"
                  checked={formEvents.includes(event.id)}
                  onchange={() => toggleEvent(event.id)}
                  class="rounded border-gray-300 dark:border-gray-600 text-primary-600 focus:ring-primary-500"
                />
                <span class="text-sm text-gray-700 dark:text-gray-300">
                  <i class="{getEventIcon(event.id)} mr-1 text-gray-400"></i>
                  {event.label}
                </span>
              </label>
            {/each}
          </div>
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
            {formEvents.length}
            {tr("webhooks.eventsSelected")}
          </p>
        </div>

        <!-- Active toggle (only in edit mode) -->
        {#if showEditModal}
          <div
            class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg"
          >
            <div>
              <div class="font-medium text-gray-900 dark:text-white">
                {tr("webhooks.enabled")}
              </div>
              <div class="text-sm text-gray-500 dark:text-gray-400">
                {tr("webhooks.enabledDesc")}
              </div>
            </div>
            <button
              aria-label="Toggle enabled"
              onclick={() => (formIsActive = !formIsActive)}
              class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {formIsActive
                ? 'bg-primary-600'
                : 'bg-gray-300 dark:bg-gray-600'}"
            >
              <span
                class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {formIsActive
                  ? 'translate-x-6'
                  : 'translate-x-1'}"
              ></span>
            </button>
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="flex items-center justify-end gap-3 p-4 border-t border-gray-200 dark:border-gray-700"
      >
        <button
          onclick={() => {
            showCreateModal = false;
            showEditModal = false;
          }}
          class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
        >
          {tr("common.cancel")}
        </button>
        <button
          onclick={showCreateModal ? createWebhook : updateWebhook}
          disabled={formSaving ||
            !formName.trim() ||
            !formUrl.trim() ||
            formEvents.length === 0}
          class="px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
        >
          {#if formSaving}
            <div
              class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"
            ></div>
          {/if}
          {showCreateModal ? tr("webhooks.create") : tr("common.save")}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Deliveries Modal -->
{#if showDeliveriesModal && editingWebhook}
  <div
    class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    onclick={(e) => {
      if (e.target === e.currentTarget) {
        showDeliveriesModal = false;
      }
    }}
    onkeydown={(e) => e.key === "Escape" && (showDeliveriesModal = false)}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-2xl max-w-4xl w-full max-h-[90vh] overflow-hidden flex flex-col"
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700"
      >
        <div>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
            {tr("webhooks.deliveryHistory")}
          </h2>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            {editingWebhook.name}
          </p>
        </div>
        <button
          aria-label="Close"
          onclick={() => (showDeliveriesModal = false)}
          class="p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 rounded-lg"
          ><i class="bi bi-x-lg" aria-hidden="true"></i></button
        >
      </div>

      <!-- Body -->
      <div class="p-4 overflow-y-auto flex-1">
        {#if deliveries.length === 0}
          <div class="text-center py-8 text-gray-500 dark:text-gray-400">
            <i class="bi bi-clock-history text-3xl mb-2" aria-hidden="true"></i>
            <p>{tr("webhooks.noDeliveries")}</p>
          </div>
        {:else}
          <div class="space-y-2">
            {#each deliveries as delivery}
              <div
                class="border border-gray-200 dark:border-gray-700 rounded-lg p-3"
              >
                <div class="flex items-center justify-between mb-2">
                  <div class="flex items-center gap-2">
                    <span
                      class="px-2 py-0.5 text-xs rounded-full {getStatusColor(
                        delivery.success
                      )}"
                    >
                      {delivery.success
                        ? tr("webhooks.success")
                        : tr("webhooks.failed")}
                    </span>
                    <span
                      class="text-sm font-medium text-gray-900 dark:text-white"
                    >
                      {delivery.event_type}
                    </span>
                  </div>
                  <div
                    class="flex items-center gap-3 text-xs text-gray-500 dark:text-gray-400"
                  >
                    {#if delivery.response_status}
                      <span>HTTP {delivery.response_status}</span>
                    {/if}
                    <span>{formatDuration(delivery.duration_ms)}</span>
                    <span>{formatDate(delivery.delivered_at)}</span>
                  </div>
                </div>
                {#if delivery.error_message}
                  <div
                    class="text-sm text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20 p-2 rounded"
                  >
                    {delivery.error_message}
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="flex items-center justify-between p-4 border-t border-gray-200 dark:border-gray-700"
      >
        <span class="text-sm text-gray-500 dark:text-gray-400">
          {deliveries.length}
          {tr("webhooks.deliveriesCount")}
        </span>
        <button
          onclick={() => (showDeliveriesModal = false)}
          class="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600"
        >
          {tr("common.close")}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Test Result Modal -->
{#if showTestModal && editingWebhook}
  <div
    class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    onclick={(e) => {
      if (e.target === e.currentTarget) {
        showTestModal = false;
      }
    }}
    onkeydown={(e) => e.key === "Escape" && (showTestModal = false)}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-2xl max-w-lg w-full overflow-hidden"
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700"
      >
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
          {tr("webhooks.testResult")}
        </h2>
        <button
          aria-label="Close"
          onclick={() => (showTestModal = false)}
          class="p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 rounded-lg"
          ><i class="bi bi-x-lg" aria-hidden="true"></i></button
        >
      </div>

      <!-- Body -->
      <div class="p-4">
        {#if testLoading}
          <div class="flex items-center justify-center py-8">
            <div
              class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"
            ></div>
          </div>
        {:else if testResult}
          <div class="space-y-4">
            <div class="flex items-center gap-3">
              <div
                class="w-12 h-12 rounded-full flex items-center justify-center {testResult.success
                  ? 'bg-green-100 text-green-600 dark:bg-green-900 dark:text-green-400'
                  : 'bg-red-100 text-red-600 dark:bg-red-900 dark:text-red-400'}"
              >
                <i
                  class="bi {testResult.success
                    ? 'bi-check-lg'
                    : 'bi-x-lg'} text-2xl"
                ></i>
              </div>
              <div>
                <div class="font-semibold text-gray-900 dark:text-white">
                  {testResult.success
                    ? tr("webhooks.testSuccess")
                    : tr("webhooks.testFailed")}
                </div>
                <div class="text-sm text-gray-500 dark:text-gray-400">
                  {editingWebhook.name}
                </div>
              </div>
            </div>

            {#if testResult.status}
              <div
                class="grid grid-cols-2 gap-4 p-3 bg-gray-50 dark:bg-gray-900 rounded-lg"
              >
                <div>
                  <div class="text-xs text-gray-500 dark:text-gray-400">
                    {tr("webhooks.httpStatus")}
                  </div>
                  <div class="font-medium text-gray-900 dark:text-white">
                    HTTP {testResult.status}
                  </div>
                </div>
                <div>
                  <div class="text-xs text-gray-500 dark:text-gray-400">
                    {tr("webhooks.duration")}
                  </div>
                  <div class="font-medium text-gray-900 dark:text-white">
                    {formatDuration(testResult.duration_ms)}
                  </div>
                </div>
              </div>
            {/if}

            {#if testResult.error}
              <div
                class="p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-700 dark:text-red-200 text-sm"
              >
                {testResult.error}
              </div>
            {/if}

            {#if testResult.response}
              <div>
                <div
                  class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                >
                  {tr("webhooks.response")}
                </div>
                <pre
                  class="p-3 bg-gray-50 dark:bg-gray-900 rounded-lg text-xs overflow-x-auto max-h-32">{testResult.response}</pre>
              </div>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="flex items-center justify-end p-4 border-t border-gray-200 dark:border-gray-700"
      >
        <button
          onclick={() => (showTestModal = false)}
          class="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600"
        >
          {tr("common.close")}
        </button>
      </div>
    </div>
  </div>
{/if}
