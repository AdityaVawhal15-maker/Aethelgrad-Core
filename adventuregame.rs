use std::io::{self, Write};

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    Quit,
    Invalid,
}

#[derive(Debug)]
#[allow(dead_code)] 
struct Item {
    name: String,
    description: String, 
    power_boost: u32,
}


#[derive(Debug)]
struct Room {
    id: usize,
    name: String,
    description: String,
    exits: Vec<Direction>,
    items: Vec<Item>,
    monster_present: bool,
}

struct Player {
    name: String,
    location_id: usize,
    health: u32,
    attack_power: u32,
    inventory: Vec<Item>,
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name,
            location_id: 0,
            health: 100,
            attack_power: 10,
            inventory: Vec::new(),
        }
    }

    fn move_to(&mut self, new_room_id: usize) {
        self.location_id = new_room_id;
        println!("\n*** You move to a new location. ***");
    }

    fn check_health(&self) {
        println!("*** {}'s Health: {} ***\n", self.name, self.health);
    }
    
    fn pickup_item(&mut self, item: Item) {
        println!("You found and picked up a {}!", item.name);
        self.attack_power += item.power_boost;
        println!("Your attack power is now {}!", self.attack_power);
        self.inventory.push(item);
    }
}

impl Room {
    fn get_next_room_id(&self, direction: &Direction) -> Option<usize> {
        match (self.id, direction) {
            (0, Direction::North) => Some(1),
            (0, Direction::East) => Some(2),
            
            (1, Direction::South) => Some(0),
            (1, Direction::West) => Some(3),

            (2, Direction::West) => Some(0),

            (3, Direction::East) => Some(1),
            
            _ => None,
        }
    }
}

fn create_world() -> Vec<Room> {
    vec![
        Room {
            id: 0,
            name: String::from("Starting Chamber"),
            description: String::from("A dusty, circular room. There is a chill in the air."),
            exits: vec![Direction::North, Direction::East],
            items: vec![
                Item {
                    name: String::from("Old Torch"),
                    description: String::from("It flickers dimly."),
                    power_boost: 0,
                },
            ],
            monster_present: false,
        },
        Room {
            id: 1,
            name: String::from("Dark Corridor"),
            description: String::from("The walls are lined with strange runes. You hear dripping water."),
            exits: vec![Direction::South, Direction::West],
            items: Vec::new(),
            monster_present: false,
        },
        Room {
            id: 2,
            name: String::from("Treasure Vault"),
            description: String::from("A small room filled with glittering dust and a single glowing sword."),
            exits: vec![Direction::West],
            items: vec![
                Item {
                    name: String::from("Sword of Glory"),
                    description: String::from("Gives a massive boost."),
                    power_boost: 50,
                },
            ],
            monster_present: false,
        },
        Room {
            id: 3,
            name: String::from("Ancient Altar"),
            description: String::from("A massive chamber dominated by an altar. A fearsome monster blocks the exit!"),
            exits: vec![Direction::East],
            items: Vec::new(),
            monster_present: true,
        },
    ]
}

fn parse_command(input: &str) -> Direction {
    match input.trim().to_lowercase().as_str() {
        "n" | "north" => Direction::North,
        "s" | "south" => Direction::South,
        "e" | "east" => Direction::East,
        "w" | "west" => Direction::West,
        "quit" | "q" => Direction::Quit,
        _ => Direction::Invalid,
    }
}

fn handle_battle(player: &mut Player) -> bool {
    println!("\n!!! A MONSTER APPEARS !!!");
    println!("You must defeat it to proceed.");
    
    let monster_health = 80;
    let monster_attack = 15;
    
    println!("Monster Health: {}", monster_health);
    println!("Your Attack Power: {}", player.attack_power);
    
    if player.attack_power >= monster_health {
        println!("You unleash a powerful blow and instantly defeat the monster!");
        true
    } else {
        println!("The monster strikes back! You take damage and flee.");
        player.health = player.health.saturating_sub(monster_attack);
        player.check_health();
        player.location_id = 1;
        false
    }
}

fn main() {
    println!("--- Welcome to The Simple Rust Dungeon! ---");

    let mut world = create_world();
    let mut player = Player::new(String::from("Hero"));

    loop {
        let current_room_id = player.location_id;
        let current_room = &mut world[current_room_id];

        println!("\n-----------------------------------------");
        println!("You are in the: **{}**", current_room.name);
        println!("{}", current_room.description);
        player.check_health();

        if current_room_id == 3 && !current_room.monster_present {
            println!("\n*** CONGRATULATIONS! You have defeated the boss and won the game! ***");
            break;
        }

        if !current_room.items.is_empty() {
            let item = current_room.items.drain(0..1).next().unwrap();
            player.pickup_item(item);
        }

        print!("Exits available: ");
        for exit in &current_room.exits {
            print!("{:?} ", exit);
        }
        println!("\n");

        print!("What do you do? (N/S/E/W or Quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let command = parse_command(&input);

        match command {
            Direction::Quit => {
                println!("\nThank you for playing!");
                break;
            }
            Direction::North | Direction::South | Direction::East | Direction::West => {
                if let Some(next_room_id) = current_room.get_next_room_id(&command) {
                    
                    if next_room_id == 3 && world[3].monster_present {
                        if handle_battle(&mut player) {
                            player.move_to(next_room_id);
                            world[3].monster_present = false;
                        }
                    } else {
                        player.move_to(next_room_id);
                    }
                } else {
                    println!("You can't go that way.");
                }
            }
            Direction::Invalid => {
                println!("Invalid command. Please use N, S, E, W, or Quit.");
            }
        }
        
        if player.health == 0 {
            println!("\n!!! Your health dropped to zero. Game Over. !!!");
            break;
        }
    }
}