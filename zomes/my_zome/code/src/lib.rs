#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    // entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
    link::LinkMatch,
};

use hdk::holochain_persistence_api::{
    cas::content::Address,
    cas::content::AddressableContent,
    hash::HashString,
};

use hdk::holochain_json_api::{
    error::JsonError,
    json::JsonString,
};

// see https://developer.holochain.org/api/0.0.25-alpha1/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct MyEntry {
    content: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct User {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct Commitment {
    title: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct GetUsersResponse {
    name: String,
    items: Vec<User>
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct GetCommitmentsResponse {
    name: String,
    items: Vec<Commitment>
}

pub fn handle_create_my_entry(entry: MyEntry) -> ZomeApiResult<Address> {
    let entry = Entry::App("my_entry".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_create_anchor() -> ZomeApiResult<Address> {
    let anchor = Entry::App("anchor".into(), "anchor".into());
    let anchor_address = hdk::commit_entry(&anchor)?;
    Ok(anchor_address)
}

pub fn handle_create_user(user: User) -> ZomeApiResult<Address> {
    let anchor = Entry::App("anchor".into(), "anchor".into());
    let anchor_address = anchor.address();

    let user = Entry::App("user".into(), user.into());
    let user_address = hdk::commit_entry(&user)?;
    hdk::link_entries(&anchor_address, &user_address, "users", "")?;
    Ok(user_address)
}

pub fn handle_create_commitment(commitment: Commitment, user_addr: HashString) -> ZomeApiResult<Address> {
    let commitment = Entry::App("commitment".into(), commitment.into());
    let commitment_address = hdk::commit_entry(&commitment)?;
    hdk::link_entries(&user_addr, &commitment_address, "commitments", "")?;
    Ok(commitment_address)
}

pub fn handle_get_my_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

pub fn handle_get_users() -> ZomeApiResult<GetUsersResponse> {
    let anchor = Entry::App("anchor".into(), "anchor".into());
    let anchor_address = anchor.address();

    // try and load the list items, filter out errors and collect in a vector
    let list_items = hdk::get_links(&anchor_address, LinkMatch::Exactly("users"), LinkMatch::Exactly(""))?.addresses()
        .iter()
        .map(|item_address| {
            hdk::utils::get_as_type::<User>(item_address.to_owned())
        })
        .filter_map(Result::ok)
        .collect::<Vec<User>>();

    // if this was successful then return the list items
    Ok(GetUsersResponse{
        name: String::from("users"),
        items: list_items
    })
}

pub fn handle_get_user_commitments(user_addr: HashString) -> ZomeApiResult<GetCommitmentsResponse> {
    // try and load the list items, filter out errors and collect in a vector
    let list_items = hdk::get_links(&user_addr, LinkMatch::Exactly("commitments"), LinkMatch::Exactly(""))?.addresses()
        .iter()
        .map(|item_address| {
            hdk::utils::get_as_type::<Commitment>(item_address.to_owned())
        })
        .filter_map(Result::ok)
        .collect::<Vec<Commitment>>();

    // if this was successful then return the list items
    Ok(GetCommitmentsResponse{
        name: String::from("commitments"),
        items: list_items
    })
}

define_zome! {
    entries: [
        entry!(
            name: "my_entry",
            description: "",
            sharing: Sharing::Public,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |validation_data: hdk::EntryValidationData<MyEntry>| {
                Ok(())
            }
        ),
        entry!(
            name: "anchor",
            description: "",
            sharing: Sharing::Public,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |validation_data: hdk::EntryValidationData<String>| {
                Ok(())
            },
            links: [
                to!(
                    "user",
                    link_type: "users",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |_validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        ),
        entry!(
            name: "user",
            description: "",
            sharing: Sharing::Public,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |validation_data: hdk::EntryValidationData<User>| {
                Ok(())
            },
            links: [
                to!(
                    "commitment",
                    link_type: "commitments",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |_validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        ),
        entry!(
            name: "commitment",
            description: "",
            sharing: Sharing::Public,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |validation_data: hdk::EntryValidationData<Commitment>| {
                Ok(())
            }
        )
    ]

    init: || { Ok(()) }

    validate_agent: |validation_data : EntryValidationData::<AgentId>| {
        Ok(())
    }

    functions: [
        create_my_entry: {
            inputs: |entry: MyEntry|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_my_entry
        }
        get_my_entry: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<Option<Entry>>|,
            handler: handle_get_my_entry
        }
        create_anchor: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_anchor
        }
        create_user: {
            inputs: |user: User|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_user
        }
        get_users: {
            inputs: | |,
            outputs: |result: ZomeApiResult<GetUsersResponse>|,
            handler: handle_get_users
        }
        create_commitment: {
            inputs: |commitment: Commitment, user_addr: HashString|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_commitment
        }
        get_user_commitments: {
            inputs: |user_addr: HashString|,
            outputs: |result: ZomeApiResult<GetCommitmentsResponse>|,
            handler: handle_get_user_commitments
        }
    ]

    traits: {
        hc_public [create_my_entry,get_my_entry,create_anchor,create_user,get_users,create_commitment,get_user_commitments]
    }
}
