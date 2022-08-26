mod user;

use std::io::{self, Write};
use user::user::User;

enum MenuActions {
    Create,
    List,
    Read,
    Delete,
}

fn main() {
    let mut user_list: Vec<user::user::User> = vec![];
    loop {
        print_menu();
        match get_user_input() {
            Some(MenuActions::List) => list_users(&user_list),
            Some(MenuActions::Create) => create_user(&mut user_list),
            Some(MenuActions::Read) => read_user(&user_list),
            Some(MenuActions::Delete) => println!("Apagar"),
            None => println!("\n! Opção inválida !\n"),
        }
    }
}

fn create_user(user_list: &mut Vec<User>) {
    user_list.push(user::user::create());

    println!("\nUsuário criado!\n")
}

fn list_users(user_list: &Vec<User>) {
    let users = user_list
        .iter()
        .fold("".to_string(), |acc: String, user: &User| {
            if acc.len() == 0 {
                user.name.to_string()
            } else {
                acc + ", " + &user.name
            }
        });

    println!(
        "Estão cadastrados {} usuários: {}\n",
        user_list.len(),
        users
    );
}

fn read_user(user_list: &Vec<User>) {
    let mut chosen_pos = prompt_user("Qual usuário você quer detalhar? ")
        .parse::<usize>()
        .unwrap();

    while let None = user_list.get(chosen_pos) {
        println!("Nenhum usuário nessa posição");
        chosen_pos = prompt_user("Qual usuário você quer detalhar? ")
            .parse::<usize>()
            .unwrap();
    }
    user_list.get(chosen_pos).unwrap().read();
}

fn print_menu() {
    println!(
        "LISTAR todos usuários\n\
      CRIAR novo usuário\n\
      LER dados de usuário\n\
      APAGAR usuário\n"
    );
}

pub fn prompt_user(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap().to_string();
    input.trim().to_string()
}

fn get_user_input() -> Option<MenuActions> {
    let input = prompt_user("Escolha sua opção: ");
    println!("\n");

    match input.to_lowercase().trim() {
        "listar" => Some(MenuActions::List),
        "criar" => Some(MenuActions::Create),
        "ler" => Some(MenuActions::Read),
        "apagar" => Some(MenuActions::Delete),
        _ => None,
    }
}
