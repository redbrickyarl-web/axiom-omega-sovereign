//! Axiom Omega Sovereign - Complete AI Stack
//! 1.5-year collaboration with Grok, Claude, and Gemini

pub mod ring_buffer;
pub mod protocol;

pub use ring_buffer::RingBuffer;

/// Version
pub const VERSION: &str = "1.0.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        assert_eq!(VERSION, "1.0.0");
    }
}