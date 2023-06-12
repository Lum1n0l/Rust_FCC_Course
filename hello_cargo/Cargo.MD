# Cargo

Cargo is Rust's build system and package manager. 
It is preinstalled with Rust. 

To create a new Project with Cargo:

With a new Git Repo: cargo new PROJECT_NAME

A new Git repo will not be created if it created within an existing Repo. 

The .toml file contains the Projects configuration files. 

The [package] section indicates that the following statements are configuring a package. 

[dependancies] is a section for me to include any dependancies my project requires.

Cargo expects all my source files to live inside the .src directory. The top level directory is just for README files, Licence info, config files and anything else not code related. 

If I want to convert a project that doesn't use Cargo into one that does, I can simply move the source files into the .src directory and create the appropriate Cargo.toml file. 

To build a Cargo project, from the hello_cargo directory, run: 
        cargo build

This command creates an executable file in target\debug\hello_cargo.exe on Windows rather than in your current directory.
Because the default build is a debug build, Cargo puts the binary in a directory named debug. You can run the executable with this command:
        .\target\debug\hello_cargo.exe

Running cargo build for the first time also causes Cargo to create a new file at the top level: Cargo.lock. 
This file keeps track of the exact versions of dependencies in your project. This project doesn’t have dependencies, so the file is a bit sparse. You won’t ever need to change this file manually; Cargo manages its contents for you.

We just built a project with cargo build and ran it with ./target/debug/hello_cargo, but we can also use cargo run to compile the code and then run the resultant executable all in one command:
        cargo run

Using cargo run is more convenient than having to remember to run cargo build and then use the whole path to the binary, so most developers use cargo run.

Notice that this time we didn’t see output indicating that Cargo was compiling hello_cargo. 
Cargo figured out that the files hadn’t changed, so it didn’t rebuild but just ran the binary. 
If you had modified your source code, Cargo would have rebuilt the project before running it.

Cargo also provides a command called cargo check. This command quickly checks your code to make sure it compiles but doesn’t produce an executable.
        cargo check

# Building For Release

When your project is finally ready for release, you can use cargo build --release to compile it with optimizations. 
This command will create an executable in target/release instead of target/debug. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. 
This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. 
If you’re benchmarking your code’s running time, be sure to run cargo build --release and benchmark with the executable in target/release.
        cargo build --release

