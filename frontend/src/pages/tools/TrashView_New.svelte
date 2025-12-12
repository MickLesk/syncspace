<script>
  import { onMount } from "svelte";
  import { currentLang } from "../../../stores/ui.js";
  import t from "../../../i18n.js";
  import api from "../../../lib/api.js";
  import { formatFileSize, formatDate } from "../../../lib/utils.js";

  // Standard UI Components
  import StandardGlassCard from "../../../components/ui/StandardGlassCard.svelte";
  import StandardButton from "../../../components/ui/StandardButton.svelte";
  import StandardModal from "../../../components/ui/StandardModal.svelte";

  // State with Svelte 5 runes
  let trashItems = $state([]);
  let loading = $state(true);
  let error = $state(null);
  let selectedItems = $state(new Set());
  let searchQuery = $state("");
  let sortBy = $state("deleted_at");
  let sortOrder = $state("desc");
  let showRestoreModal = $state(false);
  let showDeleteModal = $state(false);
  let actionLoading = $state(false);

  // Computed values
  const filteredItems = $derived(
    trashItems
      .filter((item) => {
        if (!searchQuery) return true;
        return item.file_name
          ?.toLowerCase()
          .includes(searchQuery.toLowerCase());
      })
      .sort((a, b) => {
        const order = sortOrder === "asc" ? 1 : -1;
        if (sortBy === "deleted_at") {
          return (
            order *
            (new Date(a.deleted_at).getTime() -
              new Date(b.deleted_at).getTime())
          );
        } else if (sortBy === "file_name") {
          return order * (a.file_name || "").localeCompare(b.file_name || "");
        } else if (sortBy === "file_size") {
          return order * ((a.file_size || 0) - (b.file_size || 0));
        }
        return 0;
      })
  );

  const totalSize = $derived(
    trashItems.reduce((sum, item) => sum + (item.file_size || 0), 0)
  );

  const selectedCount = $derived(selectedItems.size);

  onMount(async () => {
    await loadTrashItems();
  });

  async function loadTrashItems() {
    loading = true;
    error = null;

    try {
      const response = await api.trash.list();
      trashItems = response || [];
    } catch (err) {
      console.error("Failed to load trash items:", err);
      error = t("trash.loadError") || "Fehler beim Laden des Papierkorbs";
    } finally {
      loading = false;
    }
  }

  function toggleSelectAll() {
    if (selectedItems.size === filteredItems.length) {
      selectedItems.clear();
    } else {
      selectedItems = new Set(filteredItems.map((item) => item.id));
    }
  }

  function toggleSelectItem(itemId) {
    if (selectedItems.has(itemId)) {
      selectedItems.delete(itemId);
    } else {
      selectedItems.add(itemId);
    }
    selectedItems = selectedItems; // Trigger reactivity
  }

  async function handleBulkRestore() {
    if (selectedCount === 0) return;
    showRestoreModal = true;
  }

  async function confirmBulkRestore() {
    actionLoading = true;

    try {
      const itemsToRestore = Array.from(selectedItems);
      await Promise.all(itemsToRestore.map((id) => api.trash.restore(id)));

      selectedItems.clear();
      await loadTrashItems();
      showRestoreModal = false;
    } catch (err) {
      console.error("Failed to restore items:", err);
      error = t("trash.restoreError") || "Fehler beim Wiederherstellen";
    } finally {
      actionLoading = false;
    }
  }

  async function handleBulkDelete() {
    if (selectedCount === 0) return;
    showDeleteModal = true;
  }

  async function confirmBulkDelete() {
    actionLoading = true;

    try {
      const itemsToDelete = Array.from(selectedItems);
      await Promise.all(
        itemsToDelete.map((id) => api.trash.permanentDelete(id))
      );

      selectedItems.clear();
      await loadTrashItems();
      showDeleteModal = false;
    } catch (err) {
      console.error("Failed to delete items:", err);
      error = t("trash.deleteError") || "Fehler beim endgültigen Löschen";
    } finally {
      actionLoading = false;
    }
  }

  async function restoreItem(itemId) {
    try {
      await api.trash.restore(itemId);
      await loadTrashItems();
    } catch (err) {
      console.error("Failed to restore item:", err);
      error = t("trash.restoreError") || "Fehler beim Wiederherstellen";
    }
  }

  async function deleteItemPermanently(itemId) {
    if (!confirm(t("trash.confirmDelete") || "Datei endgültig löschen?"))
      return;

    try {
      await api.trash.permanentDelete(itemId);
      await loadTrashItems();
    } catch (err) {
      console.error("Failed to delete item:", err);
      error = t("trash.deleteError") || "Fehler beim endgültigen Löschen";
    }
  }

  async function emptyTrash() {
    if (
      !confirm(t("trash.confirmEmptyTrash") || "Papierkorb vollständig leeren?")
    )
      return;

    actionLoading = true;
    try {
      await api.trash.empty();
      await loadTrashItems();
    } catch (err) {
      console.error("Failed to empty trash:", err);
      error = t("trash.emptyError") || "Fehler beim Leeren des Papierkorbs";
    } finally {
      actionLoading = false;
    }
  }

  // Modal actions with reactive disabled state
  const restoreModalActions = $derived([
    {
      label: t("common.cancel") || "Abbrechen",
      variant: "default",
      onClick: () => (showRestoreModal = false),
      disabled: actionLoading,
    },
    {
      label: t("trash.restore") || "Wiederherstellen",
      icon: "arrow-counterclockwise",
      variant: "primary",
      onClick: confirmBulkRestore,
      disabled: actionLoading,
    },
  ]);

  const deleteModalActions = $derived([
    {
      label: t("common.cancel") || "Abbrechen",
      variant: "default",
      onClick: () => (showDeleteModal = false),
      disabled: actionLoading,
    },
    {
      label: t("trash.deletePermanently") || "Endgültig löschen",
      icon: "trash",
      variant: "danger",
      onClick: confirmBulkDelete,
      disabled: actionLoading,
    },
  ]);
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
              class="bi bi-trash text-red-600 dark:text-red-400 mr-3"
              aria-hidden="true"
            ></i>
            {t("trash.title") || "Papierkorb"}
          </h1>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            {t("trash.subtitle") ||
              "Gelöschte Dateien verwalten und wiederherstellen"}
          </p>
        </div>

        <!-- Action Buttons -->
        <div class="flex items-center space-x-2">
          <StandardButton
            variant="default"
            icon="arrow-clockwise"
            onclick={loadTrashItems}
            disabled={loading}
          >
            {t("common.refresh") || "Aktualisieren"}
          </StandardButton>

          <StandardButton
            variant="danger"
            icon="trash"
            onclick={emptyTrash}
            disabled={loading || trashItems.length === 0 || actionLoading}
          >
            {t("trash.emptyTrash") || "Papierkorb leeren"}
          </StandardButton>
        </div>
      </div>
    </div>

    <!-- Stats Cards -->
    <div class="grid-3 grid-gap mb-6">
      <StandardGlassCard
        title={t("trash.itemCount") || "Anzahl Dateien"}
        icon="file-earmark"
        padding="p-4"
      >
        <div class="text-2xl font-bold text-blue-600 dark:text-blue-400">
          {trashItems.length}
        </div>
        <p class="text-caption mt-1">
          {t("trash.itemsInTrash") || "Dateien im Papierkorb"}
        </p>
      </StandardGlassCard>

      <StandardGlassCard
        title={t("trash.totalSize") || "Gesamtgröße"}
        icon="hdd"
        padding="p-4"
      >
        <div class="text-2xl font-bold text-red-600 dark:text-red-400">
          {formatFileSize(totalSize)}
        </div>
        <p class="text-caption mt-1">
          {t("trash.spaceUsed") || "Verwendeter Speicherplatz"}
        </p>
      </StandardGlassCard>

      <StandardGlassCard
        title={t("trash.selectedCount") || "Ausgewählt"}
        icon="check-circle"
        padding="p-4"
      >
        <div class="text-2xl font-bold text-green-600 dark:text-green-400">
          {selectedCount}
        </div>
        <p class="text-caption mt-1">
          {t("trash.selectedItems") || "Ausgewählte Dateien"}
        </p>
      </StandardGlassCard>
    </div>

    <!-- Main Content -->
    <StandardGlassCard {loading} {error}>
      <!-- Controls -->
      <div
        class="flex flex-col sm:flex-row gap-4 items-start sm:items-center justify-between mb-6"
      >
        <!-- Search -->
        <div class="flex-1 max-w-md">
          <div class="relative">
            <input
              type="text"
              placeholder={t("trash.searchPlaceholder") ||
                "Dateien durchsuchen..."}
              bind:value={searchQuery}
              class="form-input pl-10"
            />
            <i
              class="bi bi-search absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400"
              aria-hidden="true"
            ></i>
          </div>
        </div>

        <!-- Sort Controls -->
        <div class="flex items-center space-x-2">
          <select bind:value={sortBy} class="form-input">
            <option value="deleted_at"
              >{t("trash.sortByDate") || "Nach Datum"}</option
            >
            <option value="file_name"
              >{t("trash.sortByName") || "Nach Name"}</option
            >
            <option value="file_size"
              >{t("trash.sortBySize") || "Nach Größe"}</option
            >
          </select>

          <StandardButton
            variant="ghost"
            icon={sortOrder === "asc" ? "sort-up" : "sort-down"}
            iconPosition="only"
            onclick={() => (sortOrder = sortOrder === "asc" ? "desc" : "asc")}
            aria-label="Sortierreihenfolge ändern"
          />
        </div>
      </div>

      <!-- Bulk Actions -->
      {#if selectedCount > 0}
        <div
          class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4 mb-6"
        >
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <i
                class="bi bi-check-circle text-blue-600 dark:text-blue-400"
                aria-hidden="true"
              ></i>
              <span class="text-body">
                {selectedCount}
                {t("trash.itemsSelected") || "Dateien ausgewählt"}
              </span>
            </div>

            <div class="flex items-center space-x-2">
              <StandardButton
                variant="primary"
                icon="arrow-counterclockwise"
                onclick={handleBulkRestore}
                disabled={actionLoading}
              >
                {t("trash.restoreSelected") || "Wiederherstellen"}
              </StandardButton>

              <StandardButton
                variant="danger"
                icon="trash"
                onclick={handleBulkDelete}
                disabled={actionLoading}
              >
                {t("trash.deleteSelected") || "Endgültig löschen"}
              </StandardButton>
            </div>
          </div>
        </div>
      {/if}

      <!-- File List -->
      {#if filteredItems.length > 0}
        <div class="space-y-2">
          <!-- Header Row -->
          <div
            class="flex items-center p-3 bg-gray-50 dark:bg-slate-700/50 rounded-lg"
          >
            <label class="flex items-center mr-4">
              <input
                type="checkbox"
                checked={selectedCount === filteredItems.length &&
                  filteredItems.length > 0}
                onchange={toggleSelectAll}
                class="form-input w-4 h-4 mr-2"
              />
              <span class="text-caption"
                >{t("common.selectAll") || "Alle auswählen"}</span
              >
            </label>

            <div class="flex-1 grid grid-cols-3 gap-4">
              <span class="text-caption font-medium"
                >{t("common.fileName") || "Dateiname"}</span
              >
              <span class="text-caption font-medium"
                >{t("common.size") || "Größe"}</span
              >
              <span class="text-caption font-medium"
                >{t("trash.deletedAt") || "Gelöscht am"}</span
              >
            </div>

            <div class="w-24 text-caption font-medium text-right">
              {t("common.actions") || "Aktionen"}
            </div>
          </div>

          <!-- File Rows -->
          {#each filteredItems as item}
            <div class="glass-card-hover p-3">
              <div class="flex items-center">
                <label class="flex items-center mr-4">
                  <input
                    type="checkbox"
                    checked={selectedItems.has(item.id)}
                    onchange={() => toggleSelectItem(item.id)}
                    class="form-input w-4 h-4"
                  />
                </label>

                <div class="flex-1 grid grid-cols-3 gap-4 items-center">
                  <!-- File Name -->
                  <div class="flex items-center space-x-3">
                    <i
                      class="bi bi-file-earmark text-gray-400"
                      aria-hidden="true"
                    ></i>
                    <span class="text-body truncate" title={item.file_name}>
                      {item.file_name ||
                        t("common.unknownFile") ||
                        "Unbekannte Datei"}
                    </span>
                  </div>

                  <!-- File Size -->
                  <span class="text-body">
                    {formatFileSize(item.file_size || 0)}
                  </span>

                  <!-- Deleted Date -->
                  <span class="text-caption">
                    {formatDate(item.deleted_at)}
                  </span>
                </div>

                <!-- Actions -->
                <div class="flex items-center space-x-1 w-24 justify-end">
                  <StandardButton
                    variant="ghost"
                    icon="arrow-counterclockwise"
                    iconPosition="only"
                    size="xs"
                    onclick={() => restoreItem(item.id)}
                    aria-label="Datei wiederherstellen"
                  />

                  <StandardButton
                    variant="ghost"
                    icon="trash"
                    iconPosition="only"
                    size="xs"
                    onclick={() => deleteItemPermanently(item.id)}
                    aria-label="Datei endgültig löschen"
                  />
                </div>
              </div>
            </div>
          {/each}
        </div>
      {:else if !loading}
        <!-- Empty State -->
        <div class="text-center py-12">
          <i class="bi bi-trash text-6xl text-gray-400 mb-4" aria-hidden="true"
          ></i>
          <h3 class="text-subheading mb-2">
            {searchQuery
              ? t("trash.noSearchResults") || "Keine Suchergebnisse"
              : t("trash.emptyTrash") || "Papierkorb ist leer"}
          </h3>
          <p class="text-caption">
            {searchQuery
              ? t("trash.tryDifferentSearch") ||
                "Versuchen Sie eine andere Suche"
              : t("trash.noDeletedFiles") ||
                "Keine gelöschten Dateien vorhanden"}
          </p>
        </div>
      {/if}
    </StandardGlassCard>
  </div>
</div>

<!-- Restore Confirmation Modal -->
<StandardModal
  bind:show={showRestoreModal}
  title={t("trash.confirmRestore") || "Dateien wiederherstellen"}
  size="md"
  actions={restoreModalActions}
  loading={actionLoading}
>
  <div class="space-y-4">
    <p class="text-body">
      {t("trash.restoreConfirmMessage") ||
        "Möchten Sie die ausgewählten Dateien wiederherstellen?"}
    </p>
    <div
      class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-3"
    >
      <p class="text-caption">
        <strong>{selectedCount}</strong>
        {t("trash.itemsWillBeRestored") || "Dateien werden wiederhergestellt"}
      </p>
    </div>
  </div>
</StandardModal>

<!-- Delete Confirmation Modal -->
<StandardModal
  bind:show={showDeleteModal}
  title={t("trash.confirmPermanentDelete") || "Dateien endgültig löschen"}
  size="md"
  actions={deleteModalActions}
  loading={actionLoading}
>
  <div class="space-y-4">
    <div class="flex items-start space-x-3">
      <i
        class="bi bi-exclamation-triangle text-red-600 dark:text-red-400 text-xl mt-1"
        aria-hidden="true"
      ></i>
      <div>
        <p class="text-body font-medium mb-2">
          {t("trash.permanentDeleteWarning") ||
            "Diese Aktion kann nicht rückgängig gemacht werden!"}
        </p>
        <p class="text-caption">
          {t("trash.permanentDeleteDescription") ||
            "Die ausgewählten Dateien werden endgültig gelöscht und können nicht wiederhergestellt werden."}
        </p>
      </div>
    </div>
    <div
      class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-3"
    >
      <p class="text-caption">
        <strong>{selectedCount}</strong>
        {t("trash.itemsWillBeDeleted") || "Dateien werden endgültig gelöscht"}
      </p>
    </div>
  </div>
</StandardModal>
