<template>
  <span :title="roleName" class="role-icon hover:opacity-70">
    <Icon v-if="role === 'developer'" name="mdi:laptop-windows" class="role-icon text-purple-400" />
    <Icon v-else-if="role === 'designer'" name="mdi:palette" class="role-icon text-yellow-400" />
    <Icon v-else-if="role === 'helper'" name="mdi:help-circle-outline" class="role-icon text-blue-400" />
    <Icon v-else-if="role === 'idea'" name="mdi:lightbulb-on-outline" class="role-icon text-orange-300" />
    <Icon v-else-if="role === 'data-entry'" name="mdi:keyboard" class="role-icon text-cyan-300" />
    <Icon v-else-if="role === 'host'" name="mdi:server" class="role-icon text-green-200" />
  </span>
</template>

<script setup lang="ts">
const props = defineProps<{
  role: "developer" | "designer" | "helper" | "idea" | "data-entry" | "host";
}>();

const roleName = computed(() => {
  switch (props.role) {
    case "developer": {
      return "Developer";
    }
    case "designer": {
      return "Designer";
    }
    case "helper": {
      return "Helper";
    }
    case "idea": {
      return "Idea/Initial Code";
    }
    case "data-entry": {
      return "Data Collection/Entry";
    }
    case "host": {
      return "Hosting";
    }
  }
});
</script>

<style scoped lang="postcss">
.role-icon {
  @apply inline-block h-6 w-6;
}
</style>
