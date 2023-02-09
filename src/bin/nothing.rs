pub struct User {
    pub name: String,
    pub age: u32
}

impl User {
    pub fn new() -> User {
        User { name: "artizon".to_string(), age: 20 }
    }

    pub fn print_name(&self) -> () {
        println!("{}", self.name);
    }
}

fn main() {
    println!("Hello world");
    let user = User::new();
    user.print_name();
}
