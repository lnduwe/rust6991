use std::io;

fn main() {
    let pattern_string = std::env::args()
        .nth(1)
        .expect("missing required command-line argument: <pattern>");

    let pattern = &pattern_string;

    // TODO: Replace the following with your code:
    // println!("The command-line argument is: {pattern}");

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut  input);

    if input.contains(pattern){
      println!("{}",input);
    }
    
}
