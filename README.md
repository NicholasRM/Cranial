# Cranial

Cranial is a superset of the Brainfuck programming language intended to add additional functionality such as UTF-8 support,
separate integer and character inputs, and double the size of the tape of bytes.

The goals of this project are to create an interpreter that is backwards-compatible with existing Brainfuck programs while also providing
extra features and a REPL for improved ease of use. At the same time, all additions should be able to integrate seamlessly into an existing
Brainfuck program.

Since Cranial is an extension of Brainfuck, the use of this program is very limited. As such, this project is largely a personal project.

***This project is still being developed, and a complete product is not available at this time. No set time can be given as to when this project will be completed***

## Original Brainfuck

Brainfuck was designed to be as simple as possible. As such, it only contains two data structures: the tape and the pointer.
The tape is, typically, an array of 30,000 bytes (unsigned characters in C) and the pointer is the current index being looked at
when the program is running. This pointer can move forward and backward, increment and decrement the value of a byte, accept a character
as input and store it in the tape, and write the character out to standard output. Simply put, Brainfuck is designed to mimic a Turing Machine
as much as possible.

In the original implementation of Brainfuck, only the following 8 commands are considered valid, with all other characters being ignored.
`>`, `<`, `+`, `-`, `.`, `,`, `[`, `]`

Their functions are described as follows:

* `>`: Move the pointer 1 byte to the right
* `<`: Move the pointer 1 byte to the left
* `+`: Increment the byte at the pointer's current position by 1
* `+`: Increment the byte at the pointer's current position by 1
* `.`: Output the byte at the pointer's current position as a character and write it to standard output
* `,`: Read a character from standard input and store its value at the pointer's current position
* `[`: Enter into a loop, or jump to the end of a loop if the value at the pointer's current position is 0
* `]`: Send pointer back to start of loop, or advance pointer right if the value at the pointer's current position is not 0

## Changes in Cranial

In Cranial, all original 8 Brainfuck commands are valid and behave as expected. As well, two more commands have been introduced:

* `:`: Output the byte at the point as a number between 0-255
* `;`: Take in a number from 0-255 from standard input and store it at the pointer's current positon

Additionally, Cranial will implement a read-eval-print loop (REPL). All 10 commands are valid, with the addition of three new commands
specifically made for the REPL:

* `!`: Terminate the program
* `*`: Output the current position of the pointer to standard output
* `?`: Output all non-zero bytes in the tape, along with their indices, to standard output

