<template>
  <div class="flex w-full flex-row">
    <div class="upload-area mr-2 mt-1.5">
      <label for="img-upload" class="upload-btn" :aria-disabled="loading">Upload</label>
      <input id="img-upload" type="file" accept="image/jpeg" :onchange="uploadImage" :disabled="loading" />
    </div>

    <input
      id="asin"
      v-model="asinQuery"
      type="text"
      class="w-full rounded-md bg-gray-700 px-4 py-2 text-white disabled:opacity-80"
      name="asin"
      placeholder="Put ASIN"
    />
    <button
      class="ml-2 rounded-md bg-gray-700 px-4 py-2 text-white transition hover:opacity-80 disabled:opacity-80"
      :disabled="loading"
      @click="search"
    >
      Search
    </button>
    <button
      class="ml-2 rounded-md bg-gray-700 px-4 py-2 text-white transition hover:opacity-80 disabled:opacity-80"
      :disabled="loading"
      @click="ermitteln.search('')"
    >
      Reset
    </button>
  </div>
</template>

<script setup lang="ts">
import { hash_image } from "ermitteln-wasm";

type HandleFileInput = Event & { currentTarget: EventTarget & HTMLInputElement };

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

function uploadImage(ev: HandleFileInput) {
  const files = ev.currentTarget.files;

  if (!files) return;

  const file = files[0];

  if (file.type !== "image/jpeg") {
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
