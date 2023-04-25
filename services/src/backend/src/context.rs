use candid::{CandidType, Deserialize, Principal};
use std::collections::BTreeMap;
use std::iter::FromIterator;

use crate::domain::{LikeId, LikeProfile, PostBountyProfile};
use crate::env::{CanisterEnvironment, EmptyEnvironment, Environment};

use crate::governance::domain::{GovernanceMember, GovernanceProposal};
use crate::governance::GovernanceService;
use crate::post::domain::PostId;
use crate::reputation::domain::{ReputationEvent, ReputationSummary};
use crate::reputation::ReputationService;
use crate::user::domain::UserProfile;
use crate::user::UserService;

use crate::post::{domain::PostProfile, PostService};

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
    pub post_bounties: Vec<PostBountyProfile>,
}

// #[derive(Debug, Clone, CandidType, Deserialize)]
// pub struct DaoDataStorage2 {
//     pub id: u64,
//     pub users: Vec<UserProfile>,
//     pub posts: Vec<PostProfile>,
//     pub likes: Vec<LikeProfile>,
//     pub reputation_summaries: Vec<ReputationSummary>,
//     pub reputation_events: Vec<ReputationEvent>,
//     pub governance_proposals: Vec<GovernanceProposal>,
//     pub governance_members: Vec<GovernanceMember>,
// }

// impl From<DaoDataStorage2> for DaoDataStorage {
//     fn from(value: DaoDataStorage2) -> Self {
//         Self {
//             id: value.id,
//             users: value.users,
//             posts: value.posts,
//             likes: value.likes,
//             reputation_summaries: value.reputation_summaries,
//             reputation_events: value.reputation_events,
//             governance_proposals: value.governance_proposals,
//             governance_members: value.governance_members,
//             post_bounties: vec![],
//         }
//     }
// }

impl From<DaoContext> for DaoDataStorage {
    fn from(context: DaoContext) -> Self {
        let id = context.id;
        let users = Vec::from_iter(context.user_service.users.values().cloned());
        let posts = Vec::from_iter(context.post_service.posts.values().cloned());
        let likes = Vec::from_iter(context.post_service.likes.values().cloned());
        let post_bounties = Vec::from_iter(context.post_service.bounties.values().cloned());

        let reputation_summaries =
            Vec::from_iter(context.reputation_service.summaries.values().cloned());
        let reputation_events = Vec::from_iter(context.reputation_service.events.values().cloned());
        let governance_proposals =
            Vec::from_iter(context.governance_service.proposals.values().cloned());
        let governance_members =
            Vec::from_iter(context.governance_service.members.values().cloned());

        Self {
            id,
            users,
            posts,
            likes,
            reputation_summaries,
            reputation_events,
            governance_proposals,
            governance_members,
            post_bounties,
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
        let users: BTreeMap<Principal, UserProfile> =
            payload.users.into_iter().map(|u| (u.owner, u)).collect();
        let posts: BTreeMap<PostId, PostProfile> =
            payload.posts.into_iter().map(|p| (p.id, p)).collect();

        let likes: BTreeMap<LikeId, LikeProfile> = payload
            .likes
            .into_iter()
            .map(|p| (p.generate_key(), p))
            .collect();
        let bounties: BTreeMap<u64, PostBountyProfile> = payload
            .post_bounties
            .into_iter()
            .map(|p| (p.id, p))
            .collect();

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
            post_service: PostService {
                posts,
                likes,
                bounties,
            },
            reputation_service: ReputationService {
                summaries: reputation_summaries,
                events: reputation_events,
            },
            governance_service: GovernanceService {
                proposals: governance_proposals,
                members: governance_members,
            },
        }
    }
}

#[cfg(test)]
mod tests {}
