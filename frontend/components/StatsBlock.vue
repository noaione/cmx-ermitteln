<template>
  <div class="block min-w-32 justify-center border-[1px] border-white p-2">
    <div class="m-1flex flex-col align-middle">
      <h3 class="font-variable text-center text-lg lowercase variation-weight-bold">{{ name }}</h3>
      <p class="font-variable break-words text-center text-sm lowercase">
        <span ref="counter" />
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
}>();

const counter = ref<HTMLSpanElement>();

onMounted(() => {
  if (counter.value && !props.noAnimate) {
    // make 0-999 wihout commas
    // 1000+ with commas until the count
    const innerHTML: (string | number)[] = [0, props.count];

    anime({
      targets: counter.value,
      innerHTML,
      easing: "easeInOutExpo",
      round: 1,
      duration: 2500,
    });
  } else if (counter.value) {
    counter.value.innerHTML = props.count.toLocaleString();
  }
});
</script>
