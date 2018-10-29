use std::io;

fn main() {
    println!("Which Fibonacci number would you like to find?");
    let mut fib_num = String::new();
    io::stdin()
        .read_line(&mut fib_num)
        .expect("Failed to read line");
    let fib_num: u32 = fib_num
        .trim()
        .parse()
        .expect("Please enter a positive number");
    let number: u64 = find_fibonacci(fib_num);
    println!("The number {} in Fibonacci Sequence = {}", fib_num, number);
}

fn find_fibonacci(number: u32) -> u64 {
    if number == 0 {
        return 0;
    }
    if number == 1 {
        return 1;
    }
    let mut n = 1;
    let mut seq: Vec<u64> = vec![0, 1];
    while n < number {
        let len = seq.len();
        let idx_a = len.checked_sub(1);
        let idx_b = len.checked_sub(2);
        let a = idx_a.and_then(|idx| seq.get(idx).cloned());
        let b = idx_b.and_then(|idx| seq.get(idx).cloned());
        let sum = a.unwrap_or(0) + b.unwrap_or(0);
        seq.push(sum);
        n = n + 1;
    }
    println!("Fibonacci Sequence {:?}", seq);
    return seq.last().cloned().unwrap_or(0);
}

#[cfg(test)]
mod tests {
    use find_fibonacci;
    #[test]
    fn it_returns_correct_number() {
        assert_eq!(21, find_fibonacci(8));
        assert_eq!(55, find_fibonacci(10));
        assert_eq!(144, find_fibonacci(12));
    }
}
