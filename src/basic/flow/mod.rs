#![allow(dead_code,unused_variables,unreachable_patterns)]
pub fn main() {
    flow_if();
    flow_for();
    flow_match();
}

fn flow_if() {

    let score = 90;

    if score > 75 {
        println!("congratulations!!");
    } else {
        println!("try again");
    }

}

fn flow_for() {

    let scores = [90, 85, 70, 75, 80];

    for (i, score) in scores.iter().enumerate() {
        print!("{}", score);

        if i < scores.len() - 1 {
            print!(", ");
        }
    }

    println!();

}

fn flow_match() {
    enum Position {
        Left,
        Right,
        Center,
    }
    
    enum Player {
        Front,
        Middle(Position),
        Back(Position),
    }

    fn print_responsibilities(player: Player) {
        match player {
           Player::Front => {
               println!("attacker")
           },
           Player::Middle(position) => {
               println!("play maker")
           },
           Player::Back(position) => {
               println!("defender")
           },
           _ => println!("player unknown position. is he a coach?")
       }
    }

    let ronaldo = Player::Middle(Position::Right);
    print_responsibilities(ronaldo);

    let pepe = Player::Back(Position::Left);
    print_responsibilities(pepe);

}