use std::io::stdin;

fn main() {
    let mut buf = String::new();

    let mut count0 = [0; 12];
    let mut count1 = [0; 12];

    while let Ok(len) = stdin().read_line(&mut buf) {
        if len == 0 {
            break;
        }

        let bin_str = buf.trim();
        for (index, c) in bin_str.chars().enumerate() {
            match c {
                '0' => count0[index] += 1,
                '1' => count1[index] += 1,
                _ => unreachable!(),
            }
        }
        buf.clear();
    }

    let mut gamma:u32 = 0;

    for (z, o) in count0.iter().zip(count1.iter()) {
        gamma <<= 1;
        if o > z {
            gamma |= 1
        }
    }

    dbg!(&gamma);

    let epsilon = !gamma & 0xFFF;

    dbg!(&epsilon);

    dbg!(gamma*epsilon);
}
