use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, Mutex};

pub fn init_store() -> Arc<Mutex<HashMap<String, String>>> {
    let store: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    store
}

pub fn set_kv(
    store: Arc<Mutex<HashMap<String, String>>>,
    k: String,
    v: String,
    w: &mut impl Write,
) -> anyhow::Result<()> {
    let mut map = store.lock().unwrap();
    writeln!(w, "inserted: {k}")?;
    map.insert(k, v);
    Ok(())
}

pub fn del_kv(
    store: Arc<Mutex<HashMap<String, String>>>,
    k: String,
    w: &mut impl Write,
) -> anyhow::Result<()> {
    let mut map = store.lock().unwrap();
    map.remove(&k);
    writeln!(w, "deleted: {k}")?;
    Ok(())
}

pub fn get_v(store: Arc<Mutex<HashMap<String, String>>>, k: String) -> anyhow::Result<String> {
    let map = store.lock().unwrap();
    if let Some(val) = map.get(&k) {
        Ok(val.clone())
    } else {
        anyhow::bail!("key not found!");
    }
}

pub fn print_kvs(
    store: Arc<Mutex<HashMap<String, String>>>,
    w: &mut impl Write,
) -> anyhow::Result<()> {
    let map = store.lock().unwrap();
    let iter = map.iter();

    for (k, v) in iter {
        writeln!(w, "{k}:{v}")?;
    }

    Ok(())
}
