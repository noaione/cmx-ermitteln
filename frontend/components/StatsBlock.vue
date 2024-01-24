<template>
  <div class="block min-w-32 justify-center border-[1px] border-white p-2">
    <div class="m-1flex flex-col align-middle">
      <h3 class="font-variable text-center text-lg lowercase variation-weight-bold">{{ name }}</h3>
      <p class="font-variable break-words text-center text-sm lowercase">
        <span ref="counter" />
        <span ref="counterSpoof" class="hidden" />
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import anime from "animejs";

const props = defineProps<{
  name: string;
  count: number;
  noAnimate?: boolean;
  formatter?: (value: number) => string;
}>();

// use double ref so we can utilize custom formatting
const counter = ref<HTMLSpanElement>();
const counterSpoof = ref<HTMLSpanElement>();

// format the number
const actualFormatter = props.formatter ?? ((value: number) => value.toLocaleString());

onMounted(() => {
  if (counterSpoof.value && !props.noAnimate) {
    // make 0-999 wihout commas
    // 1000+ with commas until the count
    anime({
      targets: counterSpoof.value,
      change: (state) => {
        if (counter.value) {
          counter.value.innerHTML = actualFormatter(state.animations[0].currentValue as unknown as number);
        }
      },
      innerHTML: [0, props.count],
      easing: "easeInOutExpo",
      round: 1,
      duration: 2500,
    });
  } else if (counter.value) {
    counter.value.innerHTML = actualFormatter(props.count);
  }
});
</script>
