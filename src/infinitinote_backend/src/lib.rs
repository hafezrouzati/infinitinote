//mod storage;

use std::fmt;

use ic_cdk::{
    api::call,
    api::call::ManualReply,
    api::call::CallResult,
};

use candid::{Encode, Decode, CandidType, Deserialize, Principal, candid_method, export_service};

//use ic_cdk::export::candid::{candid_method};
//use ic_cdk::export::candid::{export_service};

// use crate::storage::http::{
//     build_encodings, build_headers, create_token, error_response, streaming_strategy,
// };

// use crate::storage::types::http::{
//     HttpRequest, HttpResponse, StreamingCallbackHttpResponse, StreamingCallbackToken,
// };

use ic_cdk_macros::*;
use std::str;
use std::option::Option;
use std::string::String;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use sha256::{digest, try_digest};
use derive_more::{Display};

///// Storage /////
type IdStore = BTreeMap<String, candid::Principal>;
type CredentialsStore = BTreeMap<String, String>;
type ProfileStore = BTreeMap<candid::Principal, User>;
type AssetStore = BTreeMap<AssetID, Asset>;
type NotebookStore = BTreeMap<candid::Principal, BTreeMap<UUID, Notebook>>;
type UserAssetStore = BTreeMap<candid::Principal, AssetID>;
type NotebookCounter = u32;

/// type definitions ////
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct User
{
    pub name: String,
    pub email: String,
    password: String,
    pub description: String
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Display, CandidType, Default, Deserialize)]
#[display(fmt = "{}", _0)]
pub struct UUID(pub String);

#[derive(Clone, Default, CandidType)]
struct Note
{
    pub id: UUID,
    pub title: String,
    pub content: String,
    pub content_delta: String,
    pub attachments: Vec<AssetID>,
    pub tags: Vec<String>
}

#[derive(Clone, Default, CandidType)]
struct Notebook 
{
    pub id: UUID,
    pub title: String,
    pub notes: Vec<Note>,
    pub tags: Vec<String>
}

#[derive(Clone, Default, CandidType)]
struct SearchResult
{
    pub id: UUID,
    pub notebook_id: UUID,
    pub title: String,
    pub filename: String,
    pub filetype: String,
    pub result_type: String
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, CandidType, Default, Deserialize)]
pub struct AssetID(pub String);

impl fmt::Display for AssetID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, CandidType)]
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
    static NOTEBOOK_COUNTER: RefCell<u32>            = RefCell::default();
}

// impl CandidType for UUID {
//     fn id() -> candid::types::TypeId {
//         candid::types::TypeId::of::<UUID>()
//     }

//     fn _ty() -> candid::types::Type {
//         candid::types::Type::Text
//     }

//     fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error> {
//         where S: candid::ser::ValueSerializer;
//         {
//             self.0.idl_serialize(s)?;
//             Ok(())
//         }
//     }
// }

// impl CandidType for AssetID {
//     fn id() -> candid::types::TypeId {
//         candid::types::TypeId::of::<AssetID>()
//     }

//     fn _ty() -> candid::types::Type {
//         candid::types::Type::Text
//     }

//     fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error> {
//         where S: candid::ser::ValueSerializer;
//         {
//             self.0.idl_serialize(s)?;
//             Ok(())
//         }

//     }
// }

// impl CandidType for Note {
//     fn id() -> candid::types::TypeId {
//         candid::types::TypeId::of::<Note>()
//     }
//     fn _ty() -> candid::types::Type {
//         candid::types::Type::Record(vec![
//             ("id".to_string(), UUID::_ty()),
//             ("title".to_string(), String::ty()),
//             ("content".to_string(), String::ty()),
//             ("attachments".to_string(), candid::types::Type::Vec(Box::new(AssetID::_ty()))),
//             ("tags".to_string(), candid::types::Type::Vec(Box::new(String::ty()))),
//         ])
//     }
//     fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error> {
//         where S: candid::ser::ValueSerializer;
//         {
//             self.id.idl_serialize(s)?;
//             self.title.idl_serialize(s)?;
//             self.content.idl_serialize(s)?;
//             self.attachments.idl_serialize(s)?;
//             self.tags.idl_serialize(s)?;
//             Ok(())
//         }
//     }
// }

