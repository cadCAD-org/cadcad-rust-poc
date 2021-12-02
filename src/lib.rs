use std::{collections::BTreeMap, usize};
extern crate lazy_static;

pub type State<'a, T> = BTreeMap<&'a str, T>;
pub type UpdateFunc<T> = fn(&State<T>, &Signals<T>) -> Update<T>;
pub type PolicyFunc<'a, T> = fn(&State<T>) -> Signals<'a, T>;
pub type Signals<'a, T> = BTreeMap<&'a str, T>;

#[derive(Debug)]
pub struct SimConfig { 
    pub n_run: usize,
    pub timesteps: usize
}

pub struct StateKeyAndUpdateFn<T> {
    pub key: &'static str,
    pub update_func: UpdateFunc<T>
}

#[derive(Debug)]
pub struct Update<T> {
    pub key: &'static str,
    pub value: T
}

#[derive(Debug)]
pub struct Signal<T> {
    pub key: &'static str,
    pub value: T
}
