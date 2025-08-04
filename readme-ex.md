# Rust Actix CLI Scaffolding Tool

This CLI tool helps you quickly scaffold modules, routes, middleware, and utilities for an Actix-Web project — inspired by the structure and automation of NestJS.

---

## 🛡️ Features

- `generate mod <mod_name>`: Creates a general Rust module under `src/`, and injects it into `lib.rs`.
- `generate route <route_name>`: Creates a route handler module under `src/routes/`, injects it into `routes/mod.rs`, and wires it to `init_routes()`.
- `generate middleware <middleware_name>`: Scaffolds an Actix middleware with a ready-made template under `src/middleware/`.
- `generate utils <utils_name>`: Creates a utility module under `src/utils/` with helpful function templates.
- ✅ Auto-injection of new modules into relevant `mod.rs` or `lib.rs` files
- ✅ Uses **convention-based injection** with no need for manual edits
- ✅ `--no-inject` flag to skip injection if desired

---

## 📸 CLI Examples

```
$ cargo run -- generate route health_check
✔ Created file: src/routes/health_check.rs
✔ Injected mod route into: src/routes/mod.rs
✔ Hooked handler into: init_routes()
✔ Injected mod into lib.rs
```

---

## 🔁 Injection Strategy

This tool follows **NestJS-like conventions** for injecting modules and routes into the project:

### 🔹 mod injection

- Automatically inserts `pub mod <name>;` at the top of `lib.rs` or `mod.rs`, unless already present.

### 🔹 route injection

- Locates your `init_routes(cfg: &mut ServiceConfig)` function and appends `cfg.service(<name>::<handler_fn>);` inside it

### 🔹 Duplicate-safe

- Ensures the same module or handler isn’t injected more than once
