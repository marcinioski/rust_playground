macro_rules! say_hello {
    () => {
        println!("Hello");
    };
}

// export macro outside crate
#[macro_export]
macro_rules! own_vec {
    ( $( $x:expr ), *) => {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }
}

mod own_proc_macro;

fn main() {
    say_hello!();
    own_vec![1, 2, 3];
}
