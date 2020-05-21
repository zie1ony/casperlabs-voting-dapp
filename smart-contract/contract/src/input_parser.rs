use crate::error::Error;
use crate::utils::{get_arg, method_name};
use alloc::string::String;
use casperlabs_contract::contract_api::runtime;
use casperlabs_types::account::PublicKey;
use logic::{Project, ProjectId};

pub const DEPLOY: &str = "deploy";
pub const ADD_OR_UPDATE_PARTICIPANT: &str = "add_or_update_participant";
pub const REMOVE_PARTICIPANT: &str = "remove_participant";
pub const ADD_OR_UPDATE_PROJECT: &str = "add_or_update_project";
pub const REMOVE_PROJECT: &str = "remove_project";
pub const CAST_VOTE: &str = "cast_vote";

// #[allow(clippy::large_enum_variant)]
pub enum Input {
    Deploy(u64, u64),
    AddOrUpdateParticipant(PublicKey, u64),
    RemoveParticipant(PublicKey),
    AddOrUpdateProject(ProjectId, Project),
    RemoveProject(ProjectId),
    CastVote(ProjectId, u64),
}

pub fn from_args() -> Input {
    let method: String = method_name();
    match method.as_str() {
        DEPLOY => Input::Deploy(get_arg(1), get_arg(2)),
        ADD_OR_UPDATE_PARTICIPANT => Input::AddOrUpdateParticipant(get_arg(1), get_arg(2)),
        REMOVE_PARTICIPANT => Input::RemoveParticipant(get_arg(1)),
        ADD_OR_UPDATE_PROJECT => Input::AddOrUpdateProject(ProjectId(get_arg(1)), read_project(2)),
        REMOVE_PROJECT => Input::RemoveProject(ProjectId(get_arg(1))),
        CAST_VOTE => Input::CastVote(ProjectId(get_arg(1)), get_arg(2)),
        _ => runtime::revert(Error::UnknownApiCommand),
    }
}

fn read_project(shift: u32) -> Project {
    Project {
        name: get_arg(shift),
        team_name: get_arg(shift + 1),
        video_link: get_arg(shift + 2),
        github_link: get_arg(shift + 3),
        google_drive_link: get_arg(shift + 4),
    }
}
