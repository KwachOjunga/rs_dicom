struct Tram {
    a: Vec<String>,
    b: Vec<u32>,
}

fn main() {
    let a = vec!["jump".into(), "down".into(), "away".into()];
    let b = vec![32, 45, 56];
    let instance = Tram { a, b };
    for i in &instance.b {
        for m in &instance.a {
            let out = format!("{},{}", i, m);
            println!("{}", out);
        }
    }
}
