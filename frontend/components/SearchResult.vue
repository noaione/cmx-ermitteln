<template>
  <div class="flex flex-col rounded-md bg-gray-800">
    <div
      class="thumb-container rounded-t-md"
      :style="{
        'background-image': 'url(' + thumbnailUrl + ')',
      }"
    />
    <div class="flex flex-col py-2 text-center">
      <a
        :href="thumbnailUrl"
        class="font-variable mx-2 mt-1 break-all text-sm text-purple-400 variation-weight-[550] hover:text-purple-300 hover:underline"
        target="_blank"
        rel="noopener noreferer"
      >
        {{ data.id }}
      </a>
      <div class="mx-2 mt-1 break-all text-sm text-gray-300">
        {{ data.hash }}
      </div>
      <div
        v-if="data.asin !== undefined"
        class="font-variable mx-2 mb-1 mt-0.5 break-all text-xs text-gray-300 variation-weight-bold"
      >
        {{ data.asin }}
      </div>
      <div v-if="data.score !== undefined" class="mx-2 mb-1 mt-0.5 text-xs text-gray-300">
        Diff: <span class="font-variable text-gray-200 variation-weight-bold">{{ data.score }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  data: ErmittelnHash;
}>();

const thumbnailUrl = computed(() => {
  return `https://m.media-amazon.com/images/S/cmx-images-prod/Item/${props.data.id}/${props.data.id}.jpg`;
});
</script>

<style scoped lang="postcss">
.thumb-container {
  @apply flex h-48 flex-col;
  aspect-ratio: 2 / 3;
  width: fit-content;
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
}
</style>
