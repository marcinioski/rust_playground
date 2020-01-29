pub trait OwnTrait {
    fn call1(&self) -> String;

    fn call2(&self) -> i32 {
        return 2;
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];

    for &ii in list.iter() {
        if ii > largest {
            largest = ii;
        }
    }

    largest
}

pub fn findLargest() {
    let list = vec![1, 2, 3, 4, 5];
    largest(&list);
}
