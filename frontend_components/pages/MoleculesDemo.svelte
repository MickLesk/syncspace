<script lang="ts">
  import Card from "../atoms/Card.svelte";
  import Divider from "../atoms/Divider.svelte";
  import Chip from "../atoms/Chip.svelte";
  import Badge from "../atoms/Badge.svelte";
  import Button from "../atoms/Button.svelte";
  import Tabs from "../molecules/Tabs.svelte";
  import Dropdown from "../molecules/Dropdown.svelte";

  let activeTab = $state("overview");
  let enhancedTab = $state("profile");
  let gradientTab = $state("home");

  const tabs = [
    { id: "overview", label: "Overview", icon: "bi-house-fill" },
    { id: "analytics", label: "Analytics", icon: "bi-graph-up", badge: "12" },
    { id: "settings", label: "Settings", icon: "bi-gear-fill" },
  ];

  const enhancedTabs = [
    { id: "profile", label: "Profile", icon: "bi-person-fill" },
    { id: "settings", label: "Settings", icon: "bi-gear-fill", badge: "3" },
    { id: "messages", label: "Messages", icon: "bi-envelope-fill" },
    { id: "disabled", label: "Disabled", disabled: true },
  ];

  const gradientTabs = [
    { id: "home", label: "Home" },
    { id: "explore", label: "Explore" },
    { id: "bookmarks", label: "Bookmarks" },
    { id: "profile", label: "Profile" },
  ];

  const dropdownItems = [
    {
      id: "edit",
      label: "Edit",
      icon: "bi-pencil",
      action: () => console.log("Edit clicked"),
    },
    {
      id: "duplicate",
      label: "Duplicate",
      icon: "bi-files",
      action: () => console.log("Duplicate clicked"),
    },
    {
      id: "share",
      label: "Share",
      icon: "bi-share",
      action: () => console.log("Share clicked"),
    },
    { id: "divider", divider: true },
    {
      id: "delete",
      label: "Delete",
      icon: "bi-trash",
      variant: "danger",
      action: () => console.log("Delete clicked"),
    },
  ];
</script>

<div
  class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 p-8"
