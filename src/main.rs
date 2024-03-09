use itertools::Itertools;
use rand::Rng;
use std::fmt;

/// Represents a task with a completion status and associated data.
#[derive(Debug,Clone)]
struct Task{
    completed: bool,
    data: String
}

/// Implements a default Display formatter for Tasks
impl fmt::Display for Task{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Task{{ completed: {}, data: {} }}",self.completed,self.data)
    }
}

/// Implements a constructor for Tasks
impl Task{
    fn new(c:bool,d:String)->Task{
        Task{
            completed:c,
            data:d
        }
    }
}

/// Represents a list of Tasks
#[derive(Debug)]
struct TaskList {
    tasks: Vec<Task>,
}

/// Implements Task management methods for TaskList
impl TaskList{
    fn add_task(&mut self,mytask:Task)->Result<(),String>{         
        let veclen=self.tasks.len();      
        self.tasks.push(mytask);
        if veclen >= self.tasks.len() {
            return Err("Push failed.".to_string())
        }
        Ok(())
    }

    fn delete_task(&mut self,index:usize)->Result<(),String>{
        if index < self.tasks.len() {
            self.tasks.remove(index);
            return Ok(())
        }
        Err("Invalid index.".to_string())
    }

    fn toggle_completed_task(&mut self,index:usize)->Result<(),String>{
        if index < self.tasks.len() {
    
            self.tasks[index].completed = !self.tasks[index].completed;
        
            return Ok(())
        }
        Err("Invalid index.".to_string())
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

        println!("{spacing}Tasks: [\r\n{spacing}{}\r\n{spacing}]",result)
    }
}

/// Represents Task Commands user is able to input.
enum TASKCOM{
    Help,
    List,
    Add,
    Remove,
    Complete,
    Exit,
    Unknown
}

/// Reads input line from Standard Input and returns it.
fn read_input_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read line.");
    input
}

/// Returns help information for commands
fn command_help(command:Option<String>)->Result<String,String>{

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
        return Err("Invalid HELP command please try again.".to_string());
    }
    Ok(help_info.to_string())
}

/// Prints all Tasks in TaskList
fn command_list(global_tasks:&mut TaskList){
    global_tasks.print_pretty();
}

/// Adds new Task to TaskList
fn command_add(global_tasks:&mut TaskList,d:String)->Result<(),String>{
    global_tasks.add_task(Task::new(false, d))?;
    Ok(())
}

/// Removes a Task in TaskList by Index
fn command_remove(global_tasks:&mut TaskList,index:usize)->Result<(),String>{
    match global_tasks.delete_task(index){
        Ok(_)=>{Ok(())},
        Err(_)=>{Err("Invalid REMOVE command please try again.".to_string())}
    }
}

/// Completes a Task in TaskList by Index
fn command_complete(global_tasks:&mut TaskList,index:usize)->Result<(),String>{
    match global_tasks.toggle_completed_task(index) {
        Ok(_)=>{Ok(())},
        Err(_)=>{Err("Invalid COMPLETE command please try again.".to_string())}
    }
}

/// Ends the process and exits to terminal
fn command_exit(){
    std::process::exit(0);
}

/// On first run shows welcome message
fn show_welcome_msg(){
    let help = match command_help(None){
        Ok(text)=>{text},
        Err(error)=>{println!("There was an error with the HELP command: {}",error);return;}
    };
    println!(r#"
    Welcome to RUSTY TASKS!
    =======================

{}"#,help);
}

/// Parses user input into command and arguments
fn parse_input(input: &str) -> (String, Vec<String>){
    let mut parts = input.splitn(2, char::is_whitespace);

    // Parse command
    let command = parts.next().unwrap_or("").to_lowercase();

    // Parse arguments
    let arguments:Vec<String> = match parts.next() {
        Some(args)=>args.split(",").map(|arg| arg.trim().to_string()).collect(),
        None=>vec![String::from("")]// requires string inside or else there will be no "arguments[0]"
    };    

    (command, arguments)
}

/// Converts command struct into function calls to run command
fn handle_command(command:TASKCOM,arguments:Vec<String>,global_tasks:&mut TaskList)->Result<(),String>{
    match command{
        TASKCOM::Help=>{
            let help = match command_help(arguments.get(0).map(|s| s.trim().to_string())){
                Ok(text)=>{text},
                Err(error)=>{error}
            };
            println!("{}",help);
            Ok(())
        },
        TASKCOM::List=>Ok(command_list(global_tasks)),
        TASKCOM::Add=>{
            command_add(global_tasks,arguments[0].to_string())?;
            command_list(global_tasks);
            Ok(())
        },
        TASKCOM::Remove=>{
            match arguments[0].parse::<usize>() {
                Ok(index)=>{
                    command_remove(global_tasks,index)?;
                    command_list(global_tasks);
                    return Ok(())
                },
                Err(_e)=>{
                    return Err("Invalid REMOVE command please try again.".to_string())
                }
            };
        },
        TASKCOM::Complete=>{
            match arguments[0].parse::<usize>() {
                Ok(index)=>{
                    command_complete(global_tasks,index)?;
                    command_list(global_tasks);
                    return Ok(())
                },
                Err(_e)=>{
                    return Err("Invalid COMPLETE command please try again.".to_string())
                }
            };
        },
        TASKCOM::Exit=>Ok(command_exit()),
        TASKCOM::Unknown=> Err("Invalid command. Try 'help' for a list of commands.".to_string())
    }
}

/// Starts the terminal input loop, receives, parses, and initiates commands.
fn run_tasklist(first_run:bool,global_tasks:&mut TaskList){
    let _state=TASKCOM::Help;
    
    if first_run {
        show_welcome_msg();
    }

    loop{
        let input = read_input_line().trim().to_string();
        let (command,arguments) = parse_input(&input);

        let command_enum=match command.as_str(){
            "help"=>TASKCOM::Help,
            "list"=>TASKCOM::List,
            "add"=>TASKCOM::Add,
            "remove"=>TASKCOM::Remove,
            "complete"=>TASKCOM::Complete,
            "exit"=>{break;},
            &_ => TASKCOM::Unknown
        };
        match handle_command(command_enum,arguments,global_tasks){
            Ok(_)=>{},
            Err(error)=>{     
                //println!("Command was: {:?}",command);//debug       
                println!("{}",error) // we bubble these up to here from inside the commands
            }
        }
    }
}

/// Prepares mock data and runs some tests.
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

/// Generates a list of fake Tasks for testing.
fn create_mocklist(num:i32)->Vec<Task>{
    // Ensure num is positive
    assert!(num > 0, "num must be a positive integer");

    //functional for loop
    // old .collect::<Vec<_>>()
    (1..=num)
        .map(|i| Task::new(false, format!("Mock Task {}", i)))
        .collect() 
}

/// Creates state object and initiates terminal input loop.
fn main() {
    //run_mocktrial();
    let global_tasklist=&mut TaskList{
        tasks:Vec::new()
    };
    run_tasklist(true,global_tasklist);
}