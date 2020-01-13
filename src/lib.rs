use std::collections::HashMap;
use std::hash::Hash;

struct Links<T: Clone> {
    parent: Option<T>,
    children: Vec<T>,
}

#[derive(Default)]
pub struct MappedTree<T: Clone + Eq + Hash> {
    links_by_obj: HashMap<T, Links<T>>,
}

impl<T: Clone + Eq + Hash> MappedTree<T> {
    pub fn new() -> Self {
        MappedTree {
            links_by_obj: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        MappedTree {
            links_by_obj: HashMap::with_capacity(capacity),
        }
    }

    fn links(&self, obj: &T) -> Option<&Links<T>> {
        self.links_by_obj.get(obj)
    }

    pub fn parent(&self, obj: &T) -> Option<&T> {
        if let Some(links) = self.links(obj) {
            if let Some(parent) = &links.parent {
                return Some(parent);
            }
        }
        None
    }

    pub fn children(&self, obj: &T) -> Option<&Vec<T>> {
        if let Some(links) = self.links(obj) {
            return Some(&links.children);
        }
        None
    }

    pub fn insert(&mut self, obj: &T, parent: &T) {
        if self.links_by_obj.contains_key(obj) {
            panic!("mapped tree MUST contain UNIQUE elements only");
        }

        let parent_links = self
            .links_by_obj
            .get_mut(parent)
            .expect("mapped tree don't contain specified parent");

        parent_links.children.push(obj.clone());

        self.links_by_obj.insert(
            obj.clone(),
            Links {
                parent: Some(parent.clone()),
                children: Vec::new(),
            },
        );
    }

    fn remove_children_without_links(&mut self, obj: &T) {
        if let Some(children) = self.children(obj) {
            let children = children.clone();
            for child in &children {
                self.remove_children(&child);
            }

            for child in &children {
                self.links_by_obj.remove(&child);
            }
        }
    }

    pub fn remove_children(&mut self, obj: &T) {
        self.remove_children_without_links(obj);
        if let Some(links) = self.links_by_obj.get_mut(obj) {
            links.children.clear();
        }
    }

    pub fn remove(&mut self, obj: &T) -> bool {
        self.remove_children(obj);
        if let Some(links) = self.links_by_obj.remove(obj) {
            if let Some(parent) = &links.parent {
                let parent_links = self.links_by_obj.get_mut(parent).unwrap();
                let obj_index = parent_links
                    .children
                    .iter()
                    .position(|item| *item == *obj)
                    .unwrap();
                parent_links.children.swap_remove(obj_index);
            }
            return true;
        }

        false
    }

    pub fn contains(&self, obj: &T) -> bool {
        self.links_by_obj.contains_key(obj)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
