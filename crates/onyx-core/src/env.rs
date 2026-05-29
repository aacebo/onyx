use std::collections::HashMap;
use std::sync::Arc;

use crate::{fs, net};

pub fn new() -> Builder {
    Builder::default()
}

#[derive(Clone)]
pub struct Environment {
    _name: Option<String>,
    _file_system: Arc<dyn fs::FileSystem>,
    _data_sources: HashMap<String, Arc<dyn net::DataSource>>,
}

impl Environment {
    pub fn name(&self) -> Option<&str> {
        self._name.as_deref()
    }

    pub fn file_system(&self) -> &dyn fs::FileSystem {
        self._file_system.as_ref()
    }

    pub fn data_source(&self, key: impl AsRef<str>) -> Option<&dyn net::DataSource> {
        self._data_sources.get(key.as_ref()).map(|v| v.as_ref())
    }
}

#[derive(Default, Clone)]
pub struct Builder {
    _name: Option<String>,
    _data_sources: HashMap<String, Arc<dyn net::DataSource>>,
}

impl Builder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self._name = Some(name.into());
        self
    }

    pub fn file_system<T: fs::FileSystem + 'static>(self, value: T) -> WithFileSystem {
        // self._file_system = Some(Arc::new(value));
        WithFileSystem {
            inner: self,
            file_system: Arc::new(value),
        }
    }

    pub fn data_source<T: net::DataSource + 'static>(mut self, key: impl Into<String>, value: T) -> Self {
        self._data_sources.insert(key.into(), Arc::new(value));
        self
    }
}

pub struct WithFileSystem {
    inner: Builder,
    file_system: Arc<dyn fs::FileSystem>,
}

impl WithFileSystem {
    pub fn data_source<T: net::DataSource + 'static>(mut self, key: impl Into<String>, value: T) -> Self {
        self.inner = self.inner.data_source(key, value);
        self
    }

    pub fn build(self) -> Environment {
        Environment {
            _name: self.inner._name,
            _file_system: self.file_system,
            _data_sources: self.inner._data_sources,
        }
    }
}
