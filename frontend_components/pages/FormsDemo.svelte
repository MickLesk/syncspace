<script>
  import Input from "../atoms/Input.svelte";
  import Textarea from "../atoms/Textarea.svelte";
  import Checkbox from "../atoms/Checkbox.svelte";
  import Radio from "../atoms/Radio.svelte";
  import Toggle from "../atoms/Toggle.svelte";
  import Button from "../atoms/Button.svelte";
  import Chip from "../atoms/Chip.svelte";
  import ProgressBar from "../atoms/ProgressBar.svelte";
  import Tooltip from "../molecules/Tooltip.svelte";
  import Badge from "../atoms/Badge.svelte";

  let email = $state("");
  let password = $state("");
  let bio = $state("");
  let agree = $state(false);
  let newsletter = $state(true);
  let theme = $state("light");
  let notifications = $state(false);
  let progress = $state(65);
  let tags = $state(["Design", "Development", "UI/UX"]);

  function removeTag(index) {
    tags = tags.filter((_, i) => i !== index);
  }

  function handleSubmit() {
    console.log("Form submitted:", { email, password, bio, agree, theme });
  }
</script>

<div
  class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-950 dark:to-gray-900 py-12 px-4"
>
  <div class="max-w-7xl mx-auto">
    <!-- Header -->
    <div class="text-center mb-16">
      <div
        class="inline-flex items-center gap-2 bg-gradient-to-r from-blue-600 to-purple-600 text-white px-4 py-2 rounded-full text-sm font-bold mb-6 shadow-lg shadow-blue-500/30"
      >
        <i class="bi bi-ui-checks-grid"></i>
        FORM COMPONENTS
      </div>
      <h1
        class="text-5xl font-black text-gray-900 dark:text-white mb-4 tracking-tight"
      >
        Enhanced Form Elements
      </h1>
      <p class="text-xl text-gray-600 dark:text-gray-400 max-w-2xl mx-auto">
        Complete form components with advanced features: emphasized text, shape
        variants, animations, and more
      </p>
    </div>

    <!-- Demo Form -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-12">
      <!-- Left Column: Form Inputs -->
      <div
        class="bg-white dark:bg-gray-900 rounded-3xl p-8 shadow-xl border border-gray-200 dark:border-gray-800"
      >
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
            Contact Form
          </h2>
          <Badge variant="success" size="sm">Enhanced</Badge>
        </div>

        <div class="space-y-6">
          <!-- Email Input - Emphasized -->
          <Input
            bind:value={email}
            label="Email Address"
            id="email"
            type="email"
            placeholder="you@example.com"
            emphasized={true}
            shapeStyle="extra-rounded"
            size="md"
            helperText="We'll never share your email"
          />

          <!-- Password Input - Squircle -->
          <Input
            bind:value={password}
            label="Password"
            id="password"
            type="password"
            placeholder="••••••••"
            shapeStyle="squircle"
            size="md"
            helperText="At least 8 characters"
          />

          <!-- Textarea - Emphasized -->
          <Textarea
            bind:value={bio}
            label="Message"
            id="bio"
            placeholder="Tell us about yourself..."
            rows={4}
            emphasized={true}
            shapeStyle="extra-rounded"
            helperText="Max 500 characters"
          />

          <!-- Checkbox - Emphasized -->
          <Checkbox
            bind:checked={agree}
            emphasized={true}
            shapeStyle="squircle"
            color="primary"
            bouncy={true}
          >
            I agree to the terms and conditions
          </Checkbox>

          <!-- Checkbox - Newsletter -->
          <Checkbox
            bind:checked={newsletter}
            shapeStyle="extra-rounded"
            color="success"
          >
            Subscribe to newsletter
          </Checkbox>

          <!-- Submit Button -->
          <Button
            onclick={handleSubmit}
            variant="primary"
            size="lg"
            shapeStyle="extra-rounded"
            emphasized={true}
            motion="bouncy"
            gradient={true}
            class="w-full"
          >
            <i class="bi bi-send-fill mr-2"></i>
            Submit Form
          </Button>
        </div>
      </div>

      <!-- Right Column: Advanced Controls -->
      <div class="space-y-8">
        <!-- Radio Group -->
        <div
          class="bg-white dark:bg-gray-900 rounded-3xl p-8 shadow-xl border border-gray-200 dark:border-gray-800"
        >
          <div class="flex items-center justify-between mb-6">
            <h3 class="text-xl font-bold text-gray-900 dark:text-white">
              Theme Selection
            </h3>
            <Badge variant="primary" size="sm">Radio</Badge>
          </div>

          <div class="space-y-4">
            <Radio
              name="theme"
              value="light"
              bind:group={theme}
              emphasized={true}
              shapeStyle="squircle"
              color="primary"
            >
              Light Mode
            </Radio>
            <Radio
              name="theme"
              value="dark"
              bind:group={theme}
              shapeStyle="squircle"
              color="primary"
            >
              Dark Mode
            </Radio>
            <Radio
              name="theme"
              value="auto"
              bind:group={theme}
              shapeStyle="squircle"
              color="primary"
            >
              Auto (System)
            </Radio>
          </div>
        </div>

        <!-- Toggle Switches -->
        <div
          class="bg-white dark:bg-gray-900 rounded-3xl p-8 shadow-xl border border-gray-200 dark:border-gray-800"
        >
          <div class="flex items-center justify-between mb-6">
            <h3 class="text-xl font-bold text-gray-900 dark:text-white">
              Preferences
            </h3>
            <Badge variant="info" size="sm">Toggle</Badge>
          </div>

          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <span class="text-gray-700 dark:text-gray-300 font-medium"
                >Enable Notifications</span
              >
              <Toggle bind:checked={notifications} size="md" />
            </div>
            <div class="flex items-center justify-between">
              <span class="text-gray-700 dark:text-gray-300 font-medium"
                >Auto-save</span
              >
              <Toggle checked={true} size="md" />
            </div>
            <div class="flex items-center justify-between">
              <span class="text-gray-700 dark:text-gray-300 font-medium"
                >Dark Mode</span
              >
              <Toggle checked={false} size="md" />
            </div>
          </div>
        </div>

        <!-- Progress Bars -->
        <div
          class="bg-white dark:bg-gray-900 rounded-3xl p-8 shadow-xl border border-gray-200 dark:border-gray-800"
        >
          <div class="flex items-center justify-between mb-6">
            <h3 class="text-xl font-bold text-gray-900 dark:text-white">
              Upload Progress
            </h3>
            <Badge variant="warning" size="sm">Progress</Badge>
          </div>

          <div class="space-y-6">
            <ProgressBar
              value={progress}
              variant="gradient"
              size="md"
              showLabel={true}
              gradient={true}
              glowing={true}
              smooth={true}
              shapeStyle="pill"
              emphasized={true}
            >
              Uploading files...
            </ProgressBar>

            <ProgressBar
              value={85}
              variant="success"
              size="lg"
              showLabel={true}
              striped={true}
              animated={true}
              shapeStyle="extra-rounded"
            >
              Processing
            </ProgressBar>

            <ProgressBar
              value={40}
              variant="warning"
              size="sm"
              showLabel={true}
              shapeStyle="squircle"
            >
              Optimizing
            </ProgressBar>
          </div>
        </div>
      </div>
    </div>

    <!-- Chips & Tags Section -->
    <div
      class="bg-white dark:bg-gray-900 rounded-3xl p-8 shadow-xl border border-gray-200 dark:border-gray-800 mb-12"
    >
      <div class="flex items-center justify-between mb-6">
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
          Tags & Chips
        </h2>
        <Badge variant="danger" size="sm">Enhanced</Badge>
      </div>

      <div class="space-y-8">
        <!-- Removable Tags -->
        <div>
          <label
            class="block text-sm font-bold text-gray-900 dark:text-white mb-3"
            >Your Skills</label
          >
          <div class="flex flex-wrap gap-2">
            {#each tags as tag, i}
              <Chip
                variant="primary"
                size="md"
                removable={true}
                onremove={() => removeTag(i)}
                gradient={true}
                animated={true}
                shapeStyle="pill"
                emphasized={true}
                elevationLevel={2}
              >
                {tag}
              </Chip>
            {/each}
          </div>
        </div>

        <!-- Chip Variants -->
        <div>
          <label
            class="block text-sm font-bold text-gray-900 dark:text-white mb-3"
            >Chip Styles</label
          >
          <div class="flex flex-wrap gap-3">
            <Chip variant="default" size="sm" shapeStyle="normal">Default</Chip>
            <Chip
              variant="primary"
              size="sm"
              shapeStyle="extra-rounded"
              gradient={true}>Primary</Chip
            >
            <Chip
              variant="success"
              size="md"
              shapeStyle="squircle"
              gradient={true}
              icon="bi-check-circle-fill">Success</Chip
            >
            <Chip
              variant="danger"
              size="md"
              shapeStyle="ultra-rounded"
              gradient={true}
              icon="bi-exclamation-circle-fill">Danger</Chip
            >
            <Chip
              variant="warning"
              size="lg"
              shapeStyle="pill"
              gradient={true}
              emphasized={true}
              icon="bi-lightning-fill">Warning</Chip
            >
            <Chip
              variant="info"
              size="lg"
              shapeStyle="pill"
              gradient={true}
              icon="bi-info-circle-fill">Info</Chip
            >
            <Chip
              variant="glass"
              size="xl"
              shapeStyle="squircle"
              emphasized={true}
              icon="bi-stars">Glass</Chip
            >
          </div>
        </div>

        <!-- Outlined Chips -->
        <div>
          <label
            class="block text-sm font-bold text-gray-900 dark:text-white mb-3"
            >Outlined Style</label
          >
          <div class="flex flex-wrap gap-3">
            <Chip
              variant="primary"
              size="md"
              outlined={true}
              shapeStyle="extra-rounded">Outlined Primary</Chip
            >
            <Chip
              variant="success"
              size="md"
              outlined={true}
              shapeStyle="squircle">Outlined Success</Chip
            >
            <Chip variant="danger" size="md" outlined={true} shapeStyle="pill"
              >Outlined Danger</Chip
            >
            <Chip
              variant="warning"
              size="md"
              outlined={true}
              shapeStyle="ultra-rounded"
              animated={true}>Outlined Warning</Chip
            >
          </div>
        </div>
      </div>
    </div>

    <!-- Tooltip Examples -->
    <div
      class="bg-white dark:bg-gray-900 rounded-3xl p-8 shadow-xl border border-gray-200 dark:border-gray-800"
    >
      <div class="flex items-center justify-between mb-6">
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
          Enhanced Tooltips
        </h2>
        <Badge variant="info" size="sm">Interactive</Badge>
      </div>

      <div class="grid grid-cols-2 lg:grid-cols-4 gap-8">
        <!-- Dark Tooltip -->
        <div class="text-center">
          <Tooltip
            position="top"
            variant="dark"
            size="md"
            shapeStyle="normal"
            emphasized={true}
            showArrow={true}
          >
            <Button variant="secondary" size="md" shapeStyle="extra-rounded">
              Dark
            </Button>
            {#snippet content()}
              Dark tooltip style
            {/snippet}
          </Tooltip>
          <p class="text-xs text-gray-500 mt-2">Dark theme</p>
        </div>

        <!-- Light Tooltip -->
        <div class="text-center">
          <Tooltip
            position="top"
            variant="light"
            size="md"
            shapeStyle="extra-rounded"
            emphasized={true}
          >
            <Button variant="secondary" size="md" shapeStyle="extra-rounded">
              Light
            </Button>
            {#snippet content()}
              Light tooltip style
            {/snippet}
          </Tooltip>
          <p class="text-xs text-gray-500 mt-2">Light theme</p>
        </div>

        <!-- Gradient Tooltip -->
        <div class="text-center">
          <Tooltip
            position="top"
            variant="gradient"
            size="lg"
            shapeStyle="squircle"
            emphasized={true}
          >
            <Button
              variant="primary"
              size="md"
              shapeStyle="squircle"
              gradient={true}
            >
              Gradient
            </Button>
            {#snippet content()}
              Gradient tooltip 🎨
            {/snippet}
          </Tooltip>
          <p class="text-xs text-gray-500 mt-2">Gradient style</p>
        </div>

        <!-- Glass Tooltip -->
        <div class="text-center">
          <Tooltip
            position="top"
            variant="glass"
            size="lg"
            shapeStyle="pill"
            emphasized={true}
            showArrow={false}
          >
            <Button variant="secondary" size="md" shapeStyle="pill">
              Glass
            </Button>
            {#snippet content()}
              Glass tooltip ✨
            {/snippet}
          </Tooltip>
          <p class="text-xs text-gray-500 mt-2">Glassmorphism</p>
        </div>
      </div>

      <!-- Tooltip Positions -->
      <div class="mt-12 flex justify-center items-center gap-12">
        <Tooltip
          position="left"
          variant="gradient"
          size="md"
          shapeStyle="extra-rounded"
          delay={0}
        >
          <Button variant="secondary" size="sm" shapeStyle="extra-rounded"
            >Left</Button
          >
          {#snippet content()}Left tooltip{/snippet}
        </Tooltip>

        <div class="flex flex-col gap-4">
          <Tooltip
            position="top"
            variant="gradient"
            size="md"
            shapeStyle="squircle"
            delay={0}
          >
            <Button variant="secondary" size="sm" shapeStyle="squircle"
              >Top</Button
            >
            {#snippet content()}Top tooltip{/snippet}
          </Tooltip>
          <Tooltip
            position="bottom"
            variant="gradient"
            size="md"
            shapeStyle="squircle"
            delay={0}
          >
            <Button variant="secondary" size="sm" shapeStyle="squircle"
              >Bottom</Button
            >
            {#snippet content()}Bottom tooltip{/snippet}
          </Tooltip>
        </div>

        <Tooltip
          position="right"
          variant="gradient"
          size="md"
          shapeStyle="extra-rounded"
          delay={0}
        >
          <Button variant="secondary" size="sm" shapeStyle="extra-rounded"
            >Right</Button
          >
          {#snippet content()}Right tooltip{/snippet}
        </Tooltip>
      </div>
    </div>

    <!-- Feature Grid -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mt-12">
      <div
        class="bg-gradient-to-br from-blue-500 to-purple-600 rounded-3xl p-6 text-white shadow-xl"
      >
        <div class="text-4xl mb-4">🎨</div>
        <h3 class="text-xl font-bold mb-2">Shape Variety</h3>
        <p class="text-blue-100">
          13+ shape styles: normal, extra-rounded, squircle, ultra-rounded,
          pill, and more
        </p>
      </div>

      <div
        class="bg-gradient-to-br from-green-500 to-emerald-600 rounded-3xl p-6 text-white shadow-xl"
      >
        <div class="text-4xl mb-4">✨</div>
        <h3 class="text-xl font-bold mb-2">Emphasized Text</h3>
        <p class="text-green-100">
          Bold typography for labels and important text with emphasized prop
        </p>
      </div>

      <div
        class="bg-gradient-to-br from-amber-500 to-orange-500 rounded-3xl p-6 text-white shadow-xl"
      >
        <div class="text-4xl mb-4">🚀</div>
        <h3 class="text-xl font-bold mb-2">Spring Animations</h3>
        <p class="text-amber-100">
          Smooth transitions with spring physics for natural motion
        </p>
      </div>
    </div>
  </div>
</div>
