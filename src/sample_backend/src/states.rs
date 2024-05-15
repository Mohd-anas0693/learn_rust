use crate::types::PostData;
use candid::{self, types::principal::Principal};
use std::collections::HashMap;

#[derive(Default)]
pub struct UserState {
    pub users: HashMap<Principal, PostData>,
}
impl UserState {
    pub fn get_post_data(&self, user_principal: Principal) -> Result<PostData, String> {
        let user_data = self.users.get(&user_principal);
        if let Some(data) = user_data {
            Ok(data.clone())
        } else {
            Err("No data found".to_string())
        }
    }
    pub fn set_post_data(
        &mut self,
        user_principal: Principal,
        data: PostData,
    ) -> Result<(), String> {
        let check_user_exist = self.users.get(&user_principal);
        if let Some(user) = check_user_exist {
            self.users.insert(user_principal, data);
            Ok(())
        } else {
            return Err("Already uploaed the post of the User".to_string());
        }
    }
    pub fn remove_user_post(&mut self, user_principal: Principal) -> Result<PostData, String> {
        let remove_success = self.users.remove(&user_principal);
        if let Some(remove) = remove_success {
            Ok(remove)
        } else {
            Err("no User found!".to_string())
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn get_principal() -> Principal {
        Principal::from_text("bxquz-fu76r-igixs-bw537-mgkyg-h4goq-ldrwe-q4zg2-zxtqy-zupgm-nqe")
            .unwrap()
    }
    fn generate_user_data() -> PostData {
        let user_data = PostData {
            title: "No".to_string(),
            data: "New".to_string(),
            created_by: "32334".to_string(),
        };
        return user_data;
    }
    #[test]
    fn test_post_creation() {
        let mut state = UserState::default();
        let data = generate_user_data();
        let response = state.set_post_data(get_principal(), data.clone());
        match response {
            Ok(res) => assert_eq!(res, ()),
            Err(e) => assert_eq!(e, "Already uploaed the post of the User".to_string()),
        }
    }
    #[test]
    fn test_post_exist() {
        let state = UserState::default();
        let data = generate_user_data();
        let user_data = state.get_post_data(get_principal());
        match user_data {
            Ok(res) => assert_eq!(res, data),
            Err(e) => assert_eq!(e, "No data found".to_string()),
        };
    }
    #[test]
    fn test_post_exist_after_remove() {
        let mut state = UserState::default();
        let data = generate_user_data();
        let user_data = state.remove_user_post(get_principal());
        match user_data {
            Ok(res) => assert_eq!(res, data),
            Err(e) => assert_eq!(e, "no User found!".to_string()),
        };
    }
}
