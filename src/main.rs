extern crate rand;

mod types;
mod basic_ops;

use types::Bignum;

fn quicksort(slice: &mut [i32]) {
    let len = slice.len();
    if len <= 1 {
        return
    }
    if len == 2 {
        if slice[0] > slice[1] {
            slice.swap(0, 1);
        }
        return
    }
    
    let mut pivot = len / 2;
    let pivot_val = slice[pivot];
    slice.swap(pivot, len-1);
    pivot = 0;

    for i in 0..len-1 {
        if slice[i] < pivot_val {
            slice.swap(pivot, i);
            pivot += 1;
        }
    }

    slice.swap(pivot, len-1);
    
    quicksort(&mut slice[..pivot]);
    quicksort(&mut slice[pivot+1..]);
}

fn main() {
    let mut list: Vec<i32> = Vec::new();
    for _ in 0..30 {
        list.push(rand::random::<i32>());
    }
    quicksort(&mut list);
    println!("{:?}", list);
}
