use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(i64)]
#[derive(Clone, Copy, Debug, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
pub enum MagicVariableID {
    Abi = -1,
    Addmod = -2,
    Assert = -3,
    Block = -4,
    Blockhash = -5,
    Ecrecover = -6,
    Gasleft = -7,
    Keccak256 = -8,
    Msg = -15,
    Mulmod = -16,
    Now = -17,
    Require = -18,
    Revert = -19,
    Ripemd160 = -20,
    Selfdestruct = -21,
    Sha256 = -22,
    Sha3 = -23,
    Suicide = -24,
    Super = -25,
    Tx = -26,
    Type = -27,
    This = -28,
}
