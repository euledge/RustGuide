fn main() {
    // 3.5.1
    let mut i = 1;
    loop {
        if i > 10 {
            break;
        }
        println!("again");
        i = i + 1;
    }

    // 3.5.2
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFT OFF");

    // 3.5.3

    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index = index + 1;
    }

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    for number in 4..1 {
        println!("{}!", number);
    }
}
