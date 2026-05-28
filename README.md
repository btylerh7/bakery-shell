# Bakery Shell

This shell is being developed for the course at [Codecrafters](codecrafters.io) called "Build your own Shell".
I started the project because it was made free for the month, and a shell sounded like a fun project to use
to learn the ins and outs of Rust, which I have been learning off an on for a while now. 

While this is following a course, there are not speicific instructions telling you how to implement the features
of the shell. It is designed to be programming language agnostic, and therefore I am mostly left to implement it
however I want to. There are examples you can look at, but most examples utilize third party libraries, and I want
to use as much of the standard library as possible.

## Features

### Stage 1

- [x] Print a prompt
- [x] Handle invalid commands
- [x] Implement a Read-Eval-Print-Loop (REPL)
- [x] Implement `exit` builtin
- [x] Implement `echo` builtin
- [x] Implement `type` builtin
- [x] Locate executable files
- [x] Execute non-builtin programs

### Navigation

- [x] Implement `pwd` builtin
- [x] Implement the `cd` builtin
    - [x] Absolute paths
    - [x] Relative paths
    - [x] Expand `~` to home directory

### Quoting

- [x] Handle single quotes
- [x] Handle double quotes
- [x] Backslash outside quotes
- [x] Backslash within single quotes
- [x] Backslash within double quotes
- [x] Executing a quoted executable

### Redirection

- [x] Redirect stdout
- [x] Redirect stderr
- [x] Append stdout
- [x] Append stderr

### Command Completion

- [x] Builtin completion
- [x] missing completions
- [x] executable completion
- [x] multiple completions
- [x] partial completions

### filename completion

- [x] file completion
- [x] nested file completion
- [x] directory completion
- [x] multiple matches 
- [x] partial completions
- [x] multi-argument completions

### Programmable Completion

- [ ] Register complete builtin
- [ ] Printing missing specifications
- [ ] Displaying registered specifications
- [ ] Single completion
- [ ] Handling no completions
- [ ] Passing comand-line arguments
- [ ] Passing environment variables
- [ ] Multiple completer candidates
- [ ] Longest common prefix
- [ ] Unregister a completion

More to the course, but I will add the checklist later.
