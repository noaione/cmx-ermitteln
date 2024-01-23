<template>
  <NuxtLoadingIndicator />
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
  <ToastContainer />
</template>

<script setup lang="ts">
import init from "ermitteln-wasm";

onMounted(async () => {
  await init();
});
</script>
