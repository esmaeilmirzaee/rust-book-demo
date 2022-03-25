pub fn add_two(a: i32) -> i32 {
  internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
  a + b
}

#[cfg(test)]
mod tests {
  use super::*;

    #[test]
    fn it_works() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_internal_adder() {
      let result = internal_adder(4, 2);
      assert_eq!(6, result)      
    }
}
