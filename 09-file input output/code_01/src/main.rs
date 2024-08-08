use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;

const CSV_FILE_PATH: &str = "users.csv";

#[derive(Clone)]
struct User {
    username: String,
    password: String,
}

fn main() {
    let mut users = load_users_from_csv();
    loop {
        println!("Login Screen");
        println!("Enter Username:");
        let username = read_input();
        println!("Enter Password:");
        let password = read_input();

        if users
            .iter()
            .any(|user| user.username == username && user.password == password)
        {
            println!("Login successful!");
            user_list_screen(&mut users);
        } else {
            println!("Login failed. Incorrect username or password.");
        }
    }
}

fn user_list_screen(users: &mut Vec<User>) {
    loop {
        println!("User List Screen");
        println!("1. Add User");
        println!("2. Update User");
        println!("3. Delete User");
        println!("4. List Users");
        println!("5. Logout");
        println!("6. Exit Application");

        let choice = read_input();
        match choice.as_str() {
            "1" => add_user(users),
            "2" => update_user(users),
            "3" => delete_user(users),
            "4" => list_users(users),
            "5" => break, // Return to the login screen
            "6" => {
                println!("Exiting application...");
                process::exit(0); // Terminate the process
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn add_user(users: &mut Vec<User>) {
    println!("Enter new username:");
    let username = read_input();
    println!("Enter new password:");
    let password = read_input();

    users.push(User {
        username: username.clone(),
        password: password.clone(),
    });

    persist_users_to_csv(users);
    println!("User added successfully.");
}

fn update_user(users: &mut Vec<User>) {
    println!("Enter username to update:");
    let username = read_input();
    let mut updated = false;

    if let Some(pos) = users.iter().position(|u| u.username == username) {
        println!("Enter new password:");
        let new_password = read_input();
        users[pos].password = new_password;
        updated = true;
    }

    if updated {
        persist_users_to_csv(users);
        println!("User updated successfully.");
    } else {
        println!("User not found.");
    }
}

fn delete_user(users: &mut Vec<User>) {
    println!("Enter username to delete:");
    let username = read_input();
    let mut deleted = false;

    if let Some(pos) = users.iter().position(|u| u.username == username) {
        users.remove(pos);
        deleted = true;
    }

    if deleted {
        persist_users_to_csv(users);
        println!("User deleted successfully.");
    } else {
        println!("User not found.");
    }
}

fn list_users(users: &Vec<User>) {
    println!("Current Users:");
    for user in users.iter() {
        println!("Username: {}, Password: {}", user.username, user.password);
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn load_users_from_csv() -> Vec<User> {
    let file = File::open(CSV_FILE_PATH).unwrap_or_else(|_| {
        let mut file = File::create(CSV_FILE_PATH).unwrap();
        writeln!(file, "username,password").unwrap();
        file
    });

    let reader = BufReader::new(file);
    let mut users = Vec::new();
    for line in reader.lines().skip(1) {
        if let Ok(line) = line {
            let mut parts = line.split(',');
            if let (Some(username), Some(password)) = (parts.next(), parts.next()) {
                users.push(User {
                    username: username.to_string(),
                    password: password.to_string(),
                });
            }
        }
    }
    users
}

fn persist_users_to_csv(users: &Vec<User>) {
    let mut file = File::create(CSV_FILE_PATH).unwrap();
    writeln!(file, "username,password").unwrap(); // Write the header
    for user in users.iter() {
        writeln!(file, "{},{}", user.username, user.password).unwrap();
    }
}
