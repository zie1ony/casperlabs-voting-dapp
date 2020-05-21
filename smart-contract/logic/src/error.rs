#[derive(PartialEq, Debug)]
pub struct StartNotBeforeEnd;

#[derive(PartialEq, Debug)]
pub enum VotingError {
    NotEnoughVotingPower,
    ProjectDoesNotExists,
    NotAParticipant,
    VotingNotStarted,
    VotingEnded,
}
