use std::io::{self, BufRead, Write};

pub fn view_vec(a: &Vec<i32>) {
    let mut i = 0;
    for x in a {
        print!("{}:{}, ", i, x);
        i += 1;
    }
    println!();
}

pub fn read_line(index: &mut String) {
    io::stdin().read_line(index).expect("Failed to read line");
}

pub fn read_line_get_index_return_value() {
    let a = vec![1, 2, 3, 4, 5];
    view_vec(&a);

    io::stdout().flush().unwrap();
    print!("enter an index key to find a value in the vector: ");
    io::stdout().flush().unwrap();

    let mut index = String::new();
    read_line(&mut index);

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("the value at the index key {}: {}", index, element);
}
