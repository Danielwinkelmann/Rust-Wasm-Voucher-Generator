use getrandom::getrandom;
use js_sys::Array;
use js_sys::Error;
use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests {
    use crate::generate_password_list;

    #[test]
    fn it_works() {
        let password_list =
            generate_password_list(String::from("TEST-#####"), 10, true, false, true, false);
        assert_eq!(password_list.len(), 10);
    }
}

pub fn return_string_error() -> Result<String, JsValue> {
    Err(Error::new("Possible Combinations not reached!").into())
}

fn generate_char_set(
    char_set_number_enabled: bool,
    char_set_symbol_enabled: bool,
    char_set_uppercase_enabled: bool,
    char_set_lowercase_enabled: bool,
) -> String {
    let char_set_number = "0123456789";
    let char_set_symbol = "@#$%-";
    let char_set_uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let char_set_lowercase = "abcdefghijklmnopqrstovwxyz";
    let mut string_list: Vec<&str> = vec![];
    if char_set_number_enabled {
        string_list.push(char_set_number)
    }
    if char_set_symbol_enabled {
        string_list.push(char_set_symbol)
    }
    if char_set_uppercase_enabled {
        string_list.push(char_set_uppercase)
    }
    if char_set_lowercase_enabled {
        string_list.push(char_set_lowercase)
    }

    if !char_set_number_enabled
        && !char_set_symbol_enabled
        && !char_set_uppercase_enabled
        && !char_set_lowercase_enabled
    {
        string_list.push(char_set_uppercase)
    }

    return string_list.join("");
}

fn random_num(max: u8) -> u8 {
    let mut buf = [0u8];
    let getrandom = getrandom(&mut buf);
    println!("getrandom: {:?}", getrandom);
    let max_value = buf[0] as f32 / 255 as f32 * max as f32;
    let round = max_value.ceil() as u8;
    round
}

fn generate_password(char_set: &String, pattern: &str) -> String {
    let mut password = String::from("");
    for c in pattern.chars() {
        if c == '#' {
            let random_char = char_set
                .chars()
                .nth(random_num((char_set.len() - 1) as u8) as usize)
                .unwrap();
            password.push(random_char)
        } else {
            password.push(c)
        }
    }
    return password;
}

pub fn generate_password_list(
    pattern: String,
    count: i32,
    char_set_number_enabled: bool,
    char_set_symbol_enabled: bool,
    char_set_uppercase_enabled: bool,
    char_set_lowercase_enabled: bool,
) -> Vec<String> {
    let mut password_list: Vec<String> = vec![];
    let char_set = generate_char_set(
        char_set_number_enabled,
        char_set_symbol_enabled,
        char_set_uppercase_enabled,
        char_set_lowercase_enabled,
    );
    let pattern_chars = pattern.matches("#").count();
    let char_set_len = char_set.chars().count();
    let possible_combinations = char_set_len.pow(pattern_chars as u32);
    println!("possible_combinations: {}", possible_combinations);
    if possible_combinations < count as usize {
        let err = return_string_error();
        println!("err: {:?}", err);
    }
    while password_list.len() < count as usize {
        let pass = generate_password(&char_set, &pattern);
        if !password_list.contains(&pass) {
            password_list.push(pass)
        }
    }
    password_list
}

#[wasm_bindgen]
pub fn generate(
    pattern: String,
    count: i32,
    char_set_number_enabled: bool,
    char_set_symbol_enabled: bool,
    char_set_uppercase_enabled: bool,
    char_set_lowercase_enabled: bool,
) -> Array {
    let password_list = generate_password_list(
        pattern,
        count,
        char_set_number_enabled,
        char_set_symbol_enabled,
        char_set_uppercase_enabled,
        char_set_lowercase_enabled,
    );

    let arr = Array::new();
    for item in &password_list {
        arr.push(&JsValue::from(item));
    }
    println!("arr: {:?}", arr);
    return arr;
}
