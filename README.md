# World Athletics Points Calculator

## Pre-Requisites

You can add the `wasm` compilation target to rust using

```sh
rustup target add wasm32-unknown-unknown
```

In order to use tailwind, you will need to run

```sh
npm install -D tailwindcss
```

## Developing your Leptos CSR project

To develop your Leptos CSR project, running

```sh
trunk serve --port 3000 --open
```

will open your app in your default browser at `http://localhost:3000`.

## Deploying your Leptos CSR project

To build a Leptos CSR app for release, use the command

```sh
trunk build --release
```

This will output the files necessary to run your app into the `dist` folder; you can then use any static site host to serve these files.

For further information about hosting Leptos CSR apps, please refer to [the Leptos Book chapter on deployment available here][deploy-csr].

[Leptos]: https://github.com/leptos-rs/leptos
[Trunk]: https://github.com/trunk-rs/trunk
[Trunk-instructions]: https://trunkrs.dev/assets/
[deploy-csr]: https://book.leptos.dev/deployment/csr.html
