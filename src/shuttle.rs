// Customize this struct with things from `shuttle_main` needed in `bind`,
// such as secrets or database connections
struct ShuttleService {}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for ShuttleService {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        // Start your service and bind to the socket address
        Ok(())
    }
}