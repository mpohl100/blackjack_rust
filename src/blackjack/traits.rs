pub trait Allable{
    fn create_all() -> Vec<Self> where Self: Sized;
}