// #[query(name = "__get_candid_interface_tmp_hack")]
// fn export_candid() -> String {
//     export_service!();
//     //__export_service()
// }

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

    let management_canister = candid::Principal::management_canister();
    let rnd_buffer: (Vec<u8>,) = match ic_cdk::call(management_canister, "raw_rand", ()).await {
        Ok(result) => result,
        Err(err) => {
            ic_cdk::println!("Error invoking raw_rand: {:?} {}", err.0, err.1);
            return val;
        }
    };

    let rnd_string = String::from_utf8_lossy(&rnd_buffer.0).to_string();
    
    let rnd_hash = digest(rnd_string);
    val = rnd_hash;
    
    return val;
}

async fn get_notebook_mut(notebook_id: String) -> Option<Notebook>
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
            
            let notebook = notebooks_mut.get_mut(&nid);
            Some(notebook);
        }
    });

    None
}

#[candid_method(query)]
#[query]
async fn get_notebooks_for_principal() -> Vec<Notebook>
{
    let mut principal_id = ic_cdk::api::caller().clone();
    ic_cdk::println!("Pricipal ID {:?}", principal_id);
    let mut ret_val = Vec::<Notebook>::new();

    NOTEBOOK_STORE.with(|notebook_store| {
        let mut notebook_store_mut = notebook_store.borrow_mut();
        let principal_notebook_map = notebook_store_mut.get(&principal_id);
        if principal_notebook_map.is_none()
        {
            ret_val = Vec::<Notebook>::new();
        }
        else if principal_notebook_map.is_some()
        {
            let principal_notebooks_map_mut = notebook_store_mut.get_mut(&principal_id);
            let notebooks_map = principal_notebooks_map_mut.unwrap();

            let notebooks = notebooks_map.values().cloned();

            ret_val = Vec::<Notebook>::from_iter(notebooks);
        }
    });

    return ret_val;
}

#[candid_method]
#[update(name="create_notebook")]
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

