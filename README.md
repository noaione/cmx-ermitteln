# CMX Ermitteln

A quick reverse image lookup using pHash (DCT) algorithm.

Powered by Meilisearch.

> [!NOTE]
> I need help ingesting some missing IDs, please contact @noaione in Discord so I can give you
> the ingestion key for my Meilisearch instance.

### Public version

https://ermitteln.n4o.xyz (WIP)

## Requirements
1. Node 18+
2. Rust 1.72+
3. Meilisearch instance
4. wasm-pack

## Installation
1. Clone this repository
2. Install all requirements
3. Build cargo crates: `cargo build --release --all`
4. Build the WASM package: `wasm-pack build ermitteln-wasm --target web`
5. Build the frontend: `npm run build`
6. Run the server: `node ./frontend/.output/server/index.mjs`

## Ingesting
You would need to download CMX cover into a folder, we are expecting specific filename like this: `CmxID.jpg`

1. After you build all the crates, get the `aufnehmen` binary from `target/release` then copy it to your CMX cover parent directory.
2. Add `MEILI_URL=your_meili_search_instance_url` and `MEILI_KEY=your_api_key_or_master_key` to your environemnt
3. Run the binary: `./aufnehmen covers-dir`
4. Wait for the ingestion to complete
5. You should have a new index called `ermitteln-images`

## License

MIT License
