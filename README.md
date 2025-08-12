# Horse Sea Men

## Development

### Setup

First, install [Rust](https://www.rust-lang.org/tools/install).

Then install trunk

```bash
cargo install --locked trunk
```

Add webasm target

```bash
rustup target add wasm32-unknown-unknown
```

### Dev enviroment

Run trunk to serve content

```bash
turnk serve
# open in browser
trunk serve --open
```
