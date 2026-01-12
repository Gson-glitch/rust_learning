/*
    This file "claims" the bank/ folder.
    It simply makes files under banking/ folder discoverable by the compiler.
    Without it, we won't be able to use any structs, enums, methods in account.rs or ops.rs
*/
pub mod account;
pub mod ops;
