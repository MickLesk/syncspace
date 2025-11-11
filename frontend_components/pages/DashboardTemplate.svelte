<script lang="ts">
  import Card from "../atoms/Card.svelte";
  import Button from "../atoms/Button.svelte";
  import Avatar from "../atoms/Avatar.svelte";
  import Badge from "../atoms/Badge.svelte";
  import Input from "../atoms/Input.svelte";
  import { typographyEmphasized } from "../shared/index.js";

  let sidebarOpen = $state(true);
  let searchQuery = $state("");

  const stats = [
    {
      label: "Total Users",
      value: "12,458",
      change: "+12.5%",
      trend: "up",
      icon: "bi-people-fill",
      color: "blue",
    },
    {
      label: "Revenue",
      value: "$48,932",
      change: "+8.2%",
      trend: "up",
      icon: "bi-currency-dollar",
      color: "green",
    },
    {
      label: "Active Projects",
      value: "142",
      change: "-3.1%",
      trend: "down",
      icon: "bi-folder-fill",
      color: "purple",
    },
    {
      label: "Tasks Completed",
      value: "1,847",
      change: "+22.4%",
      trend: "up",
      icon: "bi-check-circle-fill",
      color: "orange",
    },
  ];

  const recentActivity = [
    {
      user: "Sarah Johnson",
      action: "completed task",
      project: "Website Redesign",
      time: "2 min ago",
      avatar: "SJ",
    },
    {
      user: "Mike Chen",
      action: "uploaded file",
      project: "Mobile App",
      time: "15 min ago",
      avatar: "MC",
    },
    {
      user: "Emma Wilson",
      action: "created issue",
      project: "Backend API",
      time: "1 hour ago",
      avatar: "EW",
    },
    {
      user: "David Brown",
      action: "commented",
      project: "Dashboard",
      time: "2 hours ago",
      avatar: "DB",
    },
  ];

  const projects = [
    {
      name: "Website Redesign",
      progress: 75,
      tasks: "18/24",
      team: 5,
      color: "blue",
      status: "On Track",
    },
    {
      name: "Mobile App",
      progress: 45,
      tasks: "12/32",
      team: 8,
      color: "purple",
      status: "At Risk",
    },
    {
      name: "Backend API",
      progress: 90,
      tasks: "27/30",
      team: 4,
      color: "green",
      status: "Ahead",
    },
    {
      name: "Dashboard",
      progress: 30,
      tasks: "6/20",
      team: 6,
      color: "orange",
      status: "Behind",
    },
  ];

  const quickActions = [
    { label: "New Project", icon: "bi-folder-plus", color: "blue" },
    { label: "Add Task", icon: "bi-plus-circle", color: "green" },
    { label: "Upload File", icon: "bi-cloud-upload", color: "purple" },
    { label: "Send Message", icon: "bi-chat-dots", color: "orange" },
  ];
</script>

<div
  class="min-h-screen bg-gradient-to-br from-slate-50 via-blue-50 to-purple-50 dark:from-slate-950 dark:via-blue-950 dark:to-purple-950"
