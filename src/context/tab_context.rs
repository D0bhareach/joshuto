use std::collections::hash_map::IterMut;
use std::collections::HashMap;

use uuid::Uuid;

use crate::tab::JoshutoTab;

pub struct TabContext {
    pub index: usize,
    pub tab_order: Vec<Uuid>,
    tabs: HashMap<Uuid, JoshutoTab>,
}

impl std::default::Default for TabContext {
    fn default() -> Self {
        Self {
            index: 0,
            tab_order: Vec::with_capacity(5),
            tabs: HashMap::with_capacity(5),
        }
    }
}
impl TabContext {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn len(&self) -> usize {
        self.tab_order.len()
    }

    pub fn tab_ref(&self, id: &Uuid) -> Option<&JoshutoTab> {
        self.tabs.get(id)
    }
    pub fn tab_mut(&mut self, id: &Uuid) -> Option<&mut JoshutoTab> {
        self.tabs.get_mut(id)
    }

    pub fn curr_tab_id(&self) -> Uuid {
        self.tab_order[self.index]
    }
    pub fn curr_tab_ref(&self) -> &JoshutoTab {
        let id = &self.tab_order[self.index];
        self.tabs.get(id).unwrap()
    }
    pub fn curr_tab_mut(&mut self) -> &mut JoshutoTab {
        let id = &self.tab_order[self.index];
        self.tabs.get_mut(id).unwrap()
    }

    pub fn insert_tab(&mut self, id: Uuid, tab: JoshutoTab) {
        self.tabs.insert(id, tab);
        self.tab_order.push(id);
    }

    pub fn remove_tab(&mut self, id: &Uuid) -> Option<JoshutoTab> {
        let tab = self.tabs.remove(id);
        for i in 0..self.tab_order.len() {
            if self.tab_order[i] == *id {
                self.tab_order.remove(i);
                break;
            }
        }
        tab
    }

    pub fn iter_mut(&mut self) -> IterMut<Uuid, JoshutoTab> {
        self.tabs.iter_mut()
    }
}
