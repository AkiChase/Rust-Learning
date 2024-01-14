fn main() {
    // scope();
    // quote();
    let mut s: String = String::from("hello");

    mut_quote(&mut s);
}

// 作用域
fn _scope() {
    let x = 666;
    println!("x={}", x);
    {
        let x = 777;
        println!("x={}", x);
    }
    println!("x={}", x);
}

fn _quote() {
    let x: &str = "hello, world";
    let y = x; // y=x将引用这一基本数据类型进行了拷贝
    println!("{}\n{}", x, y); // 此时不报错
}

fn mut_quote(v: &mut String) {
    *v = String::from("你好"); // 可变引用需要修改值时，要解引用
    println!("v={}", *v);
}
