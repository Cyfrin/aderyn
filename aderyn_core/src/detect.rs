pub mod detector;
pub mod entrypoint;
pub mod helpers;
pub mod high;
pub mod low;

pub mod test_utils;

#[macro_export]
macro_rules! capture {
    ($self:ident, $context:ident, $item:expr_2021) => {
        if let Some(id) = $context.get_node_id_of_capturable(&$item.clone().into()) {
            $self
                .found_instances
                .insert($context.get_node_sort_key_from_capturable(&$item.clone().into()), id);
        } else {
            $self
                .found_instances
                .insert($context.get_node_sort_key_from_capturable(&$item.clone().into()), 0);
        }
    };
    ($self:ident, $context:ident, $item:expr_2021, $hint:tt) => {
        $self
            .hints
            .insert($context.get_node_sort_key_from_capturable(&$item.clone().into()), $hint);
        capture!($self, $context, $item);
    };
}

pub use capture;
