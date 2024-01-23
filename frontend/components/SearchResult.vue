<template>
  <div class="flex flex-col rounded-md bg-gray-800">
    <div
      class="thumb-container rounded-t-md"
      :style="{
        'background-image': 'url(' + thumbnailUrl + ')',
      }"
    />
    <div class="flex flex-col py-2 text-center">
      <div class="mx-2 mt-1 break-all text-sm text-gray-300">
        {{ data.id }}
      </div>
      <div class="mx-2 mt-1 text-sm text-gray-300">
        {{ data.hash }}
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
