#[derive(Debug)]
struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn name(&self) {
        println!("philosopher is {}", self.name);
    }
}

fn main() {

    let philosophers = vec![
        Philosopher::new("hogehoge"),
        Philosopher::new("hugahuga"),
        Philosopher::new("piyopiyo"),
    ];

    for p in philosophers {
        p.name();
    }
}
