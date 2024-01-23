<template>
  <div class="flex w-full flex-col">
    <SearchBox />
    <p v-if="docCount !== undefined" class="mt-2 text-center text-sm text-gray-300">
      Serving {{ docCount.toLocaleString() }} images
    </p>
    <div class="mt-6 flex w-full flex-col gap-4">
      <div class="flex w-full flex-col">
        <p v-if="ermitteln.error" class="mt-4 text-center text-lg text-red-400">
          {{ ermitteln.error }}
        </p>
        <p
          v-if="ermitteln.data === undefined && !ermitteln.loading && !ermitteln.error"
          class="mt-4 text-center text-2xl font-light text-gray-400"
        >
          Start searching!
        </p>
        <SpinnerLoading v-else-if="ermitteln.loading" />
        <div v-else-if="ermitteln.data !== undefined" class="flex w-full flex-row flex-wrap justify-center gap-4">
          <p v-if="ermitteln.data.length === 0" class="mt-4 text-center text-2xl font-light text-gray-400">
            No results found!
          </p>
          <SearchResult v-for="data in ermitteln.data" :key="data.id" :data="data" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const ermitteln = useErmitteln();

const docCount = ref<number>();

onMounted(async () => {
  await ermitteln.search("");

  const stats = await ermitteln.getStats();

  docCount.value = stats.numberOfDocuments;
});
</script>
