struct User {
 active: bool,
 username: String,
 email: String,
 sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
      active: false,
      username: String::from("john.doe"),
      email: String::from("JohnDOE@example.com"),
      sign_in_count: 1,
    };

    println!("username: {}, active: {}, count: {}", user1.username, user1.active, user1.sign_in_count);
    signed_in(&mut user1);
    println!("count: {}", user1.sign_in_count);
    activating_a_user(&mut user1);
    println!("active {}", user1.active);
    signed_in(&mut user1);
    println!("count: {}", user1.sign_in_count);
    let mut user2 = build_a_user("JaneDOE".to_string(), "JaneDOE@example.com".to_string());
    println!("{}", user2.sign_in_count);
}

fn activating_a_user(u: &mut User) {
  u.active = true;
}

fn signed_in(u: &mut User) -> u64 {
  if u.active {
    u.sign_in_count += 1;
  }

  u.sign_in_count
}

fn build_a_user(username: String, email: String) -> User {
  User {
    active: false,
    sign_in_count: 0,
    username: String::from(username),
    email: email,
  }
}
