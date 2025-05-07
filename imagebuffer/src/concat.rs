pub fn concat(strings: &[&str]) -> String {
    strings.concat()
}

fn main() {
    let parts = vec!["Hello", " ", "World", "!"];
    let result = concat(&parts);
    println!("{}", result); // 输出: Hello World!
}
