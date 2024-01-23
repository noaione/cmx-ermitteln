<template>
  <NuxtLoadingIndicator />
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>

<script setup lang="ts">
import init from "ermitteln-wasm";

onMounted(async () => {
  await init();
});
</script>
