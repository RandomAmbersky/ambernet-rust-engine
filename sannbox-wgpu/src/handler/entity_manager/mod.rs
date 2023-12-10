use crate::handler::info_loader::{load_data, load_lang, DataSet, LangSet};

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

fn consistency_check(data_set: &DataSet, lang_set: &LangSet) {}
