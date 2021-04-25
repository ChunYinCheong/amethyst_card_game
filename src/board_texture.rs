use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    prelude::WorldExt,
    renderer::{ImageFormat, Texture},
    shred::World,
};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct BoardTexture {
    handles: HashMap<String, Handle<Texture>>,
}

impl BoardTexture {
    pub fn load(&mut self, world: &World, path: &str, progress_counter: &mut ProgressCounter) {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                path,
                ImageFormat::default(),
                progress_counter,
                &texture_storage,
            )
        };
        self.handles.insert(path.to_string(), texture_handle);
    }

    pub fn get(&self, path: &str) -> Handle<Texture> {
        if self.handles.contains_key(path) {
            self.handles[path].clone()
        } else {
            panic!("Key not found: {}", path);
        }
    }
}
