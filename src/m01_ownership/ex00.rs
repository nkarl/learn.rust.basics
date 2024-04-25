pub struct MyStruct {
    point: i32,
    tensor: Vec<i32>
}

pub fn ex00() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    let mut m: MyStruct = MyStruct(1, Vec<i32>::from(1, 2, 3)); // Vec is not `Copy`
}
