use itertools::Itertools;
use rand::Rng;
use std::io::{self, BufRead};

#[derive(Debug,Clone)]
struct Task{
    completed: bool,
    data: String
}

impl Task{
    fn new(c:bool,d:String)->Task{
        let result=Task{
            completed:c,
            data:d
        };
        result
    }
}

#[derive(Debug)]
struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList{
    fn add_task(&mut self,mytask:Task)->Result<&'static str,&'static str>{         
        let veclen=self.tasks.len();      
        self.tasks.push(mytask);
        if veclen >= self.tasks.len() {
            return Err("Push failed.")
        }
        Ok("Task added.")
    }

    fn delete_task(&mut self,index:usize)->Result<&'static str,&'static str>{
        if index < self.tasks.len() {
            self.tasks.remove(index);
            return Ok("Task Deleted.")
        }
        Err("Invalid index.")
    }

    fn toggle_completed_task(&mut self,index:usize)->Result<&'static str,&'static str>{
        if index < self.tasks.len() {
    
            self.tasks[index].completed = !self.tasks[index].completed;
        
            return Ok("Task Completed.")
        }
        Err("Invalid index.")
    }

    fn print(&self){
        println!("Tasks: \r\n {:?}",self.tasks.iter().format("\r\n "))
    }
}

enum TASKCOM{
    Help,
    List,
    Add,
    Remove,
    Complete,
    Exit
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read line.");
    input
}

fn command_help(){}
fn command_list(){}
fn command_add(){}
fn command_remove(){}
fn command_complete(){}
fn command_exit(){
    std::process::exit(0);
}

fn run_tasklist(first_run:bool){
    let state=TASKCOM::Help;
    if first_run {
        println!(r#"
    Welcome to RUSTY TASKS!
    =======================

    Please use these commands to interact:

    Help, List, Add, Remove, Complete, Exit
    "#);
    }
    let input = read_string().trim().to_string();
    let lowerInput = input.clone().to_lowercase();
    let formattedInput = lowerInput.as_str();
    let request=match formattedInput{
        "help"=>TASKCOM::Help,
        "list"=>TASKCOM::List,
        "add"=>TASKCOM::Add,
        "remove"=>TASKCOM::Remove,
        "complete"=>TASKCOM::Complete,
        "exit"=>TASKCOM::Exit,
        &_ => TASKCOM::Help
    };
    match request{
        TASKCOM::Help=>{
            command_help();
        },
        TASKCOM::List=>{
            command_list();
        },
        TASKCOM::Add=>{
            command_add();
        },
        TASKCOM::Remove=>{
            command_remove();
        },
        TASKCOM::Complete=>{
            command_complete();
        },
        TASKCOM::Exit=>{
            command_exit();
        }
    }
    println!("Command was: {:?}",input);
    run_tasklist(false);
}

fn run_mocktrial(){
    let mock_tasks=create_mocklist(10);
    let mut task_list=TaskList{
        tasks:mock_tasks.to_vec()
    };

    task_list.print();
    let random_task=rand::thread_rng().gen_range(0..task_list.tasks.len());
    let dtask = task_list.delete_task(random_task);
    println!("Result: {:?}",dtask);
    let random_task2=rand::thread_rng().gen_range(0..task_list.tasks.len());
    let ctask = task_list.toggle_completed_task(random_task2);
    println!("Result: {:?}",ctask);
    task_list.print();
    let temp_task=Task::new(false, String::from("Test Task 1"));
    let atask=task_list.add_task(temp_task);
    println!("Result: {:?}",atask);
    task_list.print();
}

fn create_mocklist(num:i32)->Vec<Task>{
    // brute force for loop for amount of mocks
    // let vector=&mut vec![];
    // let mut mock_task:Task;
    // let mut mock_data:String;
    // for n in 1..=num {
    //     mock_data = String::from("Mock Task ");
    //     mock_data.push_str(n.to_string().as_str());
    //     mock_task=Task::new(false,mock_data);
    //     vector.push(mock_task);
    // }
    // let result=vector.clone();
    // result

    //functional for loop
    (1..=num)
        .map(|i| {
            let data = format!("Mock Task {}", i);
            Task::new(false, data)
        })
        .collect::<Vec<_>>() 
}

fn main() {
    run_mocktrial();
    run_tasklist(true);
}