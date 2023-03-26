use std::error::Error;

use clap::__macro_refs::once_cell::sync::OnceCell;
use surrealdb::Datastore;

pub static DB: OnceCell<Datastore> = OnceCell::new();

pub async fn init() -> Result<(), Error> {}
