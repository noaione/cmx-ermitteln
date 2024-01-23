<template>
  <NuxtLoadingIndicator />
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
  <ToastContainer />
</template>

<script setup lang="ts">
import "@fontsource-variable/roboto-mono/wght.css";
import "@fontsource-variable/roboto-mono/wght-italic.css";
import "@fontsource/roboto-mono";

import init from "ermitteln-wasm";

onMounted(async () => {
  await init();
});
</script>
