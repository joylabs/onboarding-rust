pub fn valid_anagram(input_1: String, input_2: String) -> bool {
    let prime_numbers = generate_prime_numbers(30);

    for x in input_1.chars() {
        print!("{}", x);
        println!("{}", x as u32 - 96);
    }


    false
}

pub fn valid_anagram_2(input_1: String, input_2: String) -> bool {
    let mut vec_input_1: Vec<char> = input_1.chars().collect();
    let mut vec_input_2: Vec<char> = input_2.chars().collect();

    vec_input_1.sort();
    vec_input_2.sort();
    vec_input_1 == vec_input_2
}

fn generate_prime_numbers(last_number: i32) -> Vec<i32> {
    let mut prime_numbers = Vec::new();
    let mut count = 2;
    let mut prime = 2;
    let mut index = 0;

    while count <= last_number {
        prime_numbers.push(count);
        count += 1;
    }

    while prime <= (last_number as f32).sqrt() as i32 {
        prime = prime_numbers[index];
        prime_numbers = prime_numbers
            .into_iter()
            .filter(|x| x % prime != 0)
            .collect();

        prime_numbers.insert(index, prime);
        index += 1;

    }
    println!("{:?}", prime_numbers);
    prime_numbers
}
