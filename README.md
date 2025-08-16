# 🐲 scalar-rs
*a data oriented, [I'M NOT GOING TO SAY IT], ergonomic cms system*

scalar is a cms that follows in the footsteps of [sanity's work on data-driven content](https://sanity.io), written in rust for stronger type guarantees and just to prove that i can.

## project status
here be dragons. the api is not stable, everything is subject to change. consider this a pre-alpha.

## try a demo

there'll be a smoother demo experience soon, but here's what you need to see a comprehensive example:
1. make sure you have rust and bun installed
2. spin up a [surrealdb](https://surrealdb.com) instance and an s3 compatable bucket (i use [minio](https://min.io))
3. update connection and credentials in ``scalar-axum/src/examples/test.rs``
4. start the test example with ``cargo run -p scalar-axum --example test``
5. in the scalar-cp directory, run ``bun dev``
6. visit ``localhost:5173`` in your browser
