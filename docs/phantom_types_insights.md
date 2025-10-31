# Phantom Types: Insights

## When to use?

If you are working with state machines, or need to build a certain piece of logic that relies on states as the building progresses, 
the compile-time checks on the typewe work on is the way to go - no runtime cost, safe API usage, no additional memory
allocations for the type states

## Trade-offs

While the newtype pattern or builder patterns are essentially bullet-proof API designs, they can take up a bit more time to build,
especially if the app is progressively scaling upwards. However, the long-term costs are significatly lower
than of those deisgns that rely primarily on runtime check s(i.e.: checking if a certain call returns an Error form the Result type - needs separate handling,
and can take up more resource and time usage - thus potentially lowering the UX and DX)

## Key types learned

- Length-based vector: apply the vector length as phantom state that dictates the call to `.zip()`
can only work with two `Lengthvec`'s with their data vectors of same length

- Type-state machine: a simple newtype pattern that builds API on working with files that are either closed or open - for reading content from.
The compile-time check guarantees that we cannot read content from a closed file

- Builder-pattern: a slightly more sophisticated approach to the same essential idea when it came to building an HTTP Request.
It has to ensure the presence of a URL and an HTTP method - otherwise, the `.build()` will not be accessible,
even if it has the headers and / or the body

## Personal pit-falls avoided

We do not need an Option<T> as fields in the builder - as our builder patterns already guarantees the presence of the values in the field, so
then no need to call `.unwrap()` on the field value types in the final output - even if it is safe in the API design.

### Extra tip: when NOT to use Phantom types

- Simple data structures that don't have "invalid states"
- When the type complexity would confuse users more than help
- Dynamic state that changes frequently at runtime
