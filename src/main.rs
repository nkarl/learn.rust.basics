mod m00_assorted;

fn main() {
    //println!("Hello, world!");
    let m00_add = m00_assorted::add(2, 3);
    println!("{}", m00_add);

    m00_assorted::read_line_get_index_return_value();
}
