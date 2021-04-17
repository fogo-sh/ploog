use crate::GenerateSite;
use crate::Ploog;
use notify::{
    event::{AccessKind, AccessMode},
    Error, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};

pub trait Watch {
    fn watch(&self) -> Result<RecommendedWatcher, notify::Error>;
}

impl Watch for Ploog {
    fn watch(&self) -> Result<RecommendedWatcher, notify::Error> {
        let self_arc = self.inner.clone();
        let mut watch: RecommendedWatcher =
            Watcher::new_immediate(move |result: Result<Event, Error>| {
                let event = result.unwrap();

                if event.kind == EventKind::Access(AccessKind::Close(AccessMode::Write)) {
                    self_arc.generate().unwrap();
                }
            })?;

        let watch_generate_inner = self.inner.clone();

        watch
            .watch(&watch_generate_inner.source_path, RecursiveMode::Recursive)
            .unwrap();

        watch_generate_inner.generate().unwrap();

        Ok(watch)
    }
}
