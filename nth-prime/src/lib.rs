
pub fn nth(n: usize) -> Result<usize, String> {
    let mut vec = vec![2];
    let mut num = 3;
    if n <= 0 {
        return Err(String::new())
    }
    while vec.len() < n {
        if num > 10 && num % 10 == 5 {
            num += 2;
            continue;
        }
        let mut to_add = false;
        let mut is_interrupt = false;
        for el in &mut vec {
            let val = *el;
            if val*val - 1 > num {
                to_add = true;
                break;
            }
            if num % val == 0 {
                is_interrupt = true; 
                break;
            }
        }
        if to_add == true || (to_add == false && is_interrupt == false) {
            vec.push(num);
        }
        num += 2;
    }
    Ok(vec[n-1])
}
