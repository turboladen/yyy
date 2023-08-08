default:
  @just --list

watch:
  RUST_LOG=debug cargo watch -x "run serve"

serve:
  RUST_LOG=debug cargo run serve

migrate:
  RUST_LOG=debug cargo run -- db-migrate

seed: migrate
  RUST_LOG=debug cargo run -- import brands seeds/brands.yml
  RUST_LOG=debug cargo run -- import vendors seeds/vendors.yml
  RUST_LOG=debug cargo run -- import project-archetypes seeds/project-archetypes.yml

seed-one table: migrate
  RUST_LOG=debug cargo run -- import {{table}} seeds/{{table}}.yml
