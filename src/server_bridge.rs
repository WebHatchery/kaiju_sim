use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::data::{Kaiju, KaijuStats, Trait, TraitCategory, TraitCondition, TraitInheritance, KaijuId};

// --- Network DTOs (Mirroring Server Types) ---

#[derive(Debug, Deserialize)]
struct ServerKaiju {
    pub id: Uuid,
    pub name: String,
    pub generation: u32,
    pub parent_ids: Option<(Uuid, Uuid)>,
    pub visual_seed: u64,
    pub genome_hash: String,
    pub stats: KaijuStats, // Stats match structure
    pub traits: Vec<ServerTrait>,
    pub owner_id: Uuid,
    pub image_url: String,
    #[serde(default)]
    pub tournaments_won: i32,
}

#[derive(Debug, Deserialize)]
struct ServerTrait {
    pub id: String,
    pub name: String,
    pub description: String,
    pub inheritance: TraitInheritance,
    pub is_hidden: bool,
    pub power: i32,
}

impl ServerKaiju {
    fn to_client_kaiju(self) -> Kaiju {
        Kaiju {
            id: self.id,
            token_id: 0, // Placeholder
            name: self.name,
            generation: self.generation,
            created_at: 0, // Placeholder
            original_breeder: "server".to_string(),
            parent_ids: self.parent_ids.map(|(a, b)| (0, 0)), // Token IDs unknown, using 0 for now
            visual_seed: self.visual_seed,
            genome_hash: self.genome_hash,
            stats: self.stats,
            traits: self.traits.into_iter().map(|t| t.to_client_trait()).collect(),
            hidden_traits: Vec::new(),
            experience: 0,
            alive: true,
            current_owner: self.owner_id.to_string(),
            image_uri: Some(self.image_url),
            metadata_uri: String::new(),
            tournaments_won: self.tournaments_won,
        }
    }
}

