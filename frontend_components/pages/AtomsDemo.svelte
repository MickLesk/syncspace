<script lang="ts">
  import Button from "../atoms/Button.svelte";
  import Badge from "../atoms/Badge.svelte";
  import Divider from "../atoms/Divider.svelte";
  import Label from "../atoms/Label.svelte";
  import Checkbox from "../atoms/Checkbox.svelte";
  import Toggle from "../atoms/Toggle.svelte";
  import Input from "../atoms/Input.svelte";
  import Avatar from "../atoms/Avatar.svelte";
  import Card from "../atoms/Card.svelte";
  import { colorMap, sizeMap } from "../shared/index.ts";

  type ButtonVariant =
    | "primary"
    | "secondary"
    | "danger"
    | "success"
    | "warning"
    | "ghost"
    | "outline";
  type BadgeVariant =
    | "primary"
    | "secondary"
    | "danger"
    | "success"
    | "warning"
    | "info";
  type ButtonSize = "xs" | "sm" | "md" | "lg" | "xl";

  const buttonVariants: ButtonVariant[] = [
    "primary",
    "secondary",
    "danger",
    "success",
    "warning",
    "ghost",
    "outline",
  ];
  const buttonSizes: ButtonSize[] = ["xs", "sm", "md", "lg", "xl"];
  const badgeVariants: BadgeVariant[] = [
    "primary",
    "secondary",
    "danger",
    "success",
    "warning",
    "info",
  ];

  let checkboxState = false;
  let toggleState = false;
  let inputValue = "";
  let selectedSize: ButtonSize = "md";
</script>

