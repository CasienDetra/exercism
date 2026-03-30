pub fn is_leap_year(year: u64) -> bool {
    match (year % 4 == 0,year % 100 == 0,year % 400 == 0){
        (true,false,_) => true,
        (true,true,true) => true,
        _ => false,
    }
}
