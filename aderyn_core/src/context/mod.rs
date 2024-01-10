pub mod browser;
pub mod loader;

#[macro_export]
macro_rules! capture {
    ($self:ident, $loader:ident, $item:expr) => {
        $self.found_instances.insert(
            $loader.get_node_sort_key(&$item.clone().into()),
            $item.src.clone(),
        );
    };
}
