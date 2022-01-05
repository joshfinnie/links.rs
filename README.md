# Links

This is a small website built using [Yew](https://yew.rs/).
It uses [trunk](https://trunkrs.dev/) to run locally, and build for deployment.

I wanted to explore this library and see how it all works.


## Installation

```bash
$ rustup target add wasm32-unknown-unknown
$ cargo install --locked trunk
$ cargo install wasm-bindgen-cli
$ trunk serve
```

## Deployments

Deployments are handled via Github Actions.
We deploy manually to Netlify after building.
