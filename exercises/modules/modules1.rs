// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.



mod sausage_factory {
    // Make this function public
    pub fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        // Call the get_secret_recipe function using the module path
        let recipe = get_secret_recipe(); // 此处修改
        println!("Making sausage with secret recipe: {}", recipe);
    }
}

fn main() {
    sausage_factory::make_sausage();
}

