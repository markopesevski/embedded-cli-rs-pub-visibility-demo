# `pub` visibility example

This example showcases a bug by which an enum that `derive(embedded_cli::Command)` lives in an external crate, can't have its `::processor` called from a different crate.

Currently, it doesn't build. Fails with error:

```rust
error[E0624]: associated function `processor` is private
 --> app/src/main.rs:9:27
  |
9 |         &mut BaseCommand::processor(|_h, _c| match _c {
  |                           ^^^^^^^^^ private associated function
  |
 ::: /home/marko/git/embedded-cli-rs-pub-visibility-demo/cli/src/lib.rs:1:10
  |
1 | #[derive(embedded_cli::Command)]
  |          --------------------- private associated function defined here

For more information about this error, try `rustc --explain E0624`.
error: could not compile `pub-visibility` (bin "pub-visibility") due to 1 previous error
```

To get it to build, a fix must be applied. To do so, apply the following diff in the Cargo.toml file in the root:

```diff
-embedded-cli = { git = "https://github.com/funbiscuit/embedded-cli-rs.git", rev = "45597c350239d70444d6d84ec84c7d78dc1ac044" }
-# embedded-cli = { git = "https://github.com/markopesevski/embedded-cli-rs.git", rev = "cd6e064e0cecdc840bd35b3c59ccfece92a6968d" }
+# embedded-cli = { git = "https://github.com/funbiscuit/embedded-cli-rs.git", rev = "45597c350239d70444d6d84ec84c7d78dc1ac044" }
+embedded-cli = { git = "https://github.com/markopesevski/embedded-cli-rs.git", rev = "cd6e064e0cecdc840bd35b3c59ccfece92a6968d" }
```
