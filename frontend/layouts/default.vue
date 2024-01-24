<template>
  <div class="font-roboto-mono mx-auto flex h-full min-h-screen flex-col py-4 md:py-6 lg:py-12">
    <AppHeader />
    <main class="mx-2 md:mx-4 lg:mx-8">
      <slot />
    </main>
    <div class="mt-6 text-center">
      <NuxtLink to="/about" class="text-center text-sm text-gray-300 hover:text-gray-400">About</NuxtLink>
      |
      <NuxtLink to="/stats" class="text-center text-sm text-gray-300 hover:text-gray-400">Stats</NuxtLink>
    </div>
  </div>
</template>
