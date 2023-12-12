use std::io;
use std::io::Write;

// Mad libs are a simple game where you create a story tem- plate with blanks for words. 
// You, or another player, then construct a list of words and place them into the story, cre- ating an often silly or funny story as a result.
// Create a simple mad-lib program that prompts for a noun, a verb, an adverb, and an adjective and injects those into a story that you create.

// Noun, verb, adjective, adverb
// output: sentence.

// Input:
// Enter a noun: dog
// Enter a verb: walk
// Enter an adjective: blue
// Enter an adverb: quickly

// Expected Output
// Do you walk your blue dog quickly? That's hilarious!


fn main() -> io::Result<()> {
    let mut noun = String::new();
    let mut verb = String::new();
    let mut adjective = String::new();
    let mut adverb = String::new();

    print!("Enter a noun: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut noun)?;

    print!("Enter a verb: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut verb)?;

    print!("Enter an adjective: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut adjective)?;

    print!("Enter an adverb: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut adverb)?;

    println!("do you {verb} your {adj} {noun} {adv}? That's hilarious!", verb=verb.trim(), adj=adjective.trim(), noun=noun.trim(), adv=adverb.trim());
    Ok(())
}
