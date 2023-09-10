import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type BidError = { 'UpdateError' : null } |
  { 'NoSuchProposal' : null } |
  { 'ItemIsNotActive' : null };
export type Choice = { 'Approve' : null } |
  { 'Pass' : null } |
  { 'Reject' : null };
export interface CreateProposal {
  'description' : string,
  'is_active' : boolean,
}
export interface Proposal {
  'owner' : Principal,
  'voted' : Array<Principal>,
  'current_highest_vote' : number,
  'description' : string,
  'current_highest_bidder' : Principal,
  'current_reject_vote' : number,
  'current_highest_bid' : number,
  'is_active' : boolean,
  'bidders' : Array<Principal>,
  'current_pass_vote' : number,
}
export type Result = { 'Ok' : null } |
  { 'Err' : VoteError };
export type VoteError = { 'AlreadyVoted' : null } |
  { 'UpdateError' : null } |
  { 'ProposalIsNotActive' : null } |
  { 'AccessRejected' : null } |
  { 'NoSuchProposal' : null };
export interface _SERVICE {
  'bid' : ActorMethod<[bigint, number], Result>,
  'create_proposal' : ActorMethod<[bigint, CreateProposal], [] | [Proposal]>,
  'edit_proposal' : ActorMethod<[bigint, CreateProposal], Result>,
  'end_proposal' : ActorMethod<[bigint], Result>,
  'get_proposal' : ActorMethod<[bigint], [] | [Proposal]>,
  'get_proposal_count' : ActorMethod<[], bigint>,
  'most_bidded_proposal' : ActorMethod<[], [] | [Proposal]>,
  'most_expensive_proposal' : ActorMethod<[], [] | [Proposal]>,
  'vote' : ActorMethod<[bigint, Choice], Result>,
}
