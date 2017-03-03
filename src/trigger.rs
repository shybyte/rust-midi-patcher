pub trait Trigger {
    fn is_triggered(&self) -> bool;
}