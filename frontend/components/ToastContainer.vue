<template>
  <Teleport to="body">
    <div :class="`fixed right-0 top-0 mx-4 my-4 flex flex-col items-end`" data-hitagi="toaster">
      <TransitionGroup name="toast">
        <Toast
          v-for="toast in toasts.toasts"
          :id="toast.id"
          :key="toast.id"
          :color="typeToColor(toast.type)"
          :message="toast.message"
          :title="toast.title"
          :duration="toast.duration"
          :persist="toast.persist"
        />
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
const toasts = useToast();

function typeToColor(type: "success" | "error" | "warning" | "info" | "default"): string {
  switch (type) {
    case "success": {
      return "green";
    }
    case "error": {
      return "red";
    }
    case "warning": {
      return "yellow";
    }
    default: {
      return "blue";
    }
  }
}
</script>

<style scoped lang="postcss">
.toast-enter-active,
.toast-leave-active {
  transition: all 0.5s ease;
}
.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
</style>
