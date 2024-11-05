# Embedded lesson

## How to start
To work on the project fork the repository in your own Raceup GitHub profile.

## Requirements
#[no_std] must remain, you are not allowed to use any external library.

## What to do
- What does it mean to use #[no_std] in a rust project, why may it be necessary?
- What does it mean to extern crate alloc; in a rust project and why may it be necessary?

- Define a trait that represents a Mex with the following functions where data and id are 
  generic parameters:
  - new( id, length, data): return a new Mex
  - id(): given a Mex return its id
  - dlc(): given a Mex return its length
  - data(): given a Mex return its data

- Define a trait that represent the usage of a generic bus where
  - the trait has a generic parameter <P> with the constraint that <P> has to implement 
    the Mex trait
  - the following functions must exist:
        - new: initializes a new bus returning the object that implements the trait
        - send: use the object that implement the trait, take a <P> type and send it in the bus.
            return a Result with type () if Ok and Err(&str) if some errors occur
        - receive: use the object that implement the trait,
            return a <P> type populated with the data obtained from the bus

- Implement the following protocols with the trait above:
  - CAN base (id 11-bit)
  - Serial
 For each implementation define some unit-testing where you define at least 
 one function for each function defined in the implementation

 Don't worry if you don't complete.

## Troubleshooting
 If you have an amd computer delete the .cargo directory or change the target accordingly


