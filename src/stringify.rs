trait Stringify {
    fn stringify(&self) -> Vec<String>;
}

impl<T> Stringify for Vec<T>
where
    T: ToString,
{
    fn stringify(&self) -> Vec<String> {
        self.iter().map(|e| e.to_string()).collect()
    }
}

fn main() {
    assert_eq!(vec![2, 1, 3, 7].stringify(), vec!["2", "1", "3", "7"]);
}
