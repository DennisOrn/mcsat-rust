pub trait Decidable<T> {
    fn get_id(&self) -> i32;
    fn get_value(&self) -> &T;
    fn set_value(&mut self, value: T);
}