use std::io;

fn get_user_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {}
    }
    input
}

fn ask_for_user_input(prompt: &str, accept_empty: bool) -> String{
    println!("{}",prompt);
    let mut user_input = get_user_input().trim().to_string();
    match user_input.is_empty() {
        true =>  {
            if accept_empty == true {
                user_input
            } else {
                while user_input.is_empty() {
                    println!("Please give a response.");
                    user_input = get_user_input().trim().to_string();
                }
                user_input
            }
        },
        false => user_input,
    }
}
pub fn get_input(prompt: &str) -> String {
    ask_for_user_input(prompt, false)
}
pub fn get_nullable_input(prompt: &str) -> String {
    ask_for_user_input(prompt, true)
}