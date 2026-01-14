mod inline_module {
    pub fn greet() {
        println!("Hello from inline module!");
    }
}

mod file_module;
mod dir_module;

fn main() {
    inline_module::greet();
    file_module::hello_l3();
    file_module::bar::hello_bar();

    file_module::file_module_greet();
    file_module::interior_greet();

    dir_module::hello_from_dir_module();
}
