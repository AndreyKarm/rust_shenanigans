struct Player {
    money: u32,
}

const FILLER: &str = "--------------------------------";

fn main() {
    let player = Player { money: 100 };
    cycle(player);
}

fn cycle(mut player: Player){
    println!("{}", FILLER);
    println!("Your current balance is ${}", player.money);
    println!("What do you want to play?");
    if player.money == 0 {
        println!("1. Blackjack");
        println!("2. Poker");
        println!("3. Roulette");
        println!("4. Borrow money");
    } else {
        println!("1. Blackjack");
        println!("2. Poker");
        println!("3. Roulette");
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if player.money == 0 {
        match input {
            "1" => blackjack::play(&mut player),
            "2" => poker::play(&mut player),
            "3" => roulette::play(&mut player),
            "4" => borrow_money::play(&mut player),
            _ => std::process::exit(0),
        }
    } else {
        match input {
            "1" => blackjack::play(&mut player),
            "2" => poker::play(&mut player),
            "3" => roulette::play(&mut player),
            _ => std::process::exit(0),
        }
    }

    cycle(player);
}

mod borrow_money {
    use super::Player;

    pub fn play(player: &mut Player) {
        println!("{}", crate::FILLER);
        println!("You have no money left");
        println!("How much money do you want to borrow?");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse::<u32>().unwrap();

        if input > 1000 {
            println!("You greedy bastard! You can't borrow that much money!");
        } else {
            player.money += input;
            println!("You borrowed ${}", input);
        }
    }

}

mod blackjack {
    use super::Player;

    pub fn play(player: &mut Player) {
        println!("Playing blackjack with ${}", player.money);
    }
}

mod poker {
    use super::Player;

    pub fn play(player: &mut Player) {
        println!("Playing poker with ${}", player.money);
    }
}

mod roulette {

    use super::Player;
    use rand::Rng;

    pub fn play(player: &mut Player) {
        println!("{}", crate::FILLER);
        println!("Playing roulette with ${}", player.money);
        println!("Choose a number between 0 and 36");
        println!("Or choose 37 for 'even' or 38 for 'odd' or 39 for 'red' or 40 for 'black'");
        println!("{}", crate::FILLER);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse::<u8>().unwrap();

        println!("Place a bet");

        let mut bet = String::new();
        std::io::stdin().read_line(&mut bet).unwrap();
        let bet = bet.trim().parse::<u32>().unwrap();

        println!("{}", crate::FILLER);
        if bet > player.money {
            println!("You don't have enough money to place that bet");
            return;
        }

        let number = generate_random_number();
        println!("The number is {}", number);
        
        match input {
            0..=36 => {
                if input == number {
                    println!("You won! ${}", bet*36);
                    player.money += bet*36;
                } else {
                    println!("You lost!");
                    player.money -= bet;
                }
            },
            37 => {
                println!("You chose even");
                if number % 2 == 0 {
                    println!("You won! ${}", bet*2);
                    player.money += bet*2;
                } else {
                    println!("You lost!");
                    player.money -= bet;
                }
            },
            38 => {
                println!("You chose odd");
                if number % 2 != 0 {
                    println!("You won! ${}", bet*2);
                    player.money += bet*2;
                } else {
                    println!("You lost!");
                    player.money -= bet;
                }
            },
            39 => {
                println!("You chose red");
                if number % 2 == 0 {
                    println!("You won! ${}", bet*2);
                    player.money += bet*2;
                } else {
                    println!("You lost!");
                    player.money -= bet;
                }
            },
            40 => {
                println!("You chose black");
                if number % 2 != 0 {
                    println!("You won! ${}", bet*2);
                    player.money += bet*2;
                } else {
                    println!("You lost!");
                    player.money -= bet;
                }
            },
            _ => {
                println!("Incorrect input");
            }
        }
    }

    fn generate_random_number() -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..=36)
    }
}