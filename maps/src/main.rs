use std::collections::HashMap;

fn frequencies(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies: HashMap<i32, u32> = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();
    
    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}

fn main() {
    let example = vec![1,2,3,4,5,6,7,8,-5,3,4,-5];

    let frequencies = frequencies(example);

    println!("Frequency of each number in the example is: {:?}", frequencies);
}
