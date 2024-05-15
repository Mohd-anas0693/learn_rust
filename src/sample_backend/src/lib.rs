mod states;
mod types;
use ic_cdk::{caller, export_candid, query, update};
use states::UserState;
use std::cell::RefCell;
use types::PostData;

thread_local! {
    static STATE:RefCell<UserState> = RefCell::new(UserState::default())
}

#[update(name = "create_post")]
fn insert(post_data: PostData) -> Result<(), String> {
    let user_principal = caller();
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.set_post_data(user_principal, post_data)
    })
}
#[query(name = "get_post")]
fn fetch() -> Result<PostData, String> {
    let user_principal = caller();
    STATE.with(|state| {
        let state = state.borrow();
        state.get_post_data(user_principal)
    })
}
#[query(name = "remove_post")]
fn remove() -> Result<PostData, String> {
    let user_principal = caller();
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.remove_user_post(user_principal)
    })
}


export_candid!();
