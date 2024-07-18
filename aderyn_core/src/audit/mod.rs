// Main module
pub mod auditor;

// Helpers
pub mod investigators;

// AuditorDetectors
pub mod attack_surface;
pub mod delegate_call_no_address_checks;
pub mod public_functions_no_sender;
pub mod send_ether_no_checks;
