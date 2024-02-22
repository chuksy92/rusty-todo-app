use std::io::{self, Write};

struct TodoItem {
    id: u32,
    description: String,
    completed: bool
}


impl TodoItem {
    fn new(id: u32, description: String) -> TodoItem {
        TodoItem {
            id,
            description,
            completed: false
        }
    }

    fn complete(&mut self ) {
        self.completed = true;
    }
}

struct TodoList {
    list: Vec<TodoItem>
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            list: Vec::new()
        }
    }

    fn add_item(&mut self, description: String) {
        let id = (self.list.len() + 1) as u32;
        let item = TodoItem::new(id, description);
        self.list.push(item);
    }

    fn complete_item(&mut self, id: u32) {
        for i in 0..self.list.len() {
            if self.list[i].id == id {
                self.list[i].complete();
                return;

            }
        }
    }

    fn delete_item(&mut self, id: u32) {
        self.list.retain(|item| item.id != id);
    }

    fn view_list(&self) {
        for item in self.list.iter() {
            let status = if item.completed { "Completed" } else { "Pending" };
            println!("{} [{}] - {}", item.id, status, item.description);
        }
    }
}



fn display_menu() {
    println!("Todo List Menu:");
    println!("1. Add a new todo item");
    println!("2. Mark a todo item as complete");
    println!("3. Delete a todo item");
    println!("4. View all todo items");
    println!("5. Exit");
}


fn main() {
    let mut todo_list = TodoList::new();

    loop {
        display_menu();
        println!("Enter your choice: ");
        io::stdout().flush().unwrap();
    
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
    
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice");
                return;
            }
        
        };

        match choice {
            1 => {
                println!("Enter the description of the todo item: ");
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                todo_list.add_item(description.trim().to_string());
            },
            2 => {
                println!("Enter the id of the todo item to mark as complete: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid id");
                        continue;
                    }
                };
                todo_list.complete_item(id);
            },
            3 => {
                println!("Enter the id of the todo item to delete: ");	
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid id");
                        continue;
                    }
                };
                todo_list.delete_item(id);
            },
            4 => {
                todo_list.view_list();
            },
            5 => {
                println!("Goodbye!");
                break;
            },
                
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 5");
            }
        }
    }
    

}

