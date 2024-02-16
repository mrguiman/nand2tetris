// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen
// by writing 'black' in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen by writing
// 'white' in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// We set MAXROW to the last adressable screen row (by adding 8192-1 number of rows to it)
@8191
D=A
@SCREEN
D=D+A
@MAXROW
M=D
@SCREEN
D=A
@address
M=D

(LOOP)
    @screenvalue
    M=-1
    @KBD
    D=M
    @DRAW // We skip setting the new screen value to 0 if KBD value is not 0 itself, which means a key is pressed
    D;JNE
    @screenvalue
    M=0

    // Begin drawing logic
    (DRAW)
    @screenvalue
    D=M
    @address
    A=M
    M=D

    @MAXROW
    D=M
    @address
    D=D-M
    @RESET // Go to reset If we reached the latest addressable screen register
    D;JEQ

    // Increase counter and continue
    @address
    M=M+1 // Move address forward
    @LOOP
    0;JEQ

    // Reset the address pointer to the first screen register and loop back
    (RESET)
    @SCREEN
    D=A
    @address
    M=D
    @LOOP
    0;JEQ

