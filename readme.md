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

## Separation of Concerns for Binary Projects
- Split your program into a main.rs file and a lib.rs file and move your program’s logic to lib.rs.
- As long as your command line parsing logic is small, it can remain in main.rs.
- When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

function of main.rs->
- Calling the command line parsing logic with the argument values
- Setting up any other configuration
- Calling a run function in lib.rs
- Handling the error if run returns an error

improve `parse_config` function to return configuration values which are related
also have a exception if the argument are less than 3 char

return an result instead of panic-> have some error handling to handle the Result value returned from the build function and exit the process more cleanly in the error case. ; 
rename function from `new` to `build`

## handling the exit cases
define explicitly the exit case with error code 1 to avoid panic! 

## Run function
The run function now contains all the remaining logic from main, starting from reading the file. The run function takes the Config instance as an argument.

now handle for it's error handling, done with it

## Error handling
mak sure to have test edge cases for like arg check, file and path check
then move this logic to another module and import it into main.rs file via Crate

move 12.4

# Test Driven Development
1. Write a test that fails and run it to make sure it fails for the reason you expect.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed and make sure the tests continue to pass.
4. Repeat from step 1!

## Writing a failing test
we wrote sometest cases which fail as expected

## To pass Them
1. Iterate through each line of the contents.
2. Check whether the line contains our query string.
3. If it does, add it to the list of values we’re returning.
4. If it doesn’t, do nothing.
5. Return the list of results that match.

## Using the `search` Function in the `run` Function
pass the config.query value and the contents that run reads from the file to the search function. Then run will print each line returned from search:
```cargo run -- monomorphization poem.txt```

# Environment Variables
case-insensitive searching via env
just check for case sensitivity

get lines with uppcase;case-insensitive searching controlled by an environment variable. 
```bash
IGNORE_CASE=1 cargo run -- to poem.txt
```

# Writing Error Messages to Standard Error Instead of Standard Output
there are two types of output-> `stdout`:normal text, `stderr`:error messages
`println` supports only `stdout`; get something else for `errors`

write the contents of standard output to output.txt with `>`->
```sh
$ cargo run > output.txt
```

# Printing Error to Standard Error
standard library provides the eprintln! macro that prints to the standard error stream, so let’s change the two places we were calling println! to print errors to use eprintln! instead.

to print our results to output.txt
```bash
cargo run -- to poem.txt > output.txt
```
