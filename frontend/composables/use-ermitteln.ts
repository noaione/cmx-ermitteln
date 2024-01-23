import debounce from "lodash.debounce";

export interface ErmittelnHash {
  id: number;
  hash: string;
}

export const useErmitteln = defineStore("ermitteln", () => {
  const runtimeConfig = useRuntimeConfig();

  const data = ref<ErmittelnHash[]>();
  const loading = ref(false);
  const error = ref<Error>();

  async function search(query: string) {
    loading.value = true;

    try {
      const url = new URL(runtimeConfig.public.meiliHost);

      url.pathname = "/indexes/ermitteln-images/search";

      const jsonBody: Record<string, string | string[] | number> = {
        q: query,
        page: 1,
        hitsPerPage: 30,
      };

      const response = await fetch(url, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${runtimeConfig.public.meiliKey}`,
        },
        body: JSON.stringify(jsonBody),
      });

      if (!response.ok) {
        throw new Error(`MeiliSearch returned ${response.status}`);
      }

      const json = await response.json();

      data.value = json.hits;
    } catch (error_) {
      error.value = error_ instanceof Error ? error_ : new Error("Unknown error");
    } finally {
      loading.value = false;
    }
  }

  async function getStats() {
    try {
      const url = new URL(runtimeConfig.public.meiliHost);

      url.pathname = "/indexes/ermitteln-images/stats";

      const response = await fetch(url, {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${runtimeConfig.public.meiliKey}`,
        },
      });

      if (!response.ok) {
        throw new Error(`MeiliSearch returned ${response.status}`);
      }

      const json = await response.json();

      return json;
    } catch (error_) {
      error.value = error_ instanceof Error ? error_ : new Error("Unknown error");
    }
  }

  const searchDebounced = debounce(search, 500);

  return {
    search,
    searchDebounced,
    getStats,
    loading,
    data,
    error,
  };
});
