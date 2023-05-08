use std::collections::HashMap;
use std::process;

fn main() {
    print!("\x1B[2J\x1B[1;1H");

    loop {
        let mut todo = Todo::new().expect("Initialisation of db failed");
        // TODO: Read Action from terminal
        println!("Please enter an action...");
        let mut action = String::new();
        std::io::stdin()
            .read_line(&mut action)
            .expect("Please enter an action");

        print!("\x1B[2J\x1B[1;1H");

        action = action.trim().to_string();

        if action == "exit" {
            print!("\x1B[2J\x1B[1;1H");
            process::exit(exitcode::OK);
        }

        println!("Please enter an item...");
        let mut item = String::new();
        std::io::stdin()
            .read_line(&mut item)
            .expect("Please enter an item");

        item = item.trim().to_string();

        if action == "add" {
            todo.insert(item);
            match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occured: {}", why),
            }
        } else if action == "complete" {
            match todo.complete(&item) {
                None => println!("{} is not present in the list", item),
                Some(_) => match todo.save() {
                    Ok(_) => println!("todo saved"),
                    Err(why) => println!("An error occured: {}", why),
                },
            }
        }
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occured: {}", e),
        }
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;

        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &str) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
