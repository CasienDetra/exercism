pub fn factors(n: u64) -> Vec<u64> {
    fn prime_rec(n:u64,divide:u64,acc: Vec<u64>) -> Vec<u64> {
        match n {
            1 => acc,
            _ if { n % divide == 0 } => {
                prime_rec(n / divide, divide, [acc, vec![divide]].concat())
            }
            _ => prime_rec(n, divide + 1,acc)
        }
    }
    if n < 2 {
        vec![]
    }
    else {
        prime_rec(n, 2, vec![])
    }
}