impl ServerTrait {
    fn to_client_trait(self) -> Trait {
        Trait {
            id: self.id,
            name: self.name,
            description: self.description,
            category: TraitCategory::Mutation, // Default categorization
            power: self.power,
            inheritance: self.inheritance,
            condition: TraitCondition::Simple("Always".to_string()), // Default
            is_hidden: self.is_hidden,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BreedRequest {
    pub parent_a_id: Uuid,
    pub parent_b_id: Uuid,
    pub client_seed: u64,
    pub user_id: Uuid,
}

// --- Async Breeding API (Job-based) ---

#[derive(Debug, Deserialize)]
pub struct BreedStartResponse {
    pub job_id: Uuid,
    pub message: String,
    pub locked_kaiju: Vec<Uuid>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum BreedingJobStatus {
    Pending,
    Generating,
    Complete,
    Failed,
}

#[derive(Debug, Deserialize)]
struct ServerBreedStatusResponse {
    pub job_id: Uuid,
    pub status: BreedingJobStatus,
    pub offspring: Option<ServerKaiju>,
    pub error_message: Option<String>,
}

#[derive(Debug)]
pub struct BreedStatusResponse {
    pub job_id: Uuid,
    pub status: BreedingJobStatus,
    pub offspring: Option<Kaiju>,
    pub error_message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LockedKaijuResponse {
    pub locked_ids: Vec<Uuid>,
}

/// Start an async breeding job (returns immediately)
pub fn start_breeding(
    parent_a: Uuid,
    parent_b: Uuid,
    seed: u64,
) -> Result<BreedStartResponse, String> {
    let client = reqwest::blocking::Client::new();
    let user_id = Uuid::nil();

    let request = BreedRequest {
        parent_a_id: parent_a,
        parent_b_id: parent_b,
        client_seed: seed,
        user_id,
    };

    println!("[CLIENT->SERVER] Starting Breeding Job");

    let response = client.post("http://localhost:3000/breeding/breed")
        .json(&request)
        .send()
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Server refused breeding: {}", response.status()));
    }

    response.json::<BreedStartResponse>()
        .map_err(|e| format!("Invalid server response: {}", e))
}

/// Check status of a breeding job
pub fn check_breeding_status(job_id: Uuid) -> Result<BreedStatusResponse, String> {
    let client = reqwest::blocking::Client::new();

    let response = client.get(format!("http://localhost:3000/breeding/status/{}", job_id))
        .send()
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to get status: {}", response.status()));
    }

    let net_resp = response.json::<ServerBreedStatusResponse>()
        .map_err(|e| format!("Invalid server response: {}", e))?;

    Ok(BreedStatusResponse {
        job_id: net_resp.job_id,
        status: net_resp.status,
        offspring: net_resp.offspring.map(|k| k.to_client_kaiju()),
        error_message: net_resp.error_message,
    })
}

/// Get list of locked (breeding) Kaiju
pub fn get_locked_kaiju() -> Result<Vec<Uuid>, String> {
    let client = reqwest::blocking::Client::new();

    let response = client.get("http://localhost:3000/breeding/locked")
        .send()
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to get locked kaiju: {}", response.status()));
    }

    let resp = response.json::<LockedKaijuResponse>()
        .map_err(|e| format!("Invalid response: {}", e))?;

    Ok(resp.locked_ids)
}

pub fn check_server_health() -> Result<(), String> {
    let client = reqwest::blocking::Client::new();
    let response = client.get("http://localhost:3000/health")
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .map_err(|e| format!("Connection failed: {}", e))?;
    
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Server returned error: {}", response.status()))
    }
}

// --- Login API ---

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub user_id: Option<Uuid>,
    pub starter_choice: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ServerLoginResponse {
    pub user_id: Uuid,
    pub gold: i64,
    pub roster: Vec<ServerKaiju>,
}

#[derive(Debug)]
pub struct LoginResponse {
    pub user_id: Uuid,
    pub gold: i64,
    pub roster: Vec<Kaiju>,
}

pub fn login_to_server(user_id: Option<Uuid>, starter_choice: Option<String>) -> Result<LoginResponse, String> {
    let client = reqwest::blocking::Client::new();
    let request = LoginRequest { user_id, starter_choice };

    println!("[CLIENT->SERVER] Sending Login Request");
    
    let response = client.post("http://localhost:3000/user/login")
        .json(&request)
        .send()
        .map_err(|e| format!("Network error: {}", e))?;

    println!("[SERVER->CLIENT] Login Response: {}", response.status());

    if !response.status().is_success() {
         return Err(format!("Login failed: {}", response.status()));
    }

    let net_resp = response.json::<ServerLoginResponse>()
        .map_err(|e| format!("Invalid login response: {}", e))?;

    Ok(LoginResponse {
        user_id: net_resp.user_id,
        gold: net_resp.gold,
        roster: net_resp.roster.into_iter().map(|k| k.to_client_kaiju()).collect(),
    })
}

// --- Marketplace API ---

#[derive(Debug, Clone, Deserialize)]
pub struct MarketplaceItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: i32,
    pub image_url: String,
    pub stats: KaijuStats,
}

#[derive(Debug, Deserialize)]
struct MarketplaceListResponse {
    pub items: Vec<MarketplaceItem>,
}

#[derive(Debug, Serialize)]
struct PurchaseRequest {
    pub user_id: Uuid,
    pub item_id: String,
}

#[derive(Debug, Deserialize)]
struct ServerPurchaseResponse {
    pub success: bool,
    pub kaiju: Option<ServerKaiju>,
    pub new_gold: i64,
    pub message: String,
}

#[derive(Debug)]
pub struct PurchaseResponse {
    pub success: bool,
    pub kaiju: Option<Kaiju>,
    pub new_gold: i64,
    pub message: String,
}

