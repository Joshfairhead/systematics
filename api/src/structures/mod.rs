pub mod monad;
pub mod dyad;
pub mod triad;
// Add other structure modules as needed

// Re-export all structure types
pub use monad::{Monad, MonadBuilder};
pub use dyad::{Dyad, DyadBuilder};
pub use triad::{Triad, TriadBuilder}; 