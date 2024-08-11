fn main() {
    

    let s = String::from("Bonjour les amis");

    let r: &str = first_word(&s);

    println!("{r}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            let result = &s[0..i];

            return result;
        }
    }

    &s[..]
}