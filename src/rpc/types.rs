use crate::serde::{H256AsRelaxedHexStr, U256AsBigDecimal};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use web3::types::{H256, U256};

/// Special tag used when specifying the latest block.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum Tag {
    #[serde(rename = "latest")]
    Latest,
}

/// A wrapper that contains either a block hash or the special `latest` tag.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum BlockHashOrTag {
    Hash(#[serde_as(as = "H256AsRelaxedHexStr")] H256),
    Tag(Tag),
}

/// A wrapper that contains either a block number or the special `latest` tag.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum BlockNumberOrTag {
    Number(#[serde_as(as = "U256AsBigDecimal")] U256),
    Tag(Tag),
}

/// Contains hash type wrappers enabling deserialization via `*AsRelaxedHexStr`.
/// Which allows for skipping leading zeros in serialized hex strings.
pub mod relaxed {
    use crate::serde::H256AsRelaxedHexStr;
    use serde::{Deserialize, Serialize};
    use serde_with::serde_as;
    use std::convert::From;
    use web3::types;

    #[serde_as]
    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    pub struct H256(#[serde_as(as = "H256AsRelaxedHexStr")] types::H256);

    impl From<types::H256> for H256 {
        fn from(core: types::H256) -> Self {
            H256(core)
        }
    }

    use std::ops::Deref;

    impl Deref for H256 {
        type Target = types::H256;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

/// Groups all strictly output types of the RPC API.
pub mod out {
    use serde::{Deserialize, Serialize};

    /// Describes Starknet's syncing status RPC reply.
    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    #[serde(untagged)]
    #[serde(deny_unknown_fields)]
    pub enum Syncing {
        False(bool),
        Status(syncing::Status),
    }

    pub mod syncing {
        use crate::serde::H256AsRelaxedHexStr;
        use serde::{Deserialize, Serialize};
        use serde_with::serde_as;
        use web3::types::H256;

        /// Represents Starknet node syncing status.
        #[serde_as]
        #[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
        #[serde(deny_unknown_fields)]
        pub struct Status {
            #[serde_as(as = "H256AsRelaxedHexStr")]
            starting_block: H256,
            #[serde_as(as = "H256AsRelaxedHexStr")]
            current_block: H256,
            highest_block: BlockStatus,
        }

        /// Represents highest block status.
        #[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
        #[serde(deny_unknown_fields)]
        pub enum BlockStatus {
            #[serde(rename = "PENDING")]
            Pending,
            #[serde(rename = "PROVEN")]
            Proven,
            #[serde(rename = "ACCEPTED_ONCHAIN")]
            AcceptedOnChain,
            #[serde(rename = "REJECTED")]
            Rejected,
        }
    }
}
