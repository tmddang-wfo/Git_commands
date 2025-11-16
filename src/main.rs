
fn main() {
    let x:u32 = 10;
    assert_eq!("u32".to_string(), type_of(&x));
    println!("Success!")
}

fn type_of<T>(_:&T) -> String{
    format!("{}", std::any::type_name::<T>())
}



