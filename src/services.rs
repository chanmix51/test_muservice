use std::sync::Arc;

use super::model::*;
use super::StdResult;

pub trait OpenGaliperService {
    fn create_open_galiper(&self, name: &str, artufan: &Artufan) -> StdResult<OpenGaliper>;

    fn register_artufan(&mut self, artufan: &Artufan, galiper_id: &str) -> StdResult<()>;

    fn close_registration(&self) -> StdResult<OpenGaliper>;
}

pub trait GaliperService {
    fn create(&self, open_galiper: OpenGaliper) -> StdResult<Galiper>;

    fn galip(&self, artufan: &Artufan, galiper_id: &str) -> StdResult<()>;

    fn cancel(&self, galiper_id: &str) -> StdResult<FinishedGaliper>;

    fn flash(&self, galiper_id: &str) -> StdResult<FinishedGaliper>;
}

pub trait DownloadService {
    fn list(&self) -> StdResult<Vec<GaliperStatistics>>;

    fn download(&self, galiper_id: &str) -> StdResult<GaliperStatistics>;
}

pub struct ServiceContainer {
    pub open_galiper_service: Arc<dyn OpenGaliperService>,
    pub galiper_service: Arc<dyn GaliperService>,
    pub download_service: Arc<dyn DownloadService>,
}
