# Basic Rust Commands (Terminal)

rustc --version - Show's Rust Version

rustup update - Updates Rust

rustup self uninstall - Uninstall Rust & rustup




# About Rust Files

Rust files are always saved in the .rs format. When they are executed, they are compiled and a .pdb and a .exe file are outputted into the same directory as the original .rs Rust script. I need to run the .exe file on Windows to run the program. 

Hello World: 

fn main() {
    println!("Hello World!");
}

# Some Basic Principals

When indenting code (see println!), we indent with 4 spaces, not a tab in Rust.

The ! at the end of println indicates that we are calling a Rust Macro instead of a Normal Function, and Macros don't always follow the same rules as functions. 

We passed "Hello World!" as an arguement to println!, which printed it to the screen. 

We also ensure that we end the line with a semicolon ;
This shows that the expression is over. 