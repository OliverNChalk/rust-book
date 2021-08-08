fn main() {
    println!("Hello, world!");

    let result;
    {
        let a = 25;
        let b = 50;
        result = largest_copy(a, b);

        println!("{} is the largest number", largest_references(&a, &b));
    }

    println!("{} is the largest number", result);
}

fn largest_copy(a: u32, b: u32) -> u32 {
    if a > b {
        return a;
    } else {
        return b;
    }
}

fn largest_references<'a>(a: &'a u32, b: &'a u32) -> &'a u32 {
    if *a > *b {
        return a;
    } else {
        return b;
    }
}
