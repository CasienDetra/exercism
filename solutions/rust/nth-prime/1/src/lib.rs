pub fn nth(n: u32) -> u32 {
    fn is_prime(num: u32) -> bool {
        if num < 2 {
            return false;
        }
        !(2..=((num as f64).sqrt() as u32)).any(|divisor| num % divisor == 0) 
    }
    (2..).filter(|&x| is_prime(x)).nth(n as usize).expect("Prime should exist btw")
}
