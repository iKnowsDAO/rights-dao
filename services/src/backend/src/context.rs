
use candid::{CandidType, Deserialize, Principal};
use std::collections::BTreeMap;
use std::iter::FromIterator;

use crate::domain::{LikeProfile, LikeId};
use crate::env::{Environment, CanisterEnvironment, EmptyEnvironment};

use crate::governance::GovernanceService;
use crate::governance::domain::{GovernanceProposal, GovernanceMember};
use crate::post::domain::PostId;
use crate::reputation::ReputationService;
use crate::reputation::domain::{ReputationSummary, ReputationEvent};
use crate::user::UserService;
use crate::user::domain::UserProfile;

use crate::post::{
    PostService,
    domain::PostProfile,
};

// #[derive(Debug, Clone, CandidType, Deserialize)]
// pub struct DaoDataStoragePreUpdate {
//     pub id: u64,
//     pub users: Vec<UserProfile>,
//     pub posts: Vec<PostProfile>,
//     pub reputation_summaries: Vec<ReputationSummary>,
//     pub reputation_events: Vec<ReputationEvent>,
//     pub governance_proposals: Vec<GovernanceProposal>,
//     pub governance_members: Vec<GovernanceMember>,
// }

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct DaoDataStorage {
    pub id: u64,
    pub users: Vec<UserProfile>,
    pub posts: Vec<PostProfile>,
    pub likes: Vec<LikeProfile>,
    pub reputation_summaries: Vec<ReputationSummary>,
    pub reputation_events: Vec<ReputationEvent>,
    pub governance_proposals: Vec<GovernanceProposal>,
    pub governance_members: Vec<GovernanceMember>,
}

// impl From<DaoDataStoragePreUpdate> for DaoDataStorage {
//     fn from(cp: DaoDataStoragePreUpdate) -> Self {
//         Self {
//             id: cp.id,
//             users: cp.users,
//             posts: cp.posts,
//             likes: vec![],
//             reputation_summaries: cp.reputation_summaries,
//             reputation_events: cp.reputation_events,
//             governance_proposals: cp.governance_proposals,
//             governance_members: cp.governance_members
//         }
//     }
// }

impl From<DaoContext> for DaoDataStorage {
    fn from(context: DaoContext) -> Self {
        let id = context.id;
        let users = Vec::from_iter(context.user_service.users
            .iter()
            .map(|(_k, v)| v.clone()));
        let posts = Vec::from_iter(context.post_service.posts
            .iter()
            .map(|(_k, v)| v.clone()));   
        let likes = Vec::from_iter(context.post_service.likes
            .iter()
            .map(|(_k, v)| v.clone()));  

        let reputation_summaries = Vec::from_iter(context.reputation_service.summaries
            .iter()
            .map(|(_, summary)| summary.clone())
        );
        let reputation_events = Vec::from_iter(context.reputation_service.events
            .iter()
            .map(|(_, event)| event.clone())
        );
        let governance_proposals = Vec::from_iter(context.governance_service.proposals
            .iter()
            .map(|(_k, v)| (v.clone())));
        let governance_members = Vec::from_iter(context.governance_service.members
            .iter()
            .map(|(_k, v)| (v.clone())));

        Self {
            id,
            users,
            posts,
            likes,
            reputation_summaries,
            reputation_events,
            governance_proposals,
            governance_members,
        }
    }
}

pub struct DaoContext {
    pub env: Box<dyn Environment>,
    pub id: u64,
    pub user_service: UserService,
    pub post_service: PostService,
    pub reputation_service: ReputationService,
    pub governance_service: GovernanceService,
}

impl Default for DaoContext {
    fn default() -> Self {
        Self {
            env: Box::new(EmptyEnvironment {}),
            id: 10001,
            user_service: UserService::default(),
            post_service: PostService::default(),
            reputation_service: ReputationService::default(),
            governance_service: GovernanceService::default(),
        }
    }
}

impl From<DaoDataStorage> for DaoContext {
    fn from(payload: DaoDataStorage) -> Self {
        let users: BTreeMap<Principal, UserProfile> = payload
            .users
            .into_iter()
            .map(|u| (u.owner, u))
            .collect();
        let posts: BTreeMap<PostId, PostProfile> = payload
            .posts
            .into_iter()
            .map(|p| (p.id, p))
            .collect();

        let likes: BTreeMap<LikeId, LikeProfile>= payload
        .likes
        .into_iter()
        .map(|p| (p.generate_key(), p))
        .collect();
        // let likes: BTreeMap<(u64, Principal), LikeProfile> = BTreeMap::new();

        let reputation_summaries: BTreeMap<Principal, ReputationSummary> = payload
            .reputation_summaries
            .into_iter()
            .map(|summary| (summary.id, summary))
            .collect();

        let reputation_events: BTreeMap<u64, ReputationEvent> = payload
            .reputation_events
            .into_iter()
            .map(|event| (event.id, event))
            .collect();

        let governance_proposals: BTreeMap<u64, GovernanceProposal> = payload
            .governance_proposals
            .into_iter()
            .map(|gp| (gp.id, gp))
            .collect();
        let governance_members: BTreeMap<Principal, GovernanceMember> = payload
            .governance_members
            .into_iter()
            .map(|gm| (gm.id, gm))
            .collect();
            
        Self {
            env: Box::new(CanisterEnvironment {}),
            id: payload.id,
            user_service: UserService { users },
            post_service: PostService { posts, likes },
            reputation_service: ReputationService { summaries: reputation_summaries, events: reputation_events },
            governance_service: GovernanceService { proposals: governance_proposals, members: governance_members },
        }
    }
}

#[cfg(test)]
mod tests {
    
}
