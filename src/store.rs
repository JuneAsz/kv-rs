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
        Ok(String::from("key dont exist"))
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

    writeln!(w)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let store = init_store();
        let mut sink = std::io::sink();
        set_kv(
            Arc::clone(&store),
            "foo".to_string(),
            "bar".to_string(),
            &mut sink,
        )
        .unwrap();
        assert_eq!(get_v(Arc::clone(&store), "foo".to_string()).unwrap(), "bar");
    }

    #[test]
    fn test_get_missing_key() {
        let store = init_store();
        assert!(get_v(Arc::clone(&store), "nonexistent".to_string()).is_err());
    }

    #[test]
    fn test_del() {
        let store = init_store();
        let mut sink = std::io::sink();
        set_kv(
            Arc::clone(&store),
            "foo".to_string(),
            "bar".to_string(),
            &mut sink,
        )
        .unwrap();
        del_kv(Arc::clone(&store), "foo".to_string(), &mut sink).unwrap();
        assert!(get_v(Arc::clone(&store), "foo".to_string()).is_err());
    }
}