#[update(name="add_notebook_for_principal")]
async fn add_notebook_for_principal(principal_string: String, notebook_title: String) -> String
{
    let principal_id = Principal::from_text(principal_string).unwrap();
    let principal_caller = ic_cdk::api::caller();

    ic_cdk::println!("Pricipal ID {:?}", principal_id);
    ic_cdk::println!("Pricipal Caller {:?}", principal_caller);

    let notebook_id = UUID(generate_uuid().await);

    NOTEBOOK_STORE.with(|notebook_store| {
        let mut notebook_store_mut = notebook_store.borrow_mut();
        let notebook = Notebook {
            id: notebook_id.clone(),
            title: notebook_title,
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

#[update(name="add_tag_to_notebook")]
async fn add_tag_to_notebook(notebook_id: String, tag: String) -> String
{
    let principal_id = ic_cdk::api::caller();
    let nid = UUID(notebook_id);
    let mut error_condition = false;
    let the_tag = tag.clone();

    NOTEBOOK_STORE.with(|notebook_store| {
        let mut notebook_store_mut = notebook_store.borrow_mut();
        let principal_notebooks = notebook_store_mut.get(&principal_id);

        if principal_notebooks.is_none()
        {
            error_condition = true;
        }

        if principal_notebooks.is_some()
        {
            let mut notebooks_mut = notebook_store_mut.get_mut(&principal_id).unwrap();
            
            let notebook = notebooks_mut.entry(nid);
                notebook.and_modify(|n| 
                n.tags.push(tag)
            );
        }

    });

    if error_condition
    {
        return "Invalid notebook ID".to_string();
    }

    return format!("Added Tag {}", the_tag);
}

#[update(name="remove_tag_from_notebook")]
async fn remove_tag_from_notebook(notebook_id: String, tag: String) -> String
{
    let principal_id = ic_cdk::api::caller();
    let nid = UUID(notebook_id);
    let mut error_condition = false;
    let the_tag = tag.clone();

    NOTEBOOK_STORE.with(|notebook_store| {
        let mut notebook_store_mut = notebook_store.borrow_mut();
        let principal_notebooks = notebook_store_mut.get(&principal_id);

        if principal_notebooks.is_none()
        {
            error_condition = true;
        }

        if principal_notebooks.is_some()
        {
            let mut notebooks_mut = notebook_store_mut.get_mut(&principal_id).unwrap();
            
            let notebook = notebooks_mut.entry(nid);
                notebook.and_modify(|n| 

                    if let Some(index) = n.tags.iter().position(|x| x == &tag) {
                        ic_cdk::println!("matched tag now removing tag {}", tag);
                        n.tags.remove(index);
                    }
            );
        }
    });

    if error_condition
    {
        return "Invalid notebook ID".to_string();
    }

    return format!("Removed Tag {}", the_tag);
}



#[update(name="add_tag_to_note")]
async fn add_tag_to_note(notebook_id: String, note_id: String, tag: String) -> String
{
    let principal_id = ic_cdk::api::caller();
    let nb_id = UUID(notebook_id);
    let n_id = UUID(note_id);

    let mut error_condition = false;
    let the_tag = tag.clone();

    NOTEBOOK_STORE.with(|notebook_store| {
        let mut notebook_store_mut = notebook_store.borrow_mut();
        let principal_notebooks = notebook_store_mut.get(&principal_id);

        if principal_notebooks.is_none()
        {
            error_condition = true;
        }

        if principal_notebooks.is_some()
        {
            let mut notebooks_mut = notebook_store_mut.get_mut(&principal_id).unwrap();
            
            let notebook = notebooks_mut.entry(nb_id);
                notebook.and_modify(|n| {
                    if let Some(note) = n.notes.iter_mut().find(|note| note.id == n_id) {
                        ic_cdk::println!("adding tag {} to note {}", tag, n_id);
                        note.tags.push(tag.clone());
                }
            });
        }

    });

    if error_condition
    {
        return "Invalid note ID".to_string();
    }

    return format!("Added Tag {}", the_tag);
}

#[update(name="remove_tag_from_note")]
async fn remove_tag_from_note(notebook_id: String, note_id: String, tag: String) -> String
{
    let principal_id = ic_cdk::api::caller();
    let nb_id = UUID(notebook_id);
    let n_id = UUID(note_id);

    let mut error_condition = false;
    let the_tag = tag.clone();

    NOTEBOOK_STORE.with(|notebook_store| {
        let mut notebook_store_mut = notebook_store.borrow_mut();
        let principal_notebooks = notebook_store_mut.get(&principal_id);

        if principal_notebooks.is_none()
        {
            error_condition = true;
        }

        if principal_notebooks.is_some()
        {
            let mut notebooks_mut = notebook_store_mut.get_mut(&principal_id).unwrap();
            
            let notebook = notebooks_mut.entry(nb_id);
                notebook.and_modify(|n| {
                    if let Some(note) = n.notes.iter_mut().find(|note| note.id == n_id) {
                        if let Some(index) = note.tags.iter().position(|x| x == &tag) {
                            ic_cdk::println!("removing tag {} from note {}", tag, n_id);
                            note.tags.remove(index);
                        }
                }
            });
        }

    });

    if error_condition
    {
        return "Invalid note ID".to_string();
    }

    return format!("Added Tag {}", the_tag);
}


#[query(name="get_tags_for_notebook")]
async fn get_tags_for_notebook(notebook_id: String) -> Vec<String>
{
    let principal_id = ic_cdk::api::caller();
    let nid = UUID(notebook_id);
    let mut error_condition = false;
    let mut ret_value = Vec::<String>::new();
    ret_value.push("Error".to_string());

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
                ret_value = n.tags.clone()
            );
        }   
    });

    return ret_value;
}

#[query(name="get_tags_for_note")]
async fn get_tags_for_note(notebook_id: String, note_id: String) -> Vec<String>
{
    let principal_id = ic_cdk::api::caller();
    let nid = UUID(notebook_id);
    let mut error_condition = false;
    let mut ret_value = Vec::<String>::new();
    ret_value.push("Error".to_string());

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
                ret_value = n.tags.clone()
            );
        }   
    });

    return ret_value;
}

#[update(name="update_notebook_tags")]
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

#[update(name="add_note_to_notebook")]
async fn add_note_to_notebook(notebook_id: String, note_title: String, note_content: String, note_content_delta: String, note_tags: Vec<String>) -> String
{
    let mut principal_id = ic_cdk::api::caller().clone();

    let note_id = UUID(generate_uuid().await);
    let note_id_clone = note_id.clone();
    let note  = Note { 
        id: note_id,
        title: note_title,
        content: note_content,
        content_delta: note_content_delta,
        attachments: Vec::<AssetID>::new(),
        tags: note_tags
    };
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
                n.notes.push(note)
            );
        }
    });

    if error_condition
    {
        return format!("Error");
    }
    else {
        return note_id_clone.to_string();
    }
}

