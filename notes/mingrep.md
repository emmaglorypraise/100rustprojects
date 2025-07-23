# Project 026 – Minigrep

## What I Built
My own version of the classic command line search tool grep (globally search a regular expression and print). In the simplest use case, grep searches a specified file for a specified string. To do so, grep takes as its arguments a file path and a string. Then it reads the file, finds lines in that file that contain the string argument, and prints those lines. 

## What I Learned

## Notes

#### How to run CLI
Minigrep accept its two command line arguments: the file path and a string to search for. That is, we want to be able to run our program with cargo run, two hyphens to indicate the following arguments are for our program rather than for cargo, a string to search for, and a path to a file to search in, like so:
```
$ cargo run -- searchstring poem.txt
```

#### Example poem.txt:
** Poem “I’m Nobody! Who are you?” by Emily Dickinson.  Create a file called poem.txt at the root level of your project, and enter the texts below: 

```
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

The searchstring is any string you want to search for 

```
cargo run > output.txt
```

The > syntax tells the shell to write the contents of standard output to output.txt instead of the screen. The print macro used in this case is `eprintln!` instead of `println!`.

Now we see the error onscreen and output.txt contains nothing, which is the behavior we expect of command line programs.

Let’s run the program again with arguments that don’t cause an error but still redirect standard output to a file, like so:

```
cargo run -- to poem.txt > output.txt
```
We won’t see any output to the terminal, and output.txt will contain our results:

Filename: output.txt

```
Are you nobody, too?
How dreary to be somebody!
```

This demonstrates that we’re now using standard output for successful output and standard error for error output as appropriate.