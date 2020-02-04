pub struct CustomSmartPointer <T> {
    data: T,
}

impl<T> Drop for CustomSmartPointer<T> {
    fn drop(&mut self) {
        println!("Dropping smart pointer!");
    }
}

pub fn test_dropping() {
    let c = CustomSmartPointer {data: String::from("hello")};
}
