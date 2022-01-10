pub mod feature;
pub mod message;

use serde_derive::{Serialize, Deserialize};
use feature::HService;
use message::KType;

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct Doc1 {
    pub funcs: Vec<HService>,
    pub types: Vec<KType>,
}
