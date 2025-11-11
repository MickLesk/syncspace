<script>
  import Card from "../atoms/Card.svelte";
  import Divider from "../atoms/Divider.svelte";
  import Chip from "../atoms/Chip.svelte";

  // Typography scale examples
  const fontSizes = [
    { name: "text-xs", size: "0.75rem (12px)", example: "Extra small text" },
    { name: "text-sm", size: "0.875rem (14px)", example: "Small text" },
    { name: "text-base", size: "1rem (16px)", example: "Base text" },
    { name: "text-lg", size: "1.125rem (18px)", example: "Large text" },
    { name: "text-xl", size: "1.25rem (20px)", example: "Extra large text" },
    { name: "text-2xl", size: "1.5rem (24px)", example: "2XL heading" },
    { name: "text-3xl", size: "1.875rem (30px)", example: "3XL heading" },
    { name: "text-4xl", size: "2.25rem (36px)", example: "4XL heading" },
    { name: "text-5xl", size: "3rem (48px)", example: "5XL display" },
    { name: "text-6xl", size: "3.75rem (60px)", example: "6XL display" },
  ];

  const fontWeights = [
    { name: "font-light", weight: "300", example: "Light weight text" },
    { name: "font-normal", weight: "400", example: "Normal weight text" },
    { name: "font-medium", weight: "500", example: "Medium weight text" },
    { name: "font-semibold", weight: "600", example: "Semibold weight text" },
    { name: "font-bold", weight: "700", example: "Bold weight text" },
    { name: "font-extrabold", weight: "800", example: "Extra bold text" },
    { name: "font-black", weight: "900", example: "Black weight text" },
  ];

  const lineHeights = [
    { name: "leading-none", value: "1", example: "No line height spacing" },
    { name: "leading-tight", value: "1.25", example: "Tight line height" },
    { name: "leading-snug", value: "1.375", example: "Snug line height" },
    { name: "leading-normal", value: "1.5", example: "Normal line height" },
    { name: "leading-relaxed", value: "1.625", example: "Relaxed spacing" },
    { name: "leading-loose", value: "2", example: "Loose line height" },
  ];

  const letterSpacings = [
    { name: "tracking-tighter", value: "-0.05em", example: "Tighter spacing" },
    { name: "tracking-tight", value: "-0.025em", example: "Tight spacing" },
    { name: "tracking-normal", value: "0em", example: "Normal spacing" },
    { name: "tracking-wide", value: "0.025em", example: "Wide spacing" },
    { name: "tracking-wider", value: "0.05em", example: "Wider spacing" },
    { name: "tracking-widest", value: "0.1em", example: "Widest spacing" },
  ];
</script>

<div
  class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 p-8"
