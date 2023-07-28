use std::time::Instant;

fn calculate_average(numbers: &[f64]) -> Option<f64> {
    let sum: f64 = numbers.iter().sum();
    let count = numbers.len() as f64;

    if count > 0.0 {
        Some(sum / count)
    } else {
        None // Return None if the vector is empty to avoid division by zero
    }
}

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for num in 2..=limit {
        if is_prime[num] {
            primes.push(num);
            for multiple in (num * num..=limit).step_by(num) {
                is_prime[multiple] = false;
            }
        }
    }

    primes
}

fn main() {
    let mut time = vec![];

    for _ in 0..50 {
        let limit = 10_000_000; // Change this value for a longer or shorter benchmark
        let start_time = Instant::now();
        sieve_of_eratosthenes(limit);
        let elapsed_time = start_time.elapsed().as_millis() as f64;
        time.append(&mut vec![elapsed_time]);
    }

    let mut time_total: f64 = 0.0;
    for milliseconds in time.iter() {
        time_total += milliseconds;
    }
    println!("Total: {}", time_total);
    if let Some(average) = calculate_average(&time) {
        println!("Average: {}", average);
    } else {
        println!("The vector is empty.");
    }
}
