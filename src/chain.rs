#[cfg(not(feature = "liquid"))] // use regular Bitcoin data structures
pub use bitcoin::{
    address, blockdata::block::Header as BlockHeader, blockdata::script, consensus::deserialize,
    hash_types::TxMerkleNode, Address, Block, BlockHash, OutPoint, ScriptBuf as Script, Sequence,
    Transaction, TxIn, TxOut, Txid,
};

#[cfg(feature = "liquid")]
pub use {
    crate::elements::asset,
    elements::{
        address, confidential, encode::deserialize, script, Address, AssetId, Block, BlockHash,
        BlockHeader, OutPoint, Script, Sequence, Transaction, TxIn, TxMerkleNode, TxOut, Txid,
    },
};

use bitcoin::hashes::Hash;
pub use bitcoin::network::Network as BNetwork;

#[cfg(not(feature = "liquid"))]
pub type Value = u64;
#[cfg(feature = "liquid")]
pub use confidential::Value;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Serialize, Ord, PartialOrd, Eq)]
pub enum Network {
    #[cfg(not(feature = "liquid"))]
    Bitcoin,
    #[cfg(not(feature = "liquid"))]
    Testnet,
    #[cfg(not(feature = "liquid"))]
    Regtest,
    #[cfg(not(feature = "liquid"))]
    Signet,

    #[cfg(feature = "liquid")]
    Liquid,
    #[cfg(feature = "liquid")]
    LiquidTestnet,
    #[cfg(feature = "liquid")]
    LiquidRegtest,
}

impl Network {
    #[cfg(not(feature = "liquid"))]
    pub fn magic(self) -> u32 {
        u32::from_le_bytes(BNetwork::from(self).magic().to_bytes())
    }

    #[cfg(feature = "liquid")]
    pub fn magic(self) -> u32 {
        match self {
            Network::Liquid | Network::LiquidRegtest => 0xDAB5_BFFA,
            Network::LiquidTestnet => 0x62DD_0E41,
        }
    }

    pub fn is_regtest(self) -> bool {
        match self {
            #[cfg(not(feature = "liquid"))]
            Network::Regtest => true,
            #[cfg(feature = "liquid")]
            Network::LiquidRegtest => true,
            _ => false,
        }
    }

    #[cfg(feature = "liquid")]
    pub fn address_params(self) -> &'static address::AddressParams {
        // Liquid regtest uses elements's address params
        match self {
            Network::Liquid => &address::AddressParams::LIQUID,
            Network::LiquidRegtest => &address::AddressParams::ELEMENTS,
            Network::LiquidTestnet => &address::AddressParams::LIQUID_TESTNET,
        }
    }

    #[cfg(feature = "liquid")]
    pub fn native_asset(self) -> &'static AssetId {
        match self {
            Network::Liquid => &*asset::NATIVE_ASSET_ID,
            Network::LiquidTestnet => &*asset::NATIVE_ASSET_ID_TESTNET,
            Network::LiquidRegtest => &*asset::NATIVE_ASSET_ID_REGTEST,
        }
    }

    #[cfg(feature = "liquid")]
    pub fn pegged_asset(self) -> Option<&'static AssetId> {
        match self {
            Network::Liquid => Some(&*asset::NATIVE_ASSET_ID),
            Network::LiquidTestnet | Network::LiquidRegtest => None,
        }
    }

    pub fn names() -> Vec<String> {
        #[cfg(not(feature = "liquid"))]
        return vec![
            "mainnet".to_string(),
            "testnet".to_string(),
            "regtest".to_string(),
            "signet".to_string(),
        ];

        #[cfg(feature = "liquid")]
        return vec![
            "liquid".to_string(),
            "liquidtestnet".to_string(),
            "liquidregtest".to_string(),
        ];
    }
}

pub fn genesis_hash(network: Network) -> BlockHash {
    #[cfg(not(feature = "liquid"))]
    return bitcoin_genesis_hash(network.into());
    #[cfg(feature = "liquid")]
    return liquid_genesis_hash(network);
}

pub fn bitcoin_genesis_hash(network: BNetwork) -> bitcoin::BlockHash {
    match network {
        BNetwork::Bitcoin => BlockHash::from_byte_array(hex_literal::hex!(
            "1a91e3dace36e2be3bf030a65679fe821aa1d6ef92e7c9902eb318182c355691"
        )),
        BNetwork::Testnet => BlockHash::from_byte_array(hex_literal::hex!(
            "bb0a78264637406b6360aad926284d544d7049f45189db5664f3c4d07350559e"
        )),
        BNetwork::Regtest => BlockHash::from_byte_array(hex_literal::hex!(
            "3d2160a3b5dc4a9d62e7e66a295f70313ac808440ef7400d6c0772171ce973a5"
        )),
        BNetwork::Signet => BlockHash::from_byte_array(hex_literal::hex!(
            "9b7bce58999062b63bfb18586813c42491fa32f4591d8d3043cb4fa9e551541b"
        )),
        _ => panic!("unknown network {:?}", network),
    }
}

#[cfg(feature = "liquid")]
pub fn liquid_genesis_hash(network: Network) -> elements::BlockHash {
    use crate::util::DEFAULT_BLOCKHASH;

    lazy_static! {
        static ref LIQUID_GENESIS: BlockHash =
            "9b7bce58999062b63bfb18586813c42491fa32f4591d8d3043cb4fa9e551541b"
                .parse()
                .unwrap();
    }

    match network {
        Network::Liquid => *LIQUID_GENESIS,
        // The genesis block for liquid regtest chains varies based on the chain configuration.
        // This instead uses an all zeroed-out hash, which doesn't matter in practice because its
        // only used for Electrum server discovery, which isn't active on regtest.
        _ => *DEFAULT_BLOCKHASH,
    }
}

impl From<&str> for Network {
    fn from(network_name: &str) -> Self {
        match network_name {
            #[cfg(not(feature = "liquid"))]
            "mainnet" => Network::Bitcoin,
            #[cfg(not(feature = "liquid"))]
            "testnet" => Network::Testnet,
            #[cfg(not(feature = "liquid"))]
            "regtest" => Network::Regtest,
            #[cfg(not(feature = "liquid"))]
            "signet" => Network::Signet,

            #[cfg(feature = "liquid")]
            "liquid" => Network::Liquid,
            #[cfg(feature = "liquid")]
            "liquidtestnet" => Network::LiquidTestnet,
            #[cfg(feature = "liquid")]
            "liquidregtest" => Network::LiquidRegtest,

            _ => panic!("unsupported Bitcoin network: {:?}", network_name),
        }
    }
}

#[cfg(not(feature = "liquid"))]
impl From<Network> for BNetwork {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => BNetwork::Bitcoin,
            Network::Testnet => BNetwork::Testnet,
            Network::Regtest => BNetwork::Regtest,
            Network::Signet => BNetwork::Signet,
        }
    }
}

#[cfg(not(feature = "liquid"))]
impl From<BNetwork> for Network {
    fn from(network: BNetwork) -> Self {
        match network {
            BNetwork::Bitcoin => Network::Bitcoin,
            BNetwork::Testnet => Network::Testnet,
            BNetwork::Regtest => Network::Regtest,
            BNetwork::Signet => Network::Signet,
            _ => panic!("unknown network {:?}", network),
        }
    }
}
