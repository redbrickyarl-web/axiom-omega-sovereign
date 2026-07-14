//! Axiom Omega Protocol definitions

/// Protocol version
pub const PROTOCOL_VERSION: u32 = 1;

/// Core protocol message types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Intent,
    Cognition,
    GatedExecution,
}
