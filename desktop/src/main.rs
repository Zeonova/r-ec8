fn main() {
    println!("Hello, r-ec8!");
    let tuple = (8, 0, 0, 0); // 例子中的元组

    match tuple {
        (8, _, _, 1 | 2 | 3) => {
            println!("test match");
        }
        _ => {
            println!("No match or different value in the fourth position.");
        }
    }
}
