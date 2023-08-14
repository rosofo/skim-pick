use std::{borrow::Cow, sync::Arc};

use crossbeam_channel::unbounded;
use skim::{prelude::SkimOptionsBuilder, Skim, SkimItem, SkimOptions};

pub trait SkimPick {
    fn pick<T: ToString + Send + Sync + 'static>(
        self,
        items: impl IntoIterator<Item = T>,
    ) -> Option<T>;
}
impl SkimPick for SkimOptions<'_> {
    fn pick<T: ToString + Send + Sync + 'static>(
        self,
        items: impl IntoIterator<Item = T>,
    ) -> Option<T> {
        pick_with_options(items, self)
    }
}
impl SkimPick for SkimOptionsBuilder<'_> {
    fn pick<T: ToString + Send + Sync + 'static>(
        mut self,
        items: impl IntoIterator<Item = T>,
    ) -> Option<T> {
        pick_with_options(items, self.build().ok()?)
    }
}

pub fn pick_with_options<T: ToString + Send + Sync + 'static>(
    items: impl IntoIterator<Item = T>,
    options: SkimOptions,
) -> Option<T> {
    let (tx, rx) = unbounded();
    for item in items {
        let item: Arc<dyn SkimItem> = Arc::new(Item(Some(item)));
        tx.send(item).ok()?
    }
    drop(tx);

    let choice = Skim::run_with(&options, Some(rx))?;
    let mut choice = choice.selected_items.into_iter().next()?;
    let item = Arc::get_mut(&mut choice)?
        .as_any_mut()
        .downcast_mut::<Item<_>>()?;
    item.0.take()
}

pub fn pick<T: ToString + Send + Sync + 'static>(items: impl IntoIterator<Item = T>) -> Option<T> {
    let config = SkimOptions::default();
    pick_with_options(items, config)
}

struct Item<T>(Option<T>);
impl<T: ToString + Send + Sync + 'static> SkimItem for Item<T> {
    fn text(&self) -> Cow<str> {
        if let Some(ref v) = self.0 {
            Cow::Owned(v.to_string())
        } else {
            Cow::Borrowed("")
        }
    }
}
