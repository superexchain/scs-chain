// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

#![allow(unused_imports)]

use common_runtime::AccountId;
use hex_literal::hex;
use kitchensink_mainnet_runtime::{
    constants::currency::*, wasm_binary_unwrap, Block, MaxNominations, SessionKeys, StakerStatus,
};
use ecdsa_keyring::Keyring;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use polkadot_sdk::*;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
// use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_beefy::ecdsa_crypto::AuthorityId as BeefyId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::ecdsa;
use sp_core::{crypto::UncheckedInto, Pair, Public};
use sp_core::{H160, U256};
use sp_mixnet::types::AuthorityId as MixnetId;
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};
use sp_std::collections::btree_map::BTreeMap;
use std::str::FromStr;

pub use kitchensink_mainnet_runtime::RuntimeGenesisConfig;
pub use node_primitives::{Balance, Signature};

type AccountPublic = <Signature as Verify>::Signer;

// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const ENDOWMENT: Balance = 100 * DOLLARS;
const STASH: Balance = 50 * DOLLARS;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<Block>,
    /// The light sync state extension used by the sync-state rpc.
    pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;

pub fn scs_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../../res/scs-chain-spec.json")[..])
}
fn session_keys(
    // ed25519
    grandpa: GrandpaId,
    // sr25519
    babe: BabeId,
    // sr25519
    im_online: ImOnlineId,
    // sr25519
    authority_discovery: AuthorityDiscoveryId,
    // sr25519
    mixnet: MixnetId,
    // ecdsa
    beefy: BeefyId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
        mixnet,
        beefy,
    }
}

fn configure_accounts_for_staging_testnet() -> (
    Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
        MixnetId,
        BeefyId,
    )>,
    AccountId,
    Vec<AccountId>,
) {
    #[rustfmt::skip]
	// stash, controller, session-key, beefy id
	// generated with secret:
	// for i in 1 2 3 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	//
	// and
	//
	// for i in 1 2 3 ; do for j in session; do subkey inspect --scheme ed25519 "$secret"//fir//$j//$i; done; done
	//
	// and
	//
	// for i in 1 2 3 ; do for j in session; do subkey inspect --scheme ecdsa "$secret"//fir//$j//$i; done; done

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
		MixnetId,
		BeefyId,
	)> = vec![
		// 	1
		(
			AccountId::from(hex!("6816562B9589ccf4297952A8558A0451c1EB5aEc")),
			AccountId::from(hex!("6816562B9589ccf4297952A8558A0451c1EB5aEc")),

			// 5Ch6ttXs15pWBa4R2kB8RUiw7JHuChPANj6iXF1AZHQmz7cG
			array_bytes::hex2array_unchecked("1bc2636a29f850f49a69ccb416cd14812759651205d6b655298307f9365e9364")
				.unchecked_into(),

			// 5FnegENMjsr6sBnEpWHF9Ta7ARfwi6hkX97sxYeEamgsgxHf
			array_bytes::hex2array_unchecked("a4b24d7adcfd4cf21a4ab956359a973eefb385ffe4e16ec4ec78db49af296f77")
				.unchecked_into(),
			// 5FnegENMjsr6sBnEpWHF9Ta7ARfwi6hkX97sxYeEamgsgxHf
			array_bytes::hex2array_unchecked("a4b24d7adcfd4cf21a4ab956359a973eefb385ffe4e16ec4ec78db49af296f77")
				.unchecked_into(),
			// 5FnegENMjsr6sBnEpWHF9Ta7ARfwi6hkX97sxYeEamgsgxHf
			array_bytes::hex2array_unchecked("a4b24d7adcfd4cf21a4ab956359a973eefb385ffe4e16ec4ec78db49af296f77")
				.unchecked_into(),
			// 5FnegENMjsr6sBnEpWHF9Ta7ARfwi6hkX97sxYeEamgsgxHf
			array_bytes::hex2array_unchecked("a4b24d7adcfd4cf21a4ab956359a973eefb385ffe4e16ec4ec78db49af296f77")
				.unchecked_into(),

			// 5CcPsxwuGWDu1gb3WjMqio7u5WT7HmbtudaAbVoPUS8KWXU2
			array_bytes::hex2array_unchecked("03e6456ae62e2fd038fa90458fc76481bfc2b4eb9529a17891ecb10f745a06a5b6")
				.unchecked_into(),
		),

		// 2
		(
			AccountId::from(hex!("3E3d44a81C8773d7C27270E2f9830f25d87a5f41")),
			AccountId::from(hex!("3E3d44a81C8773d7C27270E2f9830f25d87a5f41")),
			
			// 5GzwG5sGrPevSYRcjWnw4JwkMD27dKTEXnxfunYPEyZjSzru
			array_bytes::hex2array_unchecked("da4d4e7d925a0a2134ba624f74be93577beab7052da6730e8b3e70e57c20b099")
				.unchecked_into(),
			
			// 5HKTL4YQLXHALyjp1aVRd9mgWgRAyhhPenVwJ9awiNa9Djvh
			array_bytes::hex2array_unchecked("e86cf1976adfb815826ab7c1f7ae02b053edb8199badd915fe7bfb4dbc01fd64")
				.unchecked_into(),
			// 5HKTL4YQLXHALyjp1aVRd9mgWgRAyhhPenVwJ9awiNa9Djvh
			array_bytes::hex2array_unchecked("e86cf1976adfb815826ab7c1f7ae02b053edb8199badd915fe7bfb4dbc01fd64")
				.unchecked_into(),
			// 5HKTL4YQLXHALyjp1aVRd9mgWgRAyhhPenVwJ9awiNa9Djvh
			array_bytes::hex2array_unchecked("e86cf1976adfb815826ab7c1f7ae02b053edb8199badd915fe7bfb4dbc01fd64")
				.unchecked_into(),
			// 5HKTL4YQLXHALyjp1aVRd9mgWgRAyhhPenVwJ9awiNa9Djvh
			array_bytes::hex2array_unchecked("e86cf1976adfb815826ab7c1f7ae02b053edb8199badd915fe7bfb4dbc01fd64")
				.unchecked_into(),
			
			// 5DJug6rkKrKDDokyhXRHJKT7ySniobtmcPT1iNY5adi9fEyR
			array_bytes::hex2array_unchecked("038f1ac9046212e3a43241044334cbfb77d5f2f3f162e125bad8189f9cfe573257")
				.unchecked_into(),
		),
	];

    let root_key: AccountId = AccountId::from(hex!("79BD79C274C845E8a29378513c0053b19395E863"));

    let endowed_accounts: Vec<AccountId> = vec![];
    (initial_authorities, root_key, endowed_accounts)
}