#[update(name="update_note")]
async fn update_note(notebook_id: String, note_id: String, note_title: String, note_content: String, note_content_delta: String, note_tags: Vec<String>, note_attachments: Vec<AssetID>) -> String
{
    let mut principal_id = ic_cdk::api::caller().clone();

    if note_id == ""
    {
        let note_id = generate_uuid().await;
    }

    let the_note_id = UUID(note_id);
    let the_note_id_clone = the_note_id.clone();

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
            notebook.and_modify(|n| {
                if let Some(note) = n.notes.iter_mut().find(|note| note.id == the_note_id) {
                    note.title = note_title;
                    note.content = note_content;
                    note.content_delta = note_content_delta;
                    note.attachments = note_attachments;
                }
                
            });
        }
    });

    if error_condition
    {
        return format!("Failed to add note");
    }
    else {
        return format!("Added note {}", the_note_id_clone);
    }
}

#[query]
async fn get_notes_for_notebook(notebook_id: String) -> Vec<Note>
{
    let principal_id = ic_cdk::api::caller().clone();
    let nid = UUID(notebook_id);
    let mut error_condition = false;
    let mut ret_val = Vec::<Note>::new();

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
                ret_val = n.notes.clone()
            );
        }
    });

    return ret_val;
}

#[query]
async fn get_shared_note(shared_principal: String, notebook_id: String, note_id: String) -> Note
{
    let sharing_principal = Principal::from_text(shared_principal).unwrap();

    let the_note_id = UUID(note_id);
    let nid = UUID(notebook_id);

    let mut return_note = Note {
        id: UUID("".to_string()),
        title: "".to_string(),
        content: "".to_string(),
        content_delta: "".to_string(),
        attachments: Vec::<AssetID>::new(),
        tags: Vec::<String>::new()
    };

    NOTEBOOK_STORE.with(|notebook_store| {
        let mut notebook_store_mut = notebook_store.borrow_mut();
        let principal_notebooks = notebook_store_mut.get(&sharing_principal);

        if principal_notebooks.is_none()
        {
         
        }

        if principal_notebooks.is_some()
        {
            let notebooks_mut = notebook_store_mut.get_mut(&sharing_principal).unwrap();
            let notebook = notebooks_mut.entry(nid);
            notebook.and_modify(|n| {
                if let Some(note) = n.notes.iter_mut().find(|note| note.id == the_note_id) {
                    return_note = note.clone();
                }
            });
        }
    });

    return return_note;
}

////////////////////////////////////////////////////
// File Upload                                  ////
////////////////////////////////////////////////////

#[update]
async fn get_new_asset_id() -> String
{
    let new_asset_id = generate_uuid().await;
    return new_asset_id;
}

#[update]
fn upload_file_chunk(asset_id: String, name: String, mut data: Vec<u8>) -> String
{
    let mut error_condition = false;
    let mut _error_message = "";

    let principal_id = ic_cdk::api::caller().clone();
        
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

        if error_condition
        {
            return format!("Failed to add attachment");
        }
        else {
            return format!("Added chunk for asset {} ", asset_uuid);
        }
}

#[query]
fn get_asset(asset_id: String) -> Asset
{
    let asset_uuid = AssetID(asset_id);
    let empty_vec: Vec<u8> = Vec::new();
    let mut ret_val = Asset {
        id : AssetID("0".to_string()),
        filename: "0".to_string(),
        bytes: empty_vec
    };

    ASSET_STORE.with(|asset_store| {
        let mut asset_store_mut = asset_store.borrow_mut();
        let asset_ref = asset_store_mut.get_mut(&asset_uuid);
        
        if asset_ref.is_none()
        {

        }
        else 
        {
            let asset_clone = asset_ref.unwrap().clone();
            
            ic_cdk::println!("asset length {}", asset_clone.filename);
            
            ret_val = asset_clone;
        }
    });

    return ret_val;
}

// #[query]
// fn http_request(
//     HttpRequest {
//         method,
//         url,
//         headers: req_headers,
//         body: _,
//     }: HttpRequest,
// ) -> HttpResponse {
//     if method != "GET" {
//         return error_response(405, "Method Not Allowed.".to_string());
//     }

// }


