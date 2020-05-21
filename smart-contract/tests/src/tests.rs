use crate::voting::{account, Sender, VotingContract};
use logic::{Participant, Project, ProjectId};
use std::collections::BTreeMap;

pub const START_AT: u64 = 1;
pub const END_AT: u64 = 2;
pub const VOTING_POWER: u64 = 10;

#[test]
fn test_init() {
    let voting = VotingContract::deployed(START_AT, END_AT);
    assert_eq!(voting.start_at(), START_AT);
    assert_eq!(voting.end_at(), END_AT);
}

#[test]
#[should_panic]
fn test_init_with_start_after_end() {
    VotingContract::deployed(END_AT, START_AT);
}

#[test]
fn test_add_and_update_participant() {
    let mut voting = VotingContract::deployed(START_AT, END_AT);
    voting.add_or_update_participant(account::ALI, VOTING_POWER, Sender(account::ADMIN));
    assert_eq!(
        voting.participant(account::ALI).unwrap(),
        Participant {
            total_voting_power: VOTING_POWER,
            used_voting_power: 0,
            votes: BTreeMap::new()
        }
    );
    let new_voting_power = 11;
    voting.add_or_update_participant(account::ALI, new_voting_power, Sender(account::ADMIN));
    assert_eq!(
        voting.participant(account::ALI).unwrap(),
        Participant {
            total_voting_power: new_voting_power,
            used_voting_power: 0,
            votes: BTreeMap::new()
        }
    );
}

#[test]
#[should_panic]
fn test_add_and_update_participant_by_non_admin() {
    let mut voting = VotingContract::deployed(START_AT, END_AT);
    voting.add_or_update_participant(account::ALI, VOTING_POWER, Sender(account::ALI));
}

#[test]
fn test_remove_participant() {
    let mut voting = VotingContract::deployed(START_AT, END_AT);
    voting.add_or_update_participant(account::ALI, VOTING_POWER, Sender(account::ADMIN));
    voting.remove_participant(account::ALI, Sender(account::ADMIN));
    assert!(voting.participant(account::ALI).is_none());
}

#[test]
#[should_panic]
fn test_remove_participant_by_none_admin() {
    let mut voting = VotingContract::deployed(START_AT, END_AT);
    voting.add_or_update_participant(account::ALI, VOTING_POWER, Sender(account::ADMIN));
    voting.remove_participant(account::ALI, Sender(account::ALI));
}

#[test]
fn test_add_and_update_project() {
    let mut voting = VotingContract::deployed(START_AT, END_AT);
    let project = example_project("project");
    let project_id = ProjectId(1);
    voting.add_or_update_project(project_id, project.clone(), Sender(account::ADMIN));
    assert_eq!(voting.project(project_id).unwrap(), project);
    let updated_project = example_project("project2");
    voting.add_or_update_project(project_id, updated_project.clone(), Sender(account::ADMIN));
    assert_eq!(voting.project(project_id).unwrap(), updated_project);
}

#[test]
#[should_panic]
fn test_add_and_update_project_by_non_admin() {
    let mut voting = VotingContract::deployed(START_AT, END_AT);
    let project = example_project("project");
    let project_id = ProjectId(1);
    voting.add_or_update_project(project_id, project, Sender(account::ALI));
}

#[test]
fn test_remove_project() {
    let mut voting = VotingContract::deployed(START_AT, END_AT);
    let project = example_project("project");
    let project_id = ProjectId(1);
    voting.add_or_update_project(project_id, project, Sender(account::ADMIN));
    voting.remove_project(project_id, Sender(account::ADMIN));
    assert!(voting.project(project_id).is_none());
}

#[test]
#[should_panic]
fn test_remove_project_by_non_admin() {
    let mut voting = VotingContract::deployed(START_AT, END_AT);
    let project = example_project("project");
    let project_id = ProjectId(1);
    voting.add_or_update_project(project_id, project, Sender(account::ADMIN));
    voting.remove_project(project_id, Sender(account::ALI));
}

#[test]
fn test_vote_casting() {
    let mut voting = VotingContract::deployed(START_AT, END_AT);
    let project = example_project("project");
    let project_id = ProjectId(1);
    voting.add_or_update_project(project_id, project, Sender(account::ADMIN));
    voting.add_or_update_participant(account::ALI, VOTING_POWER, Sender(account::ADMIN));
    voting.set_block_time(START_AT);
    voting.cast_vote(project_id, VOTING_POWER, Sender(account::ALI));
    let mut votes = BTreeMap::new();
    votes.insert(project_id, VOTING_POWER);
    assert_eq!(
        voting.participant(account::ALI).unwrap(),
        Participant {
            total_voting_power: VOTING_POWER,
            used_voting_power: VOTING_POWER,
            votes
        }
    );
}

fn example_project(name: &str) -> Project {
    Project {
        name: name.to_string(),
        team_name: "casperlabs".to_string(),
        video_link: "https://www.youtube.com/channel/UCjFz9Sfi4yFwocnDQTWDSqA".to_string(),
        github_link: "https://github.com/CasperLabs/CasperLabs".to_string(),
        google_drive_link: "http://drive.google.com".to_string(),
    }
}
