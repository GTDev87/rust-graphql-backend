use diesel_logger::LoggingConnection;
use crate::db;
use std::collections::HashMap;
use diesel::pg::PgConnection;
use anyhow::{Result, Error};
use std::sync::Arc;


pub fn load_by_ids<U, IdFn, LoadFn>(
    load_fn: LoadFn,
    id_fn: IdFn,
) -> impl Fn(&[i32]) -> Result<HashMap<i32, U>, Error>
where
    U: Send + 'static,
    IdFn: Fn(&U) -> i32 + Clone,
    LoadFn: Fn(&mut LoggingConnection<PgConnection>, &[i32]) -> Result<Vec<U>, Error> + Clone,
{
    move |ids: &[i32]| {
        let mut conn = db::establish_connection();

        // Call the load function to get the results
        let results = load_fn(&mut conn, ids)?;

        // Convert the results into a HashMap
        let result_map = results
            .into_iter()
            .map(|item| (id_fn(&item), item))
            .collect();

        Ok(result_map)
    }
}

// let users = load_by_ids(
//     |conn, ids| {
//         use crate::schema::users::dsl::{users, id};
//         users
//             .filter(id.eq_any(ids))
//             .load::<User>(conn)
//     },
//     |user: &User| user.id,
//     &[1, 2, 3],
// )?;



pub fn handle_found_items<K, V>(
    keys: &[K],
    found_items: Result<HashMap<K, V>, anyhow::Error>,
) -> HashMap<K, Result<V, Arc<anyhow::Error>>>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    match found_items {
        Ok(found_items) => found_items.into_iter().map(|(id, item)| (id, Ok(item))).collect(),
        Err(e) => {
            // Since `anyhow::Error` doesn't implement `Clone`, we have to
            // work around here.
            let e = Arc::new(e);
            keys.iter().map(|k| (k.clone(), Err(e.clone()))).collect()
        }
    }
}