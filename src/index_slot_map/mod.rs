use indexmap::IndexSet;
use slotmap::{DefaultKey, SecondaryMap, SlotMap, new_key_type};

pub struct PriorotyIndexSlotMap<T> {
    slot_map: SlotMap<DefaultKey, T>,
    priority_secondary_map: SecondaryMap<DefaultKey, usize>,
    index_set: IndexSet<DefaultKey>,
}

impl<T> PriorotyIndexSlotMap<T> {
    pub fn new() -> Self {
        Self {
            slot_map: SlotMap::new(),
            priority_secondary_map: SecondaryMap::new(),
            index_set: IndexSet::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            slot_map: SlotMap::with_capacity(capacity),
            priority_secondary_map: SecondaryMap::with_capacity(capacity),
            index_set: IndexSet::with_capacity(capacity),
        }
    }

    pub fn insert_prioritezed(&mut self, priority: usize, value: T) -> DefaultKey {
        let key = self.slot_map.insert(value);
        self.priority_secondary_map.insert(key, priority);
        self.index_set.insert(key);

        self.index_set.sort_by(|a, b| {
            let a_priority = self.priority_secondary_map.get(*a).unwrap();
            let b_priority = self.priority_secondary_map.get(*b).unwrap();
            a_priority.cmp(b_priority)
        });
        key
    }

    pub fn shift_remove(&mut self, key: DefaultKey) -> Option<T> {
        let main_val = self.slot_map.remove(key);
        let sec_val = self.priority_secondary_map.remove(key);
        let success = self.index_set.shift_remove(&key);
        debug_assert!(
            (main_val.is_some() && sec_val.is_some() && success)
                || main_val.is_none() && sec_val.is_none() && !success,
        );
        main_val
    }

    pub fn last(&self) -> Option<DefaultKey> {
        self.index_set.last().copied()
    }
    pub fn last_value(&self) -> Option<&T> {
        self.index_set
            .last()
            .and_then(|key| self.slot_map.get(*key))
    }
}
