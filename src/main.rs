use std::{io::{self, Write}, str::FromStr};
#[allow(unused_imports)]
use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};


pub mod rusty_tasks;
pub mod rusty_commands;
pub mod rusty_files;

use rusty_tasks::*;
use rusty_commands::*;
use rusty_files::load_tlfromfile;

/// TODO: 
/// Timezone fix
/// Add SaveAs and Load commands
/// SaveAs should allow saving under different filenames
/// Load command should set the default file to open to the path/file selected

fn get_localtime()->DateTime<Local> {
    let utc_time: DateTime<Utc> = Utc::now();
    let local_time: DateTime<Local> = utc_time.with_timezone(&Local);
    // println!("UTC time: {}", utc_time);
    // println!("Local time: {}", local_time);
    return local_time
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

/// Starts the terminal input loop, receives, parses, and initiates commands.
fn run_tasklist(first_run:bool,global_tasks:&mut TaskList,global_datafilepath:String){    
    if first_run {
        show_welcome_msg();
        command_list(global_tasks);
    }

    loop{
        let input = read_input_line().trim().to_string();
        let (command,arguments) = parse_input_commands(&input);
        
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

    use crate::rusty_files::*;

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