>
  <!-- Modern Sidebar -->
  <aside
    class={`fixed left-0 top-0 h-screen bg-white/80 dark:bg-slate-900/80 backdrop-blur-xl border-r border-slate-200 dark:border-slate-800 transition-all duration-300 z-50 ${sidebarOpen ? "w-64" : "w-20"}`}
  >
    <!-- Logo -->
    <div
      class="flex items-center gap-3 px-6 py-5 border-b border-slate-200 dark:border-slate-800"
    >
      <div
        class="w-10 h-10 rounded-xl bg-gradient-to-br from-blue-600 to-purple-600 flex items-center justify-center shadow-lg shadow-blue-500/30 flex-shrink-0"
      >
        <i class="bi bi-lightning-charge-fill text-white text-xl"></i>
      </div>
      {#if sidebarOpen}
        <div>
          <h1
            class="text-lg font-black bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent"
          >
            SyncSpace
          </h1>
          <p
            class="text-[10px] text-slate-500 dark:text-slate-400 font-semibold"
          >
            DASHBOARD
          </p>
        </div>
      {/if}
    </div>

    <!-- Navigation -->
    <nav class="p-4 space-y-2">
      {#each [{ label: "Overview", icon: "bi-grid-fill", active: true }, { label: "Projects", icon: "bi-folder-fill" }, { label: "Tasks", icon: "bi-check-square" }, { label: "Team", icon: "bi-people-fill" }, { label: "Analytics", icon: "bi-graph-up" }, { label: "Settings", icon: "bi-gear-fill" }] as item}
        <button
          class={`w-full flex items-center gap-3 px-4 py-3 rounded-xl transition-all duration-200 ${item.active ? "bg-gradient-to-r from-blue-600 to-purple-600 text-white shadow-lg shadow-blue-500/30" : "text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800"}`}
        >
          <i class={`${item.icon} text-xl flex-shrink-0`}></i>
          {#if sidebarOpen}
            <span class="font-semibold text-sm">{item.label}</span>
          {/if}
        </button>
      {/each}
    </nav>

    <!-- Toggle Button -->
    <button
      onclick={() => (sidebarOpen = !sidebarOpen)}
      aria-label={sidebarOpen ? "Collapse sidebar" : "Expand sidebar"}
      class="absolute -right-3 top-20 w-6 h-6 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-full flex items-center justify-center shadow-lg hover:scale-110 transition-transform"
    >
      <i
        class={`bi ${sidebarOpen ? "bi-chevron-left" : "bi-chevron-right"} text-xs`}
      ></i>
    </button>
  </aside>

  <!-- Main Content -->
  <main
    class={`transition-all duration-300 ${sidebarOpen ? "ml-64" : "ml-20"}`}
  >
    <!-- Header -->
    <header
      class="bg-white/80 dark:bg-slate-900/80 backdrop-blur-xl border-b border-slate-200 dark:border-slate-800 sticky top-0 z-40"
    >
      <div class="px-6 py-4 flex items-center justify-between">
        <div>
          <h2
            class={`${typographyEmphasized.headline.large} text-slate-900 dark:text-white`}
          >
            Welcome back, Alex! 👋
          </h2>
          <p class="text-slate-600 dark:text-slate-400 text-sm">
            Here's what's happening with your projects today
          </p>
        </div>

        <div class="flex items-center gap-4">
          <div class="relative">
            <Input
              bind:value={searchQuery}
              placeholder="Search..."
              class="w-64"
            />
            <i
              class="bi bi-search absolute right-3 top-1/2 -translate-y-1/2 text-slate-400"
            ></i>
          </div>

          <button
            aria-label="Notifications"
            class="relative p-2 rounded-xl hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
          >
            <i
              class="bi bi-bell-fill text-xl text-slate-600 dark:text-slate-400"
            ></i>
            <span class="absolute top-1 right-1 w-2 h-2 bg-red-500 rounded-full"
            ></span>
          </button>

          <Avatar
            size="md"
            name="Alex Morgan"
            decorative
            shapeStyle="squircle-md"
          />
        </div>
      </div>
    </header>

    <!-- Dashboard Content -->
    <div class="p-6 space-y-6">
      <!-- Stats Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {#each stats as stat}
          <Card
            emphasized
            shapeStyle="extra-rounded"
            elevationLevel={2}
            class="hover:scale-105 transition-transform duration-300"
          >
            <div class="flex items-start justify-between">
              <div>
                <p class="text-slate-600 dark:text-slate-400 text-sm mb-1">
                  {stat.label}
                </p>
                <h3
                  class={`${typographyEmphasized.headline.medium} text-slate-900 dark:text-white mb-2`}
                >
                  {stat.value}
                </h3>
                <div
                  class={`inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-bold ${stat.trend === "up" ? "bg-green-100 dark:bg-green-950/30 text-green-600 dark:text-green-400" : "bg-red-100 dark:bg-red-950/30 text-red-600 dark:text-red-400"}`}
                >
                  <i
                    class={`bi ${stat.trend === "up" ? "bi-arrow-up" : "bi-arrow-down"}`}
                  ></i>
                  {stat.change}
                </div>
              </div>
              <div
                class={`w-12 h-12 rounded-xl bg-${stat.color}-500/10 flex items-center justify-center`}
              >
                <i class={`bi ${stat.icon} text-2xl text-${stat.color}-600`}
                ></i>
              </div>
            </div>
          </Card>
        {/each}
      </div>

      <!-- Quick Actions -->
      <Card
        emphasized
        shapeStyle="extra-rounded"
        surfaceTone="container"
        elevationLevel={3}
      >
        <div class="flex items-center justify-between mb-4">
          <h3
            class={`${typographyEmphasized.title.large} text-slate-900 dark:text-white`}
          >
            Quick Actions
          </h3>
        </div>
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          {#each quickActions as action}
            <button
              class={`p-6 rounded-2xl bg-${action.color}-500/5 border-2 border-${action.color}-500/20 hover:border-${action.color}-500/40 hover:scale-105 transition-all duration-300 group`}
            >
              <div
                class={`w-14 h-14 mx-auto mb-3 rounded-xl bg-gradient-to-br from-${action.color}-600 to-${action.color}-700 flex items-center justify-center shadow-lg shadow-${action.color}-500/50 group-hover:scale-110 transition-transform`}
              >
                <i class={`bi ${action.icon} text-2xl text-white`}></i>
              </div>
              <div
                class={`text-sm font-bold text-${action.color}-600 dark:text-${action.color}-400`}
              >
                {action.label}
              </div>
            </button>
          {/each}
        </div>
      </Card>

      <div class="grid lg:grid-cols-3 gap-6">
        <!-- Projects -->
        <div class="lg:col-span-2">
          <Card
            emphasized
            shapeStyle="extra-rounded"
            surfaceTone="container"
            elevationLevel={3}
          >
            <div class="flex items-center justify-between mb-6">
              <h3
                class={`${typographyEmphasized.title.large} text-slate-900 dark:text-white`}
              >
                Active Projects
              </h3>
              <Button
                size="sm"
                variant="primary"
                emphasized
                shapeStyle="extra-rounded"
              >
                <i class="bi bi-plus-lg mr-2"></i>New Project
              </Button>
            </div>

            <div class="space-y-4">
              {#each projects as project}
                <div
                  class="p-4 bg-slate-50 dark:bg-slate-800/50 rounded-2xl hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
                >
                  <div class="flex items-center justify-between mb-3">
                    <div class="flex items-center gap-3">
                      <div
                        class={`w-3 h-3 rounded-full bg-${project.color}-500`}
                      ></div>
                      <h4 class="font-bold text-slate-900 dark:text-white">
                        {project.name}
                      </h4>
                    </div>
                    <Badge
                      variant={project.status === "On Track"
                        ? "success"
                        : project.status === "Ahead"
                          ? "primary"
                          : project.status === "At Risk"
                            ? "warning"
                            : "danger"}
                      size="sm"
                    >
                      {project.status}
                    </Badge>
                  </div>

                  <div class="flex items-center justify-between text-sm mb-2">
                    <span class="text-slate-600 dark:text-slate-400"
                      >Progress</span
                    >
                    <span class="font-bold text-slate-900 dark:text-white"
                      >{project.progress}%</span
                    >
                  </div>

                  <div
                    class="relative h-2 bg-slate-200 dark:bg-slate-700 rounded-full overflow-hidden mb-3"
                  >
                    <div
                      class={`absolute left-0 top-0 h-2 bg-gradient-to-r from-${project.color}-500 to-${project.color}-600 rounded-full transition-all duration-500`}
                      style={`width: ${project.progress}%`}
                    ></div>
                  </div>

                  <div class="flex items-center justify-between text-xs">
                    <div class="flex items-center gap-2">
                      <i class="bi bi-check-square text-slate-400"></i>
                      <span class="text-slate-600 dark:text-slate-400"
                        >{project.tasks} tasks</span
                      >
                    </div>
                    <div class="flex items-center gap-2">
                      <i class="bi bi-people text-slate-400"></i>
                      <span class="text-slate-600 dark:text-slate-400"
                        >{project.team} members</span
                      >
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          </Card>
        </div>

        <!-- Recent Activity -->
        <div>
          <Card
            emphasized
            shapeStyle="extra-rounded"
            surfaceTone="container"
            elevationLevel={3}
          >
            <h3
              class={`${typographyEmphasized.title.large} text-slate-900 dark:text-white mb-6`}
            >
              Recent Activity
            </h3>

            <div class="space-y-4">
              {#each recentActivity as activity}
                <div
                  class="flex items-start gap-3 p-3 rounded-xl hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors"
                >
                  <Avatar size="sm" name={activity.user} decorative />
                  <div class="flex-1 min-w-0">
                    <p
                      class="text-sm text-slate-900 dark:text-white font-medium truncate"
                    >
                      {activity.user}
                    </p>
                    <p class="text-xs text-slate-600 dark:text-slate-400">
                      {activity.action} in
                      <span class="font-semibold">{activity.project}</span>
                    </p>
                    <p class="text-xs text-slate-400 mt-1">{activity.time}</p>
                  </div>
                </div>
              {/each}
            </div>

            <Button variant="secondary" size="sm" class="w-full mt-4">
              View All Activity
            </Button>
          </Card>
        </div>
      </div>

      <!-- Chart Placeholder -->
      <Card
        emphasized
        shapeStyle="extra-rounded"
        surfaceTone="container"
        elevationLevel={3}
      >
        <h3
          class={`${typographyEmphasized.title.large} text-slate-900 dark:text-white mb-6`}
        >
          Performance Overview
        </h3>
        <div
          class="h-64 bg-gradient-to-br from-blue-50 to-purple-50 dark:from-blue-950/20 dark:to-purple-950/20 rounded-2xl flex items-center justify-center border-2 border-dashed border-slate-300 dark:border-slate-700"
        >
          <div class="text-center">
            <i
              class="bi bi-graph-up text-6xl text-slate-300 dark:text-slate-700 mb-4"
            ></i>
            <p class="text-slate-600 dark:text-slate-400 font-medium">
              Chart visualization placeholder
            </p>
            <p class="text-slate-400 dark:text-slate-500 text-sm">
              Integrate with Chart.js or similar
            </p>
          </div>
        </div>
      </Card>
    </div>
  </main>
</div>