>
  <div class="max-w-7xl mx-auto space-y-8">
    <!-- Hero Section -->
    <div class="text-center space-y-4">
      <div
        class="inline-flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-purple-500 to-pink-500 text-white rounded-full text-sm font-semibold shadow-lg shadow-purple-500/30"
      >
        <i class="bi bi-type text-lg"></i>
        <span>Typography System</span>
      </div>
      <h1
        class="text-6xl font-black bg-gradient-to-r from-purple-600 via-pink-600 to-blue-600 bg-clip-text text-transparent"
      >
        Typography
      </h1>
      <p class="text-xl text-gray-600 dark:text-gray-300 max-w-3xl mx-auto">
        Comprehensive typography system with font sizes, weights, line heights,
        and spacing utilities
      </p>
    </div>

    <Divider style="gradient" thickness={2} />

    <!-- Font Sizes -->
    <div class="space-y-6">
      <div>
        <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
          Font Sizes
        </h2>
        <p class="text-gray-600 dark:text-gray-400">
          Complete scale from extra small to display sizes
        </p>
      </div>

      <Card shapeStyle="extra-rounded" elevationLevel={1}>
        <div class="space-y-4">
          {#each fontSizes as size}
            <div
              class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-800 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
            >
              <div class="space-y-1">
                <Chip size="sm" outlined>{size.name}</Chip>
                <p class="text-sm text-gray-500 dark:text-gray-400">
                  {size.size}
                </p>
              </div>
              <p class="{size.name} font-medium text-gray-900 dark:text-white">
                {size.example}
              </p>
            </div>
          {/each}
        </div>
      </Card>
    </div>

    <Divider />

    <!-- Font Weights -->
    <div class="space-y-6">
      <div>
        <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
          Font Weights
        </h2>
        <p class="text-gray-600 dark:text-gray-400">
          From light to black for visual hierarchy
        </p>
      </div>

      <Card shapeStyle="extra-rounded" elevationLevel={1}>
        <div class="space-y-4">
          {#each fontWeights as weight}
            <div
              class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-800 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
            >
              <div class="space-y-1">
                <Chip size="sm" outlined>{weight.name}</Chip>
                <p class="text-sm text-gray-500 dark:text-gray-400">
                  Weight: {weight.weight}
                </p>
              </div>
              <p class="text-2xl {weight.name} text-gray-900 dark:text-white">
                {weight.example}
              </p>
            </div>
          {/each}
        </div>
      </Card>
    </div>

    <Divider />

    <!-- Heading Hierarchy -->
    <div class="space-y-6">
      <div>
        <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
          Heading Hierarchy
        </h2>
        <p class="text-gray-600 dark:text-gray-400">
          Semantic heading levels with recommended sizes
        </p>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card shapeStyle="extra-rounded" elevationLevel={1}>
          <div class="space-y-6">
            <div>
              <Chip size="sm" gradient emphasized>H1 - Hero</Chip>
              <h1
                class="text-6xl font-black text-gray-900 dark:text-white mt-3"
              >
                Main Hero Heading
              </h1>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-2">
                text-6xl font-black
              </p>
            </div>

            <Divider thickness={1} />

            <div>
              <Chip size="sm" gradient emphasized>H2 - Section</Chip>
              <h2 class="text-4xl font-bold text-gray-900 dark:text-white mt-3">
                Section Heading
              </h2>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-2">
                text-4xl font-bold
              </p>
            </div>

            <Divider thickness={1} />

            <div>
              <Chip size="sm" gradient emphasized>H3 - Subsection</Chip>
              <h3 class="text-3xl font-bold text-gray-900 dark:text-white mt-3">
                Subsection Heading
              </h3>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-2">
                text-3xl font-bold
              </p>
            </div>
          </div>
        </Card>

        <Card shapeStyle="extra-rounded" elevationLevel={1}>
          <div class="space-y-6">
            <div>
              <Chip size="sm" gradient emphasized>H4 - Component</Chip>
              <h4
                class="text-2xl font-semibold text-gray-900 dark:text-white mt-3"
              >
                Component Heading
              </h4>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-2">
                text-2xl font-semibold
              </p>
            </div>

            <Divider thickness={1} />

            <div>
              <Chip size="sm" gradient emphasized>H5 - Card</Chip>
              <h5
                class="text-xl font-semibold text-gray-900 dark:text-white mt-3"
              >
                Card Heading
              </h5>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-2">
                text-xl font-semibold
              </p>
            </div>

            <Divider thickness={1} />

            <div>
              <Chip size="sm" gradient emphasized>H6 - Small</Chip>
              <h6
                class="text-lg font-semibold text-gray-900 dark:text-white mt-3"
              >
                Small Heading
              </h6>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-2">
                text-lg font-semibold
              </p>
            </div>
          </div>
        </Card>
      </div>
    </div>

    <Divider />

    <!-- Line Heights -->
    <div class="space-y-6">
      <div>
        <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
          Line Heights
        </h2>
        <p class="text-gray-600 dark:text-gray-400">
          Control vertical spacing between lines
        </p>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {#each lineHeights as lineHeight}
          <Card shapeStyle="extra-rounded" elevationLevel={1}>
            <div class="space-y-3">
              <div class="flex items-center justify-between">
                <Chip size="sm" gradient>{lineHeight.name}</Chip>
                <span class="text-xs text-gray-500 dark:text-gray-400"
                  >{lineHeight.value}</span
                >
              </div>
              <p
                class="text-base {lineHeight.name} text-gray-700 dark:text-gray-300"
              >
                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do
                eiusmod tempor incididunt ut labore et dolore magna aliqua.
              </p>
            </div>
          </Card>
        {/each}
      </div>
    </div>

    <Divider />

    <!-- Letter Spacing -->
    <div class="space-y-6">
      <div>
        <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
          Letter Spacing
        </h2>
        <p class="text-gray-600 dark:text-gray-400">
          Adjust horizontal spacing between characters
        </p>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {#each letterSpacings as spacing}
          <Card shapeStyle="extra-rounded" elevationLevel={1}>
            <div class="space-y-3">
              <div class="flex items-center justify-between">
                <Chip size="sm" gradient>{spacing.name}</Chip>
                <span class="text-xs text-gray-500 dark:text-gray-400"
                  >{spacing.value}</span
                >
              </div>
              <p
                class="text-xl font-semibold {spacing.name} text-gray-900 dark:text-white"
              >
                {spacing.example}
              </p>
            </div>
          </Card>
        {/each}
      </div>
    </div>

    <Divider />

    <!-- Text Styles -->
    <div class="space-y-6">
      <div>
        <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
          Common Text Styles
        </h2>
        <p class="text-gray-600 dark:text-gray-400">
          Pre-configured styles for common use cases
        </p>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Body Text -->
        <Card shapeStyle="extra-rounded" elevationLevel={1}>
          <div class="space-y-4">
            <Chip size="sm" gradient emphasized>Body Text</Chip>
            <p
              class="text-base leading-relaxed text-gray-700 dark:text-gray-300"
            >
              This is regular body text with comfortable line height
              (leading-relaxed). It's designed for readability and comfortable
              reading experience. Perfect for paragraphs and longer content
              sections.
            </p>
            <p class="text-sm text-gray-500 dark:text-gray-400 font-mono">
              text-base leading-relaxed
            </p>
          </div>
        </Card>

        <!-- Caption Text -->
        <Card shapeStyle="extra-rounded" elevationLevel={1}>
          <div class="space-y-4">
            <Chip size="sm" gradient emphasized>Caption Text</Chip>
            <p class="text-sm leading-normal text-gray-600 dark:text-gray-400">
              Smaller caption text for image descriptions, footnotes, or
              secondary information. Uses a tighter line height for compact
              display.
            </p>
            <p class="text-sm text-gray-500 dark:text-gray-400 font-mono">
              text-sm leading-normal
            </p>
          </div>
        </Card>

        <!-- Display Text -->
        <Card shapeStyle="extra-rounded" elevationLevel={1}>
          <div class="space-y-4">
            <Chip size="sm" gradient emphasized>Display Text</Chip>
            <p
              class="text-5xl font-black leading-tight text-gray-900 dark:text-white"
            >
              Big Impact
            </p>
            <p class="text-sm text-gray-500 dark:text-gray-400 font-mono">
              text-5xl font-black leading-tight
            </p>
          </div>
        </Card>

        <!-- Code Text -->
        <Card shapeStyle="extra-rounded" elevationLevel={1}>
          <div class="space-y-4">
            <Chip size="sm" gradient emphasized>Code Text</Chip>
            <code
              class="block text-sm font-mono bg-gray-900 dark:bg-gray-950 text-green-400 p-4 rounded-lg"
            >
              const greeting = "Hello World";<br />
              console.log(greeting);
            </code>
            <p class="text-sm text-gray-500 dark:text-gray-400 font-mono">
              font-mono bg-gray-900
            </p>
          </div>
        </Card>
      </div>
    </div>

    <Divider />

    <!-- Gradient Text -->
    <div class="space-y-6">
      <div>
        <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
          Gradient Text
        </h2>
        <p class="text-gray-600 dark:text-gray-400">
          Eye-catching gradient text effects
        </p>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card shapeStyle="extra-rounded" elevationLevel={1}>
          <div class="space-y-4">
            <h3
              class="text-4xl font-black bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent"
            >
              Purple to Pink
            </h3>
            <p class="text-sm text-gray-500 dark:text-gray-400 font-mono">
              bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text
              text-transparent
            </p>
          </div>
        </Card>

        <Card shapeStyle="extra-rounded" elevationLevel={1}>
          <div class="space-y-4">
            <h3
              class="text-4xl font-black bg-gradient-to-r from-blue-600 to-cyan-600 bg-clip-text text-transparent"
            >
              Blue to Cyan
            </h3>
            <p class="text-sm text-gray-500 dark:text-gray-400 font-mono">
              bg-gradient-to-r from-blue-600 to-cyan-600 bg-clip-text
              text-transparent
            </p>
          </div>
        </Card>

        <Card shapeStyle="extra-rounded" elevationLevel={1}>
          <div class="space-y-4">
            <h3
              class="text-4xl font-black bg-gradient-to-r from-green-600 to-emerald-600 bg-clip-text text-transparent"
            >
              Green to Emerald
            </h3>
            <p class="text-sm text-gray-500 dark:text-gray-400 font-mono">
              bg-gradient-to-r from-green-600 to-emerald-600 bg-clip-text
              text-transparent
            </p>
          </div>
        </Card>

        <Card shapeStyle="extra-rounded" elevationLevel={1}>
          <div class="space-y-4">
            <h3
              class="text-4xl font-black bg-gradient-to-r from-orange-600 to-red-600 bg-clip-text text-transparent"
            >
              Orange to Red
            </h3>
            <p class="text-sm text-gray-500 dark:text-gray-400 font-mono">
              bg-gradient-to-r from-orange-600 to-red-600 bg-clip-text
              text-transparent
            </p>
          </div>
        </Card>
      </div>
    </div>

    <Divider />

    <!-- Typography Best Practices -->
    <div class="space-y-6">
      <div>
        <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
          Best Practices
        </h2>
        <p class="text-gray-600 dark:text-gray-400">
          Guidelines for effective typography
        </p>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <Card shapeStyle="extra-rounded" elevationLevel={2} gradient>
          <div class="space-y-3">
            <div
              class="w-12 h-12 bg-gradient-to-br from-purple-500 to-pink-500 rounded-xl flex items-center justify-center text-white text-2xl"
            >
              <i class="bi bi-eye"></i>
            </div>
            <h3 class="text-xl font-bold text-gray-900 dark:text-white">
              Readability
            </h3>
            <p class="text-gray-600 dark:text-gray-400">
              Use 16px base size, 1.5-1.6 line height for body text, and
              sufficient color contrast (WCAG AA minimum).
            </p>
          </div>
        </Card>

        <Card shapeStyle="extra-rounded" elevationLevel={2} gradient>
          <div class="space-y-3">
            <div
              class="w-12 h-12 bg-gradient-to-br from-blue-500 to-cyan-500 rounded-xl flex items-center justify-center text-white text-2xl"
            >
              <i class="bi bi-stack"></i>
            </div>
            <h3 class="text-xl font-bold text-gray-900 dark:text-white">
              Hierarchy
            </h3>
            <p class="text-gray-600 dark:text-gray-400">
              Create clear visual hierarchy with size, weight, and color
              variations. Limit to 2-3 font weights per project.
            </p>
          </div>
        </Card>

        <Card shapeStyle="extra-rounded" elevationLevel={2} gradient>
          <div class="space-y-3">
            <div
              class="w-12 h-12 bg-gradient-to-br from-green-500 to-emerald-500 rounded-xl flex items-center justify-center text-white text-2xl"
            >
              <i class="bi bi-rulers"></i>
            </div>
            <h3 class="text-xl font-bold text-gray-900 dark:text-white">
              Consistency
            </h3>
            <p class="text-gray-600 dark:text-gray-400">
              Stick to a consistent type scale (1.25 or 1.333 ratio). Use
              semantic HTML tags (h1-h6, p, code).
            </p>
          </div>
        </Card>
      </div>
    </div>
  </div>
</div>
