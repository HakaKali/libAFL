fn target_function(input: &[u8]) {
    let input_str = match std::str::from_utf8(input) {
        Ok(s) => s,
        Err(_) => return,
    };

    let result = input_str.to_uppercase();

    println!("Input: {:?}, Result: {}", input_str, result);
}

fn main() {
    // Example input
    let input = b"hello, world";

    // Call the target function with the input
    target_function(input);
}

//Input: "hello, world", Result: HELLO, WORLD
