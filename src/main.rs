use std::fmt::Debug;
use std::time::Instant;

fn main() {
    /*
        One Billion of item in a List will make your machine crash! 
        Binary Search is better than a simple loop in large list cases 
    */

    let num_list: Vec<i64> = (0..1_000_001).collect();
    
    mensure_time(|| {binary_search(&num_list, &1_000_000).unwrap();}, "binary_search");  
    mensure_time(|| {linear_search(&num_list, &1_000_000).unwrap();}, "linear_search");
}

fn linear_search<T: Ord + Debug>(source: &[T], item: &T) -> Result<usize, String> {
  
    for (idx, n) in source.iter().enumerate() {
        if n == item {
            return Ok(idx);
        };
    }

    Err(String::from("Item wasn't found")) 
}

fn binary_search<T: Ord + Debug>(source: &[T], item: &T) -> Result<usize, String> {
    if source.is_empty() { 
        return Err(String::from("The source list is empty")) 
    };

    let mut low: usize = 0;
    let mut high: usize = source.len().saturating_sub(1);
    
    while low <= high {
        let middle: usize = (low + high) / 2;

        if &source[middle] == item { 
            return Ok(middle) 
        } 
        else if  &source[middle] > item { 
            if middle == 0 { break; };
            high = middle - 1 
        } 
        else { 
            low = middle + 1 
        };
    };

    Err(String::from("Item wasn't found")) 
}


fn mensure_time<F: FnOnce()>(f: F, label: &str) {
    let start = Instant::now();

    f();

    let duration = start.elapsed();
    println!("The {} took {:?}", label, duration);
}

// Personal notes about methods and exercices

/*
    saturating_sub() method from num crate prevents underflow/overflow 
    
    let x: u8 = 5;
    let y: u8 = 10;
    let z: u8 = x - y; ❌ error! u8 cannot represent negative values 

    But with saturating_sub() it works fine

    let x: u8 = 5;
    let y: u8 = 10;
    let z: u8 = x.saturating_sub(y) ✅ instead of a negative value, it will return zero (the bound)
*/

/*
    Responses for the exercises of Binary Search

    1° it will need 7 steps using Binary Search
    2° it will neeed 8 steps using Binary Search
*/ 