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
  import Spinner from "../atoms/Spinner.svelte";
  import ProgressBar from "../atoms/ProgressBar.svelte";
  import Alert from "../atoms/Alert.svelte";
  import Textarea from "../atoms/Textarea.svelte";
  import Skeleton from "../atoms/Skeleton.svelte";
  import Radio from "../atoms/Radio.svelte";
  import Chip from "../atoms/Chip.svelte";
  import IconButton from "../atoms/IconButton.svelte";
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
  let progress = 65;
  let textareaValue = "";
  let showAlert = true;
  let inputValue = "";
  let selectedSize: ButtonSize = "md";
  let radioGroup = "";
  let chips = ["Design", "Development", "Marketing", "Sales"];

  function removeChip(index: number) {
    chips = chips.filter((_, i) => i !== index);
  }
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

    <!-- Spinners Section -->
    <section>
      <h2
        class="text-2xl font-bold text-slate-200 mb-6 flex items-center gap-3"
      >
        <i class="bi bi-arrow-repeat text-blue-400"></i>Spinners
      </h2>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">Sizes</h3>
          <div class="flex items-center gap-6">
            <Spinner size="sm" variant="primary" />
            <Spinner size="md" variant="primary" />
            <Spinner size="lg" variant="primary" />
            <Spinner size="xl" variant="primary" />
          </div>
        </div>

        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">Variants</h3>
          <div class="flex items-center gap-6">
            <Spinner variant="primary" />
            <Spinner variant="success" />
            <Spinner variant="danger" />
            <Spinner variant="warning" />
          </div>
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Progress Bars Section -->
    <section>
      <h2
        class="text-2xl font-bold text-slate-200 mb-6 flex items-center gap-3"
      >
        <i class="bi bi-bar-chart-fill text-green-400"></i>Progress Bars
      </h2>
      <div class="space-y-6">
        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">Basic</h3>
          <div class="space-y-4">
            <ProgressBar value={progress} variant="primary" showLabel />
            <ProgressBar value={85} variant="success" showLabel
              >Upload Progress</ProgressBar
            >
            <ProgressBar value={45} variant="warning" showLabel />
            <ProgressBar value={25} variant="danger" showLabel />
          </div>
        </div>

        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">
            Striped & Animated
          </h3>
          <div class="space-y-4">
            <ProgressBar value={70} variant="primary" striped showLabel />
            <ProgressBar
              value={55}
              variant="success"
              striped
              animated
              showLabel
            >
              Processing...
            </ProgressBar>
          </div>
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Alerts Section -->
    <section>
      <h2
        class="text-2xl font-bold text-slate-200 mb-6 flex items-center gap-3"
      >
        <i class="bi bi-bell-fill text-yellow-400"></i>Alerts
      </h2>
      <div class="space-y-4">
        {#if showAlert}
          <Alert type="info" dismissible ondismiss={() => (showAlert = false)}>
            This is an informational alert. Click the X to dismiss.
          </Alert>
        {/if}
        <Alert type="success" icon="bi-check-circle-fill">
          Your changes have been saved successfully!
        </Alert>
        <Alert type="warning">Warning: This action cannot be undone.</Alert>
        <Alert type="danger" dismissible>
          Error: Something went wrong. Please try again.
        </Alert>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Textarea Section -->
    <section>
      <h2
        class="text-2xl font-bold text-slate-200 mb-6 flex items-center gap-3"
      >
        <i class="bi bi-textarea-t text-purple-400"></i>Textarea
      </h2>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <Textarea
            bind:value={textareaValue}
            placeholder="Enter your message..."
            rows={6}
          >
            Message
          </Textarea>
        </div>

        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <Textarea
            placeholder="This textarea has an error"
            error={true}
            rows={6}
          >
            Error State
          </Textarea>
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Spinners Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-arrow-repeat mr-3 text-blue-400"></i>Spinners
      </h2>

      <!-- Spinner Types -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8 mb-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">Types</h3>
        <div class="flex flex-wrap gap-8 items-center">
          <div class="text-center">
            <Spinner type="spin" variant="primary" size="lg" />
            <p class="text-slate-400 text-sm mt-2">Spin</p>
          </div>
          <div class="text-center">
            <Spinner type="dots" variant="success" size="lg" />
            <p class="text-slate-400 text-sm mt-2">Dots</p>
          </div>
          <div class="text-center">
            <Spinner type="pulse" variant="warning" size="lg" />
            <p class="text-slate-400 text-sm mt-2">Pulse</p>
          </div>
          <div class="text-center">
            <Spinner type="bounce" variant="danger" size="lg" />
            <p class="text-slate-400 text-sm mt-2">Bounce</p>
          </div>
        </div>
      </div>

      <!-- Spinner Sizes -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">Sizes</h3>
        <div class="flex flex-wrap gap-6 items-center">
          <Spinner type="spin" variant="primary" size="sm" />
          <Spinner type="dots" variant="primary" size="md" />
          <Spinner type="pulse" variant="primary" size="lg" />
          <Spinner type="bounce" variant="primary" size="xl" />
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Chips Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-tags-fill mr-3 text-blue-400"></i>Chips
      </h2>

      <!-- Chip Variants -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8 mb-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">Variants</h3>
        <div class="flex flex-wrap gap-3">
          <Chip variant="default">Default</Chip>
          <Chip variant="primary">Primary</Chip>
          <Chip variant="success">Success</Chip>
          <Chip variant="danger">Danger</Chip>
          <Chip variant="warning">Warning</Chip>
          <Chip variant="info">Info</Chip>
          <Chip variant="glass">Glass</Chip>
        </div>
      </div>

      <!-- Chip with Icons -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8 mb-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">With Icons</h3>
        <div class="flex flex-wrap gap-3">
          <Chip variant="primary" icon="bi-star-fill">Featured</Chip>
          <Chip variant="success" icon="bi-check-circle-fill">Verified</Chip>
          <Chip variant="danger" icon="bi-x-circle-fill">Error</Chip>
          <Chip variant="warning" icon="bi-exclamation-triangle-fill"
            >Warning</Chip
          >
        </div>
      </div>

      <!-- Removable Chips -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">Removable</h3>
        <div class="flex flex-wrap gap-3">
          {#each chips as chip, index}
            <Chip
              variant="primary"
              removable
              onremove={() => removeChip(index)}
            >
              {chip}
            </Chip>
          {/each}
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Icon Buttons Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-circle-square mr-3 text-blue-400"></i>Icon Buttons
      </h2>

      <!-- Icon Button Variants -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8 mb-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">Variants</h3>
        <div class="flex flex-wrap gap-3">
          <IconButton icon="bi-heart-fill" variant="default" ariaLabel="Like" />
          <IconButton
            icon="bi-star-fill"
            variant="primary"
            ariaLabel="Favorite"
          />
          <IconButton
            icon="bi-trash-fill"
            variant="danger"
            ariaLabel="Delete"
          />
          <IconButton
            icon="bi-check-lg"
            variant="success"
            ariaLabel="Confirm"
          />
          <IconButton icon="bi-search" variant="ghost" ariaLabel="Search" />
          <IconButton
            icon="bi-gear-fill"
            variant="glass"
            ariaLabel="Settings"
          />
        </div>
      </div>

      <!-- Icon Button Sizes -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8 mb-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">Sizes</h3>
        <div class="flex flex-wrap gap-3 items-center">
          <IconButton
            icon="bi-heart-fill"
            variant="primary"
            size="sm"
            ariaLabel="Like"
          />
          <IconButton
            icon="bi-heart-fill"
            variant="primary"
            size="md"
            ariaLabel="Like"
          />
          <IconButton
            icon="bi-heart-fill"
            variant="primary"
            size="lg"
            ariaLabel="Like"
          />
          <IconButton
            icon="bi-heart-fill"
            variant="primary"
            size="xl"
            ariaLabel="Like"
          />
        </div>
      </div>

      <!-- Icon Button Shapes -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">Shapes</h3>
        <div class="flex flex-wrap gap-3">
          <IconButton
            icon="bi-heart-fill"
            variant="primary"
            rounded={false}
            ariaLabel="Like"
          />
          <IconButton
            icon="bi-heart-fill"
            variant="primary"
            rounded={true}
            ariaLabel="Like"
          />
          <IconButton
            icon="bi-arrow-repeat"
            variant="success"
            rounded={true}
            loading={true}
            ariaLabel="Loading"
          />
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Radio Buttons Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-circle-fill mr-3 text-blue-400"></i>Radio Buttons
      </h2>

      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">
          Select an Option
        </h3>
        <div class="flex flex-col gap-3">
          <Radio
            name="option"
            value="option1"
            bind:group={radioGroup}
            variant="primary"
          >
            Option 1
          </Radio>
          <Radio
            name="option"
            value="option2"
            bind:group={radioGroup}
            variant="primary"
          >
            Option 2
          </Radio>
          <Radio
            name="option"
            value="option3"
            bind:group={radioGroup}
            variant="primary"
          >
            Option 3
          </Radio>
        </div>
        <p class="text-slate-400 text-sm mt-4">
          Selected: {radioGroup || "None"}
        </p>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Skeleton Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-box mr-3 text-blue-400"></i>Skeleton Loaders
      </h2>

      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">Variants</h3>
        <div class="space-y-6">
          <div>
            <p class="text-slate-400 text-sm mb-2">Text</p>
            <Skeleton variant="text" width="300px" />
          </div>
          <div>
            <p class="text-slate-400 text-sm mb-2">Rectangle</p>
            <Skeleton variant="rect" width="400px" height="200px" />
          </div>
          <div>
            <p class="text-slate-400 text-sm mb-2">Circle</p>
            <Skeleton variant="circle" width="80px" height="80px" />
          </div>
          <div>
            <p class="text-slate-400 text-sm mb-2">Avatar</p>
            <Skeleton variant="avatar" />
          </div>
        </div>
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
