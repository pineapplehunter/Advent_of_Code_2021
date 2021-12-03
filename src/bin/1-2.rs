use std::io::stdin;

fn main() {
    let mut buf = String::new();
    let mut nums = Vec::new();

    while let Ok(len) = stdin().read_line(&mut buf) {
        if len == 0{
            break;
        }
        let num = buf.trim().parse::<u32>().unwrap();
        nums.push(num);
        buf.clear();
    }

    let mut last = None;
    let mut increase = 0;

    for i in 0..nums.len()-2 {
        let sum = nums[i] + nums[i+1] + nums[i+2];
        if let Some(last) = last {
            if sum > last {
                increase += 1;
            }
        }
        last = Some(sum);
    }
    
    dbg!(increase);
}