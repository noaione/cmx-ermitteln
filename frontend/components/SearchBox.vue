<template>
  <div class="md:0 flex w-full flex-col gap-2 md:flex-row">
    <div class="upload-area mx-auto mt-2 md:mr-2">
      <label for="img-upload" class="upload-btn" :aria-disabled="loading">Upload</label>
      <input id="img-upload" type="file" accept="image/jpeg" :disabled="loading" @change="uploadImage" />
    </div>

    <input
      id="asin"
      v-model="asinQuery"
      type="text"
      class="form-input mt-4 w-full border-2 border-white bg-black px-4 py-2 text-white transition focus:border-gray-500 focus:ring-0 md:mt-0"
      name="asin"
      placeholder="Put ASIN"
      :disabled="loading"
    />
    <div class="mb-2 mt-2 flex flex-row gap-2 md:mb-0 md:mt-0">
      <button class="btn-bw-hover w-full md:ml-2" :disabled="loading" @click="search">Search</button>
      <button class="btn-bw-hover w-full md:ml-2" :disabled="loading" @click="ermitteln.search('')">Reset</button>
    </div>
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

  await ermitteln.search(hashData, asinQuery.value);
}
</script>

<style scoped lang="postcss">
.upload-btn {
  @apply border-2 border-purple-500 bg-transparent text-white transition hover:border-white hover:bg-purple-600;
  @apply select-none px-4 py-2;
}
.upload-btn[aria-disabled="true"] {
  @apply border-gray-400 bg-purple-800 text-gray-300 !important;
  @apply cursor-not-allowed;
}
.upload-area > input[type="file"] {
  display: none;
}
.btn-bw-hover {
  @apply border-2 border-white bg-transparent px-4 py-2 text-white transition hover:bg-white hover:text-black disabled:border-gray-700 disabled:bg-gray-300 disabled:text-black disabled:hover:bg-gray-300;
}
</style>
