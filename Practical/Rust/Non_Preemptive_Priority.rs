//  Non-Preemptive Priority-Based Scheduling Algorithm

use std::io;
fn first_come_first_serve(name_of_process:Vec<String>, burst_time:Vec<i32>, priority_list:Vec<i32>,number_of_processes:usize){
    
    let mut waiting_time = 0;
    let mut total_waiting_time = 0;
    let mut total_turnaround_time = 0;    

    println!("Name Of Process\tBurst Time\tPriority\tWaiting Time\tTurnAround Time");

    for i in 0..number_of_processes{
        let turnaround_time = waiting_time + burst_time[i];
        total_turnaround_time+=turnaround_time;
        
       println!("{}\t\t{}\t\t{}\t\t{}\t\t{}",name_of_process[i],burst_time[i],priority_list[i],waiting_time,turnaround_time);
       
       total_waiting_time+=waiting_time;
       waiting_time += burst_time[i];
    }
    println!("\nAverage Waiting Time is:: {}\nAverage TurnAround Time is::{}",total_waiting_time as f64/number_of_processes as f64,total_turnaround_time as f64/number_of_processes as f64);
}
fn sort(mut name_of_process:Vec<String>, mut burst_time:Vec<i32>, mut priority_list:Vec<i32>,number_of_processes:usize){
    for i in 0..number_of_processes{
        for j in 0..(number_of_processes-1){
            if priority_list[i] < priority_list[j] {
                priority_list.swap(i, j);
                burst_time.swap(i, j);
                name_of_process.swap(i, j);
            }
        }
    }
    first_come_first_serve(name_of_process,burst_time,priority_list,number_of_processes);
}
fn main(){
    let mut processes_count = String::new();
    println!("Enter the number of processes: ");
    io::stdin().read_line(&mut processes_count).expect("Failed to read line");
    let number_of_processes: usize = processes_count.trim().parse().expect("Please enter a number");

    let mut name_of_process =Vec::new();
    let mut burst_time = Vec::new();
    let mut priority_list = Vec::new();

    for i in 0..number_of_processes {
        let mut process_name = String::new();
        let mut burst = String::new();
        let mut priority = String::new();

        println!("Enter name of process {}: ", i + 1);
        io::stdin().read_line(&mut process_name).expect("Failed to read line");
        name_of_process.push(process_name.trim().to_string());

        println!("Enter burst time for process {}: ", name_of_process[i]);
        io::stdin().read_line(&mut burst).expect("Failed to read line");
        let burst_value: i32 = burst.trim().parse().expect("Please enter a number");
        burst_time.push(burst_value);

        println!("Enter priority of process {}: ", name_of_process[i]);
        io::stdin().read_line(&mut priority).expect("Failed to read line");
        let priority_value: i32 = priority.trim().parse().expect("Please enter a number");
        priority_list.push(priority_value);
    }

    // let number_of_processes:usize =5;
    // let name_of_process = vec!["P1".to_string(),"P2".to_string(),"P3".to_string(),"P4".to_string(),"P5".to_string()];
    // let burst_time = vec![6,3,2,8,9];
    // let priority_list = vec![3,2,1,4,5];


    sort(name_of_process,burst_time,priority_list,number_of_processes);
}