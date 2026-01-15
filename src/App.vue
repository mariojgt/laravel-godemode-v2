<template>
  <div class="h-screen flex flex-col bg-dark-900">
    <!-- Title Bar -->
    <header class="titlebar-drag h-12 flex items-center justify-between px-4 bg-dark-850 border-b border-dark-700 shrink-0">
      <div class="flex items-center gap-3">
        <div class="w-20"></div> <!-- Space for macOS traffic lights -->
        <div class="flex items-center gap-2">
          <svg class="w-6 h-6 text-godmode-400" viewBox="0 0 64 64" fill="none">
            <rect width="64" height="64" rx="12" fill="currentColor"/>
            <path d="M20 18 L44 32 L20 46 Z" fill="white" opacity="0.9"/>
          </svg>
          <span class="font-semibold text-dark-100">Laravel God Mode</span>
        </div>
      </div>

      <nav class="titlebar-no-drag flex items-center gap-1">
        <router-link
          v-for="item in navItems"
          :key="item.path"
          :to="item.path"
          class="nav-link"
          :class="{ 'nav-link-active': $route.path === item.path }"
        >
          {{ item.name }}
        </router-link>
      </nav>

      <div class="w-20"></div>
    </header>

    <!-- Main Content -->
    <main class="flex-1 overflow-hidden">
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </main>
  </div>
</template>

<script setup lang="ts">
const navItems = [
  { name: 'Dashboard', path: '/' },
  { name: 'Templates', path: '/templates' },
  { name: 'Settings', path: '/settings' }
]
</script>

<style scoped>
.nav-link {
  @apply px-4 py-1.5 text-sm font-medium text-dark-400 rounded-lg transition-all duration-200 hover:text-dark-100 hover:bg-dark-700;
}

.nav-link-active {
  @apply bg-godmode-500 text-white hover:bg-godmode-600 hover:text-white;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
