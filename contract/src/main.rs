#![no_std]
#![no_main]

extern crate alloc;

use alloc::{collections::btree_map::BTreeMap, string::ToString};
use casper_contract::{
    self,
    contract_api::{runtime::put_key, storage},
};
use casper_types::{EntryPoints, Key, NamedKeys};

#[no_mangle]
pub extern "C" fn call() {
    let entry_points = EntryPoints::new();

    // let mut named_keys = NamedKeys::new();

    //let message_topics = BTreeMap::default();

    // let (contract_hash, _version) = storage::new_contract(
    //     entry_points,
    //     Some(named_keys),
    //     Some("testing_contract_package".to_string()),
    //     Some("testing_contract_access".to_string()),
    //     Some(message_topics),
    // );

    // let (contract_hash, _version) = storage::new_contract(
    //     entry_points,
    //     None,
    //     Some("testing_contract_package".to_string()),
    //     Some("testing_contract_access".to_string()),
    //     None,
    // );

    let (contract_hash, _version) = storage::new_contract(entry_points, None, None, None, None);

    // put_key("testing_key", Key::from(contract_hash));
}
