pub mod pants_repository;
pub mod shirt_repository;
pub mod outfit_repository;

pub use pants_repository::{PantsRepository, DynPantsRepository};
pub use shirt_repository::{ShirtRepository, DynShirtRepository};
pub use outfit_repository::{OutfitRepository, DynOutfitRepository};
