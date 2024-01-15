use std::fmt::Debug;

fn main() {
    let mut total: i32 = 0;
    let array = [2, 4, 6, 1];
    println!("Given this array {:?}", array);
    for item in array {
        total += item;
    }
    println!("Add up all the numbers and return the total  {}", total);
    println!(
        "Recursive version:Add up all the numbers and return the total  {}",
        up_all_recursive(&array, 0)
    );
    println!(
        "Another Recursive version:Add up all the numbers and return the total  {}",
        up_all_another_recursive(&array)
    );
    println!("Recursive version:Count items  {}", count_recursive(&array));
    println!(
        "Recursive version:Maximum  {:?}",
        maximum_recursive(&array, 0)
    );
    println!(
        "AnotherRecursive version:Maximum  {:?}",
        maximum_recursive_without_accomulator(&array)
    );
    println!("Quicksort  {:?}", quick_short(&array));

    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

    println!("Previous {:?}", numbers);
    quick_sort_2(&mut numbers);
    println!("Ordered {:?}", numbers);
    let numbers_2 = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    println!("Previous {:?}", numbers_2);
    println!("Ordered {:?}", merge_sort(&numbers_2));
}

fn up_all_recursive(array: &[i32], total: i32) -> i32 {
    if array.is_empty() {
        total
    } else {
        return up_all_recursive(array.split_at(1).1, total + array.split_at(1).0[0]);
    }
}
fn up_all_another_recursive(array: &[i32]) -> i32 {
    if array.is_empty() {
        0
    } else {
        return array.split_at(1).0[0] + up_all_another_recursive(array.split_at(1).1);
    }
}
fn count_recursive(array: &[i32]) -> i32 {
    if array.is_empty() {
        0
    } else {
        return 1 + count_recursive(array.split_at(1).1);
    }
}

fn maximum_recursive<T: PartialOrd + Copy>(array: &[T], mut maximun: T) -> T {
    if array.is_empty() {
        maximun
    } else {
        let new_one = array.split_at(1).0[0];
        if new_one > maximun {
            maximun = new_one;
        }
        return maximum_recursive(array.split_at(1).1, maximun);
    }
}

fn maximum_recursive_without_accomulator<T: PartialOrd + Copy>(array: &[T]) -> T {
    if array.len() == 2 {
        if array[0] > array[1] {
            array[0]
        } else {
            array[1]
        }
    } else {
        let new_one = maximum_recursive_without_accomulator(array.split_at(1).1);
        if array[0] > new_one {
            array[0]
        } else {
            new_one
        }
    }
}

fn quick_short<T: PartialOrd + Copy>(array: &[T]) -> Vec<T> {
    if array.len() < 2 {
        array.to_vec()
    } else {
        let pivot = array[0];
        let mut less = Vec::new();
        let mut greater = Vec::new();
        for item in array.iter().skip(1) {
            if *item <= pivot {
                less.push(*item);
            } else {
                greater.push(*item);
            }
        }
        let mut sorted: Vec<T> = quick_short(&less);
        sorted.push(pivot);
        sorted.extend(quick_short(&greater));

        sorted
    }
}

fn quick_sort_2<T: PartialOrd + Copy>(array: &mut [T]) -> &[T] {
    if array.len() > 1 {
        let pivot_index = partition(array);
        println!("pivot:{}", pivot_index);
        quick_sort_2(&mut array[..pivot_index]);
        quick_sort_2(&mut array[pivot_index + 1..]);
    }
    array
}

fn partition<T: PartialOrd + Copy>(array: &mut [T]) -> usize {
    let pivot_index = array.len() / 2;
    array.swap(pivot_index, array.len() - 1);

    let mut i = 0;
    for j in 0..array.len() - 1 {
        if array[j] <= array[array.len() - 1] {
            array.swap(i, j);
            i += 1;
        }
    }

    array.swap(i, array.len() - 1);
    i
}

fn merge_sort<T: PartialOrd + Copy + Debug>(array: &[T]) -> Vec<T> {
    if array.len() < 2 {
        return array.to_vec();
    }
    let half_index = array.len() / 2;
    let  left_array = merge_sort(&array[..half_index]);
    let   right_array = merge_sort(&array[half_index ..]);
    //println!("Left {:?}",left_array);
   // println!("Right {:?}",right_array);
     merge( left_array,   right_array)
}

fn merge<T: PartialOrd + Copy>(mut array_left: Vec<T>, mut array_right: Vec<T>) -> Vec<T> {
    let mut sorted: Vec<T> = Vec::new();
    while !array_left.is_empty() && !array_right.is_empty(){ 
            if array_left.first() <= array_right.first() {
                sorted.push(array_left.first().copied().unwrap());
                array_left.remove(0);
              
            } else {
                sorted.push(array_right.first().copied().unwrap());
                array_right.remove(0);
                  
                
            }
       
    }
    while !array_left.is_empty() {
        sorted.push(array_left.first().copied().unwrap());
        array_left.remove(0);
    }
    while !array_right.is_empty() {
        sorted.push(array_right.first().copied().unwrap());
        array_right.remove(0);
    }   

    sorted
}
