use std::collections::HashMap;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use chrono::{NaiveDate, NaiveTime, Utc};

use crate::rusty_tasks::*;
use crate::rusty_files::*;

/// Returns help information for commands
pub fn command_help(command:Option<String>)->Result<String,String>{
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
pub fn command_list(global_tasks:&mut TaskList){
    global_tasks.print_pretty();
}

/// Adds new Task to TaskList
pub fn command_add(global_tasks:&mut TaskList,data:String,date:String,global_datafilepath:String)->Result<(),String>{
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
pub fn command_remove(global_tasks:&mut TaskList,mut index:usize,global_datafilepath:String)->Result<(),String>{
    index=index.overflowing_sub(1).0;//prevent panic, handle elegantly later
    if let Ok(_) = global_tasks.delete_task(index) {
        let _ = save_tltofile(global_datafilepath, global_tasks.clone());
        return Ok(())
    }else{
        return Err("Invalid REMOVE command please try again.".to_string())
    }
}

/// Completes a Task in TaskList by Index
pub fn command_complete(global_tasks:&mut TaskList,mut index:usize,global_datafilepath:String)->Result<(),String>{
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
pub fn command_exit(){
    std::process::exit(0);
}

/// Parses user input into command and arguments
pub fn parse_input_commands(input: &str) -> (String, Vec<String>){
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
pub fn handle_command(command:TASKCOM,arguments:Vec<String>,global_tasks:&mut TaskList,global_datafilepath:String)->Result<(),String>{
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

/// allows user to load a tasklist file
pub fn command_loadfile(global_datafilepath:&mut String,filepath:String)->Result<(),Error>{
    // validate shape of filepath is a filepath
    let validate_filepath = Path::new(&filepath).parent();
    // validate the filepath exists
    match validate_filepath {
        Some(_)=>{},
        None=>{return Err(Error::new(ErrorKind::Other, "Invalid filepath provided for command_saveas."))}
    }
    set_defaultfilepath(global_datafilepath, filepath.clone())?;
    load_tlfromfile(global_datafilepath.clone());
    Ok(())
}

/// allows user to save a tasklist file
pub fn command_savefile_as(global_tasks:&mut TaskList,global_datafilepath:&mut String,filepath:String)->Result<(),Error>{
    // validate shape of filepath is a filepath
    let validate_filepath = Path::new(&filepath).parent();
    // validate the filepath exists
    match validate_filepath {
        Some(_)=>{},
        None=>{return Err(Error::new(ErrorKind::Other, "Invalid filepath provided for command_saveas."))}
    }
    // save to filepath
    let _ = save_tltofile(filepath.clone(), global_tasks.clone())?;

    // re-load from the filepath we saved to
    load_tlfromfile(filepath.clone());

    // set default filepath just like command_load
    set_defaultfilepath(global_datafilepath, filepath.clone())?;

    Ok(())
}