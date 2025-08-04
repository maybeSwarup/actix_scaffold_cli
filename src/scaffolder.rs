use std::fs::{ create_dir_all, OpenOptions };
use std::io::Write;
use std::path::Path;

use crate::template::{ get_template, get_test_template };

fn ensure_lib_rs() {
    let lib_path = std::path::Path::new("src/lib.rs");
    if !lib_path.exists() {
        std::fs::write(lib_path, "// lib.rs auto-created by Actix Scaffold CLI\n").unwrap();
    }
}

pub fn generate_mod(name: &str, no_inject: bool, with_test: bool) {
    let mod_path = format!("src/{name}.rs");
    if Path::new(&mod_path).exists() {
        println!("Module already exists: {mod_path}");
        return;
    }
    let contents = get_template("mod", name);
    std::fs::write(&mod_path, contents).unwrap();
    println!("✔ Created file: {mod_path}");

    if with_test {
        let test_path1 = format!("src/{}_test.rs", name);
        let test_path2 = format!("src/{}.test.rs", name);
        let test_content = get_test_template("mod", name);
        std::fs::write(&test_path1, &test_content).unwrap();
        std::fs::write(&test_path2, &test_content).unwrap();
        println!("✔ Created test: {test_path1}");
        println!("✔ Created test: {test_path2}");
    }

    if !no_inject {
        ensure_lib_rs();
        inject_to("src/lib.rs", &format!("pub mod {name};"));
    }
}

pub fn generate_route(name: &str, no_inject: bool, with_test: bool) {
    let route_dir = Path::new("src/routes");
    if !route_dir.exists() {
        create_dir_all(&route_dir).unwrap();
    }

    let route_path = format!("src/routes/{name}.rs");
    std::fs::write(&route_path, get_template("route", name)).unwrap();
    println!("✔ Created file: {route_path}");

    if with_test {
        let test_path1 = format!("src/routes/{}_test.rs", name);
        let test_path2 = format!("src/routes/{}.test.rs", name);
        let test_content = get_test_template("route", name);
        std::fs::write(&test_path1, &test_content).unwrap();
        std::fs::write(&test_path2, &test_content).unwrap();
        println!("✔ Created test: {test_path1}");
        println!("✔ Created test: {test_path2}");
    }

    let mod_rs = "src/routes/mod.rs";
    if !Path::new(mod_rs).exists() {
        std::fs::write(mod_rs, "// mod.rs auto-created by Actix Scaffold CLI\n").unwrap();
    }
    if !no_inject {
        ensure_lib_rs();
        inject_to(mod_rs, &format!("pub mod {name};"));
        inject_to("src/lib.rs", "pub mod routes;");
        inject_to("src/main.rs", "// Ensure lib.rs is used");
    }
}

pub fn generate_middleware(name: &str, no_inject: bool, with_test: bool) {
    let dir = Path::new("src/middleware");
    if !dir.exists() {
        create_dir_all(&dir).unwrap();
    }

    let file_path = format!("src/middleware/{name}.rs");
    std::fs::write(&file_path, get_template("middleware", name)).unwrap();
    println!("✔ Created middleware: {file_path}");

    if with_test {
        let test_path1 = format!("src/middleware/{}_test.rs", name);
        let test_path2 = format!("src/middleware/{}.test.rs", name);
        let test_content = get_test_template("middleware", name);
        std::fs::write(&test_path1, &test_content).unwrap();
        std::fs::write(&test_path2, &test_content).unwrap();
        println!("✔ Created test: {test_path1}");
        println!("✔ Created test: {test_path2}");
    }

    if !no_inject {
        ensure_lib_rs();
        inject_to("src/lib.rs", &format!("pub mod middleware;"));
        inject_to("src/lib.rs", &format!("pub mod {name};"));
    }
}

pub fn generate_utils(name: &str, no_inject: bool, with_test: bool) {
    let dir = Path::new("src/utils");
    if !dir.exists() {
        create_dir_all(&dir).unwrap();
    }

    let file_path = format!("src/utils/{name}.rs");
    std::fs::write(&file_path, get_template("utils", name)).unwrap();
    println!("✔ Created utils module: {file_path}");

    if with_test {
        let test_path1 = format!("src/utils/{}_test.rs", name);
        let test_path2 = format!("src/utils/{}.test.rs", name);
        let test_content = get_test_template("utils", name);
        std::fs::write(&test_path1, &test_content).unwrap();
        std::fs::write(&test_path2, &test_content).unwrap();
        println!("✔ Created test: {test_path1}");
        println!("✔ Created test: {test_path2}");
    }

    if !no_inject {
        ensure_lib_rs();
        inject_to("src/lib.rs", &format!("pub mod utils;"));
        inject_to("src/lib.rs", &format!("pub mod {name};"));
    }
}

fn inject_to(file: &str, line: &str) {
    let path = Path::new(file);
    if !path.exists() {
        std::fs::write(file, format!("{line}
")).unwrap();
        return;
    }

    let content = std::fs::read_to_string(path).unwrap();
    if content.contains(line) {
        return;
    }

    let mut file = OpenOptions::new().append(true).create(true).open(path).unwrap();
    writeln!(file, "{line}").unwrap();
}
