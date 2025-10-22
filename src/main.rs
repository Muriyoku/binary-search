use std::fmt::Debug;
use std::time::Instant;

fn main() {
    /*
        One Billion of item in a List will make your machine crash! 
        Binary Search is better than a simple loop in small scenarios 
    */

    let num_list: Vec<i64> = (0..1_000_001).collect();
    
    mensure_time(|| {binary_search(&num_list, &1_000_000).unwrap();}, "binary_search");  
    mensure_time(|| {linear_search(&num_list, &1_000_000);}, "linear_search");
}

fn linear_search<T: Ord + Debug>(source: &[T], item: &T) -> Option<usize> {
  
    for (idx, n) in source.iter().enumerate() {
        if n == item {
            return Some(idx);
        };
    }

    None
}

fn binary_search<T: Ord + Debug>(source: &[T], item: &T) -> Option<usize> {
    if source.is_empty() { return None };

    let mut low: usize = 0;
    let mut high: usize = source.len().saturating_sub(1);
    
    while low <= high {
        let middle: usize = (low + high) / 2;
        let guess: &T = &source[middle];

        if guess == item { 
            return Some(middle) 
        } 
        else if  guess > item { 
            if middle == 0 { break; };
            high = middle - 1 
        } 
        else { 
            low = middle + 1 
        };
    };

    None
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