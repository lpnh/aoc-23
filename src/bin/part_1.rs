fn main() {
    let input = include_str!("../input_1.txt");
    let output = solution(input);

    println!("{}", output);
}

fn solution(input: &str) -> String {
    input.to_string()
}
