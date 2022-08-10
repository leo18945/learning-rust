fn main() {
    test1();
    test2();
}

fn test1() {
    ;
    ;
    {
        ()
    }
    {
        ();
        use std::vec::Vec;
    }
    ();
    &{;}; // -> &()
        ; // -> ()
        ;
}

fn test2() {
    for i in 1..102 {
        if      i % 15 == 0 { println!("{} FizzBuzz", i) }
        else if i %  3 == 0 { println!("{} Fizz", i) }
        else if i %  5 == 0 { println!("{} Buzz", i) }
        else                { println!("{}", i) }
    }
}

fn test3() {
    let mut a = 1;
    let mut b = 1.1;
    let mut c = true;
    for i in 1..102 {
        if      i % 15 == 0 { println!("{} FizzBuzz", i) }
        else if i %  3 == 0 { a = 1; }
        else if i %  5 == 0 { b = 1.2; }
        else                { c = false; }
    }
}