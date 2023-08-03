# Zero to Production in Rust

This repository contains Rust code for practicing the concepts taught in the
book "[Zero To Production In Rust](https://www.zero2prod.com/index.html)". The application is currently deployed on
Fly.io, and you can access it
at [https://marvinhsu-zero-to-production.fly.dev/](https://marvinhsu-zero-to-production.fly.dev/).
Additionally, [Swagger-UI](https://marvinhsu-zero-to-production.fly.dev/swagger-ui/) is available.

## Differences

The following table shows the differences between the book and this repository:

| Item             | Book          | Repository          |
|:-----------------|:--------------|:--------------------|
| Framework        | Actix-web     | Axum                |
| Platform         | DigitalOcean  | Fly.io              |
| DB tool          | sqlx          | seaORM              |
| Integration test | rust build-in | pytest              |
| CI Accelerate    | Cargo-chief   | Github-action cache |