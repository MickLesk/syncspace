<script>
  import Button from "../atoms/Button.svelte";
  import Card from "../atoms/Card.svelte";
  import Badge from "../atoms/Badge.svelte";
  import Avatar from "../atoms/Avatar.svelte";
  import IconButton from "../atoms/IconButton.svelte";
  import Chip from "../atoms/Chip.svelte";

  let isAnimating = $state(false);
  let counter = $state(0);

  function triggerAnimation() {
    isAnimating = true;
    counter++;
    setTimeout(() => {
      isAnimating = false;
    }, 1000);
  }
</script>

<div
  class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-950 dark:to-gray-900 py-12 px-4"
>
  <div class="max-w-7xl mx-auto">
    <!-- Header -->
    <div class="text-center mb-16">
      <div
        class="inline-flex items-center gap-2 bg-gradient-to-r from-purple-600 to-pink-600 text-white px-4 py-2 rounded-full text-sm font-bold mb-6 shadow-lg shadow-purple-500/30 animate-pulse"
      >
        <i class="bi bi-stars"></i>
        ANIMATIONS & MOTION
      </div>
      <h1
        class="text-5xl font-black text-gray-900 dark:text-white mb-4 tracking-tight"
      >
        Spring Physics Animations
      </h1>
      <p class="text-xl text-gray-600 dark:text-gray-400 max-w-2xl mx-auto">
        Natural motion with spring physics, scale transforms, and smooth
        transitions
      </p>
    </div>

    <!-- Motion Types Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-12">
      <!-- Bouncy Motion -->
      <div
        class="bg-white dark:bg-gray-900 rounded-3xl p-8 shadow-xl border border-gray-200 dark:border-gray-800"
      >
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
            Bouncy
          </h2>
          <Badge variant="primary" size="sm" gradient={true}
            >cubic-bezier(0.34, 1.56, 0.64, 1)</Badge
          >
        </div>

        <p class="text-gray-600 dark:text-gray-400 mb-6">
          Playful, energetic motion with overshoot. Perfect for
          attention-grabbing interactions.
        </p>

        <div class="flex flex-wrap gap-4">
          <Button
            variant="primary"
            size="lg"
            motion="bouncy"
            shapeStyle="extra-rounded"
            gradient={true}
            onclick={triggerAnimation}
          >
            <i class="bi bi-rocket-takeoff-fill mr-2"></i>
            Bouncy Button
          </Button>

          <IconButton
            icon="bi-heart-fill"
            variant="danger"
            size="xl"
            shapeStyle="circle"
            gradient={true}
            animated={true}
          />

          <Badge
            variant="success"
            size="lg"
            gradient={true}
            shapeStyle="pill"
            emphasized={true}
            class="cursor-pointer hover:scale-110 active:scale-95 transition-all duration-300 ease-[cubic-bezier(0.34,1.56,0.64,1)]"
          >
            Interactive Badge
          </Badge>
        </div>

        <div
          class="mt-6 p-4 bg-blue-50 dark:bg-blue-950/30 rounded-xl border border-blue-200 dark:border-blue-800"
        >
          <code class="text-sm text-blue-900 dark:text-blue-300">
            transition: all 300ms cubic-bezier(0.34, 1.56, 0.64, 1)
          </code>
        </div>
      </div>

      <!-- Smooth Motion -->
      <div
        class="bg-white dark:bg-gray-900 rounded-3xl p-8 shadow-xl border border-gray-200 dark:border-gray-800"
      >
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
            Smooth
          </h2>
          <Badge variant="success" size="sm" gradient={true}
            >cubic-bezier(0.4, 0.0, 0.2, 1)</Badge
          >
        </div>

        <p class="text-gray-600 dark:text-gray-400 mb-6">
          Elegant, refined motion. Best for professional interfaces and subtle
          interactions.
        </p>

        <div class="flex flex-wrap gap-4">
          <Button
            variant="secondary"
            size="lg"
            motion="smooth"
            shapeStyle="squircle"
          >
            <i class="bi bi-magic mr-2"></i>
            Smooth Button
          </Button>

          <IconButton
            icon="bi-star-fill"
            variant="warning"
            size="xl"
            shapeStyle="squircle"
            gradient={true}
            animated={true}
          />

          <Chip
            variant="info"
            size="lg"
            gradient={true}
            shapeStyle="extra-rounded"
            animated={true}
            icon="bi-lightning-fill"
          >
            Smooth Chip
          </Chip>
        </div>

        <div
          class="mt-6 p-4 bg-green-50 dark:bg-green-950/30 rounded-xl border border-green-200 dark:border-green-800"
        >
          <code class="text-sm text-green-900 dark:text-green-300">
            transition: all 250ms cubic-bezier(0.4, 0.0, 0.2, 1)
          </code>
        </div>
      </div>

      <!-- Gentle Motion -->
      <div
        class="bg-white dark:bg-gray-900 rounded-3xl p-8 shadow-xl border border-gray-200 dark:border-gray-800"
      >
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
            Gentle
          </h2>
          <Badge variant="warning" size="sm" gradient={true}
            >cubic-bezier(0.25, 0.46, 0.45, 0.94)</Badge
          >
        </div>

        <p class="text-gray-600 dark:text-gray-400 mb-6">
          Soft, comfortable motion. Ideal for calm, relaxed user experiences.
        </p>

        <div class="flex flex-wrap gap-4">
          <Button
            variant="success"
            size="lg"
            motion="gentle"
            shapeStyle="ultra-rounded"
            gradient={true}
          >
            <i class="bi bi-cloud-fill mr-2"></i>
            Gentle Button
          </Button>

          <Avatar
            size="xl"
            gradient={true}
            shapeStyle="squircle"
            elevationLevel={2}
            class="cursor-pointer"
          >
            <i class="bi bi-person-fill text-2xl"></i>
          </Avatar>

          <Badge
            variant="danger"
            size="lg"
            gradient={true}
            shapeStyle="squircle"
            emphasized={true}
            class="cursor-pointer hover:scale-105 active:scale-95 transition-all duration-400 ease-[cubic-bezier(0.25,0.46,0.45,0.94)]"
          >
            99+
          </Badge>
        </div>

        <div
          class="mt-6 p-4 bg-amber-50 dark:bg-amber-950/30 rounded-xl border border-amber-200 dark:border-amber-800"
        >
          <code class="text-sm text-amber-900 dark:text-amber-300">
            transition: all 400ms cubic-bezier(0.25, 0.46, 0.45, 0.94)
          </code>
        </div>
      </div>

      <!-- Energetic Motion -->
      <div
        class="bg-white dark:bg-gray-900 rounded-3xl p-8 shadow-xl border border-gray-200 dark:border-gray-800"
      >
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
            Energetic
          </h2>
          <Badge variant="danger" size="sm" gradient={true}
            >cubic-bezier(0.68, -0.55, 0.27, 1.55)</Badge
          >
        </div>

        <p class="text-gray-600 dark:text-gray-400 mb-6">
          Dynamic, snappy motion with bounce. Great for gaming and playful
          interfaces.
        </p>

        <div class="flex flex-wrap gap-4">
          <Button
            variant="danger"
            size="lg"
            motion="energetic"
            shapeStyle="pill"
            gradient={true}
          >
            <i class="bi bi-fire mr-2"></i>
            Energetic Button
          </Button>

          <IconButton
            icon="bi-bolt-fill"
            variant="warning"
            size="xl"
            shapeStyle="circle"
            gradient={true}
            glowing={true}
            animated={true}
          />

          <Chip
            variant="danger"
            size="lg"
            gradient={true}
            shapeStyle="pill"
            animated={true}
            elevationLevel={3}
            icon="bi-fire"
          >
            Hot!
          </Chip>
        </div>

        <div
          class="mt-6 p-4 bg-red-50 dark:bg-red-950/30 rounded-xl border border-red-200 dark:border-red-800"
        >
          <code class="text-sm text-red-900 dark:text-red-300">
            transition: all 350ms cubic-bezier(0.68, -0.55, 0.27, 1.55)
          </code>
        </div>
      </div>
    </div>

    <!-- Scale Transforms -->
    <div
      class="bg-white dark:bg-gray-900 rounded-3xl p-8 shadow-xl border border-gray-200 dark:border-gray-800 mb-12"
    >
      <div class="flex items-center justify-between mb-6">
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
          Scale Transforms
        </h2>
        <Badge variant="gradient" size="sm" gradient={true}
          >Transform Effects</Badge
        >
      </div>

      <div class="grid grid-cols-2 md:grid-cols-4 gap-8">
        <!-- Hover Scale Up -->
        <div class="text-center">
          <div class="mb-4">
            <Card
              surfaceTone="muted"
              elevationLevel={2}
              shapeStyle="extra-rounded"
              class="cursor-pointer hover:scale-110 transition-transform duration-300"
            >
              <div class="p-8">
                <i class="bi bi-arrow-up-circle-fill text-4xl text-blue-600"
                ></i>
              </div>
            </Card>
          </div>
          <p class="text-sm font-semibold text-gray-700 dark:text-gray-300">
            Hover Scale 110%
          </p>
          <code class="text-xs text-gray-500">hover:scale-110</code>
        </div>

        <!-- Active Scale Down -->
        <div class="text-center">
          <div class="mb-4">
            <Card
              surfaceTone="emphasized"
              elevationLevel={2}
              shapeStyle="squircle"
              class="cursor-pointer hover:scale-105 active:scale-95 transition-transform duration-300"
            >
              <div class="p-8">
                <i class="bi bi-arrow-down-circle-fill text-4xl text-red-600"
                ></i>
              </div>
            </Card>
          </div>
          <p class="text-sm font-semibold text-gray-700 dark:text-gray-300">
            Active Scale 95%
          </p>
          <code class="text-xs text-gray-500">active:scale-95</code>
        </div>

        <!-- Rotate on Hover -->
        <div class="text-center">
          <div class="mb-4">
            <Card
              surfaceTone="base"
              elevationLevel={2}
              shapeStyle="ultra-rounded"
              class="cursor-pointer hover:rotate-6 hover:scale-110 transition-all duration-300"
            >
              <div class="p-8">
                <i class="bi bi-arrow-clockwise text-4xl text-green-600"></i>
              </div>
            </Card>
          </div>
          <p class="text-sm font-semibold text-gray-700 dark:text-gray-300">
            Rotate + Scale
          </p>
          <code class="text-xs text-gray-500">hover:rotate-6</code>
        </div>

        <!-- Pulse Animation -->
        <div class="text-center">
          <div class="mb-4">
            <Card
              surfaceTone="muted"
              elevationLevel={2}
              shapeStyle="extra-rounded"
              class="cursor-pointer animate-pulse"
            >
              <div class="p-8">
                <i class="bi bi-heart-pulse-fill text-4xl text-pink-600"></i>
              </div>
            </Card>
          </div>
          <p class="text-sm font-semibold text-gray-700 dark:text-gray-300">
            Pulse Animation
          </p>
          <code class="text-xs text-gray-500">animate-pulse</code>
        </div>
      </div>
    </div>

    <!-- Interactive Demo -->
    <div
      class="bg-gradient-to-br from-blue-600 to-purple-600 rounded-3xl p-12 shadow-2xl text-white mb-12"
    >
      <div class="text-center mb-8">
        <h2 class="text-3xl font-black mb-4">Interactive Animation Test</h2>
        <p class="text-blue-100 text-lg">
          Click the button to see the animation in action
        </p>
      </div>

      <div class="flex justify-center gap-8">
        <Button
          variant="secondary"
          size="xl"
          shapeStyle="extra-rounded"
          emphasized={true}
          onclick={triggerAnimation}
          class="shadow-2xl"
        >
          <i class="bi bi-play-fill text-2xl mr-2"></i>
          Trigger Animation
        </Button>

        <div
          class="w-24 h-24 bg-white rounded-3xl flex items-center justify-center shadow-2xl {isAnimating
            ? 'animate-bounce'
            : ''}"
        >
          <span class="text-4xl font-black text-blue-600">{counter}</span>
        </div>
      </div>
    </div>

    <!-- Animation Comparison -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- CSS Transitions -->
      <div
        class="bg-white dark:bg-gray-900 rounded-3xl p-6 shadow-xl border border-gray-200 dark:border-gray-800"
      >
        <div class="text-center mb-4">
          <div
            class="w-12 h-12 bg-gradient-to-br from-blue-500 to-purple-600 rounded-2xl mx-auto mb-3 flex items-center justify-center"
          >
            <i class="bi bi-speedometer2 text-white text-xl"></i>
          </div>
          <h3 class="text-xl font-bold text-gray-900 dark:text-white">
            CSS Transitions
          </h3>
        </div>
        <ul class="space-y-2 text-sm text-gray-600 dark:text-gray-400">
          <li class="flex items-start gap-2">
            <i class="bi bi-check-circle-fill text-green-500 mt-0.5"></i>
            <span>Lightweight & performant</span>
          </li>
          <li class="flex items-start gap-2">
            <i class="bi bi-check-circle-fill text-green-500 mt-0.5"></i>
            <span>Hardware accelerated</span>
          </li>
          <li class="flex items-start gap-2">
            <i class="bi bi-check-circle-fill text-green-500 mt-0.5"></i>
            <span>Easy to implement</span>
          </li>
        </ul>
      </div>

      <!-- Spring Physics -->
      <div
        class="bg-gradient-to-br from-green-500 to-emerald-600 rounded-3xl p-6 shadow-xl text-white"
      >
        <div class="text-center mb-4">
          <div
            class="w-12 h-12 bg-white/20 rounded-2xl mx-auto mb-3 flex items-center justify-center backdrop-blur-sm"
          >
            <i class="bi bi-stars text-2xl"></i>
          </div>
          <h3 class="text-xl font-bold">Spring Physics</h3>
        </div>
        <ul class="space-y-2 text-sm text-green-100">
          <li class="flex items-start gap-2">
            <i class="bi bi-stars mt-0.5"></i>
            <span>Natural, realistic motion</span>
          </li>
          <li class="flex items-start gap-2">
            <i class="bi bi-stars mt-0.5"></i>
            <span>Overshoot & bounce effects</span>
          </li>
          <li class="flex items-start gap-2">
            <i class="bi bi-stars mt-0.5"></i>
            <span>Enhanced user experience</span>
          </li>
        </ul>
      </div>

      <!-- Keyframe Animations -->
      <div
        class="bg-white dark:bg-gray-900 rounded-3xl p-6 shadow-xl border border-gray-200 dark:border-gray-800"
      >
        <div class="text-center mb-4">
          <div
            class="w-12 h-12 bg-gradient-to-br from-amber-500 to-orange-600 rounded-2xl mx-auto mb-3 flex items-center justify-center"
          >
            <i class="bi bi-film text-white text-xl"></i>
          </div>
          <h3 class="text-xl font-bold text-gray-900 dark:text-white">
            Keyframe Animations
          </h3>
        </div>
        <ul class="space-y-2 text-sm text-gray-600 dark:text-gray-400">
          <li class="flex items-start gap-2">
            <i class="bi bi-check-circle-fill text-orange-500 mt-0.5"></i>
            <span>Complex sequences</span>
          </li>
          <li class="flex items-start gap-2">
            <i class="bi bi-check-circle-fill text-orange-500 mt-0.5"></i>
            <span>Precise control</span>
          </li>
          <li class="flex items-start gap-2">
            <i class="bi bi-check-circle-fill text-orange-500 mt-0.5"></i>
            <span>Looping animations</span>
          </li>
        </ul>
      </div>
    </div>
  </div>
</div>
