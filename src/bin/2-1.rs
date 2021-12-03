use std::io::stdin;

fn main() {
    let mut buf = String::new();
    let mut position = (0, 0);

    while let Ok(len) = stdin().read_line(&mut buf) {
        if len == 0 {
            break;
        }

        let (command, value) = buf.trim().split_once(" ").unwrap();
        let value = value.trim().parse::<u32>().unwrap();

        match command {
            "forward" => position.0 += value,
            "down" => position.1 += value,
            "up" => position.1 -= value,
            _ => unreachable!(),
        }
        buf.clear();
    }

    dbg!(&position);
    dbg!(position.0 * position.1);
}
