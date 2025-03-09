use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Clone, Debug)]
struct Voter {
    id: String,
    voted: bool,
    vote: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Candidate {
    id: String,
    name: String,
    vote_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct VotingSystem {
    candidates: HashMap<String, Candidate>,
    voters: HashMap<String, Voter>,
    is_active: bool,
}

impl VotingSystem {
    fn new() -> Self {
        VotingSystem {
            candidates: HashMap::new(),
            voters: HashMap::new(),
            is_active: true,
        }
    }

    fn add_candidate(&mut self, id: String, name: String) -> Result<(), String> {
        if self.candidates.contains_key(&id) {
            return Err("Candidate already exists".to_string());
        }

        self.candidates.insert(
            id.clone(),
            Candidate {
                id,
                name,
                vote_count: 0,
            },
        );

        Ok(())
    }

    fn register_voter(&mut self, id: String) -> Result<(), String> {
        if self.voters.contains_key(&id) {
            return Err("Voter already registered".to_string());
        }

        self.voters.insert(
            id.clone(),
            Voter {
                id,
                voted: false,
                vote: None,
            },
        );

        Ok(())
    }

fn cast_vote(&mut self, voter_id: String, candidate_id: String) -> Result<(), String> {
        
    if !self.is_active {
            return Err("Voting is no longer active".to_string());
        }
        
        let voter = match self.voters.get(&voter_id) {
            Some(v) => v,
            None => return Err("Voter not registered".to_string()),
        };
        
        if voter.voted {
            return Err("Voter has already cast a vote".to_string());
        }

        if !self.candidates.contains_key(&candidate_id) {
            return Err("Candidate does not exist".to_string());
        }

   
        let mut voter = self.voters.get(&voter_id).unwrap().clone();
        voter.voted = true;
        voter.vote = Some(candidate_id.clone());
        self.voters.insert(voter_id, voter);

        let mut candidate = self.candidates.get(&candidate_id).unwrap().clone();
        candidate.vote_count += 1;
        self.candidates.insert(candidate_id, candidate);

        Ok(())
    }

fn get_results(&self) -> HashMap<String, u32> {
        
    let mut results = HashMap::new();
        for (id, candidate) in &self.candidates {
            results.insert(id.clone(), candidate.vote_count);
        }
        results
    }

fn end_voting(&mut self) {
        self.is_active = false;
    }
}


struct AppState {
    voting_system: Mutex<VotingSystem>,
}


#[derive(Deserialize)]
struct CandidateRequest {
    id: String,
    name: String,
}

#[derive(Deserialize)]
struct VoterRequest {
    id: String,
}

#[derive(Deserialize)]
struct VoteRequest {
    voter_id: String,
    candidate_id: String,
}


#[get("/candidates")]
async fn get_candidates(data: web::Data<Arc<AppState>>) -> impl Responder {
    let voting_system = data.voting_system.lock().unwrap();
    let candidates: Vec<&Candidate> = voting_system.candidates.values().collect();
    HttpResponse::Ok().json(candidates)
}

#[post("/candidates")]
async fn add_candidate(
    data: web::Data<Arc<AppState>>,
    req: web::Json<CandidateRequest>,
) -> impl Responder {
    let mut voting_system = data.voting_system.lock().unwrap();
    match voting_system.add_candidate(req.id.clone(), req.name.clone()) {
        Ok(_) => HttpResponse::Ok().json("Candidate added successfully"),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

#[post("/voters")]
async fn register_voter(
    data: web::Data<Arc<AppState>>,
    req: web::Json<VoterRequest>,
) -> impl Responder {
    let mut voting_system = data.voting_system.lock().unwrap();
    match voting_system.register_voter(req.id.clone()) {
        Ok(_) => HttpResponse::Ok().json("Voter registered successfully"),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

#[post("/vote")]
async fn cast_vote(
    data: web::Data<Arc<AppState>>,
    req: web::Json<VoteRequest>,
) -> impl Responder {
    let mut voting_system = data.voting_system.lock().unwrap();
    match voting_system.cast_vote(req.voter_id.clone(), req.candidate_id.clone()) {
        Ok(_) => HttpResponse::Ok().json("Vote cast successfully"),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

#[get("/results")]
async fn get_results(data: web::Data<Arc<AppState>>) -> impl Responder {
    let voting_system = data.voting_system.lock().unwrap();
    let results = voting_system.get_results();
    HttpResponse::Ok().json(results)
}

#[post("/end")]
async fn end_voting(data: web::Data<Arc<AppState>>) -> impl Responder {
    let mut voting_system = data.voting_system.lock().unwrap();
    voting_system.end_voting();
    HttpResponse::Ok().json("Voting has ended")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let app_state = Arc::new(AppState {
        voting_system: Mutex::new(VotingSystem::new()),
    });


    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(app_state.clone()))
            .service(get_candidates)
            .service(add_candidate)
            .service(register_voter)
            .service(cast_vote)
            .service(get_results)
            .service(end_voting)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}