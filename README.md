# ErrorSer

Simple hack to serialize &Error values in rust, using either serde or rustc-serialize.

Note that due to the way errors are serialized, some custom ones with side effects and the like are improperly serialized, instead giving the descriptions and causes that were true at the time of serialization, instead of what is currently true. This is fine for most uses, but may cause unexpected behavior when serializing the more obscure errors.

#Links

TODO: Actually link to these things when they exist.

Documentation
Crates.io

#Examples

```
TODO: Add Examples.
```
