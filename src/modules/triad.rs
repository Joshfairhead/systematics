
#[derive(Debug)]
struct Triad {
    name: String,
    active: String,
    passive: String,
    reconciling: String,
}

fn main() {
    let landry = Triad {
        name: "triad".to_string(),
        active: "trancendental".to_string(),
        passive: "omnisient".to_string(),
        reconciling: "immanent".to_string(),
    };

    println!("{:?}", landry);
}