use std::env;
use std::num::ParseIntError;

struct TribonacciError(String);

fn main() {
    let args: Vec<String> = env::args().collect();
    let error_message = String::from("Please enter a valid size");

    let size = match args.get(1) {
        Some(s) => s.parse::<usize>(),
        None => Ok(1),
    };

    if let Err(e) = compute_tribonacci(size, error_message) {
        println!("Error: {}", e.0)
    }
}

/// Computes the tribonacci sequence of a given size
/// Prints the sequence, and its sum
fn compute_tribonacci(
    size: Result<usize, ParseIntError>,
    // The error message your function should return
    // inside the `TribonacciError` struct
    error_msg: String,
) -> Result<(), TribonacciError> {
    // TODO: complete this function!
    let sz = size.unwrap() ;
     
      
     
       
          
            if sz < 3 || sz > 145 {
                return Err(TribonacciError(error_msg));
            }
            print!("Values: [");
            let mut sum: u128 = 0;
            let mut vec = vec![1, 1, 1];
            for i in 0..sz {
                if i < 3 {
                    print!("1");
                    if i < sz - 1 {
                        print!(", ");
                    } else {
                        print!("]\n")
                    }
                    sum += 1;
                    continue;
                }
                let tmp = vec[i - 3] + vec[i - 2] + vec[i - 1];
                print!("{}", tmp);
                sum += tmp;
                vec.push(tmp);

                if i < sz - 1 {
                    print!(", ");
                } else {
                    print!("]\n")
                }
            }
            println!("\nSum: {}", sum);
            Ok(())
         
         
     
}
