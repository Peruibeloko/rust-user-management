mod persistence;
mod shared;
mod user;

use persistence::{read_users_from_file, write_users_to_file};
use shared::prompt_user;
use user::User;

enum MenuActions {
    Create,
    List,
    Read,
    Delete,
    Exit,
}

fn main() {
    let mut user_list: Vec<user::User> = read_users_from_file("users".to_string());

    loop {
        print_menu();
        match get_user_input() {
            Some(MenuActions::List) => list_users(&user_list),
            Some(MenuActions::Create) => create_user(&mut user_list),
            Some(MenuActions::Read) => read_user(&user_list),
            Some(MenuActions::Delete) => println!("Apagar"),
            Some(MenuActions::Exit) => exit_program(&user_list),
            None => println!("\n! Opção inválida !\n"),
        }
    }
}

fn print_menu() {
    println!(
        "LISTAR todos usuários\n\
      CRIAR novo usuário\n\
      LER dados de usuário\n\
      APAGAR usuário\n\
      SAIR do programa\n"
    );
}

fn get_user_input() -> Option<MenuActions> {
    let input = prompt_user("Escolha sua opção: ");
    println!("\n");

    match input.to_lowercase().trim() {
        "listar" => Some(MenuActions::List),
        "criar" => Some(MenuActions::Create),
        "ler" => Some(MenuActions::Read),
        "apagar" => Some(MenuActions::Delete),
        "sair" => Some(MenuActions::Exit),
        _ => None,
    }
}

fn create_user(user_list: &mut Vec<User>) {
    user_list.push(user::create());

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

fn exit_program(user_list: &Vec<User>) {
    match write_users_to_file(user_list) {
        Ok(msg) => println!("{msg}"),
        Err(msg) => println!("{msg}"),
    }
}
