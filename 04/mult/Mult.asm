// a = RAM[0];
// n = RAM[1];
// res = 0;
// i = 0;
// LOOP:
//      if i - n == 0 break; 
//      res = res + a;
//      i = i + 1;
// 
// RAM[2] = res;
@R0
D=M
@a
M=D
@R1
D=M
@n
M=D
@R2
M=0 // Reset result register to 0 before using it for calcultations

// Initialize iteration variable
@i
M=0
// Loop n number of times to add a to itself
(LOOP)
    // break out of the loop if we added n many times already
    @i
    D=M
    @n
    D=D-M
    @END
    D;JEQ
    
    // Add a to R2 again
    @a
    D=M
    @R2
    M=D+M
    // Increase value of i
    @i
    M=M+1
    @LOOP
    0;JEQ

// End Of Program, update R2 register
(END)
@END
0;JEQ
