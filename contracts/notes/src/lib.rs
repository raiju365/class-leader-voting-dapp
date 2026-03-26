#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Candidate struct
#[contracttype]
#[derive(Clone, Debug)]
pub struct Candidate {
    id: u64,
    name: String,
    vote_count: u64,
}

// Storage key
const CANDIDATE_DATA: Symbol = symbol_short!("CANDIDATE");

// Contract
#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {

    // Get all candidates
    pub fn get_candidates(env: Env) -> Vec<Candidate> {
        let candidates: Vec<Candidate> = env
            .storage()
            .instance()
            .get(&CANDIDATE_DATA)
            .unwrap_or(Vec::new(&env));

        candidates
    }

    // Create new candidate
    pub fn create_candidate(env: Env, name: String) -> String {
        let mut candidates: Vec<Candidate> = env
            .storage()
            .instance()
            .get(&CANDIDATE_DATA)
            .unwrap_or(Vec::new(&env));

        let candidate = Candidate {
            id: env.prng().gen::<u64>(),
            name,
            vote_count: 0,
        };

        candidates.push_back(candidate);
        env.storage().instance().set(&CANDIDATE_DATA, &candidates);

        String::from_str(&env, "Candidate created")
    }

    // Vote candidate
    pub fn vote(env: Env, id: u64) -> String {
        let mut candidates: Vec<Candidate> = env
            .storage()
            .instance()
            .get(&CANDIDATE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..candidates.len() {
            let mut candidate = candidates.get(i).unwrap();

            if candidate.id == id {
                candidate.vote_count += 1;
                candidates.set(i, candidate);

                env.storage().instance().set(&CANDIDATE_DATA, &candidates);

                return String::from_str(&env, "Vote success");
            }
        }

        String::from_str(&env, "Candidate not found")
    }

    // Delete candidate
    pub fn delete_candidate(env: Env, id: u64) -> String {
        let mut candidates: Vec<Candidate> = env
            .storage()
            .instance()
            .get(&CANDIDATE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..candidates.len() {
            if candidates.get(i).unwrap().id == id {
                candidates.remove(i);

                env.storage().instance().set(&CANDIDATE_DATA, &candidates);

                return String::from_str(&env, "Candidate deleted");
            }
        }

        String::from_str(&env, "Candidate not found")
    }
}

mod test;