pub mod pants_write_service;
pub mod pants_read_service;
pub mod shirt_write_service;
pub mod shirt_read_service;
pub mod outfit_write_service;
pub mod outfit_read_service;

pub use pants_write_service::PantsWriteService;
pub use pants_read_service::PantsReadService;
pub use shirt_write_service::ShirtWriteService;
pub use shirt_read_service::ShirtReadService;
pub use outfit_write_service::OutfitWriteService;
pub use outfit_read_service::OutfitReadService;
