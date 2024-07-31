fn main() {
    let ary = [0, 1, 2, 3]; // [i32; 4]型と推論される

    for aa in &ary {
        println!("{aa}");
    }
    println!("ary[1] = {}", ary[1]);
}
