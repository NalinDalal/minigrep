create a new project by name `minigrep` by running:
```sh
cargo new minigrep
cd minigrep     # move to the folder
```

# Accepting Command Line Arguments

1. First Task->accept 2 cli arg: `file_path` and `string`
`$ cargo run -- searchstring example-filename.txt`
`searchstring`: string to search for
`example-filename.txt`: file to search from

wrote some code to collect argument whicha re string and then return them
runvia cmd: `cargo run -- needle haystack`
returns -> `  "needle",
    "haystack",`

2. saving argument value in variables
collect them in a vector with help of `.collect()` method.

3. first variable collect the reference to the string we are searching
   for.second collect reference to string we are searching from.

to run-> `cargo run -- test sample.txt`

# Reading a File
create a file `poem.txt` with some content in it. bring `fs` library to handle
with files
upon running -> `cargo run -- test poem.txt` it provides us with content for the
poem.txt file

# Refactoring to Improve Modularity and Error Handling
there are 4 problems:
1. as program increase the main function will be more cluttered
2. scope also increae as program increases{make a struct}
3. the error message can be more clear in case of diff type of error rather than `Should have been able to read the file`.
4. expect w/o enough arguments.

solution by refactoring:

move to 12.3

## Separation of Concerns for Binary Projects
- Split your program into a main.rs file and a lib.rs file and move your program’s logic to lib.rs.
As long as your command line parsing logic is small, it can remain in main.rs.
When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.
