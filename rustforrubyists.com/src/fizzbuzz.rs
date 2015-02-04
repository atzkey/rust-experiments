fn div_by_three(x: int) -> bool {
  x % 3 == 0
}

fn div_by_five(x: int) -> bool {
  x % 5 == 0
}

fn div_by_fifteen(x: int) -> bool {
  x % 15 == 0
}

fn main() {
  for i in range(1i, 100) {
    let answer =
      if (div_by_fifteen(i)) {
        "FizzBuzz".to_string()
      } else if (div_by_five(i)) {
        "Buzz".to_string()
      } else if (div_by_three(i)) {
        "Fizz".to_string()
      } else {
        i.to_string()
      };

      println!("{}", answer);
  }
}

#[test]
fn test_div_by_three() {
    assert!(div_by_three(3));
    assert!(!div_by_three(1));
}

#[test]
fn test_div_by_five() {
  assert!(div_by_five(15));
  assert!(!div_by_five(14));
}

#[test]
fn test_div_by_fifteen() {
  assert!(div_by_fifteen(45));
  assert!(!div_by_fifteen(42));
}
