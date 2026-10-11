use crate::browser::ChromeTab;
use parking_lot::RwLock;
use std::sync::Arc;

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Default)]
pub struct Context {
    tabs: Arc<RwLock<Vec<ChromeTab>>>,
}

impl Context {
    ////////////////////////////////////////////////////////////////////////////////
    pub fn set(&self, tabs: Vec<ChromeTab>) {
        *self.tabs.write() = tabs;
    }

    ////////////////////////////////////////////////////////////////////////////////
    pub fn all(&self) -> Vec<ChromeTab> {
        self.tabs.read().clone()
    }

    ////////////////////////////////////////////////////////////////////////////////
    pub fn get(&self, id: &str) -> Option<ChromeTab> {
        let tabs = self.tabs.read();
        tabs.iter().find(|tab| tab.id().as_ref() == id).cloned()
    }
}
