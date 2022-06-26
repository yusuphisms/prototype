mod fancy;

fn main() {
    println!("Hello, world!");
    let f = fancy::generated::Letter {
        from: "Me".into(),
        to: "You".into(),
        body: "Cause it's".into(),
        count: 1,
    };
    println!("{:?}", f);
}
