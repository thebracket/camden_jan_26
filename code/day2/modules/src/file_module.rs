mod foo;

pub use foo::interior_greet;

pub fn file_module_greet() {
    println!("Hello from file module!");
}