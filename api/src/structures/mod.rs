pub mod monad;
pub mod dyad;
pub mod triad;
pub mod tetrad;
pub mod pentad;
pub mod hexad;
pub mod heptad;
pub mod octad;
pub mod dodecad;

// Re-export all structure types
pub use monad::{Monad, MonadBuilder};
pub use dyad::{Dyad, DyadBuilder};
pub use triad::{Triad, TriadBuilder};
pub use tetrad::{Tetrad, TetradBuilder};
pub use pentad::{Pentad, PentadBuilder};
pub use hexad::{Hexad, HexadBuilder};
pub use heptad::{Heptad, HeptadBuilder};
pub use octad::{Octad, OctadBuilder};
pub use dodecad::{Dodecad, DodecadBuilder}; 