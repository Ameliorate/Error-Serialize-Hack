# ErrorSer

Simple hack to serialize &Error values in rust, using either serde or rustc-serialize.

Note that due to the way errors are serialized, some custom ones with side effects and the like are improperly serialized, instead giving the descriptions and causes that were true at the time of serialization, instead of what is currently true. This is fine for most uses, but may cause unexpected behavior when serializing the more obscure errors.

#Links

TODO: Actually link to these things when they exist.

[Documentation](https://crates.fyi/crates/errorser/)  
[Crates.io](https://crates.io/crates/errorser/)

#Examples

```rust
use errorser::{deserialize_error_string, serialize_error_string};
let error = error_prone_function();
let string = serialize_error_string(&error);
let pseudoerror = deserialize_error_string(&string).unwrap();
```
