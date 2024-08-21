use crate::handler::info_loader::{load_data, load_lang, DataSet, LangSet};
use asn_logger::info;

pub struct EntityManager {
    data_set: DataSet,
    lang_set: LangSet,
}

impl EntityManager {
    pub fn new(data: &[u8], lang: &[u8]) -> Self {
        let data_set = load_data(data);
        let lang_set = load_lang(lang);
        consistency_check(&data_set, &lang_set);
        Self { data_set, lang_set }
    }
}

fn consistency_check(data_set: &DataSet, lang_set: &LangSet) {
    data_set.entity_list.iter().for_each(|entry| {
        let (index, info) = entry;
        info!("entry: {:?} {:?}", index, info);
        let consistency = lang_set.entity_list.contains_key(index);
        assert!(consistency, "index '{}' not found into lang file!", index)
    });

    lang_set.entity_list.iter().for_each(|entry| {
        let (index, info) = entry;
        info!("entry: {:?} {:?}", index, info);
        let consistency = data_set.entity_list.contains_key(index);
        assert!(consistency, "index '{}' not found into data file!", index)
    })
}
