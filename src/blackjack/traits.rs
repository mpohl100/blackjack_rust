pub trait Allable{
    fn create_all() -> Vec<Self> where Self: Sized;
}

pub trait Stringable{
    fn to_string_internal(&self) -> String;
}