pub trait NodeInterface {
    fn init() -> Self;

    fn step(&self);
}
pub struct Node {
    id: usize,
    node: String,
}
