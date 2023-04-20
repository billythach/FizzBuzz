use crate::FizzBuzz::{Buzz, Fizz};

#[derive(Debug)]
enum FizzBuzz {
    Fizz,
    Buzz,
}

trait FizzBuzzNumberOperation {
    fn is_divisable_by_three(&self) -> bool;
    fn is_divisable_by_five(&self) -> bool;
    fn fizzbuzz(&self) -> String;
}

impl FizzBuzzNumberOperation for i32 {
    fn is_divisable_by_three(&self) -> bool {
        let res_div3 = self % 3;
        if res_div3 == 0 {
            return true;
        }
        return false;
    }

    fn is_divisable_by_five(&self) -> bool {
        let res_div5 = self % 5;
        if res_div5 == 0 {
            return true;
        }
        return false;
    }

    fn fizzbuzz(&self) -> String {
        match (self.is_divisable_by_three(), self.is_divisable_by_five()) {
            (true, true) => { return format!("{:?}{:?}", Fizz, Buzz) }
            (true, false) => { return format!("{:?}", Fizz) }
            (false, true) => { return format!("{:?}", Buzz) }
            (false, false) => { return format!("{:?}", self) }
        }
    }
}

fn main() {
    for x in 1..30 {
        println!("{} - {}" , x, x.fizzbuzz())
    }
}

#[cfg(test)]
mod tests {

    use crate::FizzBuzzNumberOperation;

    #[test]
    fn divisible_by_three_should_display_fizz() {
        assert_eq!(3.fizzbuzz(), "Fizz");
        assert_eq!(6.fizzbuzz(), "Fizz");
        assert_eq!(9.fizzbuzz(), "Fizz");
    }

    #[test]
    fn divisible_by_five_should_display_buzz() {
        assert_eq!(5.fizzbuzz(), "Buzz");
        assert_eq!(10.fizzbuzz(), "Buzz");
        assert_eq!(25.fizzbuzz(), "Buzz");
    }

    #[test]
    fn divisible_by_three_and_five_should_display_fizzbuzz() {
        assert_eq!(15.fizzbuzz(), "FizzBuzz");
        assert_eq!(30.fizzbuzz(), "FizzBuzz");
        assert_eq!(45.fizzbuzz(), "FizzBuzz");
    }

    #[test]
    fn not_divisible_by_three_or_five_should_number() {
        assert_eq!(1.fizzbuzz(), "1");
        assert_eq!(2.fizzbuzz(), "2");
        assert_eq!(11.fizzbuzz(), "11");
    }

}