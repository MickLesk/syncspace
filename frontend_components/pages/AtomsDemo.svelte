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

  let checkboxState = $state(false);
  let toggleState = $state(false);
  let progress = $state(65);
  let textareaValue = $state("");
  let showAlert = $state(true);
  let inputValue = $state("");
  let selectedSize: ButtonSize = $state("md");
  let radioGroup = $state("");
  let chips = $state(["Design", "Development", "Marketing", "Sales"]);

  function removeChip(index: number) {
    chips = chips.filter((_, i) => i !== index);
  }
</script>

<div class="min-h-screen bg-slate-900 py-12">
  <div class="max-w-7xl mx-auto px-6">
    <!-- Header -->
    <div class="mb-12 text-center">
      <div
        class="inline-block px-4 py-1.5 bg-blue-500/20 border border-blue-500/30 rounded-full mb-4"
      >
        <span class="text-blue-400 text-sm font-semibold"
          >✨ 2025 Design System</span
        >
      </div>
      <h1 class="text-5xl font-bold text-white mb-3">
        Atoms Component Library
      </h1>
      <p class="text-slate-400 text-lg max-w-2xl mx-auto">
        Modern UI components with <span class="text-blue-400"
          >subtle neumorphism</span
        >,
        <span class="text-purple-400">glowing effects</span>, and
        <span class="text-green-400">smooth microinteractions</span>
      </p>

      <!-- Feature Tags -->
      <div class="flex flex-wrap justify-center gap-3 mt-6">
        <span
          class="px-3 py-1 bg-slate-800 border border-slate-700 rounded-lg text-slate-300 text-sm"
        >
          🌊 Spring Animations
        </span>
        <span
          class="px-3 py-1 bg-slate-800 border border-slate-700 rounded-lg text-slate-300 text-sm"
        >
          ✨ Glow Effects
        </span>
        <span
          class="px-3 py-1 bg-slate-800 border border-slate-700 rounded-lg text-slate-300 text-sm"
        >
          🚀 Hover Lift
        </span>
        <span
          class="px-3 py-1 bg-slate-800 border border-slate-700 rounded-lg text-slate-300 text-sm"
        >
          🎨 Glassmorphism
        </span>
        <span
          class="px-3 py-1 bg-slate-800 border border-slate-700 rounded-lg text-slate-300 text-sm"
        >
          🌙 Dark Mode Native
        </span>
      </div>
    </div>

    <!-- Buttons Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-cursor-fill mr-3 text-blue-400"></i>Buttons - 2025
        Design
      </h2>

      <!-- Variants with Glow -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8 mb-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">
          Variants with Hover Glow ✨
        </h3>
        <div class="flex flex-wrap gap-4">
          {#each buttonVariants as variant (variant)}
            <Button {variant} glow>
              <i class="bi bi-star-fill mr-2"></i>
              {variant.charAt(0).toUpperCase() + variant.slice(1)}
            </Button>
          {/each}
        </div>
        <p class="text-slate-400 text-sm mt-4">
          💡 Hover over buttons to see the glow effect!
        </p>
      </div>

      <!-- Standard Variants (No Glow) -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8 mb-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">
          Standard (No Glow)
        </h3>
        <div class="flex flex-wrap gap-4">
          {#each buttonVariants as variant (variant)}
            <Button {variant}>
              <i class="bi bi-check-lg mr-2"></i>
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

      <!-- Interactive States -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">
          Interactive States with Lift Effect 🚀
        </h3>
        <div class="flex flex-wrap gap-4">
          <Button loading glow>
            <i class="bi bi-cloud-upload mr-2"></i>Uploading...
          </Button>
          <Button variant="success" glow onclick={() => alert("Clicked!")}>
            <i class="bi bi-check-circle-fill mr-2"></i>Click Me!
          </Button>
          <Button variant="danger" glow>
            <i class="bi bi-trash-fill mr-2"></i>Delete
          </Button>
          <Button variant="secondary" disabled>
            <i class="bi bi-lock-fill mr-2"></i>Disabled
          </Button>
        </div>
        <p class="text-slate-400 text-sm mt-4">
          ✨ Buttons lift on hover and press down on click with smooth spring
          animations
        </p>
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
        <i class="bi bi-keyboard-fill mr-3 text-green-400"></i>Inputs - 2025
        Design
      </h2>

      <div class="grid md:grid-cols-2 gap-8">
        <!-- Text Input with Glow -->
        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">
            Focus Glow Effect ✨
          </h3>
          <Label
            label="Enter your text"
            hint="Focus on input to see the glow"
          />
          <Input bind:value={inputValue} placeholder="Type something..." glow />
          <p class="text-slate-400 text-sm mt-4">
            💡 Focus triggers a glowing blue shadow effect
          </p>
        </div>

        <!-- Input Variants -->
        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">
            Input States with Animation
          </h3>
          <div class="space-y-3">
            <div>
              <Input variant="primary" placeholder="Primary (focus me!)" glow />
            </div>
            <div>
              <Input
                variant="secondary"
                placeholder="Secondary variant"
                glow={false}
              />
            </div>
            <div>
              <Input
                variant="danger"
                error
                placeholder="Error state with pulse"
                glow
              />
              <p class="text-red-400 text-xs mt-1">
                ⚠️ Error inputs pulse to grab attention
              </p>
            </div>
          </div>
        </div>
      </div>

      <!-- Interactive Demo -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8 mt-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">
          Interactive Demo - Type to See Value
        </h3>
        <Input
          bind:value={inputValue}
          placeholder="Start typing..."
          glow
          class="text-lg"
        />
        <div
          class="mt-4 p-4 bg-slate-900/50 rounded-lg border border-slate-600"
        >
          <p class="text-slate-300">
            <span class="text-slate-500">Current Value:</span>
            <span class="text-blue-400 font-mono ml-2">
              {inputValue || "(empty)"}
            </span>
          </p>
          <p class="text-slate-400 text-xs mt-2">
            ✨ Notice the subtle scale effect on focus and smooth transitions
          </p>
        </div>
      </div>
    </section>

    <Divider variant="horizontal" color="slate" />

    <!-- Checkbox & Toggle Section -->
    <section class="mb-16">
      <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
        <i class="bi bi-toggle2-on mr-3 text-yellow-400"></i>Controls - 2025
        Design
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
            Checked: {checkboxState ? "Yes ✓" : "No ✗"}
          </p>
        </div>

        <!-- Toggles with Spring Animation -->
        <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8">
          <h3 class="text-lg font-semibold text-slate-200 mb-4">
            Toggles with Spring Animation 🌊
          </h3>
          <div class="space-y-4">
            <Toggle bind:checked={toggleState} glow>
              <span class="font-medium">Primary with glow</span>
            </Toggle>
            <Toggle variant="success" glow>
              <span class="font-medium">Success with glow</span>
            </Toggle>
            <Toggle variant="danger" glow>
              <span class="font-medium">Danger with glow</span>
            </Toggle>
            <Toggle disabled>
              <span class="font-medium">Disabled toggle</span>
            </Toggle>
          </div>
          <div
            class="mt-4 p-3 bg-slate-900/50 rounded-lg border border-slate-600"
          >
            <p class="text-slate-400 text-sm">
              ✨ Enabled: <span class="text-blue-400 font-semibold"
                >{toggleState ? "Yes" : "No"}</span
              >
            </p>
            <p class="text-slate-500 text-xs mt-1">
              Toggle animations use spring physics for natural bouncy feel
            </p>
          </div>
        </div>
      </div>

      <!-- Toggle Features Demo -->
      <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-8 mt-8">
        <h3 class="text-lg font-semibold text-slate-200 mb-4">
          2025 Toggle Features Showcase
        </h3>
        <div class="grid md:grid-cols-3 gap-6">
          <div class="text-center p-4 bg-slate-900/30 rounded-lg">
            <Toggle variant="primary" glow />
            <p class="text-slate-400 text-sm mt-3">Spring Animation</p>
            <p class="text-slate-500 text-xs mt-1">Bounces on state change</p>
          </div>
          <div class="text-center p-4 bg-slate-900/30 rounded-lg">
            <Toggle variant="success" glow />
            <p class="text-slate-400 text-sm mt-3">Glow Effect</p>
            <p class="text-slate-500 text-xs mt-1">Glows when active</p>
          </div>
          <div class="text-center p-4 bg-slate-900/30 rounded-lg">
            <Toggle variant="warning" glow />
            <p class="text-slate-400 text-sm mt-3">Hover Scale</p>
            <p class="text-slate-500 text-xs mt-1">Scales up on hover</p>
          </div>
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
        <i class="bi bi-card-text mr-3 text-cyan-400"></i>Cards - 2025 Design
      </h2>

      <div class="grid md:grid-cols-3 gap-8">
        <!-- Glassmorphism Card -->
        <Card
          title="Glassmorphism Card"
          description="Modern glass effect with backdrop blur"
          glass={true}
          glow={true}
          hoverable={true}
        >
          <p class="text-slate-400">
            ✨ This card features the glassmorphism effect - translucent
            background with backdrop blur, popular in 2025 design.
          </p>
        </Card>

        <!-- Card with Glow -->
        <Card
          title="Glowing Card"
          description="Hover for subtle glow effect"
          glow={true}
          hoverable={true}
          shadow="lg"
        >
          <p class="text-slate-400 mb-4">
            🌟 Hover over this card to see the subtle blue glow effect, perfect
            for highlighting important content.
          </p>
        </Card>

        <!-- Elevated Card -->
        <Card
          title="Elevated Card"
          description="Deep shadow with lift effect"
          hoverable={true}
          shadow="xl"
          bordered={true}
        >
          <p class="text-slate-400">
            🚀 This card lifts up on hover with smooth spring animation,
            creating a tactile 3D effect.
          </p>
        </Card>
      </div>

      <!-- Card Comparison -->
      <div class="grid md:grid-cols-2 gap-8 mt-8">
        <!-- Standard Card -->
        <Card
          title="Standard Card"
          description="Classic solid background"
          shadow="md"
        >
          <p class="text-slate-400">
            Traditional card design with solid background and medium shadow.
            Still looks great!
          </p>
        </Card>

        <!-- Glass Card -->
        <Card
          title="Glass Card"
          description="Translucent with blur"
          glass={true}
        >
          <p class="text-slate-400">
            Modern glassmorphism with transparency and backdrop blur. Notice the
            subtle difference!
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
