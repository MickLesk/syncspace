<script lang="ts">
  import Button from "../atoms/Button.svelte";
  import Card from "../atoms/Card.svelte";
  import Badge from "../atoms/Badge.svelte";
  import Avatar from "../atoms/Avatar.svelte";
  import Input from "../atoms/Input.svelte";
  import Toggle from "../atoms/Toggle.svelte";
  import Checkbox from "../atoms/Checkbox.svelte";
  import Modal from "../organisms/Modal.svelte";
  import Tabs from "../molecules/Tabs.svelte";
  import Dropdown from "../molecules/Dropdown.svelte";
  import { typographyEmphasized } from "../shared/index.js";

  let selectedComponent = $state("button");

  // Button props
  let buttonVariant = $state("primary");
  let buttonSize = $state("md");
  let buttonEmphasized = $state(false);
  let buttonShapeStyle = $state("large");
  let buttonMotion = $state("bouncy");
  let buttonDisabled = $state(false);
  let buttonLoading = $state(false);

  // Card props
  let cardSurfaceTone = $state("base");
  let cardElevationLevel = $state(1);
  let cardEmphasized = $state(false);
  let cardShapeStyle = $state("large");
  let cardGlass = $state(false);

  // Modal props
  let modalOpen = $state(false);
  let modalEmphasized = $state(false);
  let modalHeroMoment = $state(false);
  let modalShapeStyle = $state("extra-large");
  let modalSize = $state("md");

  // Tabs props
  let tabsVariant = $state("default");
  let tabsEmphasized = $state(false);
  let tabsGradient = $state(false);
  let tabsSize = $state("md");
  let activeTab = $state("profile");

  // Dropdown props
  let dropdownOpen = $state(false);
  let dropdownEmphasized = $state(false);
  let dropdownGradient = $state(false);
  let dropdownSize = $state("md");

  const components = [
    { id: "button", label: "Button", icon: "bi-button" },
    { id: "card", label: "Card", icon: "bi-card-heading" },
    { id: "modal", label: "Modal", icon: "bi-window" },
    { id: "tabs", label: "Tabs", icon: "bi-layout-three-columns" },
    { id: "dropdown", label: "Dropdown", icon: "bi-list" },
  ];

  const tabs = [
    { id: "profile", label: "Profile", icon: "bi-person-fill" },
    { id: "settings", label: "Settings", icon: "bi-gear-fill" },
    { id: "messages", label: "Messages", icon: "bi-envelope-fill" },
  ];

  const dropdownItems = [
    { id: "profile", label: "Profile", icon: "bi-person-fill" },
    { id: "settings", label: "Settings", icon: "bi-gear-fill" },
    { divider: true },
    { id: "logout", label: "Logout", icon: "bi-box-arrow-right", danger: true },
  ];

  function generateCode() {
    switch (selectedComponent) {
      case "button":
        return `<Button
  variant="${buttonVariant}"
  size="${buttonSize}"
  ${buttonEmphasized ? "emphasized={true}" : ""}
  ${buttonShapeStyle !== "large" ? `shapeStyle="${buttonShapeStyle}"` : ""}
  ${buttonMotion !== "bouncy" ? `motion="${buttonMotion}"` : ""}
  ${buttonDisabled ? "disabled={true}" : ""}
  ${buttonLoading ? "loading={true}" : ""}
>
  Click Me!
</Button>`;

      case "card":
        return `<Card
  surfaceTone="${cardSurfaceTone}"
  elevationLevel={${cardElevationLevel}}
  ${cardEmphasized ? "emphasized={true}" : ""}
  ${cardShapeStyle !== "large" ? `shapeStyle="${cardShapeStyle}"` : ""}
  ${cardGlass ? "glass={true}" : ""}
>
  Card Content
</Card>`;

      case "modal":
        return `<Modal
  bind:open={modalOpen}
  title="Modal Title"
  size="${modalSize}"
  ${modalEmphasized ? "emphasized={true}" : ""}
  ${modalHeroMoment ? "heroMoment={true}" : ""}
  ${modalShapeStyle !== "extra-large" ? `shapeStyle="${modalShapeStyle}"` : ""}
>
  Modal Content
</Modal>`;

      case "tabs":
        return `<Tabs
  {tabs}
  variant="${tabsVariant}"
  ${tabsEmphasized ? "emphasized={true}" : ""}
  ${tabsGradient ? "gradient={true}" : ""}
  size="${tabsSize}"
/>`;

      case "dropdown":
        return `<Dropdown
  {items}
  ${dropdownEmphasized ? "emphasized={true}" : ""}
  ${dropdownGradient ? "gradient={true}" : ""}
  size="${dropdownSize}"
>
  {#snippet children()}
    Open Menu
  {/snippet}
</Dropdown>`;

      default:
        return "";
    }
  }

  function copyCode() {
    navigator.clipboard.writeText(generateCode());
    alert("Code copied to clipboard!");
  }
