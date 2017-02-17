use std::collections::hash_map::{Entry, HashMap};
use sfml::graphics::{Texture, IntRect};
use std::cell::RefCell;
use std::thread::current;

fn create_null_texture() -> *const Texture {
    let mut text = Texture::new_from_memory(
        include_bytes!("missing_texture.png"),
        &IntRect::new(0, 0, 8, 8)
    ).unwrap();
    text.set_repeated(true);
    Box::into_raw(Box::new(text)) as *const Texture
}

fn create_texture_map() -> HashMap<String, *const Texture> {
    let mut hm = HashMap::new();
    hm.insert("null".to_string(), TEXTURE_NULL.with(|x| *x));
    hm
}

thread_local! {
    static TEXTURES:RefCell<HashMap<String,*const Texture>>=RefCell::new(create_texture_map());
    static TEXTURE_PATH:RefCell<Option<String>>=RefCell::new(None);
    static TEXTURE_NULL:*const Texture=create_null_texture();
}

pub fn init_thread_texture_path(path: &str) {
    TEXTURE_PATH.with(|cell| {
        let mut opt = cell.borrow_mut();
        if opt.is_some() {
            panic!("texture path set more than once in thread {:?}", current().name());
        }
        *opt = Some(path.to_string());
    });
}

pub fn get(name: &str) -> &'static Texture {
    TEXTURES.with(|texture_cell| {
        let mut textures = texture_cell.borrow_mut();
        let ptr_opt: *const Texture = match textures.entry(name.to_string()) {
            Entry::Occupied(e) => {
                *e.get()
            },
            Entry::Vacant(e) => {
                TEXTURE_PATH.with(|path| {
                    let path = match *path.borrow() {
                        Some(ref p) => p.clone() + name,
                        None => panic!(format!("no texture path set in thread {:?}", current().name())),
                    };
                    match Texture::new_from_file(&path) {
                        Some(texture) => {
                            let tex_ptr = Box::into_raw(Box::new(texture));
                            *e.insert(tex_ptr)
                        },
                        None => {
                            use std::io::{stderr, Write};
                            writeln!(&mut stderr(), "error loading texture {:?} from {:?}", name, path).is_ok();
                            let ptr = TEXTURE_NULL.with(|n| *n);
                            *e.insert(ptr)
                        }
                    }
                })
            }
        };
        unsafe { ptr_opt.as_ref::<'static>().unwrap() }
    })
}