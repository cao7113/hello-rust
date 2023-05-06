# Notes

* Ownership
Each value in Rust has an owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped

We call the action of creating a reference borrowing.

We also cannot have a mutable reference while we have an immutable one to the same value.

At any given time, you can have either one mutable reference or any number of immutable references.
References must always be valid.