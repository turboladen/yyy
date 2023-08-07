# yyy

"Yes yes yessss"

## Development

### Quick Notes

- To recompile on change: `cargo watch -x "run serve"`
- Check the `Justfile` for other helpful commands.

### Getting Started

#### Create the database

You shouldn't need any dependencies outside of what's defined in the `Cargo.toml`. Before you start,
you should create the database so it's set up with schemas and such (things will work if you don't
do this, but you also won't get any enforcement from Surreal of the schema we're using).

```
$ cargo run db-create
```

You won't see any logging unless you use `RUST_LOG=debug` or something along those lines.

#### Seed the database

After that, you may want to seed the database. The CLI provides a command (`import`) for importing a
YAML file into a table. The `Justfile` task `seed` will do all of the seeding for you though, so the
easiest way here is to

```
$ just seed

```

#### Run the app

Even if you don't do the previous two steps, you should be able to simply run the app:

```
$ cargo run server

# OR...

$ just serve
```

...where the `just` command will turn on logging for you.
