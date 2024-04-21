pub trait DropExtern {
    /// Manually handles the release of resources, indicating that this must be managed
    /// externally to standard Rust `Drop` semantics. This should be used to ensure
    /// proper resource cleanup when the resource's lifecycle is not solely managed by Rust.
    fn drop(self: Box<Self>);
}