>
  <div class="max-w-7xl mx-auto space-y-8">
    <!-- Hero Section -->
    <div class="text-center space-y-4">
      <div
        class="inline-flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-blue-500 to-purple-500 text-white rounded-full text-sm font-semibold shadow-lg shadow-blue-500/30"
      >
        <i class="bi bi-grid-3x3 text-lg"></i>
        <span>Molecules Components</span>
      </div>
      <h1
        class="text-6xl font-black bg-gradient-to-r from-blue-600 via-purple-600 to-pink-600 bg-clip-text text-transparent"
      >
        Molecules
      </h1>
      <p class="text-xl text-gray-600 dark:text-gray-300 max-w-3xl mx-auto">
        Complex components built from multiple atoms with enhanced interactions
        and spring physics animations
      </p>
    </div>

    <Divider style="gradient" thickness={2} />

    <!-- Tabs Section -->
    <div class="space-y-6">
      <div>
        <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
          Tabs Component
        </h2>
        <p class="text-gray-600 dark:text-gray-400">
          Navigation tabs with smooth indicator animation
        </p>
      </div>

      <!-- Standard Tabs -->
      <Card shapeStyle="extra-rounded" elevationLevel={1}>
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <h3 class="text-xl font-bold text-gray-900 dark:text-white">
              Standard Tabs
            </h3>
            <Chip size="sm" gradient>Default</Chip>
          </div>

          <Tabs {tabs} bind:activeTab variant="default" size="md" />

          <div class="p-6 bg-gray-50 dark:bg-gray-800 rounded-xl">
            <p class="text-gray-600 dark:text-gray-400">
              Active Tab: <span class="font-bold text-gray-900 dark:text-white"
                >{activeTab}</span
              >
            </p>
          </div>
        </div>
      </Card>

      <!-- Enhanced Tabs with Spring Indicator -->
      <Card shapeStyle="extra-rounded" elevationLevel={2} gradient>
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <h3 class="text-xl font-bold text-gray-900 dark:text-white">
              Enhanced Spring Tabs
            </h3>
            <div class="flex gap-2">
              <Chip size="sm" gradient emphasized>Bouncy</Chip>
              <Chip size="sm" gradient emphasized>Spring Physics</Chip>
            </div>
          </div>

          <Tabs
            tabs={enhancedTabs}
            bind:activeTab={enhancedTab}
            variant="underline"
            size="lg"
            emphasized={true}
            motion="bouncy"
          />

          <div
            class="p-6 bg-gradient-to-br from-blue-50 to-purple-50 dark:from-blue-950/20 dark:to-purple-950/20 rounded-xl border border-blue-200 dark:border-blue-800"
          >
            <div class="flex items-start gap-3">
              <div
                class="w-12 h-12 bg-gradient-to-br from-blue-500 to-purple-600 rounded-xl flex items-center justify-center text-white text-2xl flex-shrink-0"
              >
                <i class="bi bi-stars"></i>
              </div>
              <div>
                <h4
                  class="text-lg font-bold text-gray-900 dark:text-white mb-2"
                >
                  {enhancedTab === "profile"
                    ? "Profile Content"
                    : enhancedTab === "settings"
                      ? "Settings Content"
                      : enhancedTab === "messages"
                        ? "Messages Content"
                        : "Content"}
                </h4>
                <p class="text-gray-600 dark:text-gray-400">
                  Enhanced features: <strong>Bold active tab</strong>, spring
                  indicator with bouncy physics, large size variant.
                </p>
                <div class="flex flex-wrap gap-2 mt-3">
                  <Badge variant="primary" size="sm" gradient>Emphasized</Badge>
                  <Badge variant="secondary" size="sm" gradient
                    >Bouncy Spring</Badge
                  >
                  <Badge variant="success" size="sm" gradient>Large Size</Badge>
                </div>
              </div>
            </div>
          </div>
        </div>
      </Card>

      <!-- Gradient Pill Tabs -->
      <Card shapeStyle="extra-rounded" elevationLevel={1}>
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <h3 class="text-xl font-bold text-gray-900 dark:text-white">
              Gradient Pills
            </h3>
            <Chip size="sm" gradient>Gradient</Chip>
          </div>

          <Tabs
            tabs={gradientTabs}
            bind:activeTab={gradientTab}
            variant="pills"
            size="md"
            gradient={true}
          />

          <div class="p-6 bg-gray-50 dark:bg-gray-800 rounded-xl">
            <p class="text-gray-600 dark:text-gray-400">
              Selected: <span class="font-bold text-gray-900 dark:text-white"
                >{gradientTab}</span
              >
            </p>
          </div>
        </div>
      </Card>
    </div>

    <Divider />

    <!-- Dropdown Section -->
    <div class="space-y-6">
      <div>
        <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
          Dropdown Component
        </h2>
        <p class="text-gray-600 dark:text-gray-400">
          Context menus with spring animations
        </p>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Standard Dropdown -->
        <Card shapeStyle="extra-rounded" elevationLevel={1}>
          <div class="space-y-4">
            <h3 class="text-lg font-bold text-gray-900 dark:text-white">
              Standard
            </h3>
            <Dropdown items={dropdownItems} label="Actions" size="md" />
          </div>
        </Card>

        <!-- Enhanced Dropdown -->
        <Card shapeStyle="extra-rounded" elevationLevel={2} gradient>
          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <h3 class="text-lg font-bold text-gray-900 dark:text-white">
                Enhanced
              </h3>
              <Chip size="xs" gradient>Spring</Chip>
            </div>
            <Dropdown
              items={dropdownItems}
              label="Enhanced Actions"
              size="lg"
              emphasized={true}
              motion="bouncy"
              shapeStyle="extra-rounded"
            />
          </div>
        </Card>

        <!-- Gradient Dropdown -->
        <Card shapeStyle="extra-rounded" elevationLevel={1}>
          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <h3 class="text-lg font-bold text-gray-900 dark:text-white">
                Gradient
              </h3>
              <Chip size="xs" gradient>Squircle</Chip>
            </div>
            <Dropdown
              items={dropdownItems}
              label="Squircle Menu"
              size="sm"
              gradient={true}
              shapeStyle="squircle"
            />
          </div>
        </Card>
      </div>

      <!-- Feature Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <Card shapeStyle="extra-rounded" elevationLevel={2} gradient>
          <div class="space-y-3">
            <div
              class="w-12 h-12 bg-gradient-to-br from-blue-500 to-blue-700 rounded-xl flex items-center justify-center text-white text-2xl"
            >
              <i class="bi bi-type-bold"></i>
            </div>
            <h3 class="text-xl font-bold text-gray-900 dark:text-white">
              Bold Labels
            </h3>
            <p class="text-gray-600 dark:text-gray-400 text-sm">
              Emphasized option labels with typographyEmphasized
            </p>
          </div>
        </Card>

        <Card shapeStyle="extra-rounded" elevationLevel={2} gradient>
          <div class="space-y-3">
            <div
              class="w-12 h-12 bg-gradient-to-br from-purple-500 to-purple-700 rounded-xl flex items-center justify-center text-white text-2xl"
            >
              <i class="bi bi-arrow-down-up"></i>
            </div>
            <h3 class="text-xl font-bold text-gray-900 dark:text-white">
              Spring Open
            </h3>
            <p class="text-gray-600 dark:text-gray-400 text-sm">
              Bouncy slide-in animation with spring physics
            </p>
          </div>
        </Card>

        <Card shapeStyle="extra-rounded" elevationLevel={2} gradient>
          <div class="space-y-3">
            <div
              class="w-12 h-12 bg-gradient-to-br from-pink-500 to-pink-700 rounded-xl flex items-center justify-center text-white text-2xl"
            >
              <i class="bi bi-bounding-box"></i>
            </div>
            <h3 class="text-xl font-bold text-gray-900 dark:text-white">
              Shape Variety
            </h3>
            <p class="text-gray-600 dark:text-gray-400 text-sm">
              13 shape variants including squircle
            </p>
          </div>
        </Card>

        <Card shapeStyle="extra-rounded" elevationLevel={2} gradient>
          <div class="space-y-3">
            <div
              class="w-12 h-12 bg-gradient-to-br from-green-500 to-green-700 rounded-xl flex items-center justify-center text-white text-2xl"
            >
              <i class="bi bi-palette"></i>
            </div>
            <h3 class="text-xl font-bold text-gray-900 dark:text-white">
              Gradient Hover
            </h3>
            <p class="text-gray-600 dark:text-gray-400 text-sm">
              Optional gradient hover effects on items
            </p>
          </div>
        </Card>
      </div>
    </div>

    <Divider />

    <!-- Coming Soon Section -->
    <Card shapeStyle="extra-rounded" elevationLevel={1}>
      <div class="text-center space-y-4 py-8">
        <div
          class="w-16 h-16 bg-gradient-to-br from-gray-400 to-gray-600 rounded-full mx-auto flex items-center justify-center text-white text-3xl"
        >
          <i class="bi bi-hourglass-split"></i>
        </div>
        <h3 class="text-2xl font-bold text-gray-900 dark:text-white">
          More Components Coming Soon
        </h3>
        <p class="text-gray-600 dark:text-gray-400 max-w-2xl mx-auto">
          Toast notifications, Breadcrumbs, Context menus, Tooltips, and more
          are being redesigned with modern Material Design 3 aesthetics
        </p>
        <div class="flex flex-wrap justify-center gap-2">
          <Chip size="sm" outlined>Toasts</Chip>
          <Chip size="sm" outlined>Breadcrumbs</Chip>
          <Chip size="sm" outlined>Context Menu</Chip>
          <Chip size="sm" outlined>Tooltips</Chip>
          <Chip size="sm" outlined>Drawer</Chip>
        </div>
      </div>
    </Card>
  </div>
</div>
