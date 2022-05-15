struct User {
    name: String,
    city: String,
    country: String,
}

fn print_user(user: &User) {
    let User {
        name, city, country
    } = user;
    println!("User {} is from {}, {}", name, city, country);
}

fn main() {
    let user = User {
        name: "Tim".to_string(),
        city: "Ottawa, ON".to_string(),
        country: "Canada".to_string(),
    };
    print_user(&user);
}
