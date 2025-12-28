// use std::fs::{OpenOptions};
// use std::io::{self,Write,Read};
// use std::env;
// use serde::{Serialize,Deserialize};

// #[derive(Serialize,Deserialize,Debug)]
// struct Task {
//   title : String,
//   done : bool,
// }

// fn load_tasks() -> Vec<Task>{
//   let mut file = OpenOptions::new()
//       .read(true)
//       .write(true)
//       .create(true)
//       .open("todo.json")
//       .expect("Failed to open todo.json");

//     let mut data = String::new();
//     file.read_to_string(&mut data).unwrap();

//     if data.is_empty(){
//       Vec::new()
//     } else{
//       serde_json::from_str(&data).unwrap()
//     }

// }
// fn save_tasks(tasks: &Vec<Task>){
//   let data = serde_json::to_string(tasks).unwrap();
//   let mut file = OpenOptions::new()
//       .write(true)
//       .truncate(true)
//       .open("todo.json")
//       .unwrap();
//     file.write_all(data.as_bytes()).unwrap();

// }

// fn add_task(title:String){
//   let mut tasks = load_tasks();
//   tasks.push(Task{title,done : false});
//   save_tasks(&tasks);
//   println!("Task added.");
// }

// fn list_tasks(){
//   let tasks = load_tasks();
//   for (i , task) in tasks.iter().enumerate(){
//     let status = if task.done {"[x]"} else {"[]"};
//     println!("{} {} {} ",i+1,status,task.title);
//   }
// }

// fn mark_done(index : usize){
//   let mut tasks = load_tasks();
//   if index ==0 || index > tasks.len(){
//     println!("Invalid task number.");
//     return;
//   }
//   tasks[index -1].done = true;
//   save_tasks(&tasks);
//   println!("Task marked as done ");
//   return;
// }


// fn main(){
//   let args : Vec<String> = env::args().collect();
//   if args.len() <2{
//     println!("Usage :");
//     println!("add <task>");
//     println!("list");
//     println!("done <task_number>");
//     return;
//   }

// match args[1].as_str(){
//   "add"=> {
//     let title = args[2..].join(" ");
//     add_task(title);
//   }
//   "list" => list_tasks(),
//   "done" => {
//     if args.len() < 3{
//       println!("Please provide a task number");
//     } else if let Ok(num) = args[2].parse::<usize>(){
//       mark_done(num);
//     } else {
//       println!("Invalid task number ");
//     }
//   }
//   _ => println!("Unknown command "),
// }
// }

// Add a Task
// cargo run -- add Learn Rust


// Output:

// Task added.

// List Tasks
// cargo run -- list


// Output example:

// 1 [ ] Learn Rust

// Mark Task as Done
// cargo run -- done 1


// Output:

// Task marked as done

// List Again
// cargo run -- list


// Output:

// 1 [x] Learn Rust

