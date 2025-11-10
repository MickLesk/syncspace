<script lang="ts">
  import Button from "../atoms/Button.svelte";
  import Divider from "../atoms/Divider.svelte";
  import Badge from "../atoms/Badge.svelte";
  import Breadcrumbs from "../molecules/Breadcrumbs.svelte";
  import Toast from "../molecules/Toast.svelte";
  import Filter from "../molecules/Filter.svelte";
  import Select from "../molecules/Select.svelte";
  import ContextMenu from "../molecules/ContextMenu.svelte";
  import Tooltip from "../molecules/Tooltip.svelte";
  import Tabs from "../molecules/Tabs.svelte";
  import Accordion from "../molecules/Accordion.svelte";
  import Drawer from "../molecules/Drawer.svelte";
  import Dropdown from "../molecules/Dropdown.svelte";
  import EmptyState from "../molecules/EmptyState.svelte";
  import StatCard from "../molecules/StatCard.svelte";
  import Timeline from "../molecules/Timeline.svelte";
  import Stepper from "../molecules/Stepper.svelte";

  let toastComponent: Toast;
  let selectedFilters: string[] = [];
  let selectedOption = "option1";
  let contextMenuVisible = false;
  let contextX = 0;
  let contextY = 0;
  let activeTab = "profile";
  let accordion1Open = false;
  let accordion2Open = false;
  let drawerOpen = false;
  let dropdownOpen = false;
  let currentStep = 0;

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

  const tabs = [
    { id: "profile", label: "Profile", icon: "bi-person-fill" },
    { id: "settings", label: "Settings", icon: "bi-gear-fill", badge: "3" },
    { id: "messages", label: "Messages", icon: "bi-envelope-fill" },
    { id: "disabled", label: "Disabled", disabled: true },
  ];

  const selectOptions = [
    { value: "option1", label: "Option 1" },
    { value: "option2", label: "Option 2" },
        { value: "option3", label: "Option 3" },
    { value: "option4", label: "Option 4" },
  ];

  const dropdownItems = [
    { label: "Profile", icon: "bi-person-fill", action: () => alert("Profile clicked") },
    { label: "Settings", icon: "bi-gear-fill", action: () => alert("Settings clicked") },
    { divider: true },
    { label: "Logout", icon: "bi-box-arrow-right", danger: true, action: () => alert("Logout clicked") },
  ];

  const timelineItems = [
    {
      id: "1",
      title: "Project Started",
      description: "Initial project setup and configuration",
      timestamp: "2 hours ago",
      icon: "bi-play-circle-fill",
      variant: "success" as const,
    },
    {
      id: "2",
      title: "New Feature Added",
      description: "Implemented user authentication system",
      timestamp: "1 hour ago",
      icon: "bi-plus-circle-fill",
      variant: "primary" as const,
    },
    {
      id: "3",
      title: "Bug Fixed",
      description: "Resolved critical security vulnerability",
      timestamp: "30 minutes ago",
      icon: "bi-bug-fill",
      variant: "danger" as const,
    },
    {
      id: "4",
      title: "Deployment",
      description: "Production deployment completed successfully",
      timestamp: "10 minutes ago",
      icon: "bi-cloud-upload-fill",
      variant: "success" as const,
    },
  ];

  const stepperSteps = [
    { label: "Account", description: "Basic information", icon: "bi-person-fill" },
    { label: "Profile", description: "Personal details" },
    { label: "Verification", description: "Email & phone" },
    { label: "Complete", description: "Finish setup", icon: "bi-check-circle-fill" },
  ];

  const contextMenuItems = [
    { id: "edit", label: "Edit", icon: "bi-pencil-fill" },
    { id: "duplicate", label: "Duplicate", icon: "bi-files" },
    { id: "share", label: "Share", icon: "bi-share-fill" },
    { id: "divider", divider: true },
    { id: "delete", label: "Delete", icon: "bi-trash-fill", dangerous: true },
  ];

  function handleContextMenu(e: MouseEvent) {
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
              onclick={() => showToast("error", "Error! Something went wrong.")}
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

    <!-- Tooltips Section -->
    <section>
      <h2
        class="text-2xl font-bold text-slate-200 mb-6 flex items-center gap-3"
      >
        <i class="bi bi-chat-square-text text-purple-400"></i>Tooltips
      </h2>
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <div class="flex gap-8 items-center justify-center flex-wrap">
          <Tooltip position="top">
            {#snippet children()}
              <Button>Top Tooltip</Button>
            {/snippet}
            {#snippet content()}
              Tooltip on top
            {/snippet}
          </Tooltip>

          <Tooltip position="right">
            {#snippet children()}
              <Button>Right Tooltip</Button>
            {/snippet}
            {#snippet content()}
              Tooltip on right
            {/snippet}
          </Tooltip>

          <Tooltip position="bottom">
            {#snippet children()}
              <Button>Bottom Tooltip</Button>
            {/snippet}
            {#snippet content()}
              Tooltip on bottom
            {/snippet}
          </Tooltip>

          <Tooltip position="left">
            {#snippet children()}
              <Button>Left Tooltip</Button>
            {/snippet}
            {#snippet content()}
              Tooltip on left
            {/snippet}
          </Tooltip>
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Tabs Section -->
    <section>
      <h2
        class="text-2xl font-bold text-slate-200 mb-6 flex items-center gap-3"
      >
        <i class="bi bi-layout-three-columns text-blue-400"></i>Tabs
      </h2>

      <div class="space-y-6">
        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">
            Default Tabs
          </h3>
          <Tabs {tabs} bind:active={activeTab} variant="default">
            {#snippet children(tabId)}
              <div class="p-4 bg-slate-700/30 rounded-lg">
                <p class="text-slate-300">
                  Content for <strong>{tabId}</strong> tab
                </p>
              </div>
            {/snippet}
          </Tabs>
        </div>

        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">Pills Tabs</h3>
          <Tabs {tabs} variant="pills">
            {#snippet children(tabId)}
              <div class="p-4 bg-slate-700/30 rounded-lg">
                <p class="text-slate-300">
                  Pills content for <strong>{tabId}</strong>
                </p>
              </div>
            {/snippet}
          </Tabs>
        </div>

        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">
            Underline Tabs
          </h3>
          <Tabs {tabs} variant="underline">
            {#snippet children(tabId)}
              <div class="p-4">
                <p class="text-slate-300">
                  Underline content for <strong>{tabId}</strong>
                </p>
              </div>
            {/snippet}
          </Tabs>
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Accordion Section -->
    <section>
      <h2
        class="text-2xl font-bold text-slate-200 mb-6 flex items-center gap-3"
      >
        <i class="bi bi-list-nested text-green-400"></i>Accordion
      </h2>

      <div class="space-y-4">
        <Accordion
          bind:open={accordion1Open}
          title="What is SyncSpace?"
          icon="bi-question-circle"
          variant="bordered"
        >
          <p class="text-slate-300">
            SyncSpace is a modern file synchronization service with a Rust
            backend and Svelte 5 frontend. It provides secure, fast, and
            reliable file syncing across multiple devices.
          </p>
        </Accordion>

        <Accordion
          bind:open={accordion2Open}
          title="Component Features"
          icon="bi-star-fill"
          variant="filled"
        >
          <ul class="list-disc list-inside space-y-2 text-slate-300">
            <li>Svelte 5 runes and snippets</li>
            <li>TypeScript support</li>
            <li>Dark mode ready</li>
            <li>Fully accessible</li>
            <li>Smooth animations</li>
          </ul>
        </Accordion>

        <Accordion
          title="Installation Guide"
          icon="bi-download"
          variant="default"
        >
          <pre class="bg-slate-900 p-4 rounded text-green-400 text-sm">
npm install syncspace-components
          </pre>
        </Accordion>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Drawer Section -->
    <section>
      <h2
        class="text-2xl font-bold text-slate-200 mb-6 flex items-center gap-3"
      >
        <i class="bi bi-layout-sidebar text-orange-400"></i>Drawer
      </h2>

      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <div class="flex gap-4 flex-wrap">
          <Button onclick={() => (drawerOpen = true)}
            >Open Drawer (Right)</Button
          >
        </div>
      </div>

      <Drawer
        bind:open={drawerOpen}
        position="right"
        size="md"
        title="Drawer Example"
        closable
      >
        <div class="space-y-4">
          <p class="text-gray-700 dark:text-gray-300">
            This is a drawer component! It slides in from the side.
          </p>

          <div class="space-y-2">
            <h3 class="font-semibold text-gray-900 dark:text-gray-100">
              Features:
            </h3>
            <ul
              class="list-disc list-inside space-y-1 text-gray-700 dark:text-gray-300"
            >
              <li>4 positions (left, right, top, bottom)</li>
              <li>5 sizes (sm, md, lg, xl, full)</li>
              <li>Smooth slide animations</li>
              <li>Backdrop with blur effect</li>
              <li>Click outside to close</li>
            </ul>
          </div>

          <Button variant="primary" onclick={() => (drawerOpen = false)}>
            Close Drawer
          </Button>
        </div>
      </Drawer>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Dropdown Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-list mr-3 text-blue-400"></i>Dropdown
      </h2>

      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">Dropdown Menu</h3>
        <div class="flex gap-4">
          <Dropdown
            items={dropdownItems}
            bind:open={dropdownOpen}
            position="bottom-left"
          >
            {#snippet children()}
              <Button variant="primary">
                <i class="bi bi-three-dots-vertical mr-2"></i>
                Menu
              </Button>
            {/snippet}
          </Dropdown>
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Empty State Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-inbox mr-3 text-blue-400"></i>Empty State
      </h2>

      <div
        class="bg-slate-800/50 border border-slate-700 rounded-xl p-8 space-y-6"
      >
        <EmptyState
          icon="bi-inbox"
          title="No messages yet"
          description="Your inbox is empty. Start a conversation to see messages here."
          actionLabel="New Message"
          onaction={() => alert("New message clicked")}
        />

        <Divider variant="horizontal" color="slate" />

        <EmptyState
          variant="glass"
          icon="bi-search"
          title="No results found"
          description="Try adjusting your search criteria"
        />
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Stat Cards Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-bar-chart-fill mr-3 text-blue-400"></i>Stat Cards
      </h2>

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard
          title="Total Users"
          value="12,458"
          change={12}
          trend="up"
          icon="bi-people-fill"
          variant="primary"
        />
        <StatCard
          title="Revenue"
          value="$54,231"
          change={8}
          trend="up"
          icon="bi-cash-stack"
          variant="success"
        />
        <StatCard
          title="Active Sessions"
          value="2,847"
          change={-3}
          trend="down"
          icon="bi-activity"
          variant="warning"
        />
        <StatCard
          title="Error Rate"
          value="0.12%"
          change={-5}
          trend="down"
          icon="bi-exclamation-triangle-fill"
          variant="danger"
        />
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Timeline Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-clock-history mr-3 text-blue-400"></i>Timeline
      </h2>

      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-6">
          Activity Timeline
        </h3>
        <Timeline items={timelineItems} />
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Stepper Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-signpost-split mr-3 text-blue-400"></i>Stepper
      </h2>

      <div
        class="bg-slate-800/50 border border-slate-700 rounded-xl p-8 space-y-8"
      >
        <div>
          <h3 class="text-lg font-semibold text-slate-200 mb-6">
            Horizontal Stepper
          </h3>
          <Stepper
            steps={stepperSteps}
            bind:currentStep
            orientation="horizontal"
          />
          <div class="flex gap-3 mt-6">
            <Button
              variant="outline"
              onclick={() => (currentStep = Math.max(0, currentStep - 1))}
              disabled={currentStep === 0}
            >
              Previous
            </Button>
            <Button
              variant="primary"
              onclick={() =>
                (currentStep = Math.min(
                  stepperSteps.length - 1,
                  currentStep + 1
                ))}
              disabled={currentStep === stepperSteps.length - 1}
            >
              Next
            </Button>
          </div>
        </div>

        <Divider variant="horizontal" color="slate" />

        <div>
          <h3 class="text-lg font-semibold text-slate-200 mb-6">
            Vertical Stepper (Glass)
          </h3>
          <Stepper
            steps={stepperSteps}
            bind:currentStep
            orientation="vertical"
            variant="glass"
          />
        </div>
      </div>
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
