#[must_use]
fn calc(x: u64, y: u64) -> u64 {
    x + y
}

pub fn run_must_use() {
    let v = calc(5, 6);
    println!("must used value v: {}", v);

    // 如果含下面这行代码, 编译时会受到警告, 因为其返回值没有被使用
    // calc(5, 6);
}
