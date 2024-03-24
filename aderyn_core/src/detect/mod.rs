pub mod detector;
pub(crate) mod experimental;
pub mod high;
pub mod low;
pub mod medium;
pub mod nc;
pub mod reusable;

#[macro_export]
macro_rules! capture {
    ($self:ident, $context:ident, $item:expr) => {
        if let Some(id) = $context.get_node_id_of_capturable(&$item.clone().into()) {
            $self.found_instances.insert(
                $context.get_node_sort_key_from_capturable(&$item.clone().into()),
                id,
            );
        }
    };
}

pub use capture;
