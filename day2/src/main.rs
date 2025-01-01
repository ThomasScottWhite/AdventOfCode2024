use std::io;
enum Direction {
    Up,
    Down,
}

fn is_safe(list1: &Vec<i32>) -> i32 {
    let mut previous_number: i32 = list1[0];
    let mut direction = Direction::Up;

    if list1[0] > list1[1] {
        direction = Direction::Down;
    } else if list1[0] < list1[1] {
        direction = Direction::Up;
    }

    for &number in &list1[1..] {
        if let Direction::Up = direction {
            if number < previous_number {
                return 0;
            }
        } else if let Direction::Down = direction {
            if number > previous_number {
                return 0;
            }
        }
        if (previous_number - number).abs() > 3 {
            return 0;
        }
        if (previous_number - number).abs() < 1 {
            return 0;
        }

        previous_number = number;
    }
    return 1;
}

fn is_safe_with_damp(list1: &Vec<i32>) -> i32 {
    for (index, _number) in list1.iter().enumerate() {
        let mut copy = list1.clone();
        copy.remove(index);
        if is_safe(&copy) == 1 {
            return 1;
        }
    }
    return 0;
}

fn main() {
    let mut counter = 0;
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed To Read Line");

        let input = input.trim();

        if input.is_empty() {
            break;
        }

        let numbers: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Enter Valid Numbers"))
            .collect();

        counter = counter + is_safe_with_damp(&numbers);
    }
    print!("Counter: {} \n", counter);
}
