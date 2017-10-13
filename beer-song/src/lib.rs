extern crate itertools;

use itertools::join;

pub fn verse(n: i32) -> String {
    match n {
        0 => { 
            "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
        }
        1 => { 
            "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string()
        }
        _ => {
            let bottles = match n-1 {
                0 => "no more bottles".to_string(),
                1 => "1 bottle".to_string(),
                _ => format!("{} bottles", n-1)
            }; 
            let it = match n-1 {
                0 => "it".to_string(),
                _ => "one".to_string()
            };
            format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake {it} down and pass it around, {bottles} of beer on the wall.\n", n=n, it=it, bottles=bottles)
        }
    } 
}

pub fn sing(start: i32, end: i32) -> String {
    let mut output = Vec::new();
    for i in (end..start+1).rev() {
        let verse_result = verse(i);
        output.push(verse_result);
    }
    return join(&output, "\n");
}
