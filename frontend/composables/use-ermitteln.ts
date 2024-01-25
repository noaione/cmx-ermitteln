import debounce from "lodash.debounce";
import { hamming_distance } from "ermitteln-wasm";

const INDEX_NAME = "ermitteln-images";

export interface ErmittelnHash {
  id: number;
  hash: string;
  score?: number;
}

export interface ErmittelnStats {
  numberOfDocuments: number;
  isIndexing: boolean;
  fieldDistribution: Record<string, number>;
}

export interface ErmittelnOwnedRange {
  indexUid: string;
  hits: ErmittelnHash[];
  query: string;
  processingTimeMs: number;
  estimatedTotalHits: number;
}

export const useErmitteln = defineStore("ermitteln", () => {
  const runtimeConfig = useRuntimeConfig();

  const data = ref<ErmittelnHash[]>();
  const loading = ref(false);
  const error = ref<Error>();

  /**
   * Score the results based on the hamming distance to the base hash
   * @param results The results to sort
   * @param base The base hash to compare to
   * @returns Sorted + scored results
   */
  function sortResults(results: ErmittelnHash[], base: string): ErmittelnHash[] {
    if (!base) {
      return results;
    }

    const resultsWithScore = results.map((result) => ({
      ...result,
      score: hamming_distance(base, result.hash),
    }));

    // sort by score, the lower the better
    resultsWithScore.sort((a, b) => a.score - b.score);

    return resultsWithScore;
  }

  async function search(query: string) {
    loading.value = true;

    try {
      const url = new URL(runtimeConfig.public.meiliHost);

      url.pathname = `/indexes/${INDEX_NAME}/search`;

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

      data.value = sortResults(json.hits as ErmittelnHash[], query);
    } catch (error_) {
      error.value = error_ instanceof Error ? error_ : new Error("Unknown error");
    } finally {
      loading.value = false;
    }
  }

  async function getStats(): Promise<ErmittelnStats> {
    try {
      const url = new URL(runtimeConfig.public.meiliHost);

      url.pathname = `/indexes/${INDEX_NAME}/stats`;

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
      console.error("Failed to get stats", error_);

      throw error_;
    }
  }

  async function getOwnedRange(limit: number = 1): Promise<{ first: ErmittelnOwnedRange; last: ErmittelnOwnedRange }> {
    try {
      const url = new URL(runtimeConfig.public.meiliHost);

      url.pathname = "/multi-search";

      const queries = [
        {
          indexUid: INDEX_NAME,
          limit: limit,
          sort: ["id:asc"],
        },
        {
          indexUid: INDEX_NAME,
          limit: limit,
          sort: ["id:desc"],
        },
      ];

      const response = await fetch(url, {
        body: JSON.stringify({ queries }),
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${runtimeConfig.public.meiliKey}`,
        },
      });

      if (!response.ok) {
        throw new Error(`MeiliSearch returned ${response.status}`);
      }

      const json = await response.json();

      return {
        first: json.results[0],
        last: json.results[1],
      };
    } catch (error_) {
      console.error("Failed to get stats", error_);

      throw error_;
    }
  }

  const searchDebounced = debounce(search, 500);

  return {
    search,
    searchDebounced,
    getStats,
    getOwnedRange,
    loading,
    data,
    error,
  };
});
