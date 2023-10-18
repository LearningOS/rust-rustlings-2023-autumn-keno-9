// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.



mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        let secret_recipe = get_secret_recipe();
        println!("Making sausage with secret recipe: {}!", secret_recipe);
    }
}

fn main() {
    sausage_factory::make_sausage();
}
