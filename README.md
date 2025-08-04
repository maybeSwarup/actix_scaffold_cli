# Actix Scaffold CLI

✨ **Features**

- Generate Actix-Web modules, routes, middleware, and utils with a single CLI command
- Auto-injects modules into your project (lib.rs, mod.rs, main.rs) using Rust's mod system
- Supports `--no-inject` to skip code injection
- Supports `--with-test` to generate test files for each module
- Auto-creates missing folders/files as needed
- Uses [tera](https://crates.io/crates/tera) for template rendering

🧪 **CLI Examples**

```sh
# Generate a module and inject into src/lib.rs
cargo run -- generate mod my_module

# Generate a route, inject into src/routes/mod.rs, src/lib.rs, and src/main.rs
cargo run -- generate route my_route

# Generate middleware and inject into src/lib.rs
cargo run -- generate middleware my_middleware

# Generate a utils module and inject into src/lib.rs
cargo run -- generate utils my_utils

# Skip injection
cargo run -- generate mod my_module --no-inject

# Generate with test file
cargo run -- generate route my_route --with-test
```

🛠️ **Injection Strategy**

- **Modules**: Injects `pub mod <name>;` into `src/lib.rs`
- **Routes**: Injects `pub mod <route>;` into `src/routes/mod.rs`, ensures `pub mod routes;` in `src/lib.rs`, and ensures `lib.rs` is referenced in `main.rs`
- **Middleware/Utils**: Injects `pub mod <name>;` into `src/lib.rs`
- **Auto-creates**: Any missing folders/files (e.g., `src/routes/mod.rs`)
- **No explicit comment markers**: Injection is done via Rust's conventional mod system