pub fn list_marketplace() -> Result<Vec<MarketplaceItem>, String> {
    let client = reqwest::blocking::Client::new();
    
    println!("[CLIENT->SERVER] Fetching Marketplace");
    
    let response = client.get("http://localhost:3000/marketplace/list")
        .send()
        .map_err(|e| format!("Network error: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Failed to fetch marketplace: {}", response.status()));
    }
    
    let data = response.json::<MarketplaceListResponse>()
        .map_err(|e| format!("Invalid marketplace response: {}", e))?;
    
    Ok(data.items)
}

pub fn purchase_kaiju(user_id: Uuid, item_id: &str) -> Result<PurchaseResponse, String> {
    let client = reqwest::blocking::Client::new();
    let request = PurchaseRequest {
        user_id,
        item_id: item_id.to_string(),
    };
    
    println!("[CLIENT->SERVER] Purchasing: {}", item_id);
    
    let response = client.post("http://localhost:3000/marketplace/purchase")
        .json(&request)
        .send()
        .map_err(|e| format!("Network error: {}", e))?;
    
    println!("[SERVER->CLIENT] Purchase Status: {}", response.status());
    let status = response.status();

    if !status.is_success() {
        let err_text = response.text().unwrap_or_default();
        println!("[SERVER->CLIENT] Purchase Error Body: {}", err_text);
        return Err(format!("Purchase failed: {} - {}", status, err_text));
    }
    
    let net_resp = response.json::<ServerPurchaseResponse>()
        .map_err(|e| format!("Invalid purchase response: {}", e))?;
    
    println!("[SERVER->CLIENT] Purchase Success! New Gold: {}", net_resp.new_gold);

    Ok(PurchaseResponse {
        success: net_resp.success,
        kaiju: net_resp.kaiju.map(|k| k.to_client_kaiju()),
        new_gold: net_resp.new_gold,
        message: net_resp.message,
    })
}

// --- Tournament API ---

#[derive(Debug, Clone, Deserialize)]
pub struct TournamentStatusDto {
    pub id: String,
    pub start_time: String,
    pub state: String,
    pub current_round: i32,
    pub participants_count: i64,
    pub matches: Vec<MatchDto>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MatchDto {
    pub round_number: i32,
    pub match_index: i32,
    pub kaiju_a_id: Option<String>,
    pub kaiju_b_id: Option<String>,
    pub kaiju_a_name: Option<String>,
    pub kaiju_b_name: Option<String>,
    pub winner_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct TournamentEnrollRequest {
    user_id: Uuid,
    kaiju_id: Uuid,
}

#[derive(Debug, Deserialize)]
struct TournamentEnrollResponse {
    success: bool,
    message: String,
}

pub fn get_current_tournament() -> Result<TournamentStatusDto, String> {
    let client = reqwest::blocking::Client::new();
    
    // println!("[CLIENT->SERVER] Fetching Tournament Status"); // Verbose
    
    let response = client.get("http://localhost:3000/tournament/")
        .send()
        .map_err(|e| format!("Network error: {}", e))?;
    
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err("No active tournament".to_string());
    }

    if !response.status().is_success() {
        return Err(format!("Failed to fetch tournament: {}", response.status()));
    }
    
    let data = response.json::<TournamentStatusDto>()
        .map_err(|e| format!("Invalid tournament response: {}", e))?;
    
    Ok(data)
}

pub fn enroll_in_tournament(user_id: Uuid, kaiju_id: Uuid) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let request = TournamentEnrollRequest {
        user_id,
        kaiju_id,
    };
    
    let response = client.post("http://localhost:3000/tournament/enroll")
        .json(&request)
        .send()
        .map_err(|e| format!("Network error: {}", e))?;
        
    if !response.status().is_success() {
        let err_text = response.text().unwrap_or_default();
        return Err(format!("Enrollment failed: {}", err_text));
    }
    
    let data = response.json::<TournamentEnrollResponse>()
        .map_err(|e| format!("Invalid response: {}", e))?;
        
    if data.success {
        Ok(data.message)
    } else {
        Err(data.message)
    }
}
