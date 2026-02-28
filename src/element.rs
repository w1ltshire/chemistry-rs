use serde::{Deserialize, Serialize};

/// Structure representing a chemical element.
///
/// Fields taken from https://github.com/Bowserinator/Periodic-Table-JSON
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone, PartialOrd)]
pub struct Element {
    /// Element name
    pub name: String,
    /// Element appearance
    pub appearance: Option<String>,
    /// Element atomic mass
    pub atomic_mass: f64,
    /// Element boiling temperature
    pub boil: Option<f64>,
    /// Element category
    pub category: Option<String>,
    /// Element density
    pub density: Option<f64>,
    /// Who discovered this element
    pub discovered_by: Option<String>,
    /// Element melting temperature
    pub melt: Option<f64>,
    /// Element molar heat
    pub molar_heat: Option<f64>,
    /// Who named this element
    pub named_by: Option<String>,
    /// Element periodic number
    pub number: u16,
    /// Element period
    pub period: u8,
    /// Element group
    pub group: u8,
    /// Element phase
    pub phase: String,
    /// Element information source link
    pub source: String,
    /// Summary of this element
    pub summary: String,
    /// Element symbol
    pub symbol: String,
    /// Element shells
    pub shells: Vec<u8>,
    /// Element electron configuration
    pub electron_configuration: String,
    /// Element electron configuration semantic
    pub electron_configuration_semantic: String,
    /// Electron affinity
    pub electron_affinity: Option<f64>,
    /// Element electronegativity
    pub electronegativity_pauling: Option<f64>,
    /// Element ionization energies
    pub ionization_energies: Vec<f64>,
    /// Element CPK hex colors
    #[serde(rename = "cpk-hex")]
    pub cpk_hex: Option<String>,
}
