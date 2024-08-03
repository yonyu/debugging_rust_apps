# Debugging Rust Apps

Exercise the skills to debug Rust apps. A good reference and hands-on guide for Rust developers of all levels.

Rust is secure by default, but it is not immune to bugs. This guide will help you to debug Rust applications.

There are some bugs the Rust compiler can't catch, such as:
- Underlying unsafe libraries via foreign function interfaces (FFI)
- Logic problems
- User input validation


## Some commands

Initialize a new Rust project inside a directory:

> cargo init

## Tools
### Windows
- Visual Studio Code
- Rust Analyzer
- RemedyBG
#### Extensions
- Rust Analyzer
#### Installed Application
- [RemedyBG](https://remedybg.itch.io/remedybg)


### Linux (Ubuntu)
- Visual Studio Code (sudo dpkg -i code_1.75.1-1626302803_amd64.deb)
- Rust Analyzer
- GDB
#### Extensions
- CodeLLDB by Vadim Chugunov
- Rust Analyzer
#### Installed Application
> sudo apt-get install gdb
- GDB from command line
  
### MacOS
- Visual Studio Code
- Rust Analyzer
- LLDB

## Debug Tools Overview
A good debugger should have the following features:
- Clear, understandable output
- Speed and performance
- Provide insights regarding memory usage
- Ability to work with multiple threads well

Desired features of an IDE:
- Great text editing
- Syntax highlighting
- Code completion
- Debugging: clear views
- Debugging: ways to dig in deeper
- Friendly compatibility with profiling tools

## Prepare using RemedyBG

Launch RemedyBG

### Session | Save As... |

D:\workspace-rust\debugging_rust_apps\.remedybg\debugging_rust_app-ssession.rdbg


 List installed and available targets

>  rustup target list

x86_64-pc-windows-msvc (installed)

> rustup default 

stable-x86_64-pc-windows-msvc (default)

> rustup default stable-x86_64-pc-windows-msvc

### Session | Application and Parameters

Command: D:\workspace-rust\debugging_rust_apps\target\debug\debugging_rust_apps.exe
Command Arguments: module_3:check_outstanding_orders
Working Directory: No change

### Source File | Open

main.rs
orders.rs

### F5 to start debug

In watch windows, type in "__locals", you'll see local variables

Or, type: context.orders.len

