
#[derive(Debug)]

struct Dyad {
    name: String,
    essence: String,
    existence: String,
}

fn main() {
    let yinyang = Dyad {
        name: "yinyang".to_string(),
        essence: "yin".to_string(),
        existence: "yang".to_string()
    };

    println!("{:?}", yinyang);
}