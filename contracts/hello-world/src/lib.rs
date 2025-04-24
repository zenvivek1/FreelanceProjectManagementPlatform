#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Project {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub client: String,
    pub assigned: bool,
}

#[contracttype]
pub enum ProjectBook {
    Project(u64),
}

const PROJECT_COUNT: Symbol = symbol_short!("COUNT");

#[contract]
pub struct FreelancePMContract;

#[contractimpl]
impl FreelancePMContract {
    pub fn create_project(env: Env, title: String, description: String, client: String) -> u64 {
        let mut count = env.storage().instance().get(&PROJECT_COUNT).unwrap_or(0);
        count += 1;

        let project = Project {
            id: count,
            title,
            description,
            client,
            assigned: false,
        };

        env.storage().instance().set(&ProjectBook::Project(count), &project);
        env.storage().instance().set(&PROJECT_COUNT, &count);
        count
    }

    pub fn assign_freelancer(env: Env, project_id: u64) {
        let mut project: Project = env
            .storage()
            .instance()
            .get(&ProjectBook::Project(project_id))
            .expect("Project not found");

        if project.assigned {
            panic!("Project already assigned");
        }

        project.assigned = true;
        env.storage().instance().set(&ProjectBook::Project(project_id), &project);
    }

    pub fn view_project(env: Env, project_id: u64) -> Project {
        env.storage()
            .instance()
            .get(&ProjectBook::Project(project_id))
            .unwrap_or(Project {
                id: 0,
                title: String::from_str(&env, "Not Found"),
                description: String::from_str(&env, "Not Found"),
                client: String::from_str(&env, "Unknown"),
                assigned: false,
            })
    }
}
