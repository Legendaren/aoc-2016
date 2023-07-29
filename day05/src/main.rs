fn part1() -> String {
    let input = "reyedfim";
    let prefix = String::from(input);
    let mut password = String::new();
    let mut i = 0;
    while password.len() < 8 {
        let hash_input = format!("{}{}", prefix, i);
        let hash = md5::compute(hash_input);
        let hash_str = format!("{:x}", hash);
        if hash_str.chars().take(5).collect::<String>() == "00000" {
            password.push(hash_str.chars().nth(5).unwrap());
        }
        i += 1;
    }
    password
}

fn main() {
    let part1_answer = part1();
    println!("{}", part1_answer);
}
