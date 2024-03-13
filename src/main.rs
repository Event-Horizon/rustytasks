#[allow(unused_imports)]
use rand::Rng;//import for tests
use itertools::Itertools;
use regex::Regex;
use std::{fmt, fs::{create_dir_all, File, OpenOptions}, io::{self, Error, ErrorKind, Read, Write}, path::Path};
use colored::Colorize;

/// Represents a task with a completion status and associated data.
#[derive(Debug,Clone)]
struct Task{
    completed: bool,
    data: String
}

/// Implements a default Display formatter for Tasks
impl fmt::Display for Task{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_completed = match self.completed {
            true=>"[√]".green(),
            false=>"[ ]".red()
        };
        let struct_string="Task ->".color("purple");
        let formatted_data=match self.completed{
            true=>self.data.strikethrough().truecolor(125,125,125),
            false=>self.data.color("white")
        };
        write!(f,"{} {} {} ",struct_string,formatted_data,string_completed)
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
#[derive(Debug,Clone)]
struct TaskList {
    tasks: Vec<Task>,
}

/// Implements Task management methods for TaskList
impl TaskList{
    fn add_task(&mut self,mytask:Task)->Result<usize,String>{         
        let veclen=self.tasks.len();      
        self.tasks.push(mytask);
        if veclen >= self.tasks.len() {
            return Err("Push failed.".to_string())
        }
        Ok(self.tasks.len() - 1)
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

    #[allow(dead_code)]
    fn print(&self){
        println!("    Tasks: \r\n {:?}",self.tasks.iter().enumerate().format("\r\n "))
    }

    #[allow(dead_code)]
    fn print_pretty(&self){
        let eol="\r\n";
        let indent=4;
        let spacing = " ".repeat(indent);
        let result=self.tasks
        .iter()
        .enumerate()
        .map(|(i,v)| {
            let n=i+1;
            format!("{n}: {v}")}
        )
        .join(format!("\r\n{spacing}").as_str());
        
        let struct_string="Tasks: ".color("purple");

        println!("{spacing}{struct_string}{eol}{spacing}{result}{eol}{spacing}");
    }
}

/// Represents Task Commands user is able to input.
#[derive(Debug,Clone)]
enum TASKCOM{
    Help,
    List,
    Add,
    Remove,
    Complete,
    Exit,
    Unknown
}

impl fmt::Display for TASKCOM{
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
        let com_string=match &self{
            TASKCOM::Help=>"HELP",
            TASKCOM::List=>"LIST",
            TASKCOM::Add=>"ADD",
            TASKCOM::Remove=>"REMOVE",
            TASKCOM::Complete=>"COMPLETE",
            TASKCOM::Exit=>"EXIT",
            TASKCOM::Unknown=>"UNKNOWN"
        };
        write!(f,"{com_string}")
    }
}

impl TASKCOM {
    pub fn into_iter() -> core::array::IntoIter<TASKCOM, 7> {
        [
            TASKCOM::Help,
            TASKCOM::List,
            TASKCOM::Add,
            TASKCOM::Remove,
            TASKCOM::Complete,
            TASKCOM::Exit,
            TASKCOM::Unknown
        ]
        .into_iter()
    }
}

fn list_task_commands()->colored::ColoredString{
    let mut result:String=String::new();
    for command in TASKCOM::into_iter(){
        if command.to_string() != "UNKNOWN" { // this is a value for devs only
            result+=format!("{command} ").as_str();        
        }
    }
    //cleanup and clarity
    result=result.trim_end().to_string();
    result=result.split(" ").join(", ");
    result.green()
}

/// Reads input line from Standard Input and returns it.
fn read_input_line() -> String {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
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
    let eol="\r\n";
    let indent=4;
    let spacing = " ".repeat(indent);
    let commandlist=list_task_commands();
    //println!("{commandlist}");
    let mut commandlist_formatted:String=String::new();
    let somenone;

    let somenone_case=||{        
        commandlist_formatted.push_str(&format!(
r#"
{spacing}Please use these commands to interact:{eol}
{spacing}{commandlist}{eol}    
{spacing}For further help type 'help command' like 'help add' no quotes.
"#
));
    commandlist_formatted
    };
    
    let help_info = match command {
        Some(value) if value == "list" =>{r#"
    The LIST command will LIST out your current tasks.
    "#}
        Some(value) if value == "add"=>{r#"
    The ADD command will ADD a task when used like so:

    add "This is a test!"
    "#}
        Some(value) if value == "remove"=>{r#"
    The REMOVE command will REMOVE a task when used like so:

    remove 1

    This removes task 1 from your tasklist.
    "#}
        Some(value) if value == "complete"=>{r#"
    The COMPLETE command will COMPLETE a task when used like so:

    complete 1

    This completes task 1 from your tasklist.
    "#},
    Some(value) if value =="exit"=>{r#"
    The EXIT command EXITS the CLI Rusty Tasks process.
    "#},
    Some(value) if value == "" =>{
        somenone=somenone_case();
        somenone.as_str()
    }
    None=>{
        somenone=somenone_case();
        somenone.as_str()
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
fn command_add(global_tasks:&mut TaskList,d:String,global_datafilepath:String)->Result<(),String>{
    match global_tasks.add_task(Task::new(false, d)){
        Ok(_)=>{
            let _ = save_tltofile(global_datafilepath, global_tasks.clone());
            return Ok(())
        },
        Err(_)=>{return Err("Invalid ADD command please try again.".to_string())}
    };
}

/// Removes a Task in TaskList by Index
fn command_remove(global_tasks:&mut TaskList,mut index:usize,global_datafilepath:String)->Result<(),String>{
    index=index.overflowing_sub(1).0;//prevent panic, handle elegantly later
    match global_tasks.delete_task(index){
        Ok(_)=>{
            let _ = save_tltofile(global_datafilepath, global_tasks.clone());
            return Ok(())
        },
        Err(_)=>{return Err("Invalid REMOVE command please try again.".to_string())}
    }
}

/// Completes a Task in TaskList by Index
fn command_complete(global_tasks:&mut TaskList,mut index:usize,global_datafilepath:String)->Result<(),String>{
    index=index.overflowing_sub(1).0;//prevent panic, handle elegantly later
    match global_tasks.toggle_completed_task(index) {
        Ok(_)=>{
            let _ = save_tltofile(global_datafilepath, global_tasks.clone());
            return Ok(())
        },
        Err(_)=>{return Err("Invalid COMPLETE command please try again.".to_string())}
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
fn handle_command(command:TASKCOM,arguments:Vec<String>,global_tasks:&mut TaskList,global_datafilepath:String)->Result<(),String>{
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
            command_add(global_tasks,arguments[0].to_string(),global_datafilepath)?;
            command_list(global_tasks);
            Ok(())
        },
        TASKCOM::Remove=>{
            let index=match arguments[0].parse::<usize>() {
                Ok(index)=>{index},
                Err(_e)=>{return Err("Invalid REMOVE command please try again.".to_string())}
            };
            command_remove(global_tasks,index,global_datafilepath)?;
            command_list(global_tasks);
            Ok(())
        },
        TASKCOM::Complete=>{
            let index=match arguments[0].parse::<usize>() {
                Ok(index)=>{index},
                Err(_e)=>{return Err("Invalid COMPLETE command please try again.".to_string())}
            };
            command_complete(global_tasks,index,global_datafilepath)?;
            command_list(global_tasks);
            Ok(())
        },
        TASKCOM::Exit=>Ok(command_exit()),
        TASKCOM::Unknown=> Err("Invalid command. Try 'help' for a list of commands.".to_string())
    }
}

/// Starts the terminal input loop, receives, parses, and initiates commands.
fn run_tasklist(first_run:bool,global_tasks:&mut TaskList,global_datafilepath:String){    
    if first_run {
        show_welcome_msg();
        command_list(global_tasks);
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
        let mut _last_state=command_enum.clone();
        match handle_command(command_enum,arguments,global_tasks,global_datafilepath.clone()){
            Ok(_)=>{},
            Err(error)=>{     
                //println!("Command was: {:?}",command);//debug       
                println!("{}",error) // we bubble these up to here from inside the commands
            }
        }
    }
}

/// Prepares mock data and runs some tests.
#[test]
fn test_runmocktrial(){
    let mock_tasks=create_mocklist(10);
    let mut task_list=TaskList{
        tasks:mock_tasks.to_vec()
    };

    let initial_length = task_list.tasks.len();
    
    let random_task=rand::thread_rng().gen_range(0..task_list.tasks.len());
    let _  = task_list.delete_task(random_task);
    assert_eq!(task_list.tasks.len(), initial_length - 1);
    // Verify tasklist random index is either gone or does not match deleted task
    if random_task<task_list.tasks.len(){
       assert!(
            !task_list.tasks.iter().any(|t| 
                task_list.tasks[random_task].data != t.data &&
                task_list.tasks[random_task].completed != t.completed
        ));
    }

    let random_task2 = rand::thread_rng().gen_range(0..task_list.tasks.len());
    let original_completed_state = task_list.tasks[random_task2].completed;

    let _ = task_list.toggle_completed_task(random_task2);
    let updated_completed_state = task_list.tasks[random_task2].completed;

    // Assert that the completed state is inverted
    assert_ne!(original_completed_state, updated_completed_state);

    let temp_data=String::from("Test Task 1");
    let temp_task=Task::new(false, temp_data);
    let added_task_index =task_list.add_task(temp_task).unwrap();
    // Assert that the task is present in the list at the returned index
    assert!(added_task_index < task_list.tasks.len(), "Invalid index returned.");
    assert_eq!(task_list.tasks[added_task_index].data, "Test Task 1");
}

/// Generates a list of fake Tasks for testing.
#[allow(dead_code)]
fn create_mocklist(num:i32)->Vec<Task>{
    // Ensure num is positive
    assert!(num > 0, "num must be a positive integer");

    //functional for loop
    // old .collect::<Vec<_>>()
    (1..=num)
        .map(|i| Task::new(false, format!("Mock Task {}", i)))
        .collect() 
}

#[test]
fn test_createmocklist(){
    let num = 5;
    let mock_tasks = create_mocklist(num);

    assert_eq!(num, mock_tasks.len() as i32);
}

/// Convert tasklist to string
fn convert_tltostring(tl:TaskList)->String{
    let eol = "\r\n";
    let mut result = String::new();
    result += eol;
    result += format!("# TaskList:{eol}").as_str();
    for task in tl.tasks{
        let tdata=task.data;
        let tcompleted=task.completed;
        result += format!(" - {tdata} ").as_str();
        result += match tcompleted{
            true => "[√]",
            false => "[ ]"
        };
        //result += tcompleted.to_string().as_str();
        result += eol;
    }

    result
}

/// Convert string to tasklist
fn convert_stringtotl(data:String)->TaskList{
    //println!("CONVERT STRING TO TL DATAIN:{}",data);
    let lines = data.split("\r\n");
    let mut tl:TaskList=TaskList{ tasks: Vec::new() };
    let mut tlfound=false;
    for line in lines{ 
        if line.contains("# TaskList:"){
            tlfound=true;
        }
        if tlfound{//even AFTER the line detected, this allows rest of code to run because its saved outside loop
            // convert - lines into Tasks   
            let re = Regex::new(r" - (.+) (\[[ √]\])").unwrap();
            
            let temp_task = match re.captures(line){
                Some(captures)=>captures,
                None=>continue //skip rest of loop
            };
            
            let tdata:String=temp_task[1].to_string();
            let tcompleted_string:String=temp_task[2].to_string();

            // convert brackets into completed/uncompleted
            let tcompleted:bool = tcompleted_string.contains("[√]");
            tl.tasks.push(Task::new(tcompleted,tdata));
        }
    }
    // Return TaskList, if one was not found we return an empty TaskList
    tl
}

/// Save tasklist struct to file
fn save_tltofile(filepath:String,tasklist:TaskList)->Result<String,Error>{
    let string_tasklist=convert_tltostring(tasklist);

    // println!("Requested path: {}",filepath);// ? debug

    // Create intermediate directories if they don't exist
    if let Some(parent) = std::path::Path::new(&filepath).parent() {
        if let Err(err) = create_dir_all(parent) {
            eprintln!("Error creating directories: {}", err);
            return Err(err);
        }
    };

    // Check if the file exists before canonicalizing
    let file_exists = std::path::Path::new(&filepath).exists();

    if file_exists{        
        match handle_existing_file(&filepath,&string_tasklist){
            Ok(_)=>{},
            Err(e)=>{
                eprintln!("Invalid handling of existing file:{e}")
            }
        }
    }else{
        //println!("not exists");// ? debug
        match handle_new_file(&filepath, &string_tasklist){
            Ok(_)=>{},
            Err(e)=>{
                eprintln!("Invalid handling of new file:{e}")
            }
        }
    }

    // println!("File saved successfully."); // ? debug
    Ok("File saved successfully".to_string())

}

fn handle_new_file(filepath: &str, data: &str)->Result<(),Error>{
    // Check if we have write permissions for the folder
    let parent_directory = Path::new(filepath).parent().ok_or_else(|| {
        eprintln!("Error getting parent directory for file: {}", filepath);
        Error::new(ErrorKind::Other, "Invalid parent directory")
    })?;

    let parent_notreadonly=!parent_directory.metadata()?.permissions().readonly();

    if parent_notreadonly { // Folder has write permissions, create file        
        let mut file = File::create(filepath)?;
        file.write_all(data.as_bytes())?;
    } else {
        eprintln!("Error: No write permissions for the new file's parent directory.");
        return Err(Error::new(ErrorKind::PermissionDenied, "No write permissions for parent directory"));
    }

    Ok(())
}

fn handle_existing_file(filepath: &str, data: &str)->Result<(),Error>{
    let data_directory = std::env::current_dir()?.join("data");
    let data_directory_canon = data_directory.canonicalize()?.to_string_lossy().to_string();
    let filepath_canon = Path::new(filepath).canonicalize()?.to_string_lossy().to_string();

    if !filepath_canon.contains(&data_directory_canon) {
        eprintln!(
            "Error: Path does not contain the 'data' directory: {} datadir: {}",
            filepath_canon, data_directory_canon
        );
        return Err(Error::new(ErrorKind::Other, "Invalid path"));
    }

    let file_exists = Path::new(filepath).exists();
    let not_readonly = !Path::new(filepath).metadata()?.permissions().readonly();

    match (file_exists, not_readonly) {
        (true, true) => {
            // File exists, have permissions, overwrite it
            let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(filepath)?;
            file.write_all(data.as_bytes())?;
        }
        (true, false) => {
            // File exists, no permissions, error
            eprintln!("Error: No write permissions for the existing file.");
            return Err(Error::new(ErrorKind::PermissionDenied, "No write permissions"));
        }
        (false, _) => {
            // File not exists
        }
    }

    Ok(())
}

// let mut file = File::create(&filepath).ok();
// match file{
//     Some(ref mut _f)=>{
//         file.unwrap().write_all(string_tasklist.as_bytes()).unwrap_or_default();
//     },
//     None=>{

//         eprintln!("Error saving: File not found at filepath '{}'", filepath);
//     }
// }
// Ok("TaskList saved to file.".to_string())



/// Load tasklist struct from file
fn load_tlfromfile(path:String)->TaskList{
    let mut data = String::new();
    let mut file = File::open(&path).ok();
    let mut file_opened=false;
    match file{
        Some(ref mut _f)=>{file_opened=true;}
        None=>{
            #[cfg(debug_assertions)]{//prevent this from running in release
                eprintln!("Error loading: File not found at path '{}'", &path);
            }
        }
    };
    if file_opened{
        file.unwrap().read_to_string(&mut data).unwrap_or_default();
    }
    convert_stringtotl(data)
}

#[allow(dead_code)]
fn test_filesaveload(global_tasklist:&mut TaskList,global_datafilepath:String){
    // create mock tasks
    let _=global_tasklist.add_task(Task::new(false, "test".to_string()));
    let _=global_tasklist.add_task(Task::new(true, "test2".to_string()));
    let _=global_tasklist.add_task(Task::new(true, "test3".to_string()));
    // Testing conversions and file save/load
    let string_tasklist=convert_tltostring(global_tasklist.clone());
    let string_totasklist=convert_stringtotl(string_tasklist.clone());
    println!("STRING: {}\r\n",string_tasklist);
    println!("TASKLIST: {:?}\r\n",string_totasklist);
    let _ = save_tltofile(global_datafilepath.clone(), global_tasklist.clone());
    let new_tasklist=load_tlfromfile(global_datafilepath.clone());
    println!("TASKLIST after SAVE/LOAD: {:?}",new_tasklist);
}

/// Creates state object and initiates terminal input loop.
fn main() {
    //test_runmocktrial();
    let global_datafilepath:String="data/tasklist.md".to_string();
    let global_tasklist=&mut load_tlfromfile(global_datafilepath.clone());

    //test_filesaveload(global_tasklist, global_datafilepath.clone());

    run_tasklist(true,global_tasklist,global_datafilepath.clone());
}