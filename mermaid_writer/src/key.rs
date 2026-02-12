use crate::prelude::Render;
use std::hash::Hash;

pub trait Key: Eq + Hash + Clone + Render {}
