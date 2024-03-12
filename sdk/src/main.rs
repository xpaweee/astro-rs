use crate::persistence::token_manager::{ TokenManager};

mod persistence;
mod errors;

fn main() {

    let token_manager = TokenManager { };
    let result = token_manager.save_token("test5", "test55");
    println!("{:?}", result);
    println!("Result: ? {}", token_manager.get_token("test5").unwrap());
}
