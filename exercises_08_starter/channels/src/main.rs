use itertools::Itertools;
use std::thread;
mod test;
fn main() {
    // take number from commandline arg
    // number is guaranteed to be five digits
    let input_number = std::env::args().nth(1).unwrap().parse::<u32>().unwrap();
    if !(10000..=99999).contains(&input_number) {
        panic!("Number must be five digits");
    }

    let operators = vec!['+', '-', '*', '/'];

    // let's get a massive iterator,
    // over every arrangement of
    // digits and every arrangement of operators
    let digits_operators: Vec<(Vec<i32>, Vec<char>)> = std::env::args()
        .nth(1)
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .permutations(5)
        .into_iter()
        .cartesian_product(operators.into_iter().permutations(4).into_iter())
        .collect();

    let length = digits_operators.len();
    println!("There are {length} potential combinations",);

    // you only need to change code from here onwards
    // first, split up the digits_operators into 6 vecs
    // using the chunks method
    let threads = 6;
    let mut chunks = digits_operators
        .chunks(length / threads)
        .collect::<Vec<_>>()
        .into_iter();

    let (sender, receiver) = std::sync::mpsc::channel();
    thread::scope(|s| {
        for i in 0..threads {
            if let Some(chunk) = chunks.next() {
                let sender = sender.clone();
                s.spawn(move || {
                    let mut count = 0;
                    for (digits, operators) in chunk {
                        if let Ok(_) = calculate(digits.to_vec(), operators.to_vec()) {
                            count += 1;
                        }
                    }
                    sender.send((i, count)).unwrap();
                });
            }
        }
    });

    let mut sum = 0;
    for _ in 0..threads {
        let (i, count) = receiver.recv().unwrap();
        println!("Thread {} found {} combinations", i, count);
        sum += count;
    }
    println!("Total: {}", sum);
}

// DO NOT MODIFY
fn calculate(digits: Vec<i32>, operators: Vec<char>) -> Result<(), ()> {
    let num1 = digits[0];
    let num2 = digits[1];
    let num3 = digits[2];
    let num4 = digits[3];
    let num5 = digits[4];

    let op1 = operators[0];
    let op2 = operators[1];
    let op3 = operators[2];
    let op4 = operators[3];

    let result = operate(num1, num2, op1)?;
    let result = operate(result, num3, op2)?;
    let result = operate(result, num4, op3)?;
    let result = operate(result, num5, op4)?;

    if result == 10 {
        println!(
            "{} {} {} {} {} {} {} {} {} = 10",
            num1, op1, num2, op2, num3, op3, num4, op4, num5
        );
        return Ok(());
    }
    Err(())
}

// DO NOT MODIFY
fn operate(num1: i32, num2: i32, op: char) -> Result<i32, ()> {
    match op {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '/' => num1.checked_div(num2).ok_or(()),
        _ => panic!("Invalid operation"),
    }
}
