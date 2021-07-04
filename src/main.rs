mod store;

fn main() {
    let pikachu = store::pokemons::pikachu();
    let charmendar = store::pokemons::charmendar();
    let pokemons = &[pikachu, charmendar];
    let mut k = 1;
    for i in pokemons {
        println!("{}. {}", k, i.name);
        k += 1;
    }

    println!("\nChoose your pokemon!");
    let mut k: String = String::new();

    std::io::stdin()
        .read_line(&mut k)
        .expect("Something went wrong...");
    let _k: usize = k.parse().unwrap_or(0);

    let cpu = &pokemons[1];
    let user_pokemon = &pokemons[_k - 1];

    let mut round: i32 = 0;

    println!("You choose {}", user_pokemon.name);
    while true {
        if user_pokemon.health <= 0 {
            break;
        }

        if round == 0 {
            println!(":Your Turn:\nChoose your attack!");
            let mut attack_index: String = String::new();

            std::io::stdin()
                .read_line(&mut attack_index)
                .expect("Something went wrong...");
            let _attack_index: usize = attack_index.parse().unwrap_or(0);
            println!(
                "{} uses {}",
                user_pokemon.name,
                user_pokemon.attacks[_attack_index - 1].name
            );
        }
    }
}
