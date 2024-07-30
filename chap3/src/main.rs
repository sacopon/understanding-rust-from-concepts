fn myprint<T: std::fmt::Display>(msg: T) {
    println!("{}", msg);
}

fn main() {
    let s = "Hello".to_string();
    let ss = s.clone(); // s のコピーを ss に作っておく
    myprint(s); // s の所有権が関数内の変数に移動
    myprint(ss);    // ss の所有権が関数内の変数に移動
}
