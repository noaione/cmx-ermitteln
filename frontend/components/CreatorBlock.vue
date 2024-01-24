<template>
  <div class="block h-auto w-32 justify-center border-[1px] border-white p-2">
    <div class="mx-auto mb-auto mt-2">
      <img :src="avatar" class="mx-auto h-24 w-24" />
    </div>
    <div class="m-1 my-auto mb-2 flex flex-col align-middle">
      <h3 class="font-variable mb-0.5 mt-1 text-lg variation-weight-bold">{{ name }}</h3>
      <p
        class="font-variable flex flex-row flex-wrap justify-center gap-2 break-words text-sm lowercase variation-weight-medium"
      >
        <CreatorRole v-for="role in roles" :key="role" :role="role" />
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
type Role = "developer" | "designer" | "helper" | "idea" | "data-entry" | "host";
defineProps<{
  name: string;
  roles: Role[];
  avatar: string;
}>();
</script>
