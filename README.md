
# Primestack
##### From rust to tailwind. The tech stack for Leptos.

## PrimeStack

[![Leptos](https://img.shields.io/badge/Leptos-EF3939.svg?style=for-the-badge&logoColor=white&logo=leptos)](https://github.com/leptos-rs/leptos)
[![Tailwind](https://img.shields.io/badge/Tailwind-06B6D4.svg?style=for-the-badge&logo=tailwindcss&logoColor=white)](https://tailwindcss.com/)
[![crates.io](https://img.shields.io/crates/v/cargo-primestack.svg?style=for-the-badge&colorA=orange&color=white&label=crate&logo=rust)](https://crates.io/crates/cargo-primestack)

</div>


![Primestack](./resources/images/primestack.png)

-- image provided by [**JonesTown**](https://github.com/blushell) & [**21st Century Man**](https://github.com/21st-centuryman)


## PrimeStack

1. We will mimic creating the vercel example by creating a package.json and a
   README.md on how to get started

2. we will need to create libsql example with easy steps to deploy your first
   time

3. There is some fuckery we have to do with vercel
   - we need to remove that one line.
   - you know what line...

4. Challenges
 - naming... that will be hard
 - Specific Vercel Problem: the aforementioned fuckery
 - Specific Vercel Problem: page based routing done in rust translated to javascript wasm loaders
   - OR we have a singular route and a singular response
   - I think this will be hard for definition with typescript


### Initialize the empty suppository
npx create-next-app --example https://github.com/vercel/examples/tree/main/edge-functions/wasm-rust-xor edge-wasm-rust-xor
# or
yarn create next-app --example https://github.com/vercel/examples/tree/main/edge-functions/wasm-rust-xor edge-wasm-rust-xor

