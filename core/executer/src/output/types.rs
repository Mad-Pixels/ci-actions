use serde::{Serialize, Deserialize};
use crate::error::ExecuterError;
use stf::fmt;

pub type Output<T> = Result<T, ExecuterError>;


