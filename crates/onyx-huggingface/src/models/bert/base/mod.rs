use std::cell::LazyCell;

use onyx_core::{Resource, resources::RemoteResource};

pub const CONFIG: LazyCell<RemoteResource> = LazyCell::new(|| {
    RemoteResource::parse(
        "https://huggingface.co/google-bert/bert-base-uncased/resolve/main/config.json",
    )
    .unwrap()
});

pub const MODEL: LazyCell<RemoteResource> = LazyCell::new(|| {
    RemoteResource::parse(
        "https://huggingface.co/google-bert/bert-base-uncased/resolve/main/model.safetensors",
    )
    .unwrap()
});

pub const VOCAB: LazyCell<RemoteResource> = LazyCell::new(|| {
    RemoteResource::parse(
        "https://huggingface.co/google-bert/bert-base-uncased/resolve/main/vocab.json",
    )
    .unwrap()
});
