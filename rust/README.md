# ken.rs

A simple native Rust library for parsing ken and (outputting HTML).

## Usage

To include ken.rs in your project add the following to your Cargo.toml:

```toml
[dependencies]
ken = "0.3"

```

Now you can use the crate in your code with
```rust
extern crate ken;
```

There is no full documentation right now, the only function exported by the library is `to_html`, which takes a markdown `&str` and converts it to an owned `String` containing html.

```rust
let html : String = ken::to_html("__I am ken__");

assert_eq!(&html, "<strong>I am ken</strong>")
```