fn main() {
    let a_string = "word1 word2 word3";
    println!("the first word is: {}", get_first_word(a_string))
}

fn get_first_word(a_string: &str) -> String {
    let mut result = String::new();

    for a_char in a_string.chars() {
        if a_char != ' ' {
            result.push(a_char);
        } else {
            break;
        }
    }
    result
}
