// use std::collections::HashMap;
// use uuid::Uuid;
//
// pub struct Store<T> {
//     sys: HashMap<Uuid, T>
// }
//
// pub trait StoreApi {
//     fn new();
// }
//
// impl Store<T> {
//     pub fn new() -> Self<T> {
//         Self {
//             sys: HashMap::new()
//         }
//     }
//     pub fn add_item(&mut self, item: T) -> &mut Store<T>
//     {
//         let uuid = Uuid::new_v4();
//         self.sys.insert(uuid, item);
//         self
//     }
// }
