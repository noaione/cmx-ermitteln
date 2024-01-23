<template>
  <div class="mb-4 flex w-full flex-col text-center">
    <div class="mx-auto mb-4 h-16 w-16">
      <NuxtLink v-if="router.name !== 'index'" to="/">
        <img src="/apple-touch-icon.png" class="h-16 w-16 transition-opacity hover:opacity-70" />
      </NuxtLink>
      <img v-else src="/apple-touch-icon.png" class="h-16 w-16" />
    </div>
    <h1 class="text-header font-variable text-4xl variation-weight-[550]">Ermitteln</h1>
    <h3 class="font-variable mt-1 text-sm variation-weight-light">A reverse image search for CMX cover</h3>
  </div>
</template>

<script setup lang="ts">
const router = useRoute();
</script>
