use crate::error::Error;
use crate::input_parser::{self, Input};
use crate::utils;
use casperlabs_contract::args_parser::ArgsParser;
use casperlabs_contract::contract_api::runtime;
use logic::Voting;

const CONTRACT_NAME: &str = "voting_contract";
const INDIRECT_NAME: &str = "voting_indirect";
const VOTING_KEY: &str = "voting_data";

#[no_mangle]
pub extern "C" fn call() {
    match input_parser::from_args() {
        Input::Deploy(start_at, end_at) => {
            let init_args = (input_parser::DEPLOY, start_at, end_at);
            utils::deploy_code_and_init(CONTRACT_NAME, CONTRACT_NAME, init_args);
            utils::deploy_code(INDIRECT_NAME, INDIRECT_NAME);
        }
        _ => runtime::revert(Error::UnknownDeployCommand),
    }
}

#[no_mangle]
fn voting_indirect() {
    match input_parser::from_args() {
        Input::AddOrUpdateParticipant(public_key, voting_power) => call_voting_contract((
            input_parser::ADD_OR_UPDATE_PARTICIPANT,
            public_key,
            voting_power,
        )),
        Input::RemoveParticipant(public_key) => {
            call_voting_contract((input_parser::REMOVE_PARTICIPANT, public_key))
        }
        Input::AddOrUpdateProject(project_id, project) => call_voting_contract((
            input_parser::ADD_OR_UPDATE_PROJECT,
            project_id.0,
            project.name,
            project.team_name,
            project.video_link,
            project.github_link,
            project.google_drive_link,
        )),
        Input::RemoveProject(project_id) => {
            call_voting_contract((input_parser::REMOVE_PROJECT, project_id.0))
        }
        Input::CastVote(project_id, vote) => {
            call_voting_contract((input_parser::CAST_VOTE, project_id.0, vote))
        }
        _ => runtime::revert(Error::UnknownIndirectCommand),
    }
}

#[no_mangle]
fn voting_contract() {
    utils::init_or_handle(init_voting, handle_voting);
}

fn init_voting() -> Result<(), Error> {
    match input_parser::from_args() {
        Input::Deploy(start_at, end_at) => {
            utils::set_admin_account(runtime::get_caller());
            Voting::new(start_at, end_at)
                .map(save_voting)
                .map_err(Error::from)
        }
        _ => Err(Error::UnknownInitCommand),
    }
}

fn handle_voting() -> Result<(), Error> {
    let mut voting = read_voting();
    match input_parser::from_args() {
        Input::AddOrUpdateParticipant(public_key, voting_power) => {
            utils::assert_admin();
            voting.add_or_update_participant(public_key, voting_power);
            save_voting(voting);
            Ok(())
        }
        Input::RemoveParticipant(public_key) => {
            utils::assert_admin();
            voting.remove_participant_if_exists(&public_key);
            save_voting(voting);
            Ok(())
        }
        Input::AddOrUpdateProject(project_id, project) => {
            utils::assert_admin();
            voting.add_or_update_project(project_id, project);
            save_voting(voting);
            Ok(())
        }
        Input::RemoveProject(project_id) => {
            utils::assert_admin();
            voting.remove_project_if_exists_and_cancel_votes(project_id);
            save_voting(voting);
            Ok(())
        }
        Input::CastVote(project_id, vote) => voting
            .cast_vote(
                runtime::get_caller(),
                project_id,
                vote,
                runtime::get_blocktime().into(),
            )
            .map(|_| save_voting(voting))
            .map_err(Error::from),
        _ => Err(Error::UnknownContractCommand),
    }
}

fn save_voting(voting: Voting) {
    utils::set_key(VOTING_KEY, voting.serialize());
}

fn read_voting() -> Voting {
    let serialized = utils::key(VOTING_KEY);
    Voting::deserialize(serialized)
}

fn call_voting_contract(args: impl ArgsParser) {
    let voting_contract = utils::destination_contract();
    runtime::call_contract::<_, ()>(voting_contract, args);
}
