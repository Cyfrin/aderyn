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
        $self
            .found_instances
            .insert($context.get_node_sort_key(&$item.clone().into()), $item.id);
    };
}

#[macro_export]
macro_rules! capture_raw {
    ($self:ident, $context:ident, $item:expr) => {
        if let Some(item_id) = $item.id() {
            $self
                .found_instances
                .insert($context.get_node_sort_key(&$item.clone()), item_id);
        }
    };
}

pub use capture;
pub use capture_raw;
