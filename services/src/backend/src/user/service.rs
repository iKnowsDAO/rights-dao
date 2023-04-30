use std::collections::BTreeMap;

use candid::Principal;

use crate::sbt::domain::{Achievement, Sbt};

use super::{
    domain::{UserEditCommand, UserProfile, UserStatus},
    error::UserError,
};

#[derive(Debug, Default)]
pub struct UserService {
    pub users: BTreeMap<Principal, UserProfile>,
}

impl UserService {
    pub fn insert_user(&mut self, user: UserProfile) -> Result<Principal, UserError> {
        let owner = user.owner;
        match self.users.get(&owner) {
            Some(_) => Err(UserError::UserAlreadyExists),
            None => {
                self.users.insert(owner, user);
                Ok(owner)
            }
        }
    }

    pub fn is_owner(&self, caller: &Principal) -> bool {
        matches!(self.users.get(caller), Some(u) if u.owner == *caller)
    }

    pub fn get_user(&self, principal: &Principal) -> Option<UserProfile> {
        self.users.get(principal).cloned()
    }

    pub fn edit_user(
        &mut self,
        cmd: UserEditCommand,
        principal: &Principal,
    ) -> Result<bool, UserError> {
        match self.users.get_mut(principal) {
            None => Err(UserError::UserNotFound),
            Some(user) => cmd.build_profile(user),
        }
    }

    pub fn enable_user(&mut self, principal: &Principal) -> Option<bool> {
        self.users
            .get_mut(principal)
            .map(|profile| {
                profile.status = UserStatus::Enable;
            })
            .map(|_| true)
    }

    pub fn disable_user(&mut self, principal: &Principal) -> Option<bool> {
        self.users
            .get_mut(principal)
            .map(|profile| {
                profile.status = UserStatus::Disable;
            })
            .map(|_| true)
    }

    pub fn update_wallet(&mut self, user: &Principal, wallet: Principal) -> Option<bool> {
        self.users
            .get_mut(user)
            .map(|profile| {
                profile.wallet_principal = Some(wallet);
            })
            .map(|_| true)
    }

    pub fn delete_wallet(&mut self, user: &Principal) -> Option<bool> {
        self.users
            .get_mut(user)
            .map(|profile| {
                profile.wallet_principal = None;
            })
            .map(|_| true)
    }

    /// 更新用户的成就
    pub fn update_achievement(&mut self, achievement: Achievement) -> Option<bool> {
        let user = &achievement.owner;
        self.users
            .get_mut(user)
            .map(|profile| {
                profile.achievement = Some(achievement);
            })
            .map(|_| true)
    }

    /// 更新用户的sbt
    pub fn update_sbt(&mut self, sbt: Sbt) -> Option<bool> {
        let user = &sbt.owner;
        self.users
            .get_mut(user)
            .map(|profile| {
                profile.claimed_sbt = Some(sbt);
            })
            .map(|_| true)
    }

    // 获取用户成就
    pub fn get_sbt(&self, user: &Principal) -> Option<Sbt> {
        self.get_user(user).and_then(|u| u.claimed_sbt)
    }
}
