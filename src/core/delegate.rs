//! Delegate pattern - Type-safe customization for components

/// Trait for component delegates
///
/// Delegates allow customization of component behavior without subclassing.
/// Each delegate type gets a unique Objective-C class.
pub trait ComponentDelegate: Sized + 'static {
    /// The name of this delegate (used for Objective-C class naming)
    const NAME: &'static str;

    /// Called after the component is loaded and initialized
    fn did_load(&mut self, _component: &Self) {
        // Default: do nothing
    }
}

/// Example delegate for Button
pub struct ButtonDelegate;

impl ComponentDelegate for ButtonDelegate {
    const NAME: &'static str = "ButtonDelegate";

    fn did_load(&mut self, _component: &Self) {
        // Customize button after creation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delegate_trait() {
        assert_eq!(ButtonDelegate::NAME, "ButtonDelegate");
    }

    #[test]
    fn test_delegate_did_load() {
        let mut delegate = ButtonDelegate;
        let other = ButtonDelegate;
        delegate.did_load(&other);
    }
}
