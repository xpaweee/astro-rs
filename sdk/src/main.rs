use crate::persistence::token_manager::{ TokenManager};
mod persistence;
mod errors;

fn main() {

    let token_manager = TokenManager { };
    let result = token_manager.save_token("test8", "test88");
    println!("{:?}", result);

    let value = token_manager.get_token("test8");

    match value {
        Ok(value) => println!("{}", value),
        Err(e) => println!("{}", e)
    };
}
