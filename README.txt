Team Members:
Derek Nocera
Cameron Pouliot

Acknowledgements:
-Question answered by Professor and TAs on edstem.
-rumload, rumdis, and rumasm were helplful in understanding this assignment.

Correctly Implemented:
-All specifications have been correctly implemented.

Departure from design:
-The only significant departure from our design was to simply not use bitpack.

Architecture:
main.rs
A very simple module that handles arguments and calls the primary functions contained within
rumload and rumrun.  It creates a Vec<u32> of instructions it receives from rumload's load 
function.  It then passes these instructions to rumrun's run fuction to begin execution of 
the instructions.

rumload.rs
This module's only purpose is to parse through each word in a UM binary, then extract and load
them into a Vec of u32.  

rumrun.rs
This module is where the entirety of the running UM is handled.  This module contains all functions
to perform each of the possible operations in the UM's assembly language.  This module also handles
memory management.  The memory is stored as a Vec<Vec<u32>>, in which the first index contains the 
instructions.  It keeps track of the current instruction with a u32 program counter.  An id pool 
(Vec<u32>) and max_id (u32) are also used for memory management.  Registers are implemented as a 
fixed size array of u32s. 

This module gets the instruction indexed by the program counter and extracts the opcode.  It uses
this opcode to navigate a large match statement which then calls the appropriate function, passing
in the appropriate arguments to each function.  The program counter is then incremented, except 
for in the cases of load program and halt. 

The "secrets" that rumrun knows is all the function calls needed in the match statement. Rumload and
main never need to interact directly with the actual function calls, so only rumrun needs to know how
they actually work.

Performance:
Using midmark.um, which is 30,110 instructions, our implementation ran midmark at an average of 248.6ms.
So, to calculate 50 million instructions, it would take our implementation roughly 412.8s.

Time Spent:
Analyzing Problem: 2 hours.
Preparing Design: 4 hours.
Implementing Design: 6 hours.