</script>

<div
  class="min-h-screen bg-gradient-to-br from-slate-900 via-blue-900 to-purple-900 py-12"
>
  <div class="max-w-[1600px] mx-auto px-6">
    <!-- Header -->
    <div class="mb-8">
      <div
        class="inline-flex items-center gap-3 px-4 py-2 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-full shadow-lg mb-4"
      >
        <i class="bi bi-code-slash text-xl"></i>
        <span class="text-sm font-bold">INTERACTIVE PLAYGROUND</span>
      </div>
      <h1 class={`${typographyEmphasized.display.large} text-white mb-2`}>
        Component Playground
      </h1>
      <p class="text-slate-400 text-lg">
        Experiment with component props in real-time
      </p>
    </div>

    <div class="grid lg:grid-cols-12 gap-6">
      <!-- Component Selector Sidebar -->
      <div class="lg:col-span-3">
        <Card
          emphasized
          shapeStyle="extra-rounded"
          surfaceTone="container"
          elevationLevel={2}
        >
          <h3 class="text-white font-bold text-lg mb-4">
            <i class="bi bi-box mr-2"></i>Components
          </h3>
          <div class="space-y-2">
            {#each components as component}
              <button
                onclick={() => (selectedComponent = component.id)}
                class={`
                  w-full flex items-center gap-3 px-4 py-3 rounded-xl
                  transition-all duration-300
                  ${
                    selectedComponent === component.id
                      ? "bg-gradient-to-r from-blue-600 to-purple-600 text-white shadow-lg scale-105"
                      : "bg-slate-700/30 text-slate-300 hover:bg-slate-700/50"
                  }
                `}
              >
                <i class={`bi ${component.icon} text-xl`}></i>
                <span class="font-medium">{component.label}</span>
              </button>
            {/each}
          </div>
        </Card>
      </div>

      <!-- Props Controls -->
      <div class="lg:col-span-4">
        <Card
          emphasized
          shapeStyle="extra-rounded"
          surfaceTone="container"
          elevationLevel={2}
        >
          <h3 class="text-white font-bold text-lg mb-4">
            <i class="bi bi-sliders mr-2"></i>Props Controls
          </h3>

          <div class="space-y-4 max-h-[600px] overflow-y-auto pr-2">
            {#if selectedComponent === "button"}
              <div>
                <label class="block text-slate-300 text-sm font-bold mb-2"
                  >Variant</label
                >
                <select
                  bind:value={buttonVariant}
                  class="w-full px-3 py-2 bg-slate-700 text-white rounded-lg border border-slate-600"
                >
                  <option value="primary">Primary</option>
                  <option value="secondary">Secondary</option>
                  <option value="success">Success</option>
                  <option value="danger">Danger</option>
                  <option value="warning">Warning</option>
                </select>
              </div>

              <div>
                <label class="block text-slate-300 text-sm font-bold mb-2"
                  >Size</label
                >
                <select
                  bind:value={buttonSize}
                  class="w-full px-3 py-2 bg-slate-700 text-white rounded-lg border border-slate-600"
                >
                  <option value="xs">Extra Small</option>
                  <option value="sm">Small</option>
                  <option value="md">Medium</option>
                  <option value="lg">Large</option>
                  <option value="xl">Extra Large</option>
                </select>
              </div>

              <div>
                <label class="block text-slate-300 text-sm font-bold mb-2"
                  >Shape Style</label
                >
                <select
                  bind:value={buttonShapeStyle}
                  class="w-full px-3 py-2 bg-slate-700 text-white rounded-lg border border-slate-600"
                >
                  <option value="large">Large (8px)</option>
                  <option value="extra-rounded">Extra Rounded (20px)</option>
                  <option value="extra-large">Extra Large (28px)</option>
                  <option value="squircle-md">Squircle MD</option>
                </select>
              </div>

              <div>
                <label class="block text-slate-300 text-sm font-bold mb-2"
                  >Motion</label
                >
                <select
                  bind:value={buttonMotion}
                  class="w-full px-3 py-2 bg-slate-700 text-white rounded-lg border border-slate-600"
                >
                  <option value="bouncy">Bouncy</option>
                  <option value="smooth">Smooth</option>
                  <option value="gentle">Gentle</option>
                  <option value="energetic">Energetic</option>
                </select>
              </div>

              <div class="flex items-center gap-3">
                <Toggle bind:checked={buttonEmphasized} />
                <label class="text-slate-300 text-sm">Emphasized</label>
              </div>

              <div class="flex items-center gap-3">
                <Toggle bind:checked={buttonDisabled} />
                <label class="text-slate-300 text-sm">Disabled</label>
              </div>

              <div class="flex items-center gap-3">
                <Toggle bind:checked={buttonLoading} />
                <label class="text-slate-300 text-sm">Loading</label>
              </div>
            {:else if selectedComponent === "card"}
              <div>
                <label class="block text-slate-300 text-sm font-bold mb-2"
                  >Surface Tone</label
                >
                <select
                  bind:value={cardSurfaceTone}
                  class="w-full px-3 py-2 bg-slate-700 text-white rounded-lg border border-slate-600"
                >
                  <option value="base">Base</option>
                  <option value="container">Container</option>
                  <option value="surface">Surface</option>
                  <option value="primary">Primary</option>
                  <option value="secondary">Secondary</option>
                </select>
              </div>

              <div>
                <label class="block text-slate-300 text-sm font-bold mb-2"
                  >Elevation Level: {cardElevationLevel}</label
                >
                <input
                  type="range"
                  min="0"
                  max="5"
                  bind:value={cardElevationLevel}
                  class="w-full"
                />
              </div>

              <div>
                <label class="block text-slate-300 text-sm font-bold mb-2"
                  >Shape Style</label
                >
                <select
                  bind:value={cardShapeStyle}
                  class="w-full px-3 py-2 bg-slate-700 text-white rounded-lg border border-slate-600"
                >
                  <option value="large">Large (8px)</option>
                  <option value="extra-rounded">Extra Rounded (20px)</option>
                  <option value="extra-large">Extra Large (28px)</option>
                  <option value="extra-extra-large"
                    >Extra Extra Large (32px)</option
                  >
                </select>
              </div>

              <div class="flex items-center gap-3">
                <Toggle bind:checked={cardEmphasized} />
                <label class="text-slate-300 text-sm">Emphasized</label>
              </div>

              <div class="flex items-center gap-3">
                <Toggle bind:checked={cardGlass} />
                <label class="text-slate-300 text-sm">Glassmorphism</label>
              </div>
            {:else if selectedComponent === "modal"}
              <div>
                <label class="block text-slate-300 text-sm font-bold mb-2"
                  >Size</label
                >
                <select
                  bind:value={modalSize}
                  class="w-full px-3 py-2 bg-slate-700 text-white rounded-lg border border-slate-600"
                >
                  <option value="sm">Small</option>
                  <option value="md">Medium</option>
                  <option value="lg">Large</option>
                  <option value="xl">Extra Large</option>
                  <option value="2xl">2X Large</option>
                </select>
              </div>

              <div>
                <label class="block text-slate-300 text-sm font-bold mb-2"
                  >Shape Style</label
                >
                <select
                  bind:value={modalShapeStyle}
                  class="w-full px-3 py-2 bg-slate-700 text-white rounded-lg border border-slate-600"
                >
                  <option value="extra-large">Extra Large (28px)</option>
                  <option value="extra-extra-large"
                    >Extra Extra Large (32px)</option
                  >
                  <option value="extra-extra-extra-large"
                    >Extra Extra Extra Large (40px)</option
                  >
                </select>
              </div>

              <div class="flex items-center gap-3">
                <Toggle bind:checked={modalEmphasized} />
                <label class="text-slate-300 text-sm">Emphasized</label>
              </div>

              <div class="flex items-center gap-3">
                <Toggle bind:checked={modalHeroMoment} />
                <label class="text-slate-300 text-sm">Hero Moment</label>
              </div>

              <Button
                onclick={() => (modalOpen = true)}
                variant="primary"
                class="w-full"
              >
                Open Modal
              </Button>
            {:else if selectedComponent === "tabs"}
              <div>
                <label class="block text-slate-300 text-sm font-bold mb-2"
                  >Variant</label
                >
                <select
                  bind:value={tabsVariant}
                  class="w-full px-3 py-2 bg-slate-700 text-white rounded-lg border border-slate-600"
                >
                  <option value="default">Default</option>
                  <option value="pills">Pills</option>
                  <option value="underline">Underline</option>
                </select>
              </div>

              <div>
                <label class="block text-slate-300 text-sm font-bold mb-2"
                  >Size</label
                >
                <select
                  bind:value={tabsSize}
                  class="w-full px-3 py-2 bg-slate-700 text-white rounded-lg border border-slate-600"
                >
                  <option value="sm">Small</option>
                  <option value="md">Medium</option>
                  <option value="lg">Large</option>
                </select>
              </div>

              <div class="flex items-center gap-3">
                <Toggle bind:checked={tabsEmphasized} />
                <label class="text-slate-300 text-sm">Emphasized</label>
              </div>

              <div class="flex items-center gap-3">
                <Toggle bind:checked={tabsGradient} />
                <label class="text-slate-300 text-sm">Gradient</label>
              </div>
            {:else if selectedComponent === "dropdown"}
              <div>
                <label class="block text-slate-300 text-sm font-bold mb-2"
                  >Size</label
                >
                <select
                  bind:value={dropdownSize}
                  class="w-full px-3 py-2 bg-slate-700 text-white rounded-lg border border-slate-600"
                >
                  <option value="sm">Small</option>
                  <option value="md">Medium</option>
                  <option value="lg">Large</option>
                </select>
              </div>

              <div class="flex items-center gap-3">
                <Toggle bind:checked={dropdownEmphasized} />
                <label class="text-slate-300 text-sm">Emphasized</label>
              </div>

              <div class="flex items-center gap-3">
                <Toggle bind:checked={dropdownGradient} />
                <label class="text-slate-300 text-sm">Gradient</label>
              </div>
            {/if}
          </div>
        </Card>
      </div>

      <!-- Preview & Code -->
      <div class="lg:col-span-5 space-y-6">
        <!-- Preview -->
        <Card
          emphasized
          shapeStyle="extra-rounded"
          surfaceTone="container"
          elevationLevel={3}
        >
          <h3 class="text-white font-bold text-lg mb-4">
            <i class="bi bi-eye mr-2"></i>Live Preview
          </h3>

          <!-- Light background for better component visibility -->
          <div
            class="min-h-[300px] bg-gradient-to-br from-white to-gray-100 rounded-2xl p-8 flex items-center justify-center"
          >
            {#if selectedComponent === "button"}
              <Button
                variant={buttonVariant}
                size={buttonSize}
                emphasized={buttonEmphasized}
                shapeStyle={buttonShapeStyle}
                motion={buttonMotion}
                disabled={buttonDisabled}
                loading={buttonLoading}
              >
                Click Me!
              </Button>
            {:else if selectedComponent === "card"}
              <Card
                surfaceTone={cardSurfaceTone}
                elevationLevel={cardElevationLevel}
                emphasized={cardEmphasized}
                shapeStyle={cardShapeStyle}
                glass={cardGlass}
              >
                <div class="p-6">
                  <h4 class="text-white font-bold text-lg mb-2">Card Title</h4>
                  <p class="text-slate-300">
                    This is a card with customizable props.
                  </p>
                </div>
              </Card>
            {:else if selectedComponent === "modal"}
              <div class="text-center">
                <p class="text-slate-400 mb-4">
                  Click the button in Props Controls to open the modal
                </p>
                <i class="bi bi-arrow-left text-slate-600 text-4xl"></i>
              </div>
            {:else if selectedComponent === "tabs"}
              <div class="w-full">
                <Tabs
                  {tabs}
                  bind:active={activeTab}
                  variant={tabsVariant}
                  emphasized={tabsEmphasized}
                  gradient={tabsGradient}
                  size={tabsSize}
                >
                  {#snippet children(tabId)}
                    <div class="p-4 bg-slate-700/30 rounded-xl">
                      <p class="text-slate-300">Content for {tabId}</p>
                    </div>
                  {/snippet}
                </Tabs>
              </div>
            {:else if selectedComponent === "dropdown"}
              <Dropdown
                items={dropdownItems}
                bind:open={dropdownOpen}
                emphasized={dropdownEmphasized}
                gradient={dropdownGradient}
                size={dropdownSize}
              >
                {#snippet children()}
                  <i class="bi bi-list"></i>
                  Open Menu
                {/snippet}
              </Dropdown>
            {/if}
          </div>
        </Card>

        <!-- Code -->
        <Card
          emphasized
          shapeStyle="extra-rounded"
          surfaceTone="container"
          elevationLevel={2}
        >
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-white font-bold text-lg">
              <i class="bi bi-code-square mr-2"></i>Generated Code
            </h3>
            <Button size="sm" onclick={copyCode}>
              <i class="bi bi-clipboard mr-2"></i>Copy
            </Button>
          </div>

          <pre
            class="bg-slate-900 p-4 rounded-xl overflow-x-auto text-sm text-green-400 border border-slate-700">{generateCode()}</pre>
        </Card>
      </div>
    </div>
  </div>
</div>

<!-- Modal Instance -->
<Modal
  bind:open={modalOpen}
  title="Modal Preview"
  size={modalSize}
  emphasized={modalEmphasized}
  heroMoment={modalHeroMoment}
  shapeStyle={modalShapeStyle}
>
  <div class="space-y-4">
    <p class="text-slate-300">
      This is a live preview of the modal with your selected props!
    </p>
    <div class="bg-slate-700/30 border border-slate-600 rounded-xl p-4">
      <p class="text-slate-400 text-sm">
        <i class="bi bi-info-circle mr-2"></i>
        Try different combinations of props to see how they affect the appearance.
      </p>
    </div>
  </div>
  {#snippet footer()}
    <Button variant="secondary" size="sm" onclick={() => (modalOpen = false)}>
      Close
    </Button>
    <Button variant="primary" size="sm" onclick={() => (modalOpen = false)}>
      Looks Great!
    </Button>
  {/snippet}
</Modal>
