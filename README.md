## **Xene Programming Language**

Xene is a programming language project written in Rust. The goal of this project is to build a simple interpreter and parser from scratch, focusing on basic language constructs such as variables, expressions, control flow (if, else, while), and print statements.

## **Table of Contents**

-Installation <br></br>
-Usage<br></br>
-Features<br></br>
-Examples<br></br>
-Testing<br></br>
-Contributing<br></br>
-License<br></br>
-Installation<br></br>

## **Prerequisites**
To work with this project, you'll need the following tools installed:

Rust: Make sure you have Rust installed on your system.
You can install Rust via rustup:

```bash

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Cargo: Cargo is Rust's package manager and build system. It comes with Rust, so if you installed Rust, you already have Cargo.
**Cloning the Repository** <br></br>
Clone this repository to your local machine:

```bash
git clone https://github.com/Albanovukelaj17/Xene.git
cd Xene
```
**Building the Project**
To build the project, simply run the following command inside the project directory:

```bash
cargo build
```
This will compile the source code and produce the executable files in the target directory.

Running the Project
You can run the project using:

```bash
cargo run
```
## **Usage**

Once the project is built, you can start writing programs using the syntax supported by Xene.

**Basic Syntax**
Xene supports the following constructs:

-Variable Declarations: You can declare variables using the var keyword. 
txt
Copy code
var x = 10;<br></br>
-Expressions: You can assign and manipulate variables using standard arithmetic operations.
txt
Copy code
x = x - 1; <br></br>
-Control Flow:
If-Else statements:
txt
Copy code
if x > 5 {
    print(x);
} else {
    print(0);
}<br></br>
-While Loops:
txt
Copy code
while x > 5 {
    x = x - 1;
}<br></br>
-Print Statements: Output expressions using the print() function.
txt
Copy code
print(x);<br></br>
-Running Example Programs
You can write Xene programs in text files and run them by using the Xene interpreter.

Features

Basic Arithmetic Operations: Add, subtract, multiply, and divide.
Variable Assignment: Assign values to variables using the var keyword.
Control Flow: Supports if, else, and while constructs.
Print Statement: Output results with print().
Binary Operations: Compare values with operators like >, <, >=, <=.
Examples

Below are some examples of the Xene programming language.

Example 1: Basic If-Else Statement
txt
Copy code
var x = 10;
if x > 5 {
    print(x);
} else {
    print(0);
}
Example 2: While Loop
txt
Copy code
var x = 10;
while x > 5 {
    print(x);
    x = x - 1;
}
Example 3: Simple Arithmetic Expression
txt
Copy code
var x = 10;
x = x + 2;
print(x);
Testing

Unit tests are implemented to ensure that the interpreter and parser work as expected.

To run the tests:

bash
Copy code
cargo test
Example Test Cases
Tests are written in Rust and are located in the corresponding modules (e.g., lexer.rs, parser.rs, and interpreter.rs). Here are some test cases:

Variable Assignment Test:
rust
Copy code
#[test]
fn test_parse_assignment() {
    let input = "var x = 10;";
    let mut tokens = tokenize(input);
    let ast = parse_assignment(&mut tokens);
    assert!(ast.is_some());
}
While Loop Parsing Test:
rust
Copy code
#[test]
fn test_parse_while_loop() {
    let input = "while x > 5 { x = x - 1; }";
    let mut tokens = tokenize(input);
    let ast = parse_while(&mut tokens);
    assert!(ast.is_some());
}
If-Else Parsing Test:
rust
Copy code
#[test]
fn test_parse_if_else() {
    let input = "if x > 5 { print(x); } else { print(0); }";
    let mut tokens = tokenize(input);
    let ast = parse_if(&mut tokens);
    assert!(ast.is_some());
}
