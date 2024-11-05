#![no_std]
extern crate alloc;

/*
 * #[no_std] must remain and you are not allowed to use any external library.
 *
 * 1- what does it mean to use #[no_std] in a rust project and why may it be necessary?
 * 2- what does it mean to extern crate alloc; in a rust project and why may it be necessary?
 *
 * 3- define a trait that represents a Mex with the following functions where data and id are 
 *    generic  parameters:
 *    - new(id,length,data): return a new Mex
 *    - id(): given a Mex return its id
 *    - dlc(): given a Mex return its length
 *    - data(): given a Mex return its data
 *
 * 4- define a trait that represent the usage of a generic bus where
 *    - the trait has a generic parameter <P> with the constraint that <P> has to implement 
 *      the Mex trait
 *    - the following functions must exist:
 *          - new: initializes a new bus returning the object that implements the trait
 *          - send: use the object that implement the trait, take a <P> type and send it in the bus.
 *              return a Result with type () if Ok and Err(&str) if some errors occur
 *          - receive: use the object that implement the trait,
 *              return a <P> type populated with the data obtained from the bus
 *
 * 5- Implement the following protocols with the trait above:
 *    - CAN base (id 11-bit)
 *    - Serial
 *  For each implementation define some unit-testing where you define at least 
 *  one function for each function defined in the implementation
 *
 *  IF YOU HAVE AN AMD COMPUTER DELETE THE .cargo DIRECTORY OR CHANGE THE target accordingly
 *
 *
 *  Don't worry if you don't complete.
 */
