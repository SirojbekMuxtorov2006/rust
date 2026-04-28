fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u64;

    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

   true

}

fn count_primes_in_range(start: u64, end: u64)-> u64 {
    (start..end).filter(|&n| is_prime(n)).count() as u64
}

fn count_primes_single_thread(start: u64, end: u64) -> u64 {
    count_primes_in_range(start, end)
}

fn count_primes_multi_thread(start: u64, end: u64, num_threads: usize) -> u64 {
    let range_size = end - start;
    let chunk_size = range_size / num_threads as u64;



}

fn main() {
    println!("Hello, world!");
}
