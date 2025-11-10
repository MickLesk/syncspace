<script lang="ts">
  import Button from "../atoms/Button.svelte";
  import Divider from "../atoms/Divider.svelte";
  import Badge from "../atoms/Badge.svelte";
  import Breadcrumbs from "../molecules/Breadcrumbs.svelte";
  import Toast from "../molecules/Toast.svelte";
  import Filter from "../molecules/Filter.svelte";
  import Select from "../molecules/Select.svelte";
  import ContextMenu from "../molecules/ContextMenu.svelte";

  let toastComponent: Toast;
  let selectedFilters: string[] = [];
  let selectedOption = "option1";
  let contextMenuVisible = false;
  let contextX = 0;
  let contextY = 0;

  const breadcrumbItems = [
    { label: "Home", href: "#" },
    { label: "Components", href: "#" },
    { label: "Molecules", href: "#" },
  ];

  const filterItems = [
    { id: "active", label: "Active" },
    { id: "archived", label: "Archived" },
    { id: "starred", label: "Starred" },
    { id: "shared", label: "Shared" },
  ];

  const selectOptions = [
    { value: "option1", label: "Option 1" },
    { value: "option2", label: "Option 2" },
    { value: "option3", label: "Option 3" },
    { value: "option4", label: "Option 4" },
  ];

  const contextMenuItems = [
    { id: "edit", label: "Edit", icon: "bi-pencil-fill" },
    { id: "duplicate", label: "Duplicate", icon: "bi-files" },
    { id: "share", label: "Share", icon: "bi-share-fill" },
    { id: "divider", divider: true },
    { id: "delete", label: "Delete", icon: "bi-trash-fill", dangerous: true },
  ];

  function showToast(
    type: "success" | "error" | "warning" | "info",
    message: string
  ) {
    toastComponent?.show(message, type, 3000);
  }

  function handleContextMenu(event: MouseEvent) {
    event.preventDefault();
    contextX = event.clientX;
    contextY = event.clientY;
    contextMenuVisible = true;
  }

  function handleContextMenuItem(id: string) {
    showToast("info", `Clicked: ${id}`);
    contextMenuVisible = false;
  }

  function handleFilterChange(items: string[]) {
    selectedFilters = items;
    showToast("info", `Filters selected: ${items.length}`);
  }
</script>

<div class="min-h-screen bg-slate-900 py-12">
  <div class="max-w-7xl mx-auto px-6">
    <!-- Header -->
    <div class="mb-12">
      <h1 class="text-4xl font-bold text-white mb-2">
        Molecules Component Demo
      </h1>
      <p class="text-slate-400">Complex combinations of atoms</p>
    </div>

    <!-- Breadcrumbs Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-signpost-split mr-3 text-blue-400"></i>Breadcrumbs
      </h2>

      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <Breadcrumbs items={breadcrumbItems} current="Molecules" />
        <p class="text-slate-400 text-sm mt-6">
          Breadcrumbs help users understand their location in the navigation
          hierarchy.
        </p>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Toast Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-chat-dots-fill mr-3 text-orange-400"></i>Toasts
      </h2>

      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <div class="space-y-4">
          <p class="text-slate-300 mb-6">
            Click buttons to trigger toast notifications at different positions:
          </p>

          <div class="grid grid-cols-3 gap-4">
            <Button
              size="sm"
              variant="success"
              onclick={() =>
                showToast("success", "Success! Operation completed.")}
            >
              <i class="bi bi-check-circle-fill mr-2"></i>Success
            </Button>
            <Button
              size="sm"
              variant="danger"
              onclick={() =>
                showToast("error", "Error! Something went wrong.")}
            >
              <i class="bi bi-exclamation-circle-fill mr-2"></i>Error
            </Button>
            <Button
              size="sm"
              variant="warning"
              onclick={() =>
                showToast("warning", "Warning! Please be careful.")}
            >
              <i class="bi bi-exclamation-triangle-fill mr-2"></i>Warning
            </Button>
          </div>

          <div>
            <Button
              size="sm"
              variant="primary"
              onclick={() =>
                showToast("info", "Info! This is an informational message.")}
            >
              <i class="bi bi-info-circle-fill mr-2"></i>Info
            </Button>
          </div>
        </div>
      </div>
    </section>

    <Toast bind:this={toastComponent} position="bottom-right" />

    <Divider variant="horizontal" color="slate" />

    <!-- Filter Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-funnel-fill mr-3 text-purple-400"></i>Filters
      </h2>

      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">
          Multi-Select Filter
        </h3>
        <Filter
          items={filterItems}
          selected={selectedFilters}
          onChange={handleFilterChange}
        />

        <div class="mt-6 pt-6 border-t border-slate-700">
          <p class="text-slate-400 text-sm">
            Selected filters:
            {#if selectedFilters.length > 0}
              <span class="ml-2">
                {#each selectedFilters as filter}
                  <Badge variant="primary" size="sm" class="ml-2"
                    >{filter}</Badge
                  >
                {/each}
              </span>
            {:else}
              <span class="text-slate-500 italic">None selected</span>
            {/if}
          </p>
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Select Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-dropdown mr-3 text-green-400"></i>Select Dropdown
      </h2>

      <div class="grid md:grid-cols-2 gap-8">
        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">
            Basic Select
          </h3>
          <Select
            items={selectOptions}
            bind:selected={selectedOption}
            placeholder="Choose an option..."
          />
          <p class="text-slate-400 text-sm mt-4">Selected: {selectedOption}</p>
        </div>

        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">
            Disabled Select
          </h3>
          <Select
            items={selectOptions}
            disabled
            placeholder="This is disabled..."
          />
          <p class="text-slate-400 text-sm mt-4">
            Disabled state prevents interaction
          </p>
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Context Menu Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-menu-button-wide-fill mr-3 text-pink-400"></i>Context
        Menu
      </h2>

      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <p class="text-slate-300 mb-6">
          Right-click on the box below to open a context menu:
        </p>

        <div
          oncontextmenu={handleContextMenu}
          class="bg-slate-700/50 border-2 border-dashed border-slate-600 rounded-lg p-12 text-center cursor-context-menu hover:border-slate-500 transition-colors"
        >
          <i class="bi bi-mouse text-4xl text-slate-400 mb-4"></i>
          <p class="text-slate-400">Right-click here</p>
        </div>
      </div>

      <ContextMenu
        visible={contextMenuVisible}
        x={contextX}
        y={contextY}
        items={contextMenuItems}
        on:close={() => (contextMenuVisible = false)}
        on:select={(e) => handleContextMenuItem(e.detail)}
      />
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Info Box -->
    <div
      class="bg-blue-500/10 border border-blue-500/30 rounded-xl p-6 text-slate-300"
    >
      <i class="bi bi-info-circle text-blue-400 mr-2"></i>
      <strong>Tip:</strong> Molecules combine multiple atoms to create more complex
      UI patterns. They handle more sophisticated interactions and state management.
    </div>
  </div>
</div>
