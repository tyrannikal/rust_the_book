fn main() {
    let a_string = "word1 word2 word3";
    println!("the first word is: {}", get_first_word(a_string))
}

fn get_first_word(a_string: &str) -> &str {
    let mut count = 0;

    for a_char in a_string.chars() {
        if a_char != ' ' {
            count += 1;
        } else {
            break;
        }
    }
    &a_string[..count]
}
