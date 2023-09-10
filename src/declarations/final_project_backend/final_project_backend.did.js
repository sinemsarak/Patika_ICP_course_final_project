export const idlFactory = ({ IDL }) => {
  const VoteError = IDL.Variant({
    'AlreadyVoted' : IDL.Null,
    'UpdateError' : IDL.Null,
    'ProposalIsNotActive' : IDL.Null,
    'AccessRejected' : IDL.Null,
    'NoSuchProposal' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : VoteError });
  const CreateProposal = IDL.Record({
    'description' : IDL.Text,
    'is_active' : IDL.Bool,
  });
  const Proposal = IDL.Record({
    'owner' : IDL.Principal,
    'voted' : IDL.Vec(IDL.Principal),
    'current_highest_vote' : IDL.Nat32,
    'description' : IDL.Text,
    'current_highest_bidder' : IDL.Principal,
    'current_reject_vote' : IDL.Nat32,
    'current_highest_bid' : IDL.Nat32,
    'is_active' : IDL.Bool,
    'bidders' : IDL.Vec(IDL.Principal),
    'current_pass_vote' : IDL.Nat32,
  });
  const Choice = IDL.Variant({
    'Approve' : IDL.Null,
    'Pass' : IDL.Null,
    'Reject' : IDL.Null,
  });
  return IDL.Service({
    'bid' : IDL.Func([IDL.Nat64, IDL.Nat32], [Result], []),
    'create_proposal' : IDL.Func(
        [IDL.Nat64, CreateProposal],
        [IDL.Opt(Proposal)],
        [],
      ),
    'edit_proposal' : IDL.Func([IDL.Nat64, CreateProposal], [Result], []),
    'end_proposal' : IDL.Func([IDL.Nat64], [Result], []),
    'get_proposal' : IDL.Func([IDL.Nat64], [IDL.Opt(Proposal)], ['query']),
    'get_proposal_count' : IDL.Func([], [IDL.Nat64], ['query']),
    'most_bidded_proposal' : IDL.Func([], [IDL.Opt(Proposal)], ['query']),
    'most_expensive_proposal' : IDL.Func([], [IDL.Opt(Proposal)], ['query']),
    'vote' : IDL.Func([IDL.Nat64, Choice], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
