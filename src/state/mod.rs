mod health;
mod pipeline;
mod services;
mod sources;

#[derive(Debug, Clone)]
pub struct Roverd {
    pub name: String,
}

impl Roverd {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl AsRef<Roverd> for Roverd {
    fn as_ref(&self) -> &Roverd {
        self
    }
}
