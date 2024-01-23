<template>
  <div class="md:0 flex w-full flex-col gap-2 md:flex-row">
    <div class="upload-area mx-auto mt-2 md:mr-2 md:mt-1.5">
      <label for="img-upload" class="upload-btn" :aria-disabled="loading">Upload</label>
      <input id="img-upload" type="file" accept="image/jpeg" :disabled="loading" @change="uploadImage" />
    </div>

    <input
      id="asin"
      v-model="asinQuery"
      type="text"
      class="mt-4 w-full rounded-md bg-gray-700 px-4 py-2 text-white disabled:opacity-80 md:mt-0"
      name="asin"
      placeholder="Put ASIN"
      :disabled="loading"
    />
    <button
      class="mt-2 rounded-md bg-gray-700 px-4 py-2 text-white transition hover:opacity-80 disabled:opacity-80 md:ml-2 md:mt-0"
      :disabled="loading"
      @click="search"
    >
      Search
    </button>
    <button
      class="rounded-md bg-gray-700 px-4 py-2 text-white transition hover:opacity-80 disabled:opacity-80 md:ml-2"
      :disabled="loading"
      @click="ermitteln.search('')"
    >
      Reset
    </button>
  </div>
</template>

<script setup lang="ts">
import { hash_image } from "ermitteln-wasm";

const loading = ref(false);
const ermitteln = useErmitteln();
const asinQuery = ref("");
const toasts = useToast();

async function getASINImageHash(asin: string): Promise<[boolean, string]> {
  const buildUrl = `https://m.media-amazon.com/images/P/${asin}.01._SCRM_.jpg`;

  loading.value = true;

  try {
    const response = await fetch(buildUrl);
    const blob = await response.blob();

    if (blob.type !== "image/jpeg") {
      return [false, "Not a JPEG image!"];
    }

    // convert to Uint8Array
    const arrayBuffer = await blob.arrayBuffer();
    const bytes = new Uint8Array(arrayBuffer);

    try {
      const imageHash = hash_image(bytes);

      loading.value = false;

      return [true, imageHash];
    } catch {
      loading.value = false;

      return [false, "Error while hashing image!"];
    }
  } catch (error_) {
    console.error(error_);

    loading.value = false;

    return [false, "Please see console for error!"];
  }
}

function uploadImage(ev: Event) {
  if (!(ev.currentTarget instanceof HTMLInputElement)) return;

  const files = ev.currentTarget.files;

  if (!files) {
    return;
  }

  const file = files[0];

  if (file.type !== "image/jpeg") {
    toasts.toast({
      message: "Not a JPEG image!",
      type: "warning",
    });

    return;
  }

  console.log("Loading", file);

  loading.value = true;
  file
    .arrayBuffer()
    .then((buffer) => {
      const bytes = new Uint8Array(buffer);

      const imageHash = hash_image(bytes);

      loading.value = false;

      // empty the input
      if (ev.currentTarget instanceof HTMLInputElement) {
        ev.currentTarget.value = ev.currentTarget.defaultValue;
      }

      ermitteln.search(imageHash);
    })
    .catch((error) => {
      if (error instanceof Error) {
        toasts.toast({
          title: "Error",
          message: error.message,
          type: "error",
          duration: 5000,
        });
      } else {
        toasts.toast({
          title: "Error",
          message: "An unknown error occured!",
          type: "error",
          duration: 5000,
        });
      }

      if (ev.currentTarget instanceof HTMLInputElement) {
        ev.currentTarget.value = ev.currentTarget.defaultValue;
      }

      loading.value = false;
    });
}

async function search() {
  if (asinQuery.value === "") {
    return;
  }

  const [success, hashData] = await getASINImageHash(asinQuery.value);

  if (!success) {
    toasts.toast({
      title: "Error",
      message: hashData,
      type: "error",
      duration: 5000,
    });

    return;
  }

  await ermitteln.search(hashData);
}
</script>

<style scoped lang="postcss">
.upload-btn {
  @apply select-none bg-cyan-700 pb-2 hover:bg-cyan-600 hover:text-gray-50 active:bg-cyan-600 disabled:opacity-80;
  @apply rounded-md px-4 py-2 font-bold transition-colors;
}
.upload-btn[aria-disabled="true"] {
  @apply bg-cyan-600 disabled:opacity-80 !important;
  @apply cursor-not-allowed;
}
.upload-area > input[type="file"] {
  display: none;
}

.upload-url-btn {
  @apply bg-cyan-700 hover:bg-cyan-600 hover:text-gray-50 active:bg-cyan-600 disabled:bg-cyan-600 disabled:opacity-80;
  @apply rounded-md px-4 py-2 font-bold transition-colors;
}
</style>