# cargo-example


```
cargo install cargo-example
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
