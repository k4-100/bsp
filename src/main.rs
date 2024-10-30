fn main() {
    println!("\n");

    let map: String = (0..30)
        .map(|_| {
            let row: String = (0..80).map(|_| ".").collect();
            format!("{row}\n")
        })
        .collect();

    println!("{}", map);
}
