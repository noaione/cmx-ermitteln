# CMX Ermitteln

A quick reverse image lookup using the [pHash (DCT)](https://www.phash.org/docs/design.html) algorithm, designed specifically for [CMX](https://comixology.com/) cover.

Powered by [Meilisearch](https://www.meilisearch.com/).

### Public Version

https://ermitteln.ihateani.me

## Requirements
1. Node 18+
2. Rust 1.72+
3. Meilisearch instance
4. wasm-pack

## Installation
1. Clone this repository.
2. Install Node, Rust, Meilisearch, and wasm-pack.
3. Build cargo crates: `cargo build --release --all`<br />
   You should always build as release since `aufnehmen` hashing will be slow without optimization
4. Build the WASM package: `wasm-pack build ermitteln-wasm --target web`
5. Install Node dependencies: `npm install`
6. Build the frontend: `npm run build`
7. Ingest your images into Meilisearch.<br />
   Configure your index: [Recommended Meilisearch Settings](#recommended-meilisearch-settings)
8. Run the server: `node ./frontend/.output/server/index.mjs`

## Ingesting
You would need to download CMX cover into a folder; we are expecting a specific filename like this: `CmxID.jpg`

1. After building all the crates, get the `aufnehmen` binary from `target/release`, then copy it to your CMX cover parent directory.
2. Add `MEILI_URL=your_meilisearch_instance_url` and `MEILI_KEY=your_api_key_or_master_key` to your environment.
3. Run the binary: `./aufnehmen ingest <directory>`
4. Wait for the ingestion to complete.
5. You should have a new index called `ermitteln-images`.

### Blacklisting Placeholder

Currently, the cover blacklist for placeholders is hard-coded. If you discover a new one, you can open a new issue and include the hash of the file.

To obtain the hash of the file, use the following command:
```bash
$ ./aufnehmen hash <input.jpg>
```

This command will provide you with both the pHash and SHA-256 hash of the image.

### Duplicates

Duplicates usually occur when publishers use the same cover for multiple chapters of their comics, and the hashing algorithm cannot distinguish much difference.

The program will not delete duplicates for you; you will need to manually remove them.

### Recommended Meilisearch Settings

We recommend the following settings for `ermitteln-images` index settings:
```jsonc
{
  // Display all attributes (only 'id' and 'hash' are available)
  "displayedAttributes": ["*"],
  // Search on all attributes
  "searchableAttributes": ["*"],
  // Allow filtering on ID for easy bisect search
  "filterableAttributes": ["id"],
  // Enable sorting on ID for use in the frontend
  "sortableAttributes": ["id"],
  // The following are typo-tolerance settings; only the necessary values are provided
  "typoTolerance": {
    "enabled": true,
    "minWordSizeForTypos": {
      // Since the hash is 7 characters long (base64 encoded),
      // we use the following typo tolerance
      "oneTypo": 7, // Enable single chara typo tolerance
      "twoTypos": 8 // Disable typo tolerance on two typos
    },
    // Disable typo tolerance on ID for exact matching.
    "disableOnAttributes": ["id"]
  }
}
```

Everything else can be set to defaults.

You can follow [Meilisearch Tutorial](https://www.meilisearch.com/docs/reference/api/settings) on how to configure your index.

## License

MIT License
