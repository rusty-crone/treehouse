#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the tree house, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the tree house, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("jason", VisitorAction::Accept, 30, "Hey Jason, WASSUUUP!"),
        Visitor::new(
            "brad",
            VisitorAction::AcceptWithNote {
                note: String::from("WHAT?!"),
            },
            45,
            "Wake up bro, time to feed the baby!",
        ),
        Visitor::new(
            "sierra",
            VisitorAction::AcceptWithNote {
                note: String::from("Sup dude?!"),
            },
            10,
            "Hey what's going on yo?",
        ),
        Visitor::new("todd", VisitorAction::Refuse, 51, "You are old dude."),
    ];

    loop {
        println!("Hello, what's your name?");
        let name = what_is_your_name();
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        if let Some(visitor) = known_visitor {
            visitor.greet_visitor();
        } else {
            if name.is_empty() {
                break;
            }
            println!("{} is not on the visitor list.  Adding...", name);
            visitor_list.push(Visitor::new(
                &name,
                VisitorAction::Probation,
                0,
                "New friend",
            ));
        }
    }
    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}
