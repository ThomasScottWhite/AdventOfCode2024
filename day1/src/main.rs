use std::collections::HashSet;
use std::io;

fn main() {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let stdin = io::stdin();
    let mut input = String::new();
    let mut flipflop = false;

    while let Ok(bytes_read) = stdin.read_line(&mut input) {
        if bytes_read == 0 {
            break;
        }
        for word in input.split_whitespace() {
            if let Ok(num) = word.parse::<i32>() {
                if flipflop == false {
                    list1.push(num);
                    flipflop = true;
                } else {
                    list2.push(num);
                    flipflop = false
                }
            } else {
                eprintln!("Error '{}' is not a valid integer", word);
            }
        }
        // This is apparently a better way to do this, I think this solution is unreadable garbage with almost no noticable improvement
        // input.split_whitespace()
        // .enumerate()
        // .filter_map(|(index, word)| word.parse::<i32>().ok().map(|num| (index, num)))
        // .for_each(|(index, num)| {
        //     if index % 2 == 0 {
        //         list1.push(num);
        //     } else {
        //         list2.push(num);
        //     }
        // });

        input.clear();
    }
    list1.sort();
    list2.sort();

    let sum_total: i32 = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!(
        "The total difference between the two lists is {}",
        sum_total
    );

    let set1: HashSet<_> = list1.iter().collect();
    let similarity_score: i32 = list2.iter().copied().filter(|num| set1.contains(num)).sum();

    println!("The simularity score is {}", similarity_score);
}

fn _old_main() {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let stdin = io::stdin();
    let mut input = String::new();
    let mut flipflop = false;

    let mut sum_total = 0;
    while let Ok(bytes_read) = stdin.read_line(&mut input) {
        if bytes_read == 0 {
            break;
        }
        for word in input.split_whitespace() {
            if let Ok(num) = word.parse::<i32>() {
                if flipflop == false {
                    list1.push(num);
                    flipflop = true;
                } else {
                    list2.push(num);
                    flipflop = false
                }
            } else {
                eprintln!("Error '{}' is not a valid integer", word);
            }
        }
        input.clear();
    }
    list1.sort();
    list2.sort();

    for (number1, number2) in list1.iter().zip(list2.iter()) {
        sum_total += (number1 - number2).abs();
        // println!("{}, {}", number1, number2);
    }
    println!(
        "The total difference between the two lists is {}",
        sum_total
    );

    let mut simularity_score = 0;
    for &number1 in &list1 {
        for &number2 in &list2 {
            if number1 == number2 {
                simularity_score += number1
            }
        }
    }
    println!("The simularity score is {}", simularity_score);
}
