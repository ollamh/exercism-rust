
pub fn raindrops(n: usize) -> String {
    let my_n = n as i64;
    let sqrt_n = (n as f64).sqrt() as i64 + 1;
    let pling = "Pling";
    let plang = "Plang";
    let plong = "Plong";
    let mut output = String::new();
    for number in 1..sqrt_n {
        if my_n % number == 0 {
            println!("{} + {} are factors of {}", number, (my_n/number as i64), n);
            let other_number = my_n/number;
            if number == 3 || other_number == 3 {
                output = output + &pling;
            }
            if number == 5 || other_number == 5 {
                output = output + &plang;
            } 
            if number == 7 || other_number == 7{
                output = output + &plong;
            }
        }
    }
    if output.len() > 0 {
        return String::from(output);
    } else {
        return my_n.to_string();
    }
    
}
