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
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(number: i32) {
    println!("{}", number);
}