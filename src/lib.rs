use std::collections::HashMap;
use std::hash::Hash;

struct Links<T: Clone> {
    parent: Option<T>,
    children: Vec<T>,
}

#[derive(Default)]
pub struct MappedTree<T: Clone + Eq + Hash> {
    links_by_obj: HashMap<T, Links<T>>,
    size: usize,
}

impl<T: Clone + Eq + Hash> MappedTree<T> {
    pub fn new() -> Self {
        MappedTree {
            links_by_obj: HashMap::new(),
            size: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        MappedTree {
            links_by_obj: HashMap::with_capacity(capacity),
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
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

    pub fn root(&self) -> Option<T> {
        if let Some(next) = self.links_by_obj.keys().next() {
            let mut next = next.clone();
            while let Some(parent) = self.parent(&next) {
                next = parent.clone();
            }
            return Some(next);
        }

        None
    }

    pub fn reset_root(&mut self, obj: &T) -> Option<T> {
        if let Some(old_root) = self.root() {
            let children = self.children(&old_root).unwrap().clone();
            for child in &children {
                self.links_by_obj.get_mut(child).unwrap().parent = Some(obj.clone());
            }

            self.links_by_obj.remove(&old_root);
            let links = Links {
                parent: None,
                children: children,
            };
            self.links_by_obj.insert(obj.clone(), links);
            return Some(old_root);
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
            .expect("mapped tree doesn't contain specified parent");

        parent_links.children.push(obj.clone());

        self.links_by_obj.insert(
            obj.clone(),
            Links {
                parent: Some(parent.clone()),
                children: Vec::new(),
            },
        );
        self.size += 1;
    }

    fn remove_children_without_links(&mut self, obj: &T) {
        if let Some(children) = self.children(obj) {
            let children = children.clone();
            for child in &children {
                self.remove_children(&child);
            }

            let children_count = children.len();
            for child in &children {
                self.links_by_obj.remove(&child);
            }
            self.size -= children_count;
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
            self.size -= 1;
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
    use super::MappedTree;

    #[test]
    fn size() {
        let vals = vec![0, 2, 4, 6, 8, 9];
        let map: MappedTree<i32> = MappedTree::new();
    }
}
