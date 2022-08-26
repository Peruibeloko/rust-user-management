pub mod user {
    use crate::prompt_user;

    #[derive(Debug)]
    pub struct User {
        pub name: String,
        pub age: u8,
    }

    impl User {
        pub fn read(&self) {
            println!("\n{}, {} anos de idade\n", &self.name, &self.age);
        }
    }

    pub fn create() -> User {
        let name = prompt_user("Nome: ");
        let age = prompt_user("Idade: ").parse::<u8>().unwrap();
        User { name, age }
    }
}
