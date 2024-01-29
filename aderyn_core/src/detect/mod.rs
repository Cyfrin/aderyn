pub mod detector;
pub mod high;
pub mod low;
pub mod medium;
pub mod nc;

#[macro_export]
macro_rules! capture {
    ($self:ident, $loader:ident, $item:expr) => {
        $self
            .found_instances
            .insert($loader.get_node_sort_key(&$item.clone().into()), $item.id);
    };
}

pub use capture;
