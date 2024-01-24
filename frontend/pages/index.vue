<template>
  <div class="flex w-full flex-col">
    <SearchBox />
    <div v-if="docCount !== undefined || docRange !== undefined" class="mt-2 text-center text-sm text-gray-300">
      <p v-if="docCount !== undefined">
        Serving
        <span class="font-variable text-gray-100 variation-weight-semibold">{{ docCount.toLocaleString() }}</span>
        images
      </p>
      <p v-if="docRange !== undefined">
        Last known ID: <span class="font-variable text-gray-100 variation-weight-semibold">{{ docRange[1] }}</span>
      </p>
    </div>

    <div class="mt-6 flex w-full flex-col gap-4">
      <div class="flex w-full flex-col">
        <p v-if="ermitteln.error" class="mt-4 text-center text-lg text-red-400">
          {{ ermitteln.error }}
        </p>
        <p
          v-if="ermitteln.data === undefined && !ermitteln.loading && !ermitteln.error"
          class="font-variable mt-4 text-center text-2xl text-gray-400 variation-weight-light"
        >
          Start searching!
        </p>
        <SpinnerLoading v-else-if="ermitteln.loading" />
        <div v-else-if="ermitteln.data !== undefined" class="flex w-full flex-row flex-wrap justify-center gap-4">
          <p
            v-if="ermitteln.data.length === 0"
            class="font-variable mt-4 text-center text-2xl text-gray-400 variation-weight-light"
          >
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
const docRange = ref<number[]>();

onMounted(async () => {
  await ermitteln.search("");

  const stats = await ermitteln.getStats();

  docCount.value = stats.numberOfDocuments;

  const { first, last } = await ermitteln.getOwnedRange();

  docRange.value = [first.hits[0].id, last.hits[0].id];
});
</script>
