use itertools::Itertools;
use itertools::join;
use rand::Rng;
use std::fmt;
use std::collections::VecDeque;

#[derive(Debug,Clone)]
struct Task{
    completed: bool,
    data: String
}

impl fmt::Display for Task{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Task{{ completed: {}, data: {} }}",self.completed,self.data)
    }
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
        println!("    Tasks: \r\n {:?}",self.tasks.iter().enumerate().format("\r\n "))
    }

    fn print_pretty(&self){
        let indent=4;
        let spacing = " ".repeat(indent);
        let result=self.tasks
        .iter()
        .enumerate()
        .map(|(i,v)| String::from(i.to_string()+": "+v.to_string().as_str()))
        .join((String::from("\r\n")+&spacing).as_str());

        println!("{spacing}Tasks: \r\n{spacing}{}",result)
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

fn command_help(command:Option<String>)->Result<String,&'static str>{

    // if let Some(com) = command.clone() {
    //     println!("HELP command was:{}",com.len());
    // } 
    
    let help_info = match command.as_deref() {
        Some("list")=>{r#"
    The LIST command will LIST out your current tasks.
    "#}
        Some("add")=>{r#"
    The ADD command will ADD a task when used like so:

    add "This is a test!"
    "#}
        Some("remove")=>{r#"
    The REMOVE command will REMOVE a task when used like so:

    remove 0

    This removes task 0 from your tasklist.
    "#}
        Some("complete")=>{r#"
    The COMPLETE command will COMPLETE a task when used like so:

    complete 0

    This completes task 0 from your tasklist.
    "#},
    Some("exit")=>{r#"
    The EXIT command EXITS the CLI Rusty Tasks process.
    "#},
    Some("")|None=>{
        r#"    Please use these commands to interact:

    HELP, LIST, ADD, REMOVE, COMPLETE, EXIT
    
    For further help type 'help command' like 'help add' no quotes.
"#
    }
    _=>"-1"
    };
    if help_info == "-1" {
        return Err("Invalid help command.");
    }
    Ok(help_info.to_string())
}
fn command_list(global_tasks:&mut TaskList){
    global_tasks.print_pretty();
}
fn command_add(global_tasks:&mut TaskList,d:String){
    let c=false;
    global_tasks.add_task(Task::new(c, d)).unwrap_or_default();
}
fn command_remove(global_tasks:&mut TaskList,index:usize){
    global_tasks.delete_task(index).unwrap_or_default();
}
fn command_complete(global_tasks:&mut TaskList,index:usize){
    global_tasks.toggle_completed_task(index).unwrap_or_default();
}
fn command_exit(){
    std::process::exit(0);
}

fn run_firstrun(){
    println!(r#"
    Welcome to RUSTY TASKS!
    =======================

{}"#,command_help(None).unwrap());
}

fn run_tasklist(first_run:bool,global_tasks:&mut TaskList){
    let _state=TASKCOM::Help;
    if first_run {
        run_firstrun();
    }
    let input = read_string().trim().to_string();
    let lower_input = input.clone().to_lowercase();

    let mut command_queue:VecDeque<_> = lower_input.split(" ").collect();
    let command= command_queue.pop_front();
    let queue_clone=join(command_queue.clone()," ");
    let arguments:Vec<_> = queue_clone.split(",").collect();

    let request=match command.unwrap(){
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
            println!("{}",command_help(Some(arguments[0]
                .trim()
                .to_string()))
                .unwrap_or(String::from("Invalid help command.")));
        },
        TASKCOM::List=>{
            command_list(global_tasks);
        },
        TASKCOM::Add=>{
            command_add(global_tasks,arguments[0].to_string());            
            command_list(global_tasks);
        },
        TASKCOM::Remove=>{
            let mut invalid_remove=false;
            let index = match arguments[0].parse::<usize>() {
                Ok(index)=>index,
                Err(_e)=>{invalid_remove=true;0}
            };

            if !invalid_remove{
                command_remove(global_tasks,index);
                command_list(global_tasks);
            }else{
                println!("Invalid REMOVE command please try again.");
            }
        },
        TASKCOM::Complete=>{
            let mut invalid_complete=false;
            let index=match arguments[0].parse::<usize>() {
                Ok(index)=>index,
                Err(_e)=>{invalid_complete=true;0}
            };
            if !invalid_complete{
                command_complete(global_tasks,index);
                command_list(global_tasks);
            }else{
                println!("Invalid COMPLETE command please try again.");
            }
        },
        TASKCOM::Exit=>{
            command_exit();
        }
    }
    //println!("Command was: {:?}",command);//debug
    run_tasklist(false,global_tasks);
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
    let global_tasklist=&mut TaskList{
        tasks:Vec::new()
    };
    run_tasklist(true,global_tasklist);
}