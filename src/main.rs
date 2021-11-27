use std::io::{self, Read, Write};

/// Draw out the game state in ascii
fn draw(state: &[char; 9]) {
    for i in (0..3).rev() {
        let offset = i * 3;
        println!("-------------");
        print!("| ");
        print!("{}", state[offset + 0]);
        print!(" | ");
        print!("{}", state[offset + 1]);
        print!(" | ");
        print!("{}", state[offset + 2]);
        println!(" |");
    }

    println!("-------------");
}

/// Read a value between 1..9 from standard input.
fn get_input(player: char) -> usize {
    loop {
        print!("Select your field ({}): ", player);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let number: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if number < 1 || number > 9 {
            println!("Number must range from 1 to 9.");
            continue;
        };

        return number;
    }
}

fn check_for_win(state: &[char; 9]) -> bool {
    for i in 0..3 {
        if state[i] == state[i + 3] && state[i] == state[i + 6] {
            return true;
        }

        let i = i * 3;

        if state[i] == state[i + 1] && state[i] == state[i + 2] {
            return true;
        }
    }

    if state[0] == state[4] && state[0] == state[8] ||
       state[2] == state[4] && state[2] == state[6] {
            return true;
    }

    return false;
}

fn main() {
    let players = [ 'X', 'O' ];
    let mut state = [ '1', '2', '3', '4', '5', '6', '7', '8', '9' ];
    let mut turns = 0;

    loop {
        let player = players[turns % 2];
        draw(&state);

        // loop until we get a valid user response
        loop {
            let requested_number = get_input(player) - 1;
            if state[requested_number] == players[0] ||
               state[requested_number] == players[1] {
                println!("That field is allready taken by \'{}\'!", state[requested_number]);
            } else {
                state[requested_number] = player;
                break;
            }
        }

        if check_for_win(&state) {
            draw(&state);
            println!("{} Wins the game!", player);
            break;
        }        

        // after 9 turns without any winner it's a draw
        if turns >= 8 {
            draw(&state);
            println!("DRAW, none wins the match!");
            break;
        }

        turns += 1;
    }

    println!("gg \\(^._.^)/");

    print!("Press ENTER to quit...");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}
