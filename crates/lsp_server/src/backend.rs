mod language_server;

use std::sync::Arc;

use tower_lsp::Client;

use crate::session::Session;

/// Defines the logic for handling LSP requests.
///
pub(crate) struct Backend {
    session: Arc<Session>,
}

impl Backend {
    #[must_use]
    pub(crate) fn new(client: Client) -> Self {
        Self {
            session: Arc::new(Session::new(client)),
        }
    }

    fn client(&self) -> &Client {
        self.session.client()
    }
}
