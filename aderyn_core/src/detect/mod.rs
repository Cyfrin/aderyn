pub mod detector;
pub mod high;
pub mod low;
pub mod medium;
pub mod nc;
pub mod reusable;

mod experimental; // This should not be publicly exported/exposed to nyth because of how useless the module really is :P

#[macro_export]
macro_rules! capture {
    ($self:ident, $context:ident, $item:expr) => {
        $self
            .found_instances
            .insert($context.get_node_sort_key(&$item.clone().into()), $item.id);
    };
}

pub use capture;
