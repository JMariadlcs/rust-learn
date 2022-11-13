fn main() {
    println!("Hello, Median and Mode!");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7];
    let mean = mean(&numbers);
    println!("The mean of the vec {:?} is {}", &numbers, mean);

    let mediam_result = mediam(&numbers);
    println!("The mediam of the vec {:?} is {}", &numbers, mediam_result);

    let numbers2 = vec![1, 2, 3];
    let new_mediam = mediam(&numbers2);
    println!("The mediam of the vec {:?} is {}", &numbers2, new_mediam);
   
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
    let length = numbers.len();

    if length % 2  == 0 {
        let position_first = (length / 2) - 1;
        let position_second = length / 2;
        let vector = vec![numbers[position_first], numbers[position_second]];
        let mediam = mean(&vector);
        mediam as f64

    } else {
        let position = length / 2;
        numbers[position] as f64
    }
}
