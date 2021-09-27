fn main() -> std::io::Result<()> {
    let p = std::path::Path::new("./inputs/p11.txt");

    let lines = std::fs::read_to_string(p).unwrap();

    let mut entries = [[0 ; 20] ; 20];

    for (rx, l) in lines.split("\n").enumerate() {
        for (cx, l2) in l.split_ascii_whitespace().enumerate() {
            let n : u32 = l2.parse().unwrap();

            entries[rx][cx] = n;
        }
    }

    for row in entries {
        for colel in row {
            eprint!("{:0>2} ", colel);
        }
        eprint!("\n");
    }
    Ok(())
}