use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let all_inputs: Vec<&str> = buf.split('\n').collect();

    let mut count0 = [0; 12];
    let mut count1 = [0; 12];

    // gamma, epsilon

    for input in &all_inputs {
        for (index, c) in input.chars().enumerate() {
            match c {
                '0' => count0[index] += 1,
                '1' => count1[index] += 1,
                _ => unreachable!(),
            }
        }
    }

    let mut gamma: u32 = 0;

    for (z, o) in count0.iter().zip(count1.iter()) {
        gamma <<= 1;
        if o > z {
            gamma |= 1
        }
    }

    dbg!(&gamma);

    let epsilon = !gamma & 0xFFF;

    dbg!(&epsilon);

    dbg!(gamma * epsilon);

    // O2
    let o2 = {
        let mut all_inputs = all_inputs.clone();
        for i in 0..12 {
            let mut count0 = 0;
            let mut count1 = 0;
            for input in &all_inputs {
                match input.as_bytes()[i] {
                    b'0' => count0 += 1,
                    b'1' => count1 += 1,
                    _ => unreachable!(),
                }
            }
            if count1 >= count0 {
                all_inputs = all_inputs
                    .into_iter()
                    .filter(|input| input.as_bytes()[i] == b'1')
                    .collect();
            } else {
                all_inputs = all_inputs
                    .into_iter()
                    .filter(|input| input.as_bytes()[i] == b'0')
                    .collect();
            }
        }
        assert_eq!(all_inputs.len(), 1);
        dbg!(&all_inputs);
        let mut o2 = 0;
        for c in all_inputs[0].chars() {
            o2 <<= 1;
            if c == '1' {
                o2 |= 1;
            }
        }
        dbg!(o2)
    };

    // CO2
    let co2 = {
        let mut all_inputs = all_inputs.clone();
        for i in 0..12 {
            let mut count0 = 0;
            let mut count1 = 0;
            for input in &all_inputs {
                match input.as_bytes()[i] {
                    b'0' => count0 += 1,
                    b'1' => count1 += 1,
                    _ => unreachable!(),
                }
            }
            if count1 == 0 {
                count1 = u32::MAX;
            }
            if count0 == 0 {
                count0 = u32::MAX;
            }
            assert_ne!(count1, 0);
            assert_ne!(count0, 0);
            if count1 >= count0 {
                all_inputs = all_inputs
                    .into_iter()
                    .filter(|input| input.as_bytes()[i] == b'0')
                    .collect();
            } else {
                all_inputs = all_inputs
                    .into_iter()
                    .filter(|input| input.as_bytes()[i] == b'1')
                    .collect();
            }
        }
        assert_eq!(all_inputs.len(), 1);
        dbg!(&all_inputs);
        let mut co2 = 0;
        for c in all_inputs[0].chars() {
            co2 <<= 1;
            if c == '1' {
                co2 |= 1;
            }
        }
        dbg!(co2)
    };

    dbg!(o2 * co2);
}
