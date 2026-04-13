use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if limit / 10 > factors.len() as u32 {
        sum_from_factors(limit, factors)
    } else {
        sum_from_range(limit, factors)
    }
}

fn sum_from_factors(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&&f| f != 0)
        .flat_map(|&f| (f..limit).step_by(f as usize))
        .collect::<HashSet<u32>>() // remove duplicates
        .into_iter()
        .sum()
}

fn sum_from_range(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|&n| {
            factors
                .iter()
                .any(|&f| f != 0 && n % f == 0)
        })
        .sum()
}
