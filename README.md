# cargo-example2

```
cargo install cargo-example2
```

List examples
```sh
❯ cargo example https://github.com/clap-rs/clap-verbosity-flag

error: "--example" takes one argument.
Available examples:
    log
    tracing
```

Run an example
```sh
❯ cargo example https://github.com/clap-rs/clap-verbosity-flag --example tracing

    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `/tmp/cargo-example/862177302567451381/target/debug/examples/tracing`

2023-06-08T06:42:12.641689Z ERROR tracing: Engines exploded
```

Build with --release
```sh
❯ cargo example https://github.com/clap-rs/clap-verbosity-flag --example tracing -- --release
```

Multiple crates in one repo
```sh
❯ cargo example https://github.com/DioxusLabs/dioxus/ --path packages/dioxus-tui/Cargo.toml --example border
```

## vs cargo-example ?
[cargo-example](https://github.com/BrandtM/cargo-example) is archived and does not work with multi-crate repos.


```
cargo example dioxus-tui
```
This will list all examples in the repo, which are not examples of dioxus-tui.
