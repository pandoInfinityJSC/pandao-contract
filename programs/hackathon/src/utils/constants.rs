use crate::errors::ErrorCode;

pub const DAO_PREFIX: &[u8] = b"dao";
pub const PROPOSAL_PREFIX: &[u8] = b"proposal";
pub const VOTER_PREFIX: &[u8] = b"voter";

pub const DAO_SIZE: usize = 500;
pub const PROPOSAL_SIZE: usize = 2000;
pub const VOTER_SIZE: usize = 1000;

pub const PROPOSAL_MAX_TITLE: usize = 100;
pub const PROPOSAL_MAX_DESCRIPTION: usize = 300;

#[derive(PartialEq)]
pub enum VoteType {
    Single = 0,
    Polling = 1,
}

impl TryFrom<u8> for VoteType {
    type Error = ErrorCode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VoteType::Single),
            1 => Ok(VoteType::Polling),
            _ => Err(ErrorCode::InvalidVoteType.into()),
        }
    }
}

// #[derive(PartialEq)]
// pub enum ChangeType {
//   Add,
//   Sub,
// }

// impl TryFrom<u8> for ChangeType {
//   type Error = ErrorCode;

//   fn try_from(value: u8) -> Result<Self, Self::Error> {
//     match value {
//       0 => Ok(ChangeType::Add),
//       1 => Ok(ChangeType::Sub),
//       _ => Err(ErrorCode::NotSupportedChangeType.into()),
//     }
//   }
// }

// pub enum NftType {
//   Land = 0,
//   Character = 1,
// }

// impl TryFrom<u8> for NftType {
//   type Error = ErrorCode;

//   fn try_from(value: u8) -> Result<Self, Self::Error> {
//     match value {
//       0 => Ok(NftType::Land),
//       1 => Ok(NftType::Character),
//       _ => Err(ErrorCode::NftTypeNotSupported.into()),
//     }
//   }
// }
