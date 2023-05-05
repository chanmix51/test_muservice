// The pitch:
// Artufans want to galip
// 1 - Create and register to a galiper
// 2 - Validate and galip until happy
// 3 - Cancel or flash the galiper
// 4 - download a finished galiper_statistics
//
// artufan // open_galiper // galiper // finished_galiper

use std::error::Error;

#[derive(Debug, Clone)]
pub struct Artufan {
    name: String,
}

impl Artufan {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OpenGaliper<'a> {
    identifier: String,
    artufans: Vec<&'a Artufan>,
}

impl<'a> OpenGaliper<'a> {
    fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
            artufans: Vec::new(),
        }
    }

    fn add_artufan(&mut self, artufan: &'a Artufan) {
        self.artufans.push(artufan)
    }
}

#[derive(Debug, Clone)]
pub struct Galiper {
    identifier: String,
    happiness: usize,
    artufans: Vec<Artufan>,
}

impl Galiper {
    pub fn new<'a>(open_galiper: OpenGaliper<'a>) -> Self {
        let artufans: Vec<Artufan> = open_galiper
            .artufans
            .iter()
            .map(|&f| f.to_owned())
            .collect();

        Self {
            identifier: open_galiper.identifier,
            happiness: 0,
            artufans,
        }
    }

    pub fn galip(&mut self) {
        self.happiness += 1;
    }
}

pub enum GaliperOutcome {
    Flashed,
    Canceled,
}

pub struct FinishedGaliper {
    identifier: String,
    happiness: usize,
    artufans: Vec<Artufan>,
    outcome: GaliperOutcome,
}

impl FinishedGaliper {
    pub fn new(galiper: Galiper, outcome: GaliperOutcome) -> Self {
        Self {
            identifier: galiper.identifier,
            happiness: galiper.happiness,
            artufans: galiper.artufans,
            outcome,
        }
    }
}

pub struct GaliperStatistics {
    galiper: FinishedGaliper,
    downloads: usize,
    slug: String,
    is_available: bool,
}

pub type StdError = Box<dyn Error + Sync + Send>;
pub type StdResult<T> = Result<T, StdError>;

pub trait OpenGaliperService<'a> {
    fn create_open_galiper(&self, name: &str, artufan: &'a Artufan) -> StdResult<OpenGaliper<'a>>;

    fn register_artufan(&mut self, artufan: &'a Artufan, galiper_id: &str) -> StdResult<()>;

    fn close_registration(&self) -> StdResult<OpenGaliper<'a>>;
}

pub trait GaliperService {
    fn create<'a>(&self, open_galiper: OpenGaliper<'a>) -> StdResult<Galiper>;

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
