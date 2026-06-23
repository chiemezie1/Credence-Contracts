use soroban_sdk::Symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisputeStatus {
    Open,
    InVoting,
    Finalized(bool),
    Appealed,
}

impl DisputeStatus {
    pub fn to_symbol(&self) -> Symbol {
        match self {
            DisputeStatus::Open => Symbol::short("open"),
            DisputeStatus::InVoting => Symbol::short("voting"),
            DisputeStatus::Finalized(true) => Symbol::short("upheld"),
            DisputeStatus::Finalized(false) => Symbol::short("dismissed"),
            DisputeStatus::Appealed => Symbol::short("appealed"),
        }
    }
}