fn staging_testnet_config_genesis(chain_id: u32) -> serde_json::Value {
    let (initial_authorities, root_key, endowed_accounts) =
        configure_accounts_for_staging_testnet();
    let extra_endowed_accounts_balance = vec![(root_key.clone(), ENDOWMENT), ];
    testnet_genesis(
        initial_authorities,
        vec![],
        root_key,
        Some(endowed_accounts),
        extra_endowed_accounts_balance,
        chain_id,
    )
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
    let chain_id = 1970u32;
    ChainSpec::builder(wasm_binary_unwrap(), Default::default())
        .with_name("Super Smart Chain")
        .with_id("scs")
        .with_protocol_id("scs")
        .with_fork_id("scs")
        .with_properties(
            serde_json::from_str(
                "{\"isEthereum\": true, \"tokenDecimals\": 18, \"tokenSymbol\": \"SCS\"}",
            )
            .expect("Provided valid json map"),
        )
        .with_chain_type(ChainType::Live)
        .with_genesis_config_patch(staging_testnet_config_genesis(chain_id))
        .build()
}

/// Helper function to generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}


/// Helper function to generate stash, controller and session key from seed.
pub fn authority_keys_from_alice() -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
    MixnetId,
    BeefyId,
) {
    let seed = "Alice";
    (
        Keyring::Alith.into(),
        Keyring::Alith.into(),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
        get_from_seed::<MixnetId>(seed),
        get_from_seed::<BeefyId>(seed),
    )
}

