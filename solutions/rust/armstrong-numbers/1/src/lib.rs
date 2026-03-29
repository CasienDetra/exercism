pub fn is_armstrong_number(num: u32) -> bool {
    fn extract_digits(num: u32,xs :&mut Vec<u32>){
        if num >= 10 {
            extract_digits(num / 10, xs);
        }
        xs.push(num % 10);
    }
    let mut digits = Vec::new();
    extract_digits(num, &mut digits);

    let power = digits.len() as u32;
    let mut sum : u32 = 0;

    for i in digits {
        match i.pow(power).checked_add(sum) {
            Some(val) => sum = val,
            None => return false
        }
    }
    sum == num
}

