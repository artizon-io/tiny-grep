pub struct User {
    pub name: String,
    pub age: u32,
}

impl User {
    pub fn new() -> User {
        User {
            name: "artizon".to_string(),
            age: 20,
        }
    }

    pub fn print_name(&self) -> () {
        println!("{}", self.name);
    }
}

fn main() {
    println!("Hello world");
    let user = User::new();
    user.print_name();

    let mut vec = vec![1, 2, 3];
    // let mut vec = vec![User::new(), User::new(), User::new()];
    let first = vec[0];
    // let second = &mut vec[1];
    vec.remove(0);
    println!("{}", vec[0]);
    // println!("{}", vec[0].age);
    println!("{}", first);
    // println!("{}", first.age);
    // println!("{second}");
}
