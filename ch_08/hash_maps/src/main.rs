use std::collections::HashMap;

fn counting_words(text: String) {
  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);
}

fn main() {
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  for (k, v) in scores {
 //   scores[k] = *v + 10;
    println!("{}: {}", k, v);
  }

  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];
  let scores: HashMap<_, _> = 
    teams.into_iter().zip(initial_scores.into_iter()).collect();

  for (item, value) in scores {
    println!("{}: {}", item, value);
  }
  
//  let team_name = String::from("Blue");
//  match scores.get(&team_name) {
//    Some(&team_score) => println!("{}: {}", team_name, &team_score),
//    None => println!("Can't find the team in scores."),
//  }

  counting_words(String::from("hello world wonderful world"));
}
