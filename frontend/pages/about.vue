<template>
  <hr />
  <div class="mt-2 flex flex-col items-center justify-center">
    <h2 class="font-variable mt-4 text-xl variation-weight-medium">
      <ruby>Ermitteln<rp>(</rp><rt class="mb-1 font-serif text-base">/ɛɐ̯ˈmɪtl̩n/</rt><rp>)</rp></ruby>
    </h2>

    <div class="text-about mt-4 flex flex-col text-center font-normal">
      <p>
        A quick reverse image lookup using the
        <a href="https://www.phash.org/docs/design.html" target="_blank" rel="noopener noreferer">pHash (DCT)</a>
        algorithm, designed specifically for
        <a href="https://comixology.com" target="_blank" rel="noopener noreferer">CMX</a> cover.
      </p>
      <p class="mt-2 text-center">
        Powered by <a href="https://www.meilisearch.com/" target="_blank" rel="noopener noreferer">Meilisearch</a>
      </p>
    </div>

    <div class="text-about mt-4 flex flex-col text-center font-normal">
      <h3 class="font-lg font-variable text-lg variation-weight-semibold">Involved People</h3>
      <div class="mt-4 flex w-[90vw] flex-row flex-wrap justify-center gap-2 md:w-[70vw] lg:w-[50vw]">
        <CreatorBlock
          name="noaione"
          avatar="https://avatars.githubusercontent.com/u/34302902?v=4"
          :roles="['developer', 'designer', 'data-entry', 'host']"
        />
        <CreatorBlock
          name="neckothy"
          avatar="https://avatars.githubusercontent.com/u/109988355?v=4"
          :roles="['idea', 'data-entry']"
        />
      </div>
    </div>
    <div class="text-about mt-4 flex flex-col text-center font-normal">
      <h3 class="font-lg font-variable text-lg variation-weight-semibold">Source Code</h3>
      <div class="mt-2">
        <a href="https://github.com/noaione/cmx-ermitteln" class="h-12 w-12" target="_blank" rel="noopener noreferer">
          <Icon class="h-12 w-12" name="mdi:github" />
        </a>
      </div>
    </div>
  </div>
</template>

<style scoped lang="postcss">
.text-about a {
  @apply text-purple-400 hover:text-purple-300 hover:underline;
}
</style>
