use std::collections::{BTreeMap, BTreeSet};

use candid::Principal;

use super::domain::{ReputationEvent, ReputationSummary};

#[derive(Debug, Default)]
pub struct ReputationService {
    pub summaries: BTreeMap<Principal, ReputationSummary>,
    pub events: BTreeMap<u64, ReputationEvent>,
}

impl ReputationService {
    pub fn insert_reputation(&mut self, rs: ReputationSummary) {
        self.summaries.insert(rs.id, rs);
    }

    pub fn get_reputation(&self, user: &Principal) -> ReputationSummary {
        self.summaries
            .get(user)
            .cloned()
            .unwrap_or(ReputationSummary::new(*user))
    }

    pub fn get_reputations(&self, users: &BTreeSet<Principal>) -> Vec<ReputationSummary> {
        self.summaries
            .values()
            .filter(|r| users.contains(&r.id))
            .cloned()
            .collect()
    }

    pub fn handle_reputation_event(&mut self, event: ReputationEvent) -> bool {
        self.events.insert(event.id, event.clone());
        let user = event.operator;
        *self
            .summaries
            .entry(user)
            .or_insert_with(|| ReputationSummary::new(user)) += event.amount;

        true
    }
}
