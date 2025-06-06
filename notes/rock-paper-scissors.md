# Project 008 – Rock-Paper-Scissors Game 

## What I Built
A Rust CLI game where user can play the Rock-Paper-Scissors game against the computer.

## What I Learned
 
### Enum vs Pattern matching
Think of an enum as a special kind of container that can hold one of a few labeled “boxes,” and only one at a time. In our game, we have:
```
enum GameResult {
    Win,
    Lose,
    Draw,
}
```

That means “GameResult” can be either a Win box, a Lose box, or a Draw box. It’s like if you had three toy chests labeled “Win,” “Lose,” and “Draw,” and each time you play, you pick exactly one of those chests to hold your result.

A pattern match is a way to look inside that enum (or other data) and say “If it’s this box, do that; if it’s that box, do something else.” In the code:
```
match determine_winner(&user_choice, &computer_choice) {
    GameResult::Win  => println!("🏆 You win!"),
    GameResult::Lose => println!("😔 You lose!"),
    GameResult::Draw => println!("🤝 It's a draw!"),
}
```

we call determine_winner, which returns one of those three GameResult boxes. The match (...) { ... } line then looks at which box we got. If it’s GameResult::Win, we print “You win!” If it’s GameResult::Lose, we print “You lose!” And if it’s GameResult::Draw, we print “It’s a draw!”

Even inside determine_winner, we use pattern matching on the choices:
```
match (user, computer) {
    ("rock",     "scissors") => GameResult::Win,
    ("paper",    "rock")     => GameResult::Win,
    ("scissors", "paper")    => GameResult::Win,
    (a, b) if a == b        => GameResult::Draw,
    _                       => GameResult::Lose,
}
```

Here we treat (user, computer) like a pair of stickers—if they match exactly one of the winning pairs (e.g. ("rock", "scissors")), we return GameResult::Win. If both stickers are the same (like both "rock"), the if a == b arm says “it’s a draw.” The final _ case means “anything else left over must be a losing scenario,” so it returns GameResult::Lose.

In summary:

Enum: A container that can be one of a few named things (Win, Lose, or Draw).

Pattern matching: A way to “look inside” that container (or a pair of values) and pick exactly the right action or return value for each possible shape—just like checking labels on boxes before deciding what to do.

## Notes
