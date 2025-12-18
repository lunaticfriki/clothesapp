use shaku::module;

use crate::{
    infrastructure::http::handlers::{OutfitHandlers, PantsHandlers, ShirtHandlers},
    application::{
        OutfitReadService, OutfitWriteService,
        PantsReadService, PantsWriteService,
        ShirtReadService, ShirtWriteService,
    },
    infrastructure::persistence::{
        InMemoryOutfitRepository, InMemoryPantsRepository, InMemoryShirtRepository,
    },
};

module! {
    pub AppModule {
        components = [
            InMemoryPantsRepository,
            InMemoryShirtRepository,
            InMemoryOutfitRepository,
            PantsReadService,
            PantsWriteService,
            ShirtReadService,
            ShirtWriteService,
            OutfitReadService,
            OutfitWriteService,
            PantsHandlers,
            ShirtHandlers,
            OutfitHandlers,
        ],
        providers = []
    }
}
