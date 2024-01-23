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
  modules: [
    "@nuxtjs/tailwindcss",
    "nuxt-icon",
    "@pinia/nuxt",
    "@nuxtjs/google-fonts",
    "@nuxtjs/eslint-module",
    "@vueuse/nuxt",
  ],
  runtimeConfig: {
    public: resolvePublicConfig(),
  },
  googleFonts: {
    families: {
      Inter: true,
      Raleway: true,
      Incosolata: true,
      "DM Sans": true,
    },
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
        {
          rel: "apple-touch-icon",
          sizes: "180x180",
          href: "/apple-touch-icon.png",
        },
        {
          rel: "icon",
          href: "/favicon-32x32.png",
          type: "image/png",
          sizes: "32x32",
        },
        {
          rel: "icon",
          href: "/favicon-16x165.png",
          type: "image/png",
          sizes: "16x16",
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