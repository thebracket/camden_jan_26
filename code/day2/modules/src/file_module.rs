mod foo;
pub mod bar;

pub use foo::interior_greet;

pub fn file_module_greet() {
    println!("Hello from file module!");
}

 pub fn hello_l3() {
    println!("Hello from hello l3!");
}