watch:
  RUST_LOG=debug cargo watch -x "run serve"

serve:
  RUST_LOG=debug cargo run serve

seed:
  RUST_LOG=debug cargo run -- db-create
  RUST_LOG=debug cargo run -- import brands seeds/brands.yml
