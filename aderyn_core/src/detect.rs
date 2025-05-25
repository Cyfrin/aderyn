pub mod detector;
pub mod entrypoint;
pub mod helpers;
pub mod high;
pub mod low;

pub mod test_utils;

#[macro_export]
macro_rules! capture {
    ($self:ident, $context:ident, $item:expr) => {
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
    ($self:ident, $context:ident, $item:expr, $hint:tt) => {
        $self
            .hints
            .insert($context.get_node_sort_key_from_capturable(&$item.clone().into()), $hint);
        capture!($self, $context, $item);
    };
}

#[macro_export]
macro_rules! issue_detector {
    (
        $detector_struct:ident;

        severity: $detector_severity:ident,
        title: $detector_title:tt,
        desc: $detector_desc:tt,
        name: $detector_name:ident,

        |$context: ident| $e:expr
    ) => {

        #[derive(Default)]
        pub struct $detector_struct {
            found_instances: std::collections::BTreeMap<(String, usize, String), $crate::ast::NodeID>,
        }

        impl $crate::detect::detector::IssueDetector for $detector_struct {

            fn detect(&mut self, context: &$crate::context::workspace::WorkspaceContext) -> Result<bool, Box<dyn std::error::Error>> {

                let $context = context;

                macro_rules! grab {
                    ($item:expr) => {
                        if let Some(id) = context.get_node_id_of_capturable(&$item.clone().into()) {
                            self.found_instances.insert(
                                $context.get_node_sort_key_from_capturable(&$item.clone().into()),
                                id,
                            );
                        } else {
                            self.found_instances.insert(
                                $context.get_node_sort_key_from_capturable(&$item.clone().into()),
                                0,
                            );
                        }
                    };
                }

                $e
                Ok(!self.found_instances.is_empty())
            }

            fn severity(&self) -> $crate::detect::detector::IssueSeverity {
                $crate::detect::detector::IssueSeverity::$detector_severity
            }

            fn title(&self) -> String {
                String::from($detector_title)
            }

            fn description(&self) -> String {
                String::from($detector_desc)
            }

            fn instances(&self) -> std::collections::BTreeMap<(String, usize, String), $crate::ast::NodeID> {
                self.found_instances.clone()
            }

            fn name(&self) -> String {
                $crate::detect::detector::IssueDetectorNamePool::$detector_name.to_string()
            }
        }

    };
}

pub use capture;

pub use issue_detector;
