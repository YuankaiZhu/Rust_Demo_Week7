use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut array_size = String::new();

    println!("Enter the size of the array to be sorted:");
    std::io::stdin().read_line(&mut array_size).unwrap();

    let array_size: usize = array_size.trim().parse().unwrap();

    let mut array = (0..array_size).map(|_| rng.gen::<i32>()).collect::<Vec<_>>();

    let start_time = std::time::Instant::now();
    selection_sort(&mut array);
    let selection_sort_time = start_time.elapsed();

    let start_time = std::time::Instant::now();
    insertion_sort(&mut array);
    let insertion_sort_time = start_time.elapsed();

    let start_time = std::time::Instant::now();
    quicksort(&mut array, 0, array_size - 1);
    let quicksort_time = start_time.elapsed();

    let start_time = std::time::Instant::now();
    mergesort(&mut array);
    let mergesort_time = start_time.elapsed();

    println!("Selection sort time: {:?}", selection_sort_time);
    println!("Insertion sort time: {:?}", insertion_sort_time);
    println!("Quicksort time: {:?}", quicksort_time);
    println!("Mergesort time: {:?}", mergesort_time);
}

fn selection_sort(array: &mut [i32]) {
    for i in 0..array.len() {
        let mut min_index = i;
        for j in i+1..array.len() {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }
        array.swap(i, min_index);
    }
}

fn insertion_sort(array: &mut [i32]) {
    for i in 1..array.len() {
        let current_value = array[i];
        let mut j = i;
        while j > 0 && array[j-1] > current_value {
            array[j] = array[j-1];
            j -= 1;
        }
        array[j] = current_value;
    }
}

fn quicksort(array: &mut [i32], left: usize, right: usize) {
    if left < right {
        let pivot = partition(array, left, right);
        quicksort(array, left, pivot-1);
        quicksort(array, pivot+1, right);
    }
}

fn partition(array: &mut [i32], left: usize, right: usize) -> usize {
    let pivot = array[right];
    let mut i = left;
    for j in left..right {
        if array[j] < pivot {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, right);
    i
}

fn mergesort(array: &mut [i32]) {
    if array.len() <= 1 {
        return;
    }
    let mid = array.len() / 2;
    mergesort(&mut array[..mid]);
    mergesort(&mut array[mid..]);
    let mut temp = Vec::with_capacity(array.len());
    let (mut i, mut j) = (0, mid);
    while i < mid && j < array.len() {
        if array[i] < array[j] {
            temp.push(array[i]);
            i += 1;
        } else {
            temp.push(array[j]);
            j += 1;
        }
    }
    temp.extend(array[i..mid].iter());
    temp.extend(array[j..].iter());
    array.copy_from_slice(&temp);
}
