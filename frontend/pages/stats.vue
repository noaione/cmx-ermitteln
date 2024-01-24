<template>
  <hr />
  <div class="mt-2 flex flex-col items-center justify-center">
    <h2 class="font-variable mt-4 text-xl variation-weight-bold">Stats</h2>

    <span v-if="loading && !error" class="mt-4 animate-pulse">Now loading...</span>
    <span v-if="error" class="mt-4 text-red-400">{{ error }}</span>
    <div v-if="!loading && !error" class="text-about mt-4 flex flex-col gap-2 font-normal">
      <div class="flex flex-row justify-center gap-2">
        <StatsBlock name="Total Documents" :count="docTotal!" />
        <StatsBlock name="First known IDs" :count="docKnownIds![0]" no-animate :formatter="(n) => n.toString()" />
        <StatsBlock name="Last known IDs" :count="docKnownIds![1]" no-animate :formatter="(n) => n.toString()" />
      </div>
      <h2 class="font-variable mt-4 text-center text-xl variation-weight-bold">Distribution</h2>
      <div class="mt-1 flex flex-row justify-center gap-2">
        <StatsBlock v-for="[key, value] of Object.entries(distributions!)" :key="key" :name="key" :count="value" />
      </div>

      <!-- First range -->
      <div v-if="docRangeFirst !== undefined" class="mt-2 min-w-full">
        <h2 class="font-variable mt-4 text-center text-lg variation-weight-semibold">
          First {{ docRangeFirst.length }} IDs
        </h2>
        <div class="mt-4 flex min-w-full flex-row flex-wrap justify-center gap-2 border-[1px] border-white px-2 py-4">
          <span v-for="data in docRangeFirst" :key="data.id" class="range-ids">
            {{ data.id }}
          </span>
        </div>
      </div>

      <!-- Last range -->
      <div v-if="docRangeLast !== undefined" class="mt-2 min-w-full">
        <h2 class="font-variable mt-4 text-center text-lg variation-weight-semibold">
          Last {{ docRangeLast.length }} IDs
        </h2>
        <div class="mt-4 flex min-w-full flex-row flex-wrap justify-center gap-2 border-[1px] border-white px-2 py-4">
          <span v-for="data in docRangeLast" :key="data.id" class="range-ids">
            {{ data.id }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ErmittelnOwnedRange } from "~/composables/use-ermitteln";

const loading = ref(true);
const error = ref<string>();
const ermitteln = useErmitteln();

const docKnownIds = ref<number[]>();
const docTotal = ref<number>();
const distributions = ref<Record<string, number>>();

const docRangeFirst = ref<ErmittelnHash[]>();
const docRangeLast = ref<ErmittelnHash[]>();

onMounted(async () => {
  const promises: [Promise<ErmittelnStats>, Promise<{ first: ErmittelnOwnedRange; last: ErmittelnOwnedRange }>] = [
    ermitteln.getStats(),
    ermitteln.getOwnedRange(500),
  ];

  try {
    const [stats, range] = await Promise.all(promises);

    docKnownIds.value = [range.first.hits[0].id, range.last.hits[range.last.hits.length - 1].id];
    docTotal.value = stats.numberOfDocuments;
    distributions.value = stats.fieldDistribution;
    docRangeFirst.value = range.first.hits;
    docRangeLast.value = range.last.hits;
  } catch (error_) {
    console.error(error_);

    error.value = "Error while fetching stats!";
  } finally {
    loading.value = false;
  }
});
</script>

<style scoped lang="postcss">
.text-about a {
  @apply text-purple-400 hover:text-purple-300 hover:underline;
}

.range-ids {
  @apply cursor-pointer text-purple-400 hover:text-white hover:underline hover:opacity-80;
}
</style>
