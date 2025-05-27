#[derive(Debug)]
struct Tetrad {
    name: String,
    ground: String,
    ideal: String,
    instrumental: String,
    directive: String,
}
 
fn main() {
    let cetetrad = Tetrad {
        name: "CE Tetrad".to_string(),
        ground: "Existential Foundation".to_string(),
        ideal: "Origination Myth".to_string(),
        instrumental: "Rich Narrative Structure".to_string(),
        directive: "Law Conformity".to_string(),
    };

    println!("{:?}", cetetrad);
}