fn main() {
    let num_list = vec![34, 234, 22, 34, 123, 598];
    //let mut largest = num_list[0];

    //for num in num_list.iter() {
    //    if num > largest {
    //        largest = num;
    //    }
    //}
    //println!("The largest number is {}", largest);

    let result = get_largest(&num_list);
    println!("The largest number is {}", result);
}

fn get_largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
