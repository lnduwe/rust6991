struct English;
struct Spanish;
struct French;

trait Greeting {
    fn greet(&self);
}

impl Greeting for English {
    fn greet(&self) {
        println!("Hello!");
    }
}

impl Greeting for Spanish {
    fn greet(&self) {
        println!("Hola!");
    }
}

impl Greeting for French {
    fn greet(&self) {
        println!("Bonjour!");
    }
}

struct Person {
    name: String,
    // Do you understand why this is `Box<dyn Greeting>>` instead of `Box<Greeting>` ?
    greetings: Vec<Box<dyn Greeting>>,
}

// implement the From trait, such that you can convert a &str into a Box<dyn Greeting>.
//You can assume that only valid strings will be given.
//In a real codebase, you would want to handle errors (maybe by using the TryFrom trait), but for this exercise, you can assume that the input is valid.

impl From<&str> for Box<dyn Greeting> {
    fn from(lan: &str) -> Box<dyn Greeting> {
        match lan {
            "English" => Box::new(English),
            "Spanish" => Box::new(Spanish),
            "French" => Box::new(French),
            _ => unreachable!("Invalid greeting"),
        }
    }
}

// DO NOT NEED TO CHANGE MAIN
fn main() {
    // john can speak English and Spanish
    let person = Person {
        name: "John".to_string(),
        greetings: vec!["English".into(), "Spanish".into()],
    };

    speak_all_greetings(&person);

    // jane can speak French
    let person = Person {
        name: "Jane".to_string(),
        greetings: vec!["French".into()],
    };

    speak_all_greetings(&person);
}

fn speak_all_greetings(person: &Person) {
    println!("{} says:", person.name);
    //TODO (2): iterate over the greetings and call greet() on each one
    for greeting in &person.greetings {
        greeting.greet();
    }
}

#[test]
fn t() {
    let greeting: Box<dyn Greeting> = "English".into();
    greeting.greet();
}