fn configure_accounts(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
        MixnetId,
        BeefyId,
    )>,
    initial_nominators: Vec<AccountId>,
    endowed_accounts: Option<Vec<AccountId>>,
    stash: Balance,
) -> (
    Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
        MixnetId,
        BeefyId,
    )>,
    Vec<AccountId>,
    usize,
    Vec<(AccountId, AccountId, Balance, StakerStatus<AccountId>)>,
) {
    let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            Keyring::Alith.into(),
            Keyring::Baltathar.into(),
            Keyring::CharLeth.into(),
            Keyring::Dorothy.into(),
            Keyring::Ethan.into(),
            Keyring::Faith.into(),
        ]
    });
    // endow all authorities and nominators.
    initial_authorities
        .iter()
        .map(|x| &x.0)
        .chain(initial_nominators.iter())
        .for_each(|x| {
            if !endowed_accounts.contains(x) {
                endowed_accounts.push(x.clone())
            }
        });

    // stakers: all validators and nominators.
    // let mut rng = rand::thread_rng();
    let stakers = initial_authorities
        .iter()
        .map(|x| (x.0.clone(), x.1.clone(), stash, StakerStatus::Validator))
        .collect::<Vec<_>>();

    let num_endowed_accounts = endowed_accounts.len();

    (
        initial_authorities,
        endowed_accounts,
        num_endowed_accounts,
        stakers,
    )
}

/// Helper function to create RuntimeGenesisConfig json patch for testing.
pub fn testnet_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
        MixnetId,
        BeefyId,
    )>,
    initial_nominators: Vec<AccountId>,
    root_key: AccountId,
    endowed_accounts: Option<Vec<AccountId>>,
    extra_endowed_accounts_balance: Vec<(AccountId, u128)>,
    evm_chain_id: u32,
) -> serde_json::Value {
    let (initial_authorities, endowed_accounts, _num_endowed_accounts, stakers) =
        configure_accounts(
            initial_authorities,
            initial_nominators,
            endowed_accounts,
            STASH,
        );

    serde_json::json!({
        "balances": {
            "balances": endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).chain(extra_endowed_accounts_balance).collect::<Vec<_>>(),
        },
        "session": {
            "keys": initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(
                            x.2.clone(),
                            x.3.clone(),
                            x.4.clone(),
                            x.5.clone(),
                            x.6.clone(),
                            x.7.clone(),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        },
        "staking": {
            "validatorCount": initial_authorities.len() as u32,
            "minimumValidatorCount": initial_authorities.len() as u32,
            "invulnerables": initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
            "slashRewardFraction": Perbill::from_percent(10),
            "stakers": stakers.clone(),
        },

        "sudo": { "key": Some(root_key.clone()) },
        "babe": {
            "epochConfig": Some(kitchensink_mainnet_runtime::BABE_GENESIS_EPOCH_CONFIG),
        },

        "nominationPools": {
            "minCreateBond": 10 * DOLLARS,
            "minJoinBond": 1 * DOLLARS,
        },
        "evmChainId": { "chainId": evm_chain_id },
    })
}


#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::service::{new_full_base, NewFullBase};
    use sc_service_test;
    use sp_runtime::BuildStorage;

    /// Local testnet config (single validator - Alice).
    pub fn integration_test_config_with_single_authority() -> ChainSpec {
        ChainSpec::builder(wasm_binary_unwrap(), Default::default())
            .with_name("Integration Test")
            .with_id("test")
            .with_chain_type(ChainType::Development)
            .with_genesis_config_patch(testnet_genesis(
                vec![authority_keys_from_seed("Alice")],
                vec![],
                get_account_id_from_seed::<ecdsa::Public>("Alice"),
                None,
            ))
            .build()
    }

    /// Local testnet config (multivalidator Alice + Bob).
    pub fn integration_test_config_with_two_authorities() -> ChainSpec {
        ChainSpec::builder(wasm_binary_unwrap(), Default::default())
            .with_name("Integration Test")
            .with_id("test")
            .with_chain_type(ChainType::Development)
            .with_genesis_config_patch(local_testnet_genesis())
            .build()
    }

    #[test]
    #[ignore]
    fn test_connectivity() {
        sp_tracing::try_init_simple();

        sc_service_test::connectivity(integration_test_config_with_two_authorities(), |config| {
            let NewFullBase {
                task_manager,
                client,
                network,
                sync,
                transaction_pool,
                ..
            } = new_full_base::<sc_network::NetworkWorker<_, _>>(config, None, false, |_, _| ())?;
            Ok(sc_service_test::TestNetComponents::new(
                task_manager,
                client,
                network,
                sync,
                transaction_pool,
            ))
        });
    }

    #[test]
    fn test_create_development_chain_spec() {
        development_config().build_storage().unwrap();
    }

    #[test]
    fn test_create_local_testnet_chain_spec() {
        local_testnet_config().build_storage().unwrap();
    }

    // #[test]
    // fn test_staging_test_net_chain_spec() {
    // 	staging_testnet_config().build_storage().unwrap();
    // }
}
