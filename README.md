# CMX Ermitteln

A quick reverse image lookup using the [pHash (DCT)] algorithm, designed specifically for CMX cover.

Powered by [Meilisearch](https://www.meilisearch.com/).

> [!NOTE]
> I need help ingesting some missing IDs. Please contact **@noaione** on Discord so that I can provide you
> with the ingestion key for my Meilisearch instance.

### Public Version

https://ermitteln.ihateani.me

Currently in the process of ingesting old IDs (below `5xxxxx`).

## Requirements
1. Node 18+
2. Rust 1.72+
3. Meilisearch instance
4. wasm-pack

## Installation
1. Clone this repository.
2. Install Node, Rust, Meilisearch, and wasm-pack.
3. Build cargo crates: `cargo build --release --all`
4. Build the WASM package: `wasm-pack build ermitteln-wasm --target web`
5. Install Node dependencies: `npm install`
6. Build the frontend: `npm run build`
7. Ingest your images into Meilisearch.
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

## License

MIT License
