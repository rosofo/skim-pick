use std::{
    borrow::Cow,
    collections::HashMap,
    hash::Hash,
    num,
    str::FromStr,
    sync::{mpsc::sync_channel, Arc},
};

use crossbeam_channel::{bounded, unbounded};
use skim::{prelude::SkimItemReader, Skim, SkimItem, SkimOptions};

fn main() {
    let a = pick(["a", "b", "c"]);
    let b = pick([1, 2, 3]);
}

fn pick<T: ToString + Send + Sync + 'static>(items: impl IntoIterator<Item = T>) -> Option<T> {
    let config = SkimOptions::default();

    let (tx, rx) = unbounded();
    for item in items {
        let item: Arc<dyn SkimItem> = Arc::new(Item(Some(item)));
        tx.send(item).ok()?
    }
    drop(tx);

    let choice = Skim::run_with(&config, Some(rx))?;
    let mut choice = choice.selected_items.into_iter().next()?;
    let item = Arc::get_mut(&mut choice)?
        .as_any_mut()
        .downcast_mut::<Item<_>>()?;
    item.0.take()
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
