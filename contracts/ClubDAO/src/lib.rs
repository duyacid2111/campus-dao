#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    Address, Env, String,
};

#[derive(Clone)]
#[contracttype]
pub struct Poll {
    pub id: u32,
    pub title: String,
    pub creator: Address,
    pub yes_votes: u32,
    pub no_votes: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Member(Address),
    Poll(u32),
    PollCount,
    Voted(u32, Address),
}

#[contracterror]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ClubError {
    NotMember = 1,
    AlreadyMember = 2,
    PollNotFound = 3,
    AlreadyVoted = 4,
}

#[contract]
pub struct ClubDAO;

#[contractimpl]
impl ClubDAO {
    /// Tham gia câu lạc bộ
    pub fn join_club(
        env: Env,
        member: Address,
    ) -> Result<(), ClubError> {
        member.require_auth();

        let exists: bool = env
            .storage()
            .persistent()
            .get(&DataKey::Member(member.clone()))
            .unwrap_or(false);

        if exists {
            return Err(ClubError::AlreadyMember);
        }

        env.storage()
            .persistent()
            .set(&DataKey::Member(member), &true);

        Ok(())
    }

    /// Kiểm tra thành viên
    pub fn is_member(
        env: Env,
        member: Address,
    ) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::Member(member))
            .unwrap_or(false)
    }

    /// Tạo poll mới
    pub fn create_poll(
        env: Env,
        creator: Address,
        title: String,
    ) -> Result<u32, ClubError> {
        creator.require_auth();

        let is_member: bool = env
            .storage()
            .persistent()
            .get(&DataKey::Member(creator.clone()))
            .unwrap_or(false);

        if !is_member {
            return Err(ClubError::NotMember);
        }

        let mut poll_count: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::PollCount)
            .unwrap_or(0);

        poll_count += 1;

        let poll = Poll {
            id: poll_count,
            title,
            creator,
            yes_votes: 0,
            no_votes: 0,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Poll(poll_count), &poll);

        env.storage()
            .persistent()
            .set(&DataKey::PollCount, &poll_count);

        Ok(poll_count)
    }

    /// Vote YES hoặc NO
    pub fn vote(
        env: Env,
        voter: Address,
        poll_id: u32,
        support: bool,
    ) -> Result<(), ClubError> {
        voter.require_auth();

        let is_member: bool = env
            .storage()
            .persistent()
            .get(&DataKey::Member(voter.clone()))
            .unwrap_or(false);

        if !is_member {
            return Err(ClubError::NotMember);
        }

        let voted_key =
            DataKey::Voted(poll_id, voter.clone());

        let already_voted: bool = env
            .storage()
            .persistent()
            .get(&voted_key)
            .unwrap_or(false);

        if already_voted {
            return Err(ClubError::AlreadyVoted);
        }

        let mut poll: Poll = env
            .storage()
            .persistent()
            .get(&DataKey::Poll(poll_id))
            .ok_or(ClubError::PollNotFound)?;

        if support {
            poll.yes_votes += 1;
        } else {
            poll.no_votes += 1;
        }

        env.storage()
            .persistent()
            .set(&DataKey::Poll(poll_id), &poll);

        env.storage()
            .persistent()
            .set(&voted_key, &true);

        Ok(())
    }

    /// Xem thông tin poll
    pub fn get_poll(
        env: Env,
        poll_id: u32,
    ) -> Result<Poll, ClubError> {
        env.storage()
            .persistent()
            .get(&DataKey::Poll(poll_id))
            .ok_or(ClubError::PollNotFound)
    }

    /// Tổng số poll
    pub fn get_poll_count(
        env: Env,
    ) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::PollCount)
            .unwrap_or(0)
    }
}