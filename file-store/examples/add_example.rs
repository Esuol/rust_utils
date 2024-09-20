fn main() {
    let result = file_store::add(2, 3);
    println!("2 + 3 = {}", result);
    assert_eq!(result, 5);
}
