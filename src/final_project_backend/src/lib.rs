use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager,VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl,StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell}; 
use std::clone::Clone;

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 5000;

#[derive(CandidType,Deserialize)]
enum Choice {
    Approve,
    Reject,
    Pass,
}

#[derive(CandidType)]
enum VoteError{
    AlreadyVoted,
    ProposalIsNotActive,
    NoSuchProposal,
    AcsessRejected,
    UpdateError,
}

#[derive(CandidType)]
enum BidError {
    ItemIsNotActive, 
    NoSuchProposal,
    UpdateError,
}

#[derive(CandidType, Deserialize, Clone)]
struct Proposal {
    description: String,
    approve: u32,
    reject: u32,
    pass: u32,
    is_active: bool,
    voted: Vec<candid::Principal>,
    owner: candid::Principal,
    current_highest_bid: u32,
    current_highest_bidder: candid::Principal,
    bidders: Vec<candid::Principal>,
}

#[derive(CandidType, Deserialize)]
    struct CreateProposal {
        description: String,
        is_active: bool,
    }

    impl Storable for Proposal {
        fn to_bytes(&self) -> Cow<[u8]> {
            Cow::Owned(Encode!(self).unwrap())
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            Decode!(bytes.as_ref(), Self).unwrap()
        }
    }

    impl BoundedStorable for Proposal {
        const MAX_SIZE: u32 = MAX_VALUE_SIZE;
        const IS_FIXED_SIZE: bool = false;
    }

    thread_local! {
        static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

        static PROPOSAL_MAP: RefCell<StableBTreeMap<u64, Proposal, Memory>> = RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        ))
    }

#[ic_cdk::query]
fn get_proposal(key: u64) -> Option<Proposal>{
    PROPOSAL_MAP.with(|p| p.borrow().get(&key) )
}

#[ic_cdk::query]
fn get_proposal_count() -> u64 {
    PROPOSAL_MAP.with(|p| p.borrow().len())
}


/*
#[ic_cdk::query]
fn get_list_of_proposals() -> Vec<(u64, Proposal)> {
    let proposal_map_ref = PROPOSAL_MAP.with(|map| { 
        let map = map.borrow_mut();
        map.iter().collect::<Vec<_>>() 
    });
    proposal_map_ref
}
*/

#[ic_cdk::query]
fn most_expensive_proposal() -> Option<Proposal> {
 
    let proposal_map_ref = PROPOSAL_MAP.with(|map| { 
        let map = map.borrow_mut();
        map.iter().collect::<Vec<_>>() 
    });
    
        if let Some((_key, max_value)) = proposal_map_ref.iter().max_by_key(|(_k, v)| v.current_highest_bid) {
        Some(max_value.clone())
    } else {
        None
    }
}

#[ic_cdk::query]
fn most_bidded_proposal() -> Option<Proposal> {
    let proposal_map_ref = PROPOSAL_MAP.with(|map| {
        let map = map.borrow_mut();
        map.iter().collect::<Vec<_>>()
    });

    proposal_map_ref
        .iter()
        .max_by(|a, b| a.1.bidders.len().cmp(&b.1.bidders.len()))
        .map(|(_k, v)| v.clone()) 
}
 

#[ic_cdk::update]
fn create_proposal(key: u64, proposal: CreateProposal) -> Option<Proposal>{
    let value: Proposal = Proposal {
         description: proposal.description , 
         approve: 0u32 , 
         reject: 0u32 , 
         pass: 0u32 , 
         is_active: proposal.is_active , 
         voted: vec![], 
         owner: ic_cdk::caller(),
         current_highest_bid: 0u32,
         current_highest_bidder: ic_cdk::caller(),
         bidders: vec![]
        };

    PROPOSAL_MAP.with(|p| p.borrow_mut().insert(key,value))
}

#[ic_cdk::update]
    fn edit_proposal(key: u64, proposal: CreateProposal) -> Result<(), VoteError> {
        PROPOSAL_MAP.with(|p|{
            let old_proposal_opt = p.borrow().get(&key);
            let old_proposal = match old_proposal_opt{

                Some(value) => value,
                None => return Err(VoteError::NoSuchProposal),
            };

            if ic_cdk::caller() != old_proposal.owner{
                return Err(VoteError::AcsessRejected);
            }

            let value = Proposal{
                description: proposal.description,
                approve: old_proposal.approve,
                reject: old_proposal.reject,
                pass: old_proposal.pass,
                is_active: old_proposal.is_active,
                voted: old_proposal.voted,
                owner: ic_cdk::caller(),
                current_highest_bid: old_proposal.current_highest_bid,
                current_highest_bidder: old_proposal.current_highest_bidder,
                bidders:old_proposal.bidders
            };

            let res = p.borrow_mut().insert(key, value);

            match res {
                Some(_) => Ok(()),
                None => Err(VoteError::UpdateError),
            }
        })
    }

#[ic_cdk::update]
fn end_proposal(key: u64) -> Result<(), VoteError> {
    PROPOSAL_MAP.with(|p|{
        let proposal_opt = p.borrow().get(&key);
        let mut proposal: Proposal;

        match proposal_opt{
            Some(value) => proposal = value,
            None => return Err(VoteError::NoSuchProposal),
        }

        if ic_cdk::caller() != proposal.owner{
            return Err(VoteError::AcsessRejected);
        }

        proposal.is_active = false;
        proposal.owner = proposal.current_highest_bidder;
        proposal.current_highest_bid = 0u32;
        proposal.bidders = vec![];

        let res = p.borrow_mut().insert(key, proposal);

        match res {
            Some(_) => Ok(()),
            None => Err(VoteError::UpdateError),
        }
    })
}

#[ic_cdk::update]

fn vote(key:u64, choice: Choice) -> Result<(), VoteError>{
    PROPOSAL_MAP.with(|p| {
        let proposal_opt = p.borrow().get(&key);
        let mut proposal: Proposal;
        
        match proposal_opt {
            Some(value) => proposal = value,
            None => return Err(VoteError::NoSuchProposal),
        };

        let caller = ic_cdk::caller();

        if proposal.voted.contains(&caller){
            return Err(VoteError::AlreadyVoted)
        } else if proposal.is_active == false{
            return Err(VoteError::ProposalIsNotActive);
        }

        match choice {
            Choice::Approve => proposal.approve += 1,
            Choice::Reject => proposal.reject+= 1,
            Choice::Pass => proposal.pass += 1,
        };

        proposal.voted.push(caller);

        let res = p.borrow_mut().insert(key, proposal);
        match res {
            Some(_) => Ok(()),
            None => return Err(VoteError::UpdateError)
        }


    })
}
#[ic_cdk::update]
fn bid(key: u64, bid_amount: u32) -> Result<(), BidError> {
    PROPOSAL_MAP.with(|p| {
        let proposal_opt = p.borrow().get(&key);
        let mut proposal: Proposal;
        
        match proposal_opt {
            Some(value) => proposal = value,
            None => return Err(BidError::NoSuchProposal),
        };

        let caller = ic_cdk::caller();

        //check if item is active
        if proposal.is_active == false {
            return Err(BidError::ItemIsNotActive);
        }

        //if caller bids higher then the previous bidders edit the item
        if bid_amount > proposal.current_highest_bid {
            proposal.current_highest_bid = bid_amount;
            proposal.current_highest_bidder = caller;
        }

        proposal.bidders.push(caller);

        let res = p.borrow_mut().insert(key, proposal);
        match res {
            Some(_) => Ok(()),
            None => Err(BidError::UpdateError),
        }
    })
}