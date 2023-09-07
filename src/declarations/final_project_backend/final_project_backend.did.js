export const idlFactory = ({ IDL }) => {
  const CreateProposal = IDL.Record({
    'description' : IDL.Text,
    'is_active' : IDL.Bool,
  });
  const Proposal = IDL.Record({
    'reject' : IDL.Nat32,
    'owner' : IDL.Principal,
    'voted' : IDL.Vec(IDL.Principal),
    'pass' : IDL.Nat32,
    'approve' : IDL.Nat32,
    'description' : IDL.Text,
    'is_active' : IDL.Bool,
  });
  const VoteError = IDL.Variant({
    'AlreadyVoted' : IDL.Null,
    'UpdateError' : IDL.Null,
    'ProposalIsNotActive' : IDL.Null,
    'NoSuchProposal' : IDL.Null,
    'AcsessRejected' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : VoteError });
  const Choice = IDL.Variant({
    'Approve' : IDL.Null,
    'Pass' : IDL.Null,
    'Reject' : IDL.Null,
  });
  return IDL.Service({
    'create_proposal' : IDL.Func(
        [IDL.Nat64, CreateProposal],
        [IDL.Opt(Proposal)],
        [],
      ),
    'edit_proposal' : IDL.Func([IDL.Nat64, CreateProposal], [Result], []),
    'end_proposal' : IDL.Func([IDL.Nat64], [Result], []),
    'get_proposal' : IDL.Func([IDL.Nat64], [IDL.Opt(Proposal)], ['query']),
    'get_proposal_count' : IDL.Func([], [IDL.Nat64], ['query']),
    'vote' : IDL.Func([IDL.Nat64, Choice], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
