#![allow(dead_code)] // Integration binaries select only their relevant test setup.
use eve_interop::{
    AssetKind, AssetOrigin, ChainIdentity, ChainNamespace, CryptoCoverage, RouteManifest,
    RouteState, U256, VerificationRequirement, decode_chain_address,
};

pub fn network(namespace: ChainNamespace) -> ChainIdentity {
    ChainIdentity {
        namespace,
        genesis: [namespace as u8; 32],
        numeric_id: match namespace {
            ChainNamespace::Eve => Some(U256::from(31337)),
            ChainNamespace::Ethereum => Some(U256::from(1)),
            ChainNamespace::Solana => None,
        },
    }
}

pub fn named_routes() -> [RouteManifest; 4] {
    use ChainNamespace::{Ethereum, Eve, Solana};
    [
        (Ethereum, Eve),
        (Eve, Ethereum),
        (Solana, Eve),
        (Eve, Solana),
    ]
    .map(|(from, to)| {
        let source = network(from);
        let destination = network(to);
        let origin = if from == Eve { destination } else { source };
        RouteManifest {
            version: 1,
            route_id: [match (from, to) {
                (Ethereum, Eve) => 1,
                (Eve, Ethereum) => 2,
                (Solana, Eve) => 3,
                (Eve, Solana) => 4,
                _ => unreachable!(),
            }; 32],
            source,
            destination,
            source_custody: decode_chain_address(
                source,
                if from == Solana { &[4; 32] } else { &[4; 20] },
            )
            .unwrap(),
            destination_custody: decode_chain_address(
                destination,
                if to == Solana { &[5; 32] } else { &[5; 20] },
            )
            .unwrap(),
            asset: AssetOrigin {
                chain: origin,
                kind: AssetKind::Native,
            },
            source_decimals: if from == Solana { 9 } else { 18 },
            destination_decimals: if to == Solana { 9 } else { 18 },
            maximum_source_amount: U256::from(if from == Solana {
                1_000_000_000u64
            } else {
                1_000_000_000_000_000_000u64
            }),
            maximum_destination_amount: U256::from(if to == Solana {
                1_000_000_000u64
            } else {
                1_000_000_000_000_000_000u64
            }),
            verification: match from {
                Ethereum => VerificationRequirement::EthereumBeaconFinalityAndReceipt,
                Eve => VerificationRequirement::EveConsensusAndApplicationCommitment,
                Solana => VerificationRequirement::SolanaAuthenticatedReplay,
            },
            crypto_coverage: CryptoCoverage::Classical,
            state: RouteState::VerifierIncomplete,
        }
    })
}
