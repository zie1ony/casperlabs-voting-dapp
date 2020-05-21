# Voting smart contract.

## Build
```
make build-contract
```

## Test
```
make test
```

## Initialization

Contract has to be initialized with two arguments that specify time bounds of the voting period. Calling account becomes the `Admin` of the contract. After the deployment the `Admin` should have two new named keys: `voting_indirect` and `voting_contract`. All further calls should be directed to the `voting_indirect` session code.

##### Arguments
| name     | type     | description
| -------- | -------- | ------------
| method   | String   | Has to be `deploy`.
| start_at | u64      | Should be before `end_at`.
| end_at   | u64      |

## Add or update the participant.
Participants of the hackathon can vote on the projects. First the `Admin` should register all the participants in the system with their voting power. If the method is called twice on the same participant then the voting power will be updated.

##### Restricions
`Admin` only!

##### Arguments
| name        | type      | description
| ----------- | --------- | ------------
| method      | String    | Has to be `add_or_update_participant`.
| participant | PublicKey | Participant's account hash.
| voting_power| u64       | Voting power of the participant.

## Remove participant.
Remove the participant and all its votes.

##### Restricions
`Admin` only!

##### Arguments
| name        | type      | description
| ----------- | --------- | ------------
| method      | String    | Has to be `remove_participant`.
| participant | PublicKey | Participant's account hash.

## Add of update project.
Project is an object that participants can vote on. It contains metadata about the project and its participants. Those fields might slightly change, but will remain strings. If the method is called twice with the same `project_id`, the project will be updated with the new metadata.

##### Restricions
`Admin` only!

##### Arguments
| name        | type      | description
| ----------- | --------- | ------------
| method      | String    | Has to be `add_or_update_project`.
| project_id  | u64       | Unique id of the project.
| name        | String    | Project's name.
| team        | String    | Team name.
| video       | String    | Link to the video.
| github      | String    | Link to the Github.
| google_drive| String    | Link to the Google Drive.

## Remove project
Remove project by `project_id`. It cancels all the votes casted on this project.

##### Restricions
`Admin` only!

##### Arguments
| name        | type      | description
| ----------- | --------- | ------------
| method      | String    | Has to be `remove_project`.
| project_id  | u64       | Unique id of the project.

## Cast Vote
Participants can use this call to cast their votes of the given voting power to the project.

##### Restricions
This should be called by the Participant.

##### Arguments
| name        | type      | description
| ----------- | --------- | ------------
| method      | String    | Has to be `cast_vote`.
| project_id  | u64       | Unique id of the project.
| voting_power| u64       | Voting power of the vote.

## Reading data from the blockchain.
All the data is saved inside the `voting_contract` under `voting_data` named key. This object is encoded as one large CLValue.

```
(
    (u64, u64),          // Start At, End At
    Map<                 // Projects:
        u64,             //   - Project ID.
        [String; 5]      //   - List of strings that describe the project:
                         //     [name, team, video, github, google_drive]
    >,
    Map<                 // Participants:
        [u8; 32],        //   - Public key as list of bytes
        (                //   - Particpant:
            u64,         //     - Total voting power.
            u64,         //     - Used voting power.
            Map<         //     - Votes of the participant:
                u64,     //       - Project ID
                u64,     //       - Voting power.
            >
        )
)
```

Example `query-state` output:
```
cl_value {
  cl_type {
    tuple3_type {
      type0 {
        tuple2_type {
          type0 {
            simple_type: U64
          }
          type1 {
            simple_type: U64
          }
        }
      }
      type1 {
        map_type {
          key {
            simple_type: U64
          }
          value {
            list_type {
              inner {
                simple_type: STRING
              }
            }
          }
        }
      }
      type2 {
        map_type {
          key {
            fixed_list_type {
              inner {
                simple_type: U8
              }
              len: 32
            }
          }
          value {
            tuple3_type {
              type0 {
                simple_type: U64
              }
              type1 {
                simple_type: U64
              }
              type2 {
                map_type {
                  key {
                    simple_type: U64
                  }
                  value {
                    simple_type: U64
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  value {
    tuple3_value {
      value_1 {
        tuple2_value {
          value_1 {
            u64: 1
          }
          value_2 {
            u64: 2
          }
        }
      }
      value_2 {
        map_value {
        }
      }
      value_3 {
        map_value {
        }
      }
    }
  }
}
```