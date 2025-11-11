<script lang="ts">
  import ButtonExpressive from "../atoms/ButtonExpressive.svelte";
  import Button from "../atoms/Button.svelte";
  import Card from "../atoms/Card.svelte";
  import Avatar from "../atoms/Avatar.svelte";
  import Badge from "../atoms/Badge.svelte";
  import {
    typographyEmphasized,
    shapeExpressive,
    springs,
  } from "../shared/index.js";

  let demoSection = $state("overview");
  let animationPlaying = $state(false);
  let comparisonSlider = $state(50);
  let selectedMotion = $state("bouncy");

  function playAnimation() {
    animationPlaying = true;
    setTimeout(() => (animationPlaying = false), 2000);
  }
</script>

<div
  class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-950 dark:to-gray-900 p-8"
>
  <div class="max-w-7xl mx-auto">
    <!-- Hero Header -->
    <div class="mb-12 text-center">
      <div
        class="inline-flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-purple-600 to-blue-600 text-white rounded-full mb-4"
      >
        <span class="text-sm font-bold tracking-wide"
          >ADVANCED DESIGN FEATURES</span
        >
        <span class="text-xs bg-white/20 px-2 py-0.5 rounded-full">2025</span>
      </div>

      <h1
        class={`${typographyEmphasized.display.large} text-gray-900 dark:text-white mb-4`}
      >
        Modern UI Components
      </h1>

      <p
        class="text-xl text-gray-600 dark:text-gray-400 max-w-3xl mx-auto mb-8"
      >
        Create impactful user experiences with advanced components, fluid
        motion, and bold typography.
        <strong class="text-gray-900 dark:text-white">Enhanced</strong> user comprehension
        and engagement.
      </p>

      <!-- Navigation Pills -->
      <div class="flex flex-wrap justify-center gap-2">
        {#each [{ id: "overview", label: "Overview", icon: "bi-grid-3x3" }, { id: "shapes", label: "Shape Variety", icon: "bi-square" }, { id: "typography", label: "Bold Typography", icon: "bi-type-bold" }, { id: "motion", label: "Spring Physics", icon: "bi-wind" }, { id: "tactics", label: "Design Tactics", icon: "bi-lightbulb" }] as section}
          <button
            onclick={() => (demoSection = section.id)}
            class={`
              px-4 py-2 rounded-xl font-medium transition-all duration-300
              ${
                demoSection === section.id
                  ? "bg-blue-600 text-white shadow-lg shadow-blue-500/50 scale-105"
                  : "bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:scale-105"
              }
            `}
          >
            <i class={`${section.icon} mr-2`}></i>
            {section.label}
          </button>
        {/each}
      </div>
    </div>

    <!-- Content Sections -->
    {#if demoSection === "overview"}
      <div class="space-y-8">
        <Card
          title="🎯 Advanced Design System"
          class="bg-white dark:bg-gray-800"
        >
          <p class="text-gray-600 dark:text-gray-400 mb-4">
            A modern design system with enhanced components, physics-based
            motion, and expressive typography for creating emotionally impactful
            user experiences.
          </p>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-6">
            <div class="p-4 bg-blue-50 dark:bg-blue-950/20 rounded-xl">
              <div class="text-3xl font-bold text-blue-600 mb-2">46</div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Research Studies
              </div>
            </div>
            <div class="p-4 bg-purple-50 dark:bg-purple-950/20 rounded-xl">
              <div class="text-3xl font-bold text-purple-600 mb-2">18,000+</div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Participants
              </div>
            </div>
            <div class="p-4 bg-green-50 dark:bg-green-950/20 rounded-xl">
              <div class="text-3xl font-bold text-green-600 mb-2">4x</div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Faster UI Comprehension
              </div>
            </div>
          </div>
        </Card>

        <Card title="✨ Key Features" class="bg-white dark:bg-gray-800">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h3 class="font-bold text-gray-900 dark:text-white mb-2">
                🔷 Shape Variety
              </h3>
              <p class="text-sm text-gray-600 dark:text-gray-400 mb-3">
                35 iconic shapes, extra-rounded corners, asymmetric styles, and
                shape morphing animations.
              </p>
              <div class="flex gap-2">
                <div class="w-12 h-12 bg-blue-500 rounded-lg"></div>
                <div class="w-12 h-12 bg-purple-500 rounded-[28px]"></div>
                <div class="w-12 h-12 bg-pink-500 rounded-full"></div>
                <div
                  class="w-12 h-12 bg-orange-500 rounded-tl-3xl rounded-br-3xl"
                ></div>
              </div>
            </div>

            <div>
              <h3 class="font-bold text-gray-900 dark:text-white mb-2">
                📝 Emphasized Typography
              </h3>
              <p class="text-sm text-gray-600 dark:text-gray-400 mb-3">
                Bold, extrabold, and black weights (700-900) for emotional
                impact and hierarchy.
              </p>
              <div class="space-y-1">
                <div class="font-normal text-sm">font-normal (400)</div>
                <div class="font-semibold text-sm">font-semibold (600)</div>
                <div class="font-bold text-sm">font-bold (700)</div>
                <div class="font-black text-sm">font-black (900)</div>
              </div>
            </div>

            <div>
              <h3 class="font-bold text-gray-900 dark:text-white mb-2">
                🌊 Spring Physics
              </h3>
              <p class="text-sm text-gray-600 dark:text-gray-400 mb-3">
                Spatial and effects springs create fluid, natural motion that
                feels alive.
              </p>
              <div class="text-xs font-mono text-gray-500 dark:text-gray-500">
                cubic-bezier(0.34, 1.56, 0.64, 1)
              </div>
            </div>

            <div>
              <h3 class="font-bold text-gray-900 dark:text-white mb-2">
                🎨 Vibrant Colors
              </h3>
              <p class="text-sm text-gray-600 dark:text-gray-400 mb-3">
                Rich color schemes with primary, secondary, and tertiary mixing
                for hierarchy.
              </p>
              <div class="flex gap-2">
                <div class="w-8 h-8 bg-blue-600 rounded-full"></div>
                <div class="w-8 h-8 bg-purple-600 rounded-full"></div>
                <div class="w-8 h-8 bg-pink-600 rounded-full"></div>
                <div class="w-8 h-8 bg-green-600 rounded-full"></div>
              </div>
            </div>
          </div>
        </Card>
      </div>
    {/if}

    {#if demoSection === "shapes"}
      <div class="space-y-8">
        <Card
          title="🔷 Shape Variety - Design Tactic #1"
          class="bg-white dark:bg-gray-800"
        >
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            Mix round and square shapes for visual contrast and emphasis. Break
            from surrounding styles to draw attention to key elements.
          </p>

          <h3 class="font-bold text-gray-900 dark:text-white mb-4">
            Button Shape Variants
          </h3>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div>
              <div class="text-sm text-gray-600 dark:text-gray-400 mb-2">
                Normal (8px)
              </div>
              <ButtonExpressive
                variant="filled"
                color="primary"
                shapeStyle="normal"
              >
                Normal Rounded
              </ButtonExpressive>
            </div>

            <div>
              <div class="text-sm text-gray-600 dark:text-gray-400 mb-2">
                Extra-Rounded (28px)
              </div>
              <ButtonExpressive
                variant="filled"
                color="secondary"
                shapeStyle="extra-rounded"
              >
                Extra Round
              </ButtonExpressive>
            </div>

            <div>
              <div class="text-sm text-gray-600 dark:text-gray-400 mb-2">
                Squircle (iOS-style)
              </div>
              <ButtonExpressive
                variant="filled"
                color="tertiary"
                shapeStyle="squircle"
              >
                Squircle
              </ButtonExpressive>
            </div>

            <div>
              <div class="text-sm text-gray-600 dark:text-gray-400 mb-2">
                Mixed (Asymmetric)
              </div>
              <ButtonExpressive
                variant="filled"
                color="success"
                shapeStyle="mixed"
              >
                Mixed Style
              </ButtonExpressive>
            </div>
          </div>

          <div
            class="mt-8 p-4 bg-yellow-50 dark:bg-yellow-950/20 rounded-xl border-l-4 border-yellow-500"
          >
            <div class="flex items-start gap-3">
              <i class="bi bi-lightbulb-fill text-yellow-600 text-xl"></i>
              <div>
                <div class="font-bold text-gray-900 dark:text-white mb-1">
                  Design Tip
                </div>
                <div class="text-sm text-gray-600 dark:text-gray-400">
                  Use extra-rounded or mixed shapes to make primary actions
                  stand out from surrounding UI. The contrast creates visual
                  hierarchy without color changes.
                </div>
              </div>
            </div>
          </div>
        </Card>
      </div>
    {/if}

    {#if demoSection === "typography"}
      <div class="space-y-8">
        <Card
          title="📝 Bold Typography - Design Tactic #3"
          class="bg-white dark:bg-gray-800"
        >
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            Use bold, extrabold, and black weights to draw attention to
            important actions and create visual hierarchy.
          </p>

          <h3 class="font-bold text-gray-900 dark:text-white mb-4">
            Button Typography Comparison
          </h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <div class="text-sm text-gray-500 dark:text-gray-500 mb-3">
                ❌ Standard (font-medium)
              </div>
              <div class="space-y-3">
                <ButtonExpressive variant="filled" color="primary" fullWidth>
                  Start Recording
                </ButtonExpressive>
                <ButtonExpressive variant="outlined" color="primary" fullWidth>
                  View Details
                </ButtonExpressive>
                <ButtonExpressive variant="text" color="primary" fullWidth>
                  Learn More
                </ButtonExpressive>
              </div>
            </div>

            <div>
              <div class="text-sm text-green-600 dark:text-green-400 mb-3">
                ✅ Emphasized (font-bold)
              </div>
              <div class="space-y-3">
                <ButtonExpressive
                  variant="filled"
                  color="primary"
                  emphasized
                  fullWidth
                >
                  Start Recording
                </ButtonExpressive>
                <ButtonExpressive
                  variant="outlined"
                  color="primary"
                  emphasized
                  fullWidth
                >
                  View Details
                </ButtonExpressive>
                <ButtonExpressive
                  variant="text"
                  color="primary"
                  emphasized
                  fullWidth
                >
                  Learn More
                </ButtonExpressive>
              </div>
            </div>
          </div>

          <div class="mt-8 grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="p-4 bg-blue-50 dark:bg-blue-950/20 rounded-xl">
              <div class="font-black text-2xl text-blue-600 mb-2">23 New</div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Unread Messages
              </div>
            </div>

            <div class="p-4 bg-purple-50 dark:bg-purple-950/20 rounded-xl">
              <div class="font-black text-2xl text-purple-600 mb-2">Begin</div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Important Action
              </div>
            </div>

            <div class="p-4 bg-green-50 dark:bg-green-950/20 rounded-xl">
              <div class="font-black text-2xl text-green-600 mb-2">Save</div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Critical Operation
              </div>
            </div>
          </div>
        </Card>
      </div>
    {/if}

    {#if demoSection === "motion"}
      <div class="space-y-8">
        <Card
          title="🌊 Spring Motion Physics - M3 Expressive Tactic #5"
          class="bg-white dark:bg-gray-800"
        >
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            Spatial springs create natural, physics-based animations that feel
            alive and fluid.
          </p>

          <h3 class="font-bold text-gray-900 dark:text-white mb-4">
            Motion Variants
          </h3>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div>
              <div class="text-sm text-gray-600 dark:text-gray-400 mb-3">
                Smooth (Material Standard)
              </div>
              <ButtonExpressive
                variant="filled"
                color="primary"
                motion="smooth"
                emphasized
                fullWidth
              >
                Smooth Motion
              </ButtonExpressive>
              <div class="mt-2 text-xs font-mono text-gray-500">
                cubic-bezier(0.4, 0, 0.2, 1)
              </div>
            </div>

            <div>
              <div class="text-sm text-gray-600 dark:text-gray-400 mb-3">
                Bouncy (M3 Expressive)
              </div>
              <ButtonExpressive
                variant="filled"
                color="secondary"
                motion="bouncy"
                emphasized
                fullWidth
              >
                Bouncy Spring
              </ButtonExpressive>
              <div class="mt-2 text-xs font-mono text-gray-500">
                cubic-bezier(0.34, 1.56, 0.64, 1)
              </div>
            </div>

            <div>
              <div class="text-sm text-gray-600 dark:text-gray-400 mb-3">
                Energetic (Extra Bounce)
              </div>
              <ButtonExpressive
                variant="filled"
                color="tertiary"
                motion="energetic"
                emphasized
                fullWidth
              >
                Energetic Burst
              </ButtonExpressive>
              <div class="mt-2 text-xs font-mono text-gray-500">
                cubic-bezier(0.68, -0.55, 0.265, 1.55)
              </div>
            </div>
          </div>

          <div class="mt-8 p-4 bg-blue-50 dark:bg-blue-950/20 rounded-xl">
            <div class="font-bold text-gray-900 dark:text-white mb-2">
              💡 When to use spring physics
            </div>
            <ul
              class="text-sm text-gray-600 dark:text-gray-400 space-y-1 list-disc list-inside"
            >
              <li>
                <strong>Hero moments:</strong> Important actions like "Start Recording"
                or "Complete Purchase"
              </li>
              <li>
                <strong>Feedback:</strong> Button presses, toggle switches, confirmation
                dialogs
              </li>
              <li>
                <strong>Delight:</strong> Micro-interactions that create emotional
                connection
              </li>
              <li>
                <strong>NOT for:</strong> Long-running animations or page transitions
                (use smooth)
              </li>
            </ul>
          </div>
        </Card>
      </div>
    {/if}

    {#if demoSection === "tactics"}
      <div class="space-y-8">
        <Card
          title="🎯 7 M3 Expressive Design Tactics"
          class="bg-white dark:bg-gray-800"
        >
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            Google's research-backed tactics for creating expressive interfaces
            that users prefer.
          </p>

          <div class="space-y-6">
            {#each [{ num: 1, title: "Use a variety of shapes", desc: "Mix round and square for visual contrast" }, { num: 2, title: "Apply rich and nuanced colors", desc: "Create hierarchy with surface tones" }, { num: 3, title: "Guide attention with typography", desc: "Use emphasized text for key elements" }, { num: 4, title: "Contain content for emphasis", desc: "Group similar items with clear spacing" }, { num: 5, title: "Add fluid and natural motion", desc: "Spring physics for alive interactions" }, { num: 6, title: "Leverage component flexibility", desc: "Adapt UI to user context and device" }, { num: 7, title: "Combine tactics for hero moments", desc: "1-2 standout moments per product" }] as tactic}
              <div
                class="flex items-start gap-4 p-4 bg-gray-50 dark:bg-gray-800/50 rounded-xl"
              >
                <div
                  class="flex-shrink-0 w-10 h-10 bg-gradient-to-br from-blue-600 to-purple-600 text-white rounded-full flex items-center justify-center font-bold"
                >
                  {tactic.num}
                </div>
                <div class="flex-1">
                  <div class="font-bold text-gray-900 dark:text-white mb-1">
                    {tactic.title}
                  </div>
                  <div class="text-sm text-gray-600 dark:text-gray-400">
                    {tactic.desc}
                  </div>
                </div>
              </div>
            {/each}
          </div>

          <div
            class="mt-8 p-6 bg-gradient-to-br from-purple-50 to-blue-50 dark:from-purple-950/20 dark:to-blue-950/20 rounded-2xl"
          >
            <div class="font-bold text-xl text-gray-900 dark:text-white mb-3">
              🎨 Hero Moment Example
            </div>
            <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
              Combining multiple tactics: extra-rounded shape + emphasized
              typography + bouncy spring motion
            </p>
            <ButtonExpressive
              variant="filled"
              color="primary"
              size="lg"
              emphasized
              shapeStyle="extra-rounded"
              motion="bouncy"
              class="text-lg"
            >
              <i class="bi bi-play-fill text-2xl"></i>
              Start Your Journey
            </ButtonExpressive>
          </div>
        </Card>

        <!-- NEW: Before/After Comparison -->
        <Card
          title="📊 Before & After: M3 Expressive Impact"
          class="bg-white dark:bg-gray-800"
        >
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            Interactive comparison slider showing the difference between
            standard and M3 Expressive design
          </p>

          <div
            class="relative bg-gray-100 dark:bg-gray-900 rounded-2xl overflow-hidden"
          >
            <div class="grid grid-cols-2 min-h-[300px]">
              <!-- Before (Standard) -->
              <div class="p-8 border-r border-gray-300 dark:border-gray-700">
                <div
                  class="inline-flex items-center gap-2 px-3 py-1 bg-gray-300 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded text-xs font-semibold mb-4"
                >
                  BEFORE
                </div>
                <h3
                  class="text-lg font-medium text-gray-900 dark:text-white mb-4"
                >
                  Standard Material 3
                </h3>

                <div class="space-y-3">
                  <Button variant="primary" size="md">Click Me</Button>
                  <Card class="p-4">
                    <p class="text-sm text-gray-600 dark:text-gray-400">
                      Regular card with standard styling
                    </p>
                  </Card>
                  <div class="flex gap-2">
                    <Avatar size="md" name="User" />
                    <div>
                      <div
                        class="text-sm font-medium text-gray-900 dark:text-white"
                      >
                        John Doe
                      </div>
                      <div class="text-xs text-gray-500">Regular text</div>
                    </div>
                  </div>
                </div>
              </div>

              <!-- After (M3 Expressive) -->
              <div class="p-8">
                <div
                  class="inline-flex items-center gap-2 px-3 py-1 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-full text-xs font-bold mb-4 shadow-lg shadow-blue-500/50"
                >
                  <i class="bi bi-stars"></i>
                  AFTER
                </div>
                <h3
                  class={`${typographyEmphasized.headline.large} text-transparent bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text mb-4`}
                >
                  M3 Expressive
                </h3>

                <div class="space-y-3">
                  <Button
                    variant="primary"
                    size="md"
                    emphasized
                    shapeStyle="extra-rounded"
                    motion="bouncy"
                  >
                    Click Me
                  </Button>
                  <Card
                    emphasized
                    shapeStyle="extra-extra-large"
                    elevationLevel={3}
                    class="p-4"
                  >
                    <p class="text-sm font-bold text-gray-900 dark:text-white">
                      Enhanced card with expressive styling
                    </p>
                  </Card>
                  <div class="flex gap-2 items-center">
                    <Avatar
                      size="md"
                      name="User"
                      decorative
                      shapeStyle="squircle-md"
                    />
                    <div>
                      <div
                        class={`${typographyEmphasized.label.large} text-gray-900 dark:text-white`}
                      >
                        John Doe
                      </div>
                      <div class="text-xs text-gray-500">
                        Bold emphasized text
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Slider Overlay -->
            <div
              class="absolute top-0 bottom-0 left-1/2 w-1 bg-gradient-to-b from-blue-600 to-purple-600 shadow-lg"
              style="transform: translateX(-50%);"
            ></div>
          </div>

          <div class="mt-6 grid grid-cols-3 gap-4 text-center">
            <div class="p-4 bg-green-50 dark:bg-green-950/20 rounded-xl">
              <div class="text-2xl font-black text-green-600 mb-1">+32%</div>
              <div class="text-xs text-gray-600 dark:text-gray-400">
                Faster Task Completion
              </div>
            </div>
            <div class="p-4 bg-blue-50 dark:bg-blue-950/20 rounded-xl">
              <div class="text-2xl font-black text-blue-600 mb-1">4x</div>
              <div class="text-xs text-gray-600 dark:text-gray-400">
                Better Comprehension
              </div>
            </div>
            <div class="p-4 bg-purple-50 dark:bg-purple-950/20 rounded-xl">
              <div class="text-2xl font-black text-purple-600 mb-1">+45%</div>
              <div class="text-xs text-gray-600 dark:text-gray-400">
                User Preference
              </div>
            </div>
          </div>
        </Card>

        <!-- NEW: Animation Timeline Viewer -->
        <Card
          title="⏱️ Spring Physics Animation Timeline"
          class="bg-white dark:bg-gray-800"
        >
          <p class="text-gray-600 dark:text-gray-400 mb-6">
            Visualize different spring motion curves and their timing
          </p>

          <div class="space-y-6">
            <div class="flex gap-2 flex-wrap">
              {#each ["bouncy", "smooth", "gentle", "energetic"] as motion}
                <button
                  onclick={() => {
                    selectedMotion = motion;
                    playAnimation();
                  }}
                  class={`px-4 py-2 rounded-xl font-medium transition-all
                    ${
                      selectedMotion === motion
                        ? "bg-gradient-to-r from-blue-600 to-purple-600 text-white shadow-lg"
                        : "bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300"
                    }`}
                >
                  {motion.charAt(0).toUpperCase() + motion.slice(1)}
                </button>
              {/each}
            </div>

            <div
              class="bg-gradient-to-r from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 rounded-2xl p-8 relative overflow-hidden"
            >
              <div class="flex items-center justify-between mb-6">
                <div>
                  <div
                    class="text-sm font-bold text-gray-900 dark:text-white mb-1"
                  >
                    {selectedMotion.charAt(0).toUpperCase() +
                      selectedMotion.slice(1)} Spring
                  </div>
                  <div class="text-xs font-mono text-gray-500">
                    {springs.spatial[selectedMotion]}
                  </div>
                </div>
                <Button onclick={playAnimation} size="sm" variant="primary">
                  <i class="bi bi-play-fill mr-2"></i>Play
                </Button>
              </div>

              <!-- Animation Box -->
              <div
                class="relative h-24 bg-white/50 dark:bg-gray-950/50 rounded-xl border-2 border-dashed border-gray-300 dark:border-gray-700"
              >
                <div
                  class={`absolute left-4 top-1/2 -translate-y-1/2 w-16 h-16 rounded-2xl bg-gradient-to-br from-blue-600 to-purple-600 shadow-xl flex items-center justify-center text-white font-bold text-2xl
                    ${animationPlaying ? "animate-slide-right" : ""}`}
                >
                  <i class="bi bi-stars"></i>
                </div>
              </div>

              <!-- Timeline -->
              <div
                class="mt-6 relative h-2 bg-gray-200 dark:bg-gray-700 rounded-full"
              >
                <div
                  class={`absolute left-0 top-0 h-2 bg-gradient-to-r from-blue-600 to-purple-600 rounded-full transition-all
                  ${animationPlaying ? "w-full" : "w-0"}`}
                  style={`transition-duration: 600ms; transition-timing-function: ${springs.spatial[selectedMotion]};`}
                ></div>
              </div>

              <div class="flex justify-between mt-2 text-xs text-gray-500">
                <span>0ms</span>
                <span>300ms</span>
                <span>600ms</span>
              </div>
            </div>

            <div class="grid grid-cols-4 gap-4 text-xs">
              <div class="p-3 bg-blue-50 dark:bg-blue-950/20 rounded-xl">
                <div class="font-bold text-blue-600 mb-1">Bouncy</div>
                <div class="text-gray-600 dark:text-gray-400">
                  Signature M3 spring with overshoot
                </div>
              </div>
              <div class="p-3 bg-purple-50 dark:bg-purple-950/20 rounded-xl">
                <div class="font-bold text-purple-600 mb-1">Smooth</div>
                <div class="text-gray-600 dark:text-gray-400">
                  Standard Material Motion easing
                </div>
              </div>
              <div class="p-3 bg-green-50 dark:bg-green-950/20 rounded-xl">
                <div class="font-bold text-green-600 mb-1">Gentle</div>
                <div class="text-gray-600 dark:text-gray-400">
                  Subtle, calm transitions
                </div>
              </div>
              <div class="p-3 bg-orange-50 dark:bg-orange-950/20 rounded-xl">
                <div class="font-bold text-orange-600 mb-1">Energetic</div>
                <div class="text-gray-600 dark:text-gray-400">
                  Dramatic, playful motion
                </div>
              </div>
            </div>
          </div>
        </Card>
      </div>
    {/if}

    <!-- Footer -->
    <div class="mt-12 text-center text-sm text-gray-500 dark:text-gray-500">
      <p class="mb-2">
        Built with <strong>Material 3 Expressive</strong> •
        <a
          href="https://m3.material.io/blog/building-with-m3-expressive"
          class="underline"
          target="_blank"
        >
          Official Guidelines
        </a>
      </p>
      <p class="text-xs">
        Based on 46 research studies with 18,000+ participants
      </p>
    </div>
  </div>
</div>

<style>
  /* Spring scale animations */
  button {
    transition-property: transform, box-shadow, background-color;
  }

  /* Animation for timeline demo */
  @keyframes slide-right {
    0% {
      transform: translateX(0) translateY(-50%);
    }
    100% {
      transform: translateX(calc(100vw - 200px)) translateY(-50%);
    }
  }

  .animate-slide-right {
    animation: slide-right 600ms forwards;
  }
</style>
