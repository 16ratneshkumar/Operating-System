//First Come First Serve Scheduling Algorithm (FCFS)


#include <iostream>
void First_Come_First_Serve(std::string name_of_process[], int burst_time[],int number_of_processes){
    std::cout<<"Name Of Process\tBurst Time\tWaiting Time\tTurnAround Time"<<std::endl;
    
    double waiting_time = 0;
    double turnaround_time = 0;
    double total_waiting_time = 0;
    double total_turnaround_time = 0;    
    for (int i=0;i<number_of_processes;i++){
        turnaround_time += burst_time[i];
        total_turnaround_time+=turnaround_time;
        
       std::cout<<name_of_process[i]<<"\t\t"<<burst_time[i]<<"\t\t"<<waiting_time<<"\t\t"<<turnaround_time<<std::endl;
       total_waiting_time+=waiting_time;
       waiting_time += burst_time[i];
    }
    std::cout<<"\nAverage Waiting Time is:: "<<total_waiting_time/number_of_processes<<"\nAverage TurnAround Time is::"<<total_turnaround_time/number_of_processes<<std::endl;
}

int main(){
    int number_of_processes;
    std::cout<<"Enter the number of processes: ";
    std::cin>>number_of_processes;
    std::string name_of_process[number_of_processes];
    int burst_time[number_of_processes];
    for (int i=0;i<number_of_processes;i++){
        std::cout<<"Enter Name of Process:: ";
        std::cin>>name_of_process[i];
        std::cout<<"Enter Burst Time of Process "<<name_of_process[i]<<":: ";
        std::cin>>burst_time[i];
    }
    First_Come_First_Serve(name_of_process,burst_time,number_of_processes);
}