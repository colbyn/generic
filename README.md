# Generics
> Higher level serialization of rust values to rust values, and from rust values.

Reusable stuff for mapping values to and from an intermediate serde-like data
format. That is actually something you can pattern match on.

For use cases that work with native values directly, and don’t involve parsing
from some textual or binary format.

Should be useful for people writing higher level abstractions upon e.g. some
DynamoDB SDK that handles HTTP/JSON serialization internally. Yet still
exposes a very tedious data format/api.


# Status

Currently just `IntoGeneric` is implemented. Eventually I would like to support
both `IntoGeneric` and perhaps some `FromGeneric` trait and deriving macro.


