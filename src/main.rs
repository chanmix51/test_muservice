// The pitch:
// Artufans want to galip
// 1 - Create and register to a galiper
// 2 - Validate and galip until happy
// 3 - Cancel or flash the galiper
// 4 - download a finished galiper_statistics
//
// artufan // open_galiper // galiper // finished_galiper

use test_muservice::model::*;

use test_muservice::StdResult;

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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {}
