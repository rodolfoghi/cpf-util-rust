![GitHub Workflow Status](https://img.shields.io/github/workflow/status/rodolfoghi/cpf-util-rust/Rust) ![Crates.io](https://img.shields.io/crates/v/cpf-util) ![Crates.io](https://img.shields.io/crates/d/cpf-util) ![GitHub issues](https://img.shields.io/github/issues/rodolfoghi/cpf-util-rust)

# CPF util

CPF util inspired in [brazilian-utils/cpf](https://github.com/brazilian-utils/brazilian-utils/blob/master/src/utilities/cpf/index.ts).

## Usage

Add the following to your `Cargo.toml`:
```rust
[dependencies]
cpf_util = "0.1.0"
```

## Examples

Format:
```rust
use cpf_util as cpf;

fn main() {
    println!("{}", cpf::format("94389575104")); // 943.895.751-04
    println!("{}", cpf::format("94389575104000000")); // 943.895.751-04
    println!("{}", cpf::format("943.?ABC895.751-04abc")); // 943.895.751-04
}
```

Validate:
```rust

use cpf_util as cpf;

fn main() {
    println!("{}", cpf::is_valid("foo391.838.38test0-66"); // false
    println!("{}", cpf::is_valid("40364478829")); // true
    println!("{}", cpf::is_valid("962.718.458-60")); // true
}
```