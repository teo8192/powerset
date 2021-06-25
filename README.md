# Powerset

Implements a way to iterate over the powerset of some type.
Each type needs to have implemented `Index<usize>` and the trait `SizableContainer`, which should
in essence return the length of the container.

See the documentation with
```
cargo doc --open
```
