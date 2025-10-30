# Phantom Types: Theory

## Question 1: What is a PhantomData and why it exists?

It's a zero-sized marker type that tells the compiler that we "own" a type T but wihtout actual memory allocation.
The primary purpose of such functionality:

- Variance: to inform this type is a sub-type of another type we defined it for
- Drop check: when dropping the MyType<T>, the compiler checks through this phantom type what T we are referring to

## Question 2: Type-state Pattern

When building a state machine (in other words, a system where we deal with transitioning from one state to another),
phantom types assist in pseudo-allocating the state of a type, based on which we enable certain methods we can call.
Such availability is checked purely at compile-time.

### Example: Door State Machine
```rust
Door.unlock() -> Door  // Consumes Locked, produces Unlocked
Door.open()            // Only available on Unlocked
```

**Note** such methods MUST consume `self` -> otherwise, the type in a state that is not supposed to have a certain method - *will* have it
Consequence: Unnecessary logic bugs

## Question 3: Builder pattern evolution

If we were not to opt in for the phantom type to assert the API validity at compile-time, checks for API
calls at runtime, though seemingly looking easier and more convenient to build, can encounter
situations like returned errors at runtime. 

It's not as reliable to have to deal with the errors at runtime because if the app is scaled up - more resources 
could be eaten up - and that would continuously happen *as the program is still running* (not exactly the type of UX one wants to experience)

## Question 4: Key insights

If we have a case where we possess the ability to employ type-safety at compile-time, 
opt in for it as often as possible because not only it aligns with Rust's type safety principles, but your fellow devs who decide to use your API
have a significantly lower chance of misusing it.
