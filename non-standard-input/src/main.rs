use std::io::stdin;

fn main() {
    println!("What is 3 + 2? Type your answer and press enter. ");

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Unable to read standard input");

    // ORIGINAL with the mistake - if input == "5"
    if input.trim() == "5" {
        println!("Correct");
    } else {
        println!("Incorrect");
    }
    println!("{:#?}", input);
}

/*
What is 3 + 2? Type your answer and press enter.
5
"5\n"
Incorrect

This is because as we send in the debug println!() statement, the standard input system adds "control sequences" such as /n, /r etc....
What we need to do is the use the trim() function for this to work right. I have seen in common practice the trim() function used

 non-standard-input git:(main) ✗ cargo run
   Compiling non-standard-input v0.1.0 (/Users/squeeko/RustBrainTeasers/non-standard-input)
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/non-standard-input`
What is 3 + 2? Type your answer and press enter.
5
Correct
"5\n"

Don’t Trust Input
A good rule of thumb is to never trust input; however, there are a few things
you can do to minimize your problems when input is necessary:

When working with strings, use trim to remove whitespace.

When comparing strings, use to_lowercase or to_uppercase to ensure that
you’re comparing strings in the same case. These functions take care of
Unicode case-folding.[6]

When parsing complicated strings, use regular expressions to extract parts of
a string.
*/
