
Notes for 3336 Presentation

cargo build

//demonstrating the output of test1.txt//

cargo run -- test1.txt
 	This is to show the output explain where the array of hashed bytes comes from, and the contents of the hashed array are translated into the string of 64 characters

now we test the edge case of an error.

cargo run --
 	this exits , because the program catches that there isn't enough arguments for the program to proceed

//not actual file//

cargo run -- fake.txt
 	The function exits in an error at the ?, BUT this is not a crash this is exactly what we planned to do and what is expected in Rust

//testing content with a different character at the end//

cargo run -- test2.txt
 	Shows that one change to the contents produces a completely unique string

//empty file test//

cargo run -- test3.txt
 	Shows the string for an empty file, this string is constant with all empty files. This algorithm will always produce the same output given the same input

QUESTIONS?
