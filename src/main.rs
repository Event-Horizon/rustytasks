use std::{collections::HashMap, fmt, fs::{create_dir_all, File, OpenOptions}, io::{self, Error, ErrorKind, Read, Write}, path::Path, str::FromStr};
#[allow(unused_imports)]
use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use itertools::Itertools;
use colored::Colorize;
use regex::Regex;

// pub mod rusty_tasks;
// pub mod rusty_commands;
// pub mod rusty_files;

/// TODO: 
/// Timezone fix

/// Represents a task with a completion status and associated data.
#[derive(Default, Debug,Clone)]
struct Task{
    completed: bool,
    data: String,
    due_date: Option<DateTime<Utc>>,
    completed_date: Option<DateTime<Utc>>
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
        let due_date = match self.due_date {
            Some(value) => value.with_timezone(&Local).to_string().yellow(),
            None=>{"N/A".to_string().truecolor(125,125,125)}
        };
        let completed_date=match self.completed_date{
            Some(value) => value.with_timezone(&Local).to_string().green(),
            None=>{"N/A".to_string().green()}
        };
        write!(f,"{} {} [Due: {}] [Completed: {}] {}",struct_string,string_completed,due_date,completed_date,formatted_data)
    }
}

/// Implements a constructor for Tasks
impl Task{
    fn new(c:bool,d:String)->Task{
        Task{
            completed:c,
            data:d,
            ..Default::default()
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
#[derive(Default,Debug,Clone)]
enum TASKCOM{
    #[default]
    Help,
    List,
    Add,
    Remove,
    Complete,
    Exit,
    Unknown
}

impl FromStr for TASKCOM{
    type Err = ();
    fn from_str(input: &str) -> Result<TASKCOM, Self::Err> {
        match input {
            "HELP"  => Ok(TASKCOM::Help),
            "LIST"  => Ok(TASKCOM::List),
            "ADD"  => Ok(TASKCOM::Add),
            "REMOVE" => Ok(TASKCOM::Remove),
            "COMPLETE" => Ok(TASKCOM::Complete),
            "EXIT" => Ok(TASKCOM::Exit),
            "UNKNOWN" => Ok(TASKCOM::Unknown),
            _      => Err(()),
        }
    }
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
    /// When you want ALL values TASKCOM can make
    #[allow(dead_code)]
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

    /// When you want to print out commands for the user
    pub fn into_iter_client() -> core::array::IntoIter<TASKCOM, 6> {
        [
            TASKCOM::Help,
            TASKCOM::List,
            TASKCOM::Add,
            TASKCOM::Remove,
            TASKCOM::Complete,
            TASKCOM::Exit,
        ]
        .into_iter()

    }
}

fn get_localtime()->DateTime<Local> {
    let utc_time: DateTime<Utc> = Utc::now();
    let local_time: DateTime<Local> = utc_time.with_timezone(&Local);
    // println!("UTC time: {}", utc_time);
    // println!("Local time: {}", local_time);
    return local_time
}

fn list_task_commands()->colored::ColoredString{
    let mut result:String=String::new();
    for command in TASKCOM::into_iter_client(){
            result+=format!("{command} ").as_str();        
    }    
    result.trim_end()
        .split(" ")
        .join(", ")
        .green()
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
    let debug = false;
    if debug{
        if let Some(com) = command.clone() {
            println!("HELP command was:{}",com.len());
        } 
    }
    let eol="\r\n";
    let indent=4;
    let spacing = " ".repeat(indent);
    let commandlist=list_task_commands();
    if debug {println!("{commandlist}");}

    let noneempty_case = &format!(
r#"
{spacing}Please use these commands to interact:{eol}
{spacing}{commandlist}{eol}    
{spacing}For further help type 'help command' like 'help add' no quotes.
"#);

    let response_hash = HashMap::from([
        ("list", r#"
        The LIST command will LIST out your current tasks.
        "#),
        ("add", r#"
        The ADD command will ADD a task when used like so:
    
        add This is a test!
    
        or to add with a due date:
    
        add Testing,2024-03-30 12:00:00 -05:00
        "#),
        ("remove", r#"
        The REMOVE command will REMOVE a task when used like so:
    
        remove 1
    
        This removes task 1 from your tasklist.
        "#),
        ("complete", r#"
        The COMPLETE command will COMPLETE a task when used like so:
    
        complete 1
    
        This completes task 1 from your tasklist.
        "#),
        ("exit", r#"
        The EXIT command EXITS the CLI Rusty Tasks process.
        "#),
        ("empty_string", noneempty_case.as_str()),
    ]);
    
    let help_info = match command {
        Some(value) if value == "list" =>{response_hash["list"]}
        Some(value) if value == "add"=>{response_hash["add"]}
        Some(value) if value == "remove"=>{response_hash["remove"]}
        Some(value) if value == "complete"=>{response_hash["complete"]},
        Some(value) if value =="exit"=>{response_hash["exit"]},
        Some(value) if value == "" =>{response_hash["empty_string"]}
        None=>{response_hash["empty_string"]}
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
fn command_add(global_tasks:&mut TaskList,data:String,date:String,global_datafilepath:String)->Result<(),String>{
    let default_date_format="%Y-%m-%d %H:%M:%S %z";
    let mut temp_task = Task::new(false, data);
    let default_time = NaiveTime::default(); // equivelant to NaiveTime::from_hms_opt(0, 0, 0).unwrap()

    if date != "" {
        let parsed_date=NaiveDate::parse_from_str(date.as_str(), default_date_format)
                                        .ok()
                                        .unwrap_or_default()
                                        .and_time(default_time);
        let dt_utc=parsed_date.and_utc();
        //let dt_localtimezone : DateTime<Local> = dt_utc.with_timezone(&Local);
        temp_task.due_date=Some(dt_utc);
    }else{
        temp_task.due_date=None;
    }
    
    match global_tasks.add_task(temp_task){
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
    if let Ok(_) = global_tasks.delete_task(index) {
        let _ = save_tltofile(global_datafilepath, global_tasks.clone());
        return Ok(())
    }else{
        return Err("Invalid REMOVE command please try again.".to_string())
    }
}

/// Completes a Task in TaskList by Index
fn command_complete(global_tasks:&mut TaskList,mut index:usize,global_datafilepath:String)->Result<(),String>{
    index=index.overflowing_sub(1).0;//prevent panic, handle elegantly later
    if let Ok(_) = global_tasks.toggle_completed_task(index){
        // this must run before the save_tltofile
        match global_tasks.tasks[index].completed{
            true=>global_tasks.tasks[index].completed_date=Some(Utc::now()),
            false=>{}
        }
        match save_tltofile(global_datafilepath, global_tasks.clone()) {
            Ok(value)=>Some(value),
            Err(_)=>{
                eprintln!("Complete Command was unable to save to file.");
                None
            }
        };
        return Ok(())
    }else{
        return Err("Invalid COMPLETE command please try again.".to_string())
    }  
}

/// Ends the process and exits to terminal
fn command_exit(){
    std::process::exit(0);
}

/// On first run shows welcome message
fn show_welcome_msg(){
    let help = command_help(None)
                       .unwrap_or_else(|_|{
                            eprintln!("Expected error: command_help has failed with 'None' as the parameter, this indicates a DEV bug.");
                            "".to_string()
                        });
    let indent=4;
    let spacing = " ".repeat(indent);
    println!(r#"
{spacing}Welcome to RUSTY TASKS!
{spacing}=======================
{spacing}Version 0.0.1
{spacing}=======================

{}"#,help);

}

/// Parses user input into command and arguments
fn parse_command_input(input: &str) -> (String, Vec<String>){
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
            if arguments.len()>1{
                command_add(global_tasks,arguments[0].to_string(), arguments[1].to_string(),global_datafilepath)?;
            }else{
                command_add(global_tasks,arguments[0].to_string(), "".to_string(),global_datafilepath)?;
            }
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
        let (command,arguments) = parse_command_input(&input);
        
        let command_enum=match TASKCOM::from_str(command.to_uppercase().as_str()).ok(){
            Some(t)=>t,
            None=>{
                let help = match command_help(None){
                    Ok(t)=>t,
                    Err(e)=>format!("There was an error with the help command: {}",e)
                };
                eprintln!("Invalid command string, defaulting to HELP.");
                eprintln!("{}",help);
                //skip the rest of the loop there is no valid command to handle.
                continue;
            }
        };

        let mut _last_state=command_enum.clone();

        match handle_command(command_enum,arguments,global_tasks,global_datafilepath.clone()){
            Ok(_)=>{},
            Err(error)=>{     
                //println!("Command was: {:?}",command);//debug       
                eprintln!("Error: {} \r\nLast State: {}",error,_last_state.to_string()) // we bubble these up to here from inside the commands
            }
        }
    }
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
        let tdue_date=match task.due_date{
            Some(value)=>value.with_timezone(&Local).to_string(),
            None=>"".to_string()
        };
        let tcompleted_date=match task.completed_date{
            Some(value)=>value.with_timezone(&Local).to_string(),
            None=>"".to_string()
        };
        result += format!(" - ").as_str();
        result += match tcompleted{
            true => "[√]",
            false => "[ ]"
        };
        result += format!(" [Due: {tdue_date}]").as_str();
        result += format!(" [Completed: {tcompleted_date}]").as_str();
        result += format!(" {tdata}").as_str();
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
            let re_full = Regex::new(r" - (\[[ √]\]) \[Due: (.*?)\] \[Completed: (.*?)\] (.*)").unwrap();
            let _re_simple = Regex::new(r" - (\[[ √]\]) (.*)");
            
            let temp_task = match re_full.captures(line){
                Some(captures)=>captures,
                None=>continue //skip rest of loop
            };
            
            let tcompleted_string:String=temp_task[1].to_string();
            let tdue_date:String=temp_task[2].to_string();
            let tcompleted_date:String=temp_task[3].to_string();
            let tdata:String=temp_task[4].to_string();
            
            // convert brackets into completed/uncompleted
            let tcompleted:bool = tcompleted_string.contains("[√]");

            // build task
            let mut new_task=Task::new(tcompleted,tdata);

            // date management
            // always convert from LOCAL string, to UTC struct
            let default_date_format="%Y-%m-%d %H:%M:%S %z";
            
            new_task.due_date=match DateTime::parse_from_str(tdue_date.as_str(), default_date_format){
                Ok(value)=>Some(value.to_utc()),
                Err(_)=>None
            };
            new_task.completed_date=match DateTime::parse_from_str(tcompleted_date.as_str(), default_date_format){
                Ok(value)=>Some(value.to_utc()),
                Err(_)=>None
            };

            // correct disparity between CHECK completed and COMPLETED date
            if new_task.completed_date == None {
                if tcompleted == true {
                    new_task.completed_date = Some(Utc::now())
                }
            }

            // task building complete
            tl.tasks.push(new_task);
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
        if let Err(directory_missing_error) = create_dir_all(parent) {
            eprintln!("Error creating directories: {}", directory_missing_error);
            return Err(directory_missing_error);
        }
    };

    // Check if the file exists before handling
    let file_exists = std::path::Path::new(&filepath).exists();

    if file_exists{     
        if let Err(file_exist_error) = handle_existing_file(&filepath,&string_tasklist){
                eprintln!("Invalid handling of existing file:{file_exist_error}")
        }
    }else{
        //println!("not exists");// ? debug
        if let Err(new_file_error) = handle_new_file(&filepath, &string_tasklist){
                eprintln!("Invalid handling of new file:{new_file_error}")
        }
    }
    Ok("File saved successfully.".to_string())
}

/// Non-existing file save
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

/// Existing file save
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

/// !Creates state object and initiates terminal input loop.
fn main() {
    let global_datafilepath:String="data/tasklist.md".to_string();
    let global_tasklist=&mut load_tlfromfile(global_datafilepath.clone());
    
    get_localtime();

    run_tasklist(true,global_tasklist,global_datafilepath.clone());
}

/// !Start of the testing module for this app
#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;//import for tests

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

    #[test]
    fn test_filesaveload(){        
        let global_datafilepath:String="testdata/tasklist.md".to_string();
        let global_tasklist=&mut load_tlfromfile(global_datafilepath.clone());
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
}