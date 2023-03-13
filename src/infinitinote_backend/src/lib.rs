
use ic_cdk::{
    api::call,
    api::call::ManualReply,
    api::call::CallResult,
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    },
};

use ic_cdk_macros::*;
use std::str;
use std::option::Option;
use std::string::String;
use std::cell::RefCell;
use std::collections::BTreeMap;
use sha256::{digest, try_digest};
use egui::widgets::text_edit::TextEditState;
use derive_more::{Display};

///// Storage /////
type IdStore = BTreeMap<String, Principal>;
type CredentialsStore = BTreeMap<String, String>;
type ProfileStore = BTreeMap<Principal, User>;
type AssetStore = BTreeMap<AssetID, Asset>;
type NotebookStore = BTreeMap<Principal, BTreeMap<UUID, Notebook>>;
type UserAssetStore = BTreeMap<Principal, AssetID>;

/// type definitions ////
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct User
{
    pub name: String,
    pub email: String,
    password: String,
    pub description: String
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Display)]
#[display(fmt = "{}", _0)]
pub struct UUID(pub String);

#[derive(Clone)]
struct Note
{
    pub id: UUID,
    pub title: String,
    pub content: TextEditState,
    pub attachments: [AssetID; 128],
    pub tags: Vec<String>
}

#[derive(Clone)]
struct Notebook 
{
    pub id: UUID,
    pub title: String,
    pub notes: Vec<Note>,
    pub tags: Vec<String>
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssetID(pub String);

#[derive(Clone, Debug)]
struct Asset 
{
    pub id: AssetID,
    pub filename: String,
    pub bytes: Vec<u8>
}

struct Error
{
    pub error_type: String,
    pub error_description: String
}

thread_local! {
    static PROFILE_STORE: RefCell<ProfileStore>         = RefCell::default();
    static ID_STORE: RefCell<IdStore>                   = RefCell::default();
    static CREDENTIALS_STORE: RefCell<CredentialsStore> = RefCell::default();
    static ASSET_STORE: RefCell<AssetStore>             = RefCell::default();
    static USER_ASSET_STORE: RefCell<UserAssetStore>    = RefCell::default();
    static NOTEBOOK_STORE: RefCell<NotebookStore>       = RefCell::default();
}

#[query]
fn greet(name: String) -> String {
    format!("Hello Jello, {}!", name)
}

#[query]
fn test_greet(name: String) -> String {
    let g = format!("Hello, {}!", name);
    return g.to_string();
}

#[query]
fn test_principal() -> String {
    let mut principal_id = ic_cdk::api::caller().clone();
    let g = format!("Hello, {}", principal_id);
    return g.to_string();
}

#[query]
fn test_greet_() -> String {
    let g = "GREETINGS EARTHLINGS";
    return g.to_string();
}

////////////////////////////////////////////////////
// User Registration & Login                    ////
////////////////////////////////////////////////////

#[update(name="signUpWithUserPassword")]
fn signup_with_user_password(email: String, password: String) -> bool
{
    CREDENTIALS_STORE.with(|credentials_store| {
        credentials_store
            .borrow_mut()
            .insert(email, password)
    });

    // todo: catch errors

    return true;
}

#[query(name="loginWithUserPassword")]
fn login_with_user_password(email: String, password: String) -> bool
{
    let user_credential = CREDENTIALS_STORE.with(|credentials_store| -> bool {
        credentials_store
            .borrow()
            .get(&email)
            .and_then(|userpw| {
                if userpw.eq(&password)
                {
                    return Some(true)
                }
                else 
                {
                    return None;
                }
            });
        return false;
    });

    return user_credential;
}

async fn generate_uuid() -> String
{
    let mut val = String::from("error");

    let management_canister = ic_cdk::export::Principal::management_canister();
    let rnd_buffer: (Vec<u8>,) = match ic_cdk::call(management_canister, "raw_rand", ()).await {
        Ok(result) => result,
        Err(err) => {
            ic_cdk::println!("Error invoking raw_rand: {:?} {}", err.0, err.1);
            return val;
        }
    };

    let rnd = Ok::<std::vec::Vec<u8>, &str>(rnd_buffer.0);

    let rnd_string = String::from_utf8(rnd.ok().unwrap());
    if rnd_string.is_ok()
    {
        let rnd_hash = digest(rnd_string.ok().unwrap());
        val = rnd_hash;
    }
    
    return val;
}

#[update(name="createNotebook")]
async fn create_notebook(title: String) -> String
{
    let mut principal_id = ic_cdk::api::caller().clone();
    let notebook_id = UUID(generate_uuid().await);

    NOTEBOOK_STORE.with(|notebook_store| {
        let mut notebook_store_mut = notebook_store.borrow_mut();
        let notebook = Notebook {
            id: notebook_id.clone(),
            title: title,
            notes: Vec::<Note>::new(),
            tags: Vec::<String>::new()
        };

        let principal_notebook_map = notebook_store_mut.get(&principal_id);

        if principal_notebook_map.is_none()
        {
            let mut notebook_map = BTreeMap::<UUID, Notebook>::new();
            notebook_map.insert(notebook.id.clone(), notebook);

            notebook_store_mut.insert(principal_id, notebook_map);
        }
        else if principal_notebook_map.is_some()
        {
            let principal_notebooks_map_mut = notebook_store_mut.get_mut(&principal_id);
            let notebooks_map = principal_notebooks_map_mut.unwrap();
            notebooks_map.insert(notebook_id.clone(), notebook.clone());
        }
        
    });

    return notebook_id.to_string();
}

#[update(name="updateNotebookTags")]
async fn update_notebook_tags(notebook_id: String, tags: Vec<String>) -> Result<String, String>
{
    let principal_id = ic_cdk::api::caller();
    let nid = UUID(notebook_id);
    let mut error_condition = false;

    NOTEBOOK_STORE.with(|notebook_store| {
        let mut notebook_store_mut = notebook_store.borrow_mut();
        let principal_notebooks = notebook_store_mut.get(&principal_id);

        if principal_notebooks.is_none()
        {
            error_condition = true;
        }

        if principal_notebooks.is_some()
        {
            let notebooks_mut = notebook_store_mut.get_mut(&principal_id).unwrap();
            
            let notebook = notebooks_mut.entry(nid);
            notebook.and_modify(|n| 
                n.tags = tags 
            );
        }

    });

    if error_condition
    {
        return Err("Invalid notebook ID".to_string());
    }

    return Ok("Updated Tags".to_string())
}

////////////////////////////////////////////////////
// File Upload                                  ////
////////////////////////////////////////////////////

#[query(name="getNewAssetID")]
async fn get_new_asset_id() -> String
{
    let new_asset_id = generate_uuid().await;
    return new_asset_id;
}

#[update(name="uploadFileChunk")]
fn upload_file_chunk(asset_id: String, name: String, mut data: Vec<u8>) -> ManualReply<Option<String>>
{
    let mut error_condition = false;
    let mut _error_message = "";

    let principal_id = ic_cdk::api::caller();
        
        let asset_uuid = AssetID(asset_id);

        ASSET_STORE.with(|asset_store| {
            let mut asset_store_mut = asset_store.borrow_mut();
            let asset_ref = asset_store_mut.get_mut(&asset_uuid);

                    if asset_ref.is_none()
                    {
                        let asset = Asset {
                            id: asset_uuid.clone(),
                            filename: name,
                            bytes: data 
                        };
                    
                        asset_store_mut.insert(asset_uuid.clone(), asset.clone());
                    
                        USER_ASSET_STORE.with(|user_asset_store| {
                            user_asset_store
                                .borrow_mut()
                                .insert(principal_id, asset_uuid.clone())
                        });
                    } else {
                        // push chunk into asset store
                        asset_ref.unwrap().bytes.append(&mut data);
                    }
                });

    if error_condition == false
    {
        //Ok(asset.bytes.len())
        return ManualReply::one(Some("OK"));
    }
    else 
    {
        return ManualReply::one(Some("Error uploading chunk {_errorMessage}"));
    }
}