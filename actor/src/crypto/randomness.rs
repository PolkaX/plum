/// Specifies a domain for randomness generation.
pub type DomainSeparationTag = usize;

pub const DOMAIN_SEPARATION_TAG_TICKET_PRODUCTION: DomainSeparationTag = 1;
pub const DOMAIN_SEPARATION_TAG_ELECTION_PROOF_PRODUCTION: DomainSeparationTag = 2;
pub const DOMAIN_SEPARATION_TAG_WINNING_POST_CHALLENGE_SEED: DomainSeparationTag = 3;
pub const DOMAIN_SEPARATION_TAG_WINDOWED_POST_CHALLENGE_SEED: DomainSeparationTag = 4;
pub const DOMAIN_SEPARATION_TAG_SEAL_RANDOMNESS: DomainSeparationTag = 5;
pub const DOMAIN_SEPARATION_TAG_INTERACTIVE_SEAL_CHALLENGE_SEED: DomainSeparationTag = 6;
pub const DOMAIN_SEPARATION_TAG_WINDOWED_POST_DEADLINE_ASSIGNMENT: DomainSeparationTag = 7;
