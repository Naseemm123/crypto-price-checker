
#[derive(Debug)]
struct a {
    b : Option<String>,
}


fn main() {
    let x = a {
        b : Some("usd".to_string()),
    };

    println!("{:?}", x);
}