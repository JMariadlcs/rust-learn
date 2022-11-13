use std::collections::HashMap;

fn main() {
    println!("Hello, Median and Mode!");
    let numbers = vec![1, 2, 7, 4, 3, 1, 7];
    let mean = mean(&numbers);
    println!("The mean of the vec {:?} is {}", &numbers, mean);

    let mediam_result = mediam(&numbers);
    println!("The mediam of the vec {:?} is {}", &numbers, mediam_result);

    let numbers2 = vec![3, 1, 2];
    let new_mediam = mediam(&numbers2);
    println!("The mediam of the vec {:?} is {}", &numbers2, new_mediam);

    let numbers3 = vec![3, 1, 2, 3 , 2 ,4, 3, 3];
    let new_mode = mode(&numbers3);
    println!("The mode of the vec {:?} is {}", &numbers3, new_mode);
   
}

// 1st: sum the elements on the vec
// 2nd: divide the sum by the length of the vec
fn mean(numbers: &[i32]) -> f64 {
    let mut sum = 0.0; 
    for num in numbers {
        sum += *num as f64;
    }
    sum / numbers.len() as f64
}

// 1st: sort the vector
// 2nd: return the middle number
fn mediam(numbers: &[i32]) -> f64 {

    let mut sorted_mumbers = numbers.to_vec();
    sorted_mumbers.sort();
    println!("Sorted vector is {:?}", sorted_mumbers);
    let length = sorted_mumbers.len();

    if length % 2  == 0 {
        let position_first = (length / 2) - 1;
        let position_second = length / 2;
        let vector = vec![sorted_mumbers[position_first], sorted_mumbers[position_second]];
        let mediam = mean(&vector);
        mediam as f64

    } else {
        let position = length / 2;
        sorted_mumbers[position] as f64
    }
}

// Compute the number that appears more in a vector
fn mode(numbers: &[i32]) -> i32 {
    let mut map = HashMap::new();

    for num in numbers {
        let count = map.entry(num).or_insert(0); // if number is present on the mapping -> use it. if not insert -> it on a mapping and assign value 0
        *count += 1;
    }
    // At this point we have a map of the count
    println!("Map of times {:?}", map);
    
    let mut max_value = 0;
    let mut mode = 0;
    for (key, value) in map {
        if value > max_value {
            max_value = value;
            mode = *key;
        }
    }
    mode 
}