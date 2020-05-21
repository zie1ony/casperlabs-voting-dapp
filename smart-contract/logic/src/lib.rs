#![no_std]

extern crate alloc;

mod error;
mod voting;

pub use error::{StartNotBeforeEnd, VotingError};
pub use voting::{Participant, Project, ProjectId, Voting};
