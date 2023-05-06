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
pub struct OpenGaliper {
    identifier: String,
    artufans: Vec<Artufan>,
}

impl OpenGaliper {
    fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
            artufans: Vec::new(),
        }
    }

    fn add_artufan(&mut self, artufan: &Artufan) {
        self.artufans.push(artufan.to_owned())
    }
}

#[derive(Debug, Clone)]
pub struct Galiper {
    identifier: String,
    happiness: usize,
    artufans: Vec<Artufan>,
}

impl Galiper {
    pub fn new(open_galiper: OpenGaliper) -> Self {
        Self {
            identifier: open_galiper.identifier,
            happiness: 0,
            artufans: open_galiper.artufans,
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
