pub fn adding<T>(x: T, y: T) -> T
    where T: PartialOrd + std::ops::Add {
        let result =  x + y;
        return result;
}

