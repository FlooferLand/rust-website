# My website !! (rust edition)
Used to be in TS using SvelteKit, now it's 100% Rust!

## Compiling for Release
```bash
cargo leptos build --release
```

Will generate the server binary in `target/server/release` and the site package in `target/site`

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
rust-website
site/
```
Set the following environment variables (updating for your project as needed):
```text
LEPTOS_OUTPUT_NAME="rust-website"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## Licensing
This project is licensed under [GPL-3](./LICENSE)
