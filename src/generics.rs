fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list[1..].to_owned() {
        if number > largest {
            largest = number
        }
    }
    largest
}

const HOLLE: u8 = 3;

pub fn execute() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("largest number {}", get_largest(number_list));
    println!("{}", HOLLE);
    let chrs_list = vec!["a", "b", "c"];
    println!("largest char {}", get_largest(chrs_list));
}
