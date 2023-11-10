// FILE: ./lib.rs
// ___________________________________________________________

pub mod utils {
    pub mod error;
    pub mod utilities;
}

pub mod models {
    pub mod health;
    pub mod login;
}

pub mod web {
    pub mod auth {
        pub mod routes_login;
    }
    pub mod handlers;
    pub mod router;
    pub mod run_server;
}
// ___________________________________________________________
// ___________________________________________________________