<div class="min-h-screen bg-slate-900 py-12">
  <div class="max-w-7xl mx-auto px-6">
    <!-- Header -->
    <div class="mb-12">
      <h1 class="text-4xl font-bold text-white mb-2">Atoms Component Demo</h1>
      <p class="text-slate-400">Basic building blocks for your UI</p>
    </div>

    <!-- Buttons Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-cursor-fill mr-3 text-blue-400"></i>Buttons
      </h2>

      <!-- Variants -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8 mb-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">Variants</h3>
        <div class="flex flex-wrap gap-4">
          {#each buttonVariants as variant (variant)}
            <Button {variant}>
              <i class="bi bi-star-fill mr-2"></i>
              {variant.charAt(0).toUpperCase() + variant.slice(1)}
            </Button>
          {/each}
        </div>
      </div>

      <!-- Sizes -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8 mb-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">Sizes</h3>
        <div class="flex flex-wrap gap-4 items-center">
          {#each buttonSizes as size (size)}
            <Button {size}>
              {size.toUpperCase()}
            </Button>
          {/each}
        </div>
      </div>

      <!-- Loading State -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">Loading State</h3>
        <div class="flex flex-wrap gap-4">
          <Button loading>
            <i class="bi bi-cloud-upload mr-2"></i>Uploading...
          </Button>
          <Button variant="secondary" loading>Processing</Button>
          <Button variant="success" disabled>Disabled</Button>
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Badges Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-tag-fill mr-3 text-purple-400"></i>Badges
      </h2>

      <!-- Standard Variants -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8 mb-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">
          Standard Variants
        </h3>
        <div class="flex flex-wrap gap-4">
          {#each badgeVariants as variant (variant)}
            <Badge {variant}>
              <i class="bi bi-check-circle-fill mr-2"></i>
              {variant.charAt(0).toUpperCase() + variant.slice(1)}
            </Badge>
          {/each}
        </div>
      </div>

      <!-- Outline Variants -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8 mb-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">
          Outline Variants
        </h3>
        <div class="flex flex-wrap gap-4">
          {#each badgeVariants as variant (variant)}
            <Badge {variant} outline>
              <i class="bi bi-lightning-fill mr-2"></i>
              {variant.charAt(0).toUpperCase() + variant.slice(1)}
            </Badge>
          {/each}
        </div>
      </div>

      <!-- Sizes -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">Sizes</h3>
        <div class="flex flex-wrap gap-4 items-center">
          <Badge variant="primary" size="sm">Small</Badge>
          <Badge variant="primary" size="md">Medium</Badge>
          <Badge variant="primary" size="lg">Large</Badge>
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Input Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-keyboard-fill mr-3 text-green-400"></i>Inputs
      </h2>

      <div class="grid md:grid-cols-2 gap-8">
        <!-- Text Input -->
        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">Text Input</h3>
          <Label label="Enter your text" hint="This is a helpful hint" />
          <Input bind:value={inputValue} placeholder="Type something..." />
          <p class="text-slate-400 text-sm mt-4">
            Value: {inputValue || "empty"}
          </p>
        </div>

        <!-- Input Variants -->
        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">Variants</h3>
          <div class="space-y-3">
            <div>
              <Input variant="primary" placeholder="Primary variant" />
            </div>
            <div>
              <Input variant="secondary" placeholder="Secondary variant" />
            </div>
            <div>
              <Input
                variant="danger"
                error="This field is required"
                placeholder="Error state"
              />
            </div>
          </div>
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Checkbox & Toggle Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-toggle2-on mr-3 text-yellow-400"></i>Controls
      </h2>

      <div class="grid md:grid-cols-2 gap-8">
        <!-- Checkboxes -->
        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">Checkboxes</h3>
          <div class="space-y-4">
            <Checkbox label="Primary variant" bind:checked={checkboxState} />
            <Checkbox label="Secondary variant" variant="secondary" />
            <Checkbox label="Danger variant" variant="danger" />
            <Checkbox label="Disabled" disabled />
          </div>
          <p class="text-slate-400 text-sm mt-4">
            Checked: {checkboxState ? "Yes" : "No"}
          </p>
        </div>

        <!-- Toggles -->
        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">Toggles</h3>
          <div class="space-y-4">
            <Toggle bind:checked={toggleState} label="Primary toggle" />
            <Toggle variant="success" label="Success toggle" />
            <Toggle variant="danger" label="Danger toggle" />
            <Toggle disabled label="Disabled toggle" />
          </div>
          <p class="text-slate-400 text-sm mt-4">
            Enabled: {toggleState ? "Yes" : "No"}
          </p>
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Avatar Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-person-circle mr-3 text-pink-400"></i>Avatars
      </h2>

      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-6">
          User Avatars with Auto-Colors
        </h3>
        <div class="flex flex-wrap gap-8 items-center">
          <div class="flex flex-col items-center gap-2">
            <Avatar name="Alice Johnson" size="sm" />
            <span class="text-slate-400 text-sm">Small</span>
          </div>
          <div class="flex flex-col items-center gap-2">
            <Avatar name="Bob Smith" size="md" />
            <span class="text-slate-400 text-sm">Medium</span>
          </div>
          <div class="flex flex-col items-center gap-2">
            <Avatar name="Charlie Brown" size="lg" />
            <span class="text-slate-400 text-sm">Large</span>
          </div>
          <div class="flex flex-col items-center gap-2">
            <Avatar name="Diana Prince" size="lg" online />
            <span class="text-slate-400 text-sm">Online</span>
          </div>
          <div class="flex flex-col items-center gap-2">
            <Avatar size="md" />
            <span class="text-slate-400 text-sm">No name</span>
          </div>
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Card Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-card-text mr-3 text-cyan-400"></i>Cards
      </h2>

      <div class="grid md:grid-cols-3 gap-8">
        <!-- Basic Card -->
        <Card
          title="Basic Card"
          description="A simple card with title and description"
        >
          <p class="text-slate-400">
            This is the card content area. You can put any content here using
            slots.
          </p>
        </Card>

        <!-- Card with Footer -->
        <Card
          title="Card with Actions"
          description="Interactive card with footer"
          footer={true}
        >
          <p class="text-slate-400 mb-4">
            This card has a footer section for actions.
          </p>
          <div class="flex gap-2 pt-4 border-t border-slate-600">
            <Button size="sm" variant="primary">Save</Button>
            <Button size="sm" variant="secondary">Cancel</Button>
          </div>
        </Card>

        <!-- Hoverable Card -->
        <Card
          title="Hoverable Card"
          description="Interactive card with hover effect"
          hoverable={true}
          bordered={true}
        >
          <p class="text-slate-400">
            Hover over this card to see the effect. Great for clickable items.
          </p>
        </Card>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Info Box -->
    <div
      class="bg-blue-500/10 border border-blue-500/30 rounded-xl p-6 text-slate-300"
    >
      <i class="bi bi-info-circle text-blue-400 mr-2"></i>
      <strong>Tip:</strong> All atoms are fully customizable through props and Tailwind
      classes. Check the component files for more options and detailed prop documentation.
    </div>
  </div>
</div>
