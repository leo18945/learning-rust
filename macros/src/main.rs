macro_rules! calc {
    (eval $e: expr) => {{
        {
            // force type to be integers
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
}

fn main() {
    calc! {
        // eval is not a rust keyword.
        eval 1 + 1
    }

    calc! {
        eval (1 + 2) * (3 / 1)
    }
}


