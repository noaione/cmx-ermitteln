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

const ermitteln = useErmitteln();

onMounted(async () => {
  await init();

  if (window) {
    window.ermitteln = {
      search: ermitteln.search,
    };
  }
});

declare global {
  interface Window {
    ermitteln: {
      search: typeof ermitteln.search;
    };
  }
}
</script>
