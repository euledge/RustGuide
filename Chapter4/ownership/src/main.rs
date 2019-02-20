fn main() {
    // 4.1.5.1 : move
    {
        let s1 = String::from("Hello");
        let s2 = s1;
        println!("{}!", s2);
        // 所有権がs2に移っているのでs1は参照できない
        //println!("{}!", s1); 
    }

    // 4.1.5.2 : clone
    {
        let s1 = String::from("Hello");
        println!("{}!", s1);
        // cloneを使用することで別のメモリにディープコピーされ、使用することができる
        let s2 = s1.clone();
        println!("{}!", s2);
        println!("{}!", s1);
    }

    //4.1.5.3 : copy
    {
        let x = 5;
        println!("x is: {}!", x);
        let y = x;
        println!("y is: {}!", y);
        // スカラ型はスタック上でコピーされるので参照可能
        println!("x is: {}!", x);
    }

    // 4.1.6
    {
        let s = String::from("Hello!");

        takes_ownership(s);
        // 関数の引数に渡すことで所有権が移ってしまう。
        // println!("{}", s);
        let x = 5;

        makes_copy(x);
        println!("The Value is x: {}", x);
    }

    // 4.1.7
    {
        let s = String::from("Hello World!");
        let len = calculate_length(&s);
        println!("{} length is {}", s, len);
    }

    {
        let mut s = String::from("Hello!");
        s = return_ownership(s);
        println!("{}", s);
    }

    {
        let s = String::from("Hello!");
        let len = reference_ownership(&s);
        println!("{} length is {}", s, len);
        borrow_ownership(&s);
    }

    {
        let mut s = String::from("Hello!");
        borrow_mutable_ownership(&mut s);
        println!("{}", s);
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(number: i32) {
    println!("{}", number);
}

fn calculate_length(s: &String) -> usize {
    //s.push_str("string: &str");
    s.len()
}

fn return_ownership(some_string: String) -> String {
    some_string
}

fn reference_ownership(some_string: &String) -> usize {
    println!("referenced {}", some_string);
    some_string.len()
}

fn borrow_ownership(some_string: &String)  {
//    some_string.push_str(", World!");  
}

fn borrow_mutable_ownership(some_string: &mut String)  {
    some_string.push_str(", World!");
}
