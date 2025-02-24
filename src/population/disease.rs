#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiseaseName {
    Bacteria, // should probably be real name and not diseases type
              //...
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiseaseLethality {
    Deadly,
    Moderate,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiseaseContagionRate {
    High,
    Mederate,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Disease {
    pub name: DiseaseName,
    pub lethality: DiseaseLethality,
    pub contagion: DiseaseContagionRate,
}