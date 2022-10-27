use std::io;
fn main() {
    let mut keep: i32 = 0;
    let mut count1: i32 = 0;
    let mut count2: i32 = 0;
    println!("Enter a factorail");
    let mut userinput: String = String::new();
    io::stdin().read_line(&mut userinput).expect("mmm");
    let count = userinput.trim().parse::<i32>().unwrap() - 1;
    let mut number: Vec<String> = Vec::new();
    if userinput.trim().parse::<i32>().unwrap() == 0 {
        println!("1");
    } else if userinput.trim().parse::<i32>().unwrap() == 1 {
        println!("1");
    } else {
        loop {
            if count2 > 0 {
                let last = number[number.len() - 1].parse::<i32>().unwrap();
                keep = keep * (last - 1);
                count1 = count1 + 1;
                if count1 == count {
                    println!("{}", keep);
                    break;
                } else {
                    number.push((last - 1).to_string());
                }
            } else {
                number.push(userinput.trim().to_string());
                keep += userinput.trim().parse::<i32>().unwrap();
                count2 += 1;
            }
        }
    }
}
