use ::common::entry::*;

pub trait SourceChainInterface {
    fn get(h: Hash) -> Entry;
    fn get_header(h: Hash) -> Hash;
}

#[derive(Clone, Debug, PartialEq)]
pub struct SourceChain {

}

impl SourceChain {
    pub fn push(_e: Entry) {

    }
    //fn serialize() -> str {}
    //fn deseriealize(input: str) {}
}
