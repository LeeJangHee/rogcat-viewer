pub trait Mapper<T>
where
    T: Clone,
{
    fn map_to(data: T) -> Self;
}
