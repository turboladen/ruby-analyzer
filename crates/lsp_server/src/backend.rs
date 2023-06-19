mod language_server;

use std::sync::Arc;

use tower_lsp::Client;

use crate::session::Session;

/// Defines the logic for handling LSP requests.
///
pub struct Backend {
    session: Arc<Session>,
}

impl Backend {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self {
            session: Arc::new(Session::new(client)),
        }
    }

    pub fn client(&self) -> &Client {
        self.session.client()
    }
}
