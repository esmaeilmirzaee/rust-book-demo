fn main() {
  let mut count = 0;
  'counting_up: loop {
    println!("Count: {}", count);
    let mut remaining = 10;

    loop {
      println!("remaining {}", remaining);
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {}", count);

  using_while_loop([10, 20, 30, 40, 50, 60, 70, 80, 90]);
}


fn using_while_loop (arr: [i32; 9]) {
  let mut index = 0;
  while index < arr.len() {
    println!("{} of array is {}", index, arr[index]);
    index += 1;
  }
  println!("Finished printing the array");
}

fn raning_for_loop (arr: [5; f64]) {
  for 1..arr.len() in arr {
    println!("{}", )
  }
}
