use std::io::stdin;

fn main() {
    let mut last = 0;
    let mut increase = 0;
    let mut buf = String::new();
    while let Ok(len) = stdin().read_line(&mut buf) {
        if len == 0 {
            break;
        }
        dbg!(&buf);
        let num = buf.trim().parse::<u32>().unwrap();
        if num > last {
            increase += 1;
        }
        last = num;
        buf.clear();
    }
    println!("{}", increase - 1);
}
