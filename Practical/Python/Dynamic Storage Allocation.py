# 13. Write a program to implement first-fit, best-fit and worst-fit allocation strategies.

def flag(No_of_Frames,memory):
    for Frame in range(No_of_Frames):
        if memory[Frame][0] == "free":
            return Frame
        

def Display(memory):
    for frames in memory:
        print(frames)
    print()


def First_Fit(No_of_Frames,memory,process):
    Flag = False
    for frames in range(No_of_Frames):
        if memory[frames][0] == "free" and memory[frames][1] >= process[1]:
            memory[frames][0] = process[0]
            Display(memory)
            Flag = True
            break
    if not Flag:
        print("\nYou Have Not Enough Space To Run New Process")


def Best_Fit(No_of_Frames,memory,process):
    Flag = flag(No_of_Frames,memory)
    for frames in range(1,No_of_Frames):
        if memory[frames][0] == "free" and memory[frames][1]>= process[1] and memory[frames][1] < memory[Flag][1]:
            Flag = frames
    
    if memory[Flag][1]>= process[1]:
        memory[Flag][0] = process[0]
        Display(memory)
    else:
        print("\nYou Have Not Enough Space To Run New Process")
        

def Worst_Fit(No_of_Frames,memory,process):
    Flag = flag(No_of_Frames,memory)
    for frames in range(1,No_of_Frames):
        if memory[frames][0] == "free" and memory[frames][1]>= process[1] and memory[frames][1] > memory[Flag][1]:
            Flag = frames

    if memory[Flag][1]>= process[1]:
        memory[Flag][0] = process[0]
        Display(memory)
    else:
        print("\nYou Have Not Enough Space To Run New Process")


def main():
    No_of_Frames = int(input("Enter Your Number Of Frames :: "))
    memory = []
    Ready_process = []
    for i in range(No_of_Frames):
        print(f"\n____Enter Detail of {i+1} Frame of Memory____")
        Running_process = input("Enter Process Name(If no Process is Running Enter 'free') :: ").lower()
        Size_of_Frame = int(input("Enter Size Of Frame(in Kb) :: "))
        memory.append([Running_process,Size_of_Frame])
    print("\n____Enter Detail of New Process____")
    Ready_process.append(input("Enter New Process Name:: ").lower())
    Ready_process.append(int(input("Enter Size Of New Process:: ")))
    
    Choice = int(input("Main Menu\n1. First Fit\n2. Best Fit\n3. Worst Fit\nEnter your Choice:: "))
    if Choice == 1:
        First_Fit(No_of_Frames,memory,Ready_process)
    elif Choice == 2:
        Best_Fit(No_of_Frames,memory,Ready_process)
    elif Choice == 3:
        Worst_Fit(No_of_Frames,memory,Ready_process)
    else:
        print("Wrong Input!")


if __name__ == "__main__":
    main()