    use std::{fmt, str::FromStr};
    use chrono::{DateTime, Local, Utc};
    use colored::Colorize;
    use itertools::Itertools;
    use regex::Regex;

    /// Represents a task with a completion status and associated data.
    #[derive(Default, Debug,Clone)]
    pub struct Task{
        pub completed: bool,
        pub data: String,
        pub due_date: Option<DateTime<Utc>>,
        pub completed_date: Option<DateTime<Utc>>
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
        pub fn new(c:bool,d:String)->Task{
            Task{
                completed:c,
                data:d,
                ..Default::default()
            }
        }
    }

    /// Represents a list of Tasks
    #[derive(Debug,Clone)]
    pub struct TaskList {
        pub tasks: Vec<Task>,
    }

    /// Implements Task management methods for TaskList
    impl TaskList{
        pub fn add_task(&mut self,mytask:Task)->Result<usize,String>{         
            let veclen=self.tasks.len();      
            self.tasks.push(mytask);
            if veclen >= self.tasks.len() {
                return Err("Push failed.".to_string())
            }
            Ok(self.tasks.len() - 1)
        }

        pub fn delete_task(&mut self,index:usize)->Result<(),String>{
            if index < self.tasks.len() {
                self.tasks.remove(index);
                return Ok(())
            }
            Err("Invalid index.".to_string())
        }

        pub fn toggle_completed_task(&mut self,index:usize)->Result<(),String>{
            if index < self.tasks.len() {
        
                self.tasks[index].completed = !self.tasks[index].completed;
            
                return Ok(())
            }
            Err("Invalid index.".to_string())
        }

        #[allow(dead_code)]
        pub fn print(&self){
            println!("    Tasks: \r\n {:?}",self.tasks.iter().enumerate().format("\r\n "))
        }

        #[allow(dead_code)]
        pub fn print_pretty(&self){
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
    pub enum TASKCOM{
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

    /// Lists all commands available in a TASKCOM
    pub fn list_task_commands()->colored::ColoredString{
        let mut result:String=String::new();
        for command in TASKCOM::into_iter_client(){
                result+=format!("{command} ").as_str();        
        }    
        result.trim_end()
            .split(" ")
            .join(", ")
            .green()
    }

    /// Convert tasklist to string
pub fn convert_tltostring(tl:TaskList)->String{
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
pub fn convert_stringtotl(data:String)->TaskList{
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