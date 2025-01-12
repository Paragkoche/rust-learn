pub fn fun1(name: &str) {
    println!("{} !!", name)
}

pub fn fibonacci_number(number: i32) -> i32 {
    if number == 1 || number == 0 {
        return 1;
    }

    return fibonacci_number(number - 1) + fibonacci_number(number - 2);
}

pub fn factorial_number(number: i128) -> i128 {
    if number == 1 || number == 0 {
        return 1;
    }
    return number * factorial_number(number - 1);
}

pub fn sort_bobble(array: &[i128]) -> Vec<i128> {
    let mut array = array.to_vec();
    for i in 0..array.len() {
        for j in i + 1..array.len() {
            if array[i] > array[j] {
                array.swap(i, j);
            }
        }
    }
    array
}
fn middle_value(array: &mut Vec<i128>, low: usize, high: usize) -> usize {
    let pivot = array[high];
    let mut i = low as isize - 1; // Initialize i as isize to handle -1 case

    for j in low..high {
        if array[j] < pivot {
            i += 1;
            array.swap(i as usize, j);
        }
    }

    // Place the pivot in the correct position
    let i = (i + 1) as usize;
    array.swap(i, high);
    i
}

pub fn quick_sort(array: &mut Vec<i128>, low: usize, high: usize) {
    if low < high {
        let pi = middle_value(array, low, high);

        if pi > 0 {
            quick_sort(array, low, pi - 1); // Sort the left part
        }
        quick_sort(array, pi + 1, high); // Sort the right part
    }
}
pub fn liner_search(array: Vec<i128>, search_element: i128) -> Result<usize, &'static str> {
    for i in 0..array.len() {
        if array[i] == search_element {
            return Ok(i);
        }
    }
    return Err("element not found");
}

pub fn binary_search(array: Vec<i128>, search_element: i128) -> i128 {
    let mut low = 0;
    let mut high = array.len();
    while low < high {
        let mid = (low + high) / 2;
        if array[mid] == search_element {
            return mid as i128;
        } else if array[mid] < search_element {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    -1
}
