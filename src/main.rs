use itertools::Itertools;
use rand::Rng;

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

fn main() {
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
    //for loop for amount of mocks
    let vector=&mut vec![];
    let mut mock_task:Task;
    let mut mock_data:String;
    for n in 1..=num {
        mock_data = String::from("Mock Task ");
        mock_data.push_str(n.to_string().as_str());
        mock_task=Task::new(false,mock_data);
        vector.push(mock_task);
    }
    let result=vector.clone();
    result
}