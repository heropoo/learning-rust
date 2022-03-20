fn main() {
    // if 3 != 0 {
    //     println!("three");
    // }

    // let condition = true;

    // let number = if condition { 5.to_string() } else { "six".to_string() };

    // println!("The value of number is: {}", number);

    let mut count = 0;
    'condition_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaing = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'condition_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