/////// SEARCH /////////
#[query]
fn search_notes_by_tag(tag: String) -> Vec<SearchResult>
{
    let mut principal_id = ic_cdk::api::caller().clone();
    let mut search_results = Vec::<SearchResult>::new();

    NOTEBOOK_STORE.with(|notebook_store| {
        let mut notebook_store_mut = notebook_store.borrow_mut();
        let principal_notebook_map = notebook_store_mut.get(&principal_id);

        if principal_notebook_map.is_some()
        {
            let principal_notebooks_map_mut = notebook_store_mut.get_mut(&principal_id);
            let notebooks_map = principal_notebooks_map_mut.unwrap();

            let notebooks = notebooks_map.values().cloned();

            let notebooks_vec = Vec::<Notebook>::from_iter(notebooks);

            for notebook in &notebooks_vec
            {
                for note in &notebook.notes
                {
                    if note.tags.contains(&tag)
                    {
                        let search_result = SearchResult {
                            id : note.id.clone(),
                            notebook_id : notebook.id.clone(),
                            title : note.title.clone(),
                            filename : "".to_string(),
                            filetype : "".to_string(),
                            result_type : "note".to_string()
                        };

                        search_results.push(search_result);
                    }
                }
            }

            
        }
    });

    return search_results;

}

#[query]
fn search_notebooks_by_tag(tag: String) -> Vec<SearchResult>
{
    let mut principal_id = ic_cdk::api::caller().clone();
    let mut search_results = Vec::<SearchResult>::new();

    NOTEBOOK_STORE.with(|notebook_store| {
        let mut notebook_store_mut = notebook_store.borrow_mut();
        let principal_notebook_map = notebook_store_mut.get(&principal_id);

        if principal_notebook_map.is_some()
        {
            let principal_notebooks_map_mut = notebook_store_mut.get_mut(&principal_id);
            let notebooks_map = principal_notebooks_map_mut.unwrap();

            let notebooks = notebooks_map.values().cloned();

            let notebooks_vec = Vec::<Notebook>::from_iter(notebooks);

            for notebook in &notebooks_vec
            {
                if notebook.tags.contains(&tag)
                {
                    let search_result = SearchResult {
                        id : notebook.id.clone(),
                        notebook_id : notebook.id.clone(),
                        title : notebook.title.clone(),
                        filename : "".to_string(),
                        filetype : "".to_string(),
                        result_type : "notebook".to_string()
                    };

                    search_results.push(search_result);
                }
            }
        
        }
    });

    return search_results;
}

fn search_notes_by_text(text: String) -> Vec<SearchResult>
{
    // todo: use proper search index
    let mut principal_id = ic_cdk::api::caller().clone();
    let mut search_results = Vec::<SearchResult>::new();

    NOTEBOOK_STORE.with(|notebook_store| {
        let mut notebook_store_mut = notebook_store.borrow_mut();
        let principal_notebook_map = notebook_store_mut.get(&principal_id);

        if principal_notebook_map.is_some()
        {
            let principal_notebooks_map_mut = notebook_store_mut.get_mut(&principal_id);
            let notebooks_map = principal_notebooks_map_mut.unwrap();

            let notebooks = notebooks_map.values().cloned();

            let notebooks_vec = Vec::<Notebook>::from_iter(notebooks);

            for notebook in &notebooks_vec
            {
                for note in &notebook.notes
                {
                    if note.content.contains(&text)
                    {
                        let search_result = SearchResult {
                            id : note.id.clone(),
                            notebook_id: notebook.id.clone(),
                            title : note.title.clone(),
                            filename : "".to_string(),
                            filetype : "".to_string(),
                            result_type : "note".to_string()
                        };

                        search_results.push(search_result);
                    }
                }
            }
        }
    });

    return search_results;
}

fn search_assets_by_filename(filename: String) -> Vec<SearchResult>
{
    let mut search_results = Vec::<SearchResult>::new();
    let mut principal_id = ic_cdk::api::caller().clone();

    ASSET_STORE.with(|asset_store| {
        let mut asset_store_mut = asset_store.borrow_mut();
    });

    return search_results;
}



/// Tests

#[cfg(test)]
mod tests {
    use super::export_candid;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let dir = dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("src")
            .join("infinitinote_backend");
        write(dir.join("infinitinote_backed.candid.did"), export_candid()).expect("Write failed.");
    }
}