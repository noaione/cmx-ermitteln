<template>
  <div ref="maxRef" class="mb-2 w-fit">
    <div :class="`disabled:cursor-not-allow toast-hitagi group toast-bord-${selectedColor} ${$props.class ?? ''}`">
      <div v-if="title" class="font-variable variation-weight-semibold text-lg">
        <span>{{ title }}</span>
        <Icon name="mdi:close" class="float-right ml-1 mt-1.5 cursor-pointer" @click="removeToast" />
      </div>
      <div>
        <span>{{ message }}</span>
        <Icon v-if="!title" name="mdi:close" class="mb-0.5 ml-1 cursor-pointer" @click="removeToast" />
      </div>
    </div>
    <div
      v-if="!persist"
      ref="timerRef"
      :class="`absolute h-1 rounded-b-md toast-loader-${selectedColor}`"
      :style="{
        width: `${maxRef?.offsetWidth !== undefined ? (maxRef.offsetWidth * currentFrame) / 100 : 0}px`,
        maxWidth: `${maxRef?.offsetWidth}px`,
      }"
    />
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  id: string;
  message: string;
  title?: string;
  class?: string;
  color?: string;
  duration: number;
  persist?: boolean;
}>();

const timerRef = ref();
const maxRef = ref<HTMLDivElement>();
const startTime = ref();
const currentFrame = ref(100);
const toasts = useToast();

const selectedColor = computed(() => {
  const col = props.color ?? "blue";

  return col;
});

function removeToast() {
  toasts.removeToast(props.id);
}

function animationStep(timestamp: number) {
  const startTimeTest = startTime.value ?? timestamp;

  if (startTime.value === undefined) {
    startTime.value = timestamp;
  }

  const elapsed = timestamp - startTimeTest;

  // make the currentFrame lower
  const newFrame = 100 - (elapsed / props.duration) * 100;

  // Only update if the new frame is lower
  if (newFrame < currentFrame.value) {
    currentFrame.value = newFrame;
  }

  if (elapsed < props.duration) {
    window.requestAnimationFrame(animationStep);
  }
}

onMounted(() => {
  // do animation
  if (props.persist) {
    return;
  }

  // do requestAnimationFrame
  window.requestAnimationFrame(animationStep);

  setTimeout(() => {
    removeToast();
  }, props.duration + 100);
});
</script>
