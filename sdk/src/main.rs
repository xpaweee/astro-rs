use crate::persistence::token_manager::{ TokenManager};

mod persistence;
mod errors;

fn main() {
    println!("Hello, world!");

    let token_manager = TokenManager { };
    let result = token_manager.save_token("test1", "test2");
    println!("Hello, world!2");
}
