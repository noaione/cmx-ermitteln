function getEnv(key: string): string | undefined {
  const metaKey = import.meta.env[key];
  const processKey = process.env[key];

  // Ensure empty strings are treated as undefined
  if (metaKey) return metaKey;

  if (processKey) return processKey;

  return;
}

function resolvePublicConfig() {
  return {
    meiliHost: getEnv("MEILISEARCH_HOST") ?? getEnv("NUXT_PUBLIC_MEILISEARCH_HOST") ?? undefined,
    meiliKey: getEnv("MEILISEARCH_KEY") ?? getEnv("NUXT_PUBLIC_MEILISEARCH_KEY") ?? undefined,
  };
}

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: ["@nuxtjs/tailwindcss", "nuxt-icon", "@pinia/nuxt", "@nuxtjs/eslint-module", "@vueuse/nuxt"],
  runtimeConfig: {
    public: resolvePublicConfig(),
  },
  app: {
    head: {
      title: "Ermitteln",
      meta: [
        {
          "http-equiv": "x-ua-compatible",
          content: "IE=edge",
        },
        {
          name: "description",
          content: "A reverse image search for Comixology cover",
        },
        {
          property: "og:title",
          content: "Ermitteln",
        },
        {
          property: "og:description",
          content: "A reverse image search for Comixology cover",
        },
        {
          property: "og:site:name",
          content: "Ermitteln",
        },
        {
          property: "og:image",
          content: "/apple-touch-icon.png",
        },
        {
          name: "keywords",
          content: "comixology, amazon, reverse image, comic, manga, search",
        },
      ],
      link: [
        {
          rel: "icon",
          href: "/favicon.ico",
          sizes: "any",
        },
      ],
    },
  },
  eslint: {
    lintOnStart: false,
  },
  imports: {
    imports: [
      {
        name: "FetchError",
        from: "ofetch",
      },
    ],
  },
  hooks: {
    "build:before": () => {
      // raise error if env vars are not set
      const publicConfig = resolvePublicConfig();

      if (!publicConfig.meiliHost) {
        throw new Error("MEILISEARCH_HOST is not set");
      }

      if (!publicConfig.meiliKey) {
        throw new Error("MEILISEARCH_KEY is not set");
      }
    },
  },
});
