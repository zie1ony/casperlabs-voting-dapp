use crate::utils::SmartContractContext;
pub use crate::utils::{account, Sender};
use casperlabs_types::account::PublicKey;
use logic::{Participant, Project, ProjectId, Voting};

mod method {
    pub const DEPLOY: &str = "deploy";
    pub const ADD_OR_UPDATE_PARTICIPANT: &str = "add_or_update_participant";
    pub const REMOVE_PARTICIPANT: &str = "remove_participant";
    pub const ADD_OR_UPDATE_PROJECT: &str = "add_or_update_project";
    pub const REMOVE_PROJECT: &str = "remove_project";
    pub const CAST_VOTE: &str = "cast_vote";
}

const VOTING_KEY: &str = "voting_data";

pub struct VotingContract(SmartContractContext);

impl VotingContract {
    pub fn deployed(start_at: u64, end_at: u64) -> Self {
        let init_args = (method::DEPLOY, start_at, end_at);
        let context =
            SmartContractContext::deployed("voting_indirect", "voting_contract", init_args);
        VotingContract(context)
    }

    pub fn set_block_time(&mut self, block_time: u64) {
        self.0.set_block_time(block_time);
    }

    fn data(&self) -> Voting {
        let serialized = self.0.query_contract(VOTING_KEY).unwrap();
        Voting::deserialize(serialized)
    }

    pub fn start_at(&self) -> u64 {
        self.data().start_at()
    }

    pub fn end_at(&self) -> u64 {
        self.data().end_at()
    }

    pub fn participant(&self, public_key: PublicKey) -> Option<Participant> {
        self.data().participants.get(&public_key).cloned()
    }

    pub fn project(&self, project_id: ProjectId) -> Option<Project> {
        self.data().projects.get(&project_id).cloned()
    }

    pub fn add_or_update_participant(
        &mut self,
        public_key: PublicKey,
        voting_power: u64,
        sender: Sender,
    ) {
        self.0.call_indirect(
            sender,
            (
                (method::ADD_OR_UPDATE_PARTICIPANT, self.0.contract_hash),
                public_key,
                voting_power,
            ),
        );
    }

    pub fn remove_participant(&mut self, public_key: PublicKey, sender: Sender) {
        self.0.call_indirect(
            sender,
            (
                (method::REMOVE_PARTICIPANT, self.0.contract_hash),
                public_key,
            ),
        );
    }

    pub fn add_or_update_project(
        &mut self,
        project_id: ProjectId,
        project: Project,
        sender: Sender,
    ) {
        self.0.call_indirect(
            sender,
            (
                (method::ADD_OR_UPDATE_PROJECT, self.0.contract_hash),
                project_id.0,
                project.name,
                project.team_name,
                project.video_link,
                project.github_link,
                project.google_drive_link,
            ),
        );
    }

    pub fn remove_project(&mut self, project_id: ProjectId, sender: Sender) {
        self.0.call_indirect(
            sender,
            ((method::REMOVE_PROJECT, self.0.contract_hash), project_id.0),
        );
    }

    pub fn cast_vote(&mut self, project_id: ProjectId, voting_power: u64, sender: Sender) {
        self.0.call_indirect(
            sender,
            (
                (method::CAST_VOTE, self.0.contract_hash),
                project_id.0,
                voting_power,
            ),
        );
    }
}
