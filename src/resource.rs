use std::collections::HashSet;
use std::fs::read;
use std::path::Path;

use fontdue::layout::{CoordinateSystem, Layout};
use fontdue::{Font, FontSettings};
use image::io::Reader as ImageReader;

#[derive(Copy, Clone, Debug)]
pub struct ImageHandle {
    pub id: usize,
    _index: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct FontHandle {
    pub id: usize,
    _index: usize,
}

pub trait ImageResource {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn get_buf(&self) -> &[u8];
    fn get_buf_mut(&mut self) -> &mut [u8];
    fn get_buf_u32(&self) -> &[u32];
    fn get_buf_u32_mut(&mut self) -> &mut [u32];
}

pub struct Image {
    pub buf: Vec<u8>,
    _width: u32,
    _height: u32,
}

impl Image {
    pub fn new(width: u32, height: u32, buf: Vec<u8>) -> Self {
        Self {
            _width: width,
            _height: height,
            buf,
        }
    }
}

impl ImageResource for Image {
    #[inline]
    fn width(&self) -> u32 {
        self._width
    }
    #[inline]
    fn height(&self) -> u32 {
        self._height
    }
    #[inline]
    fn get_buf(&self) -> &[u8] {
        &self.buf
    }
    #[inline]
    fn get_buf_mut(&mut self) -> &mut [u8] {
        &mut self.buf
    }
    #[inline]
    fn get_buf_u32(&self) -> &[u32] {
        unsafe { &self.buf.align_to::<u32>().1 }
    }
    #[inline]
    fn get_buf_u32_mut(&mut self) -> &mut [u32] {
        unsafe { self.buf.align_to_mut::<u32>().1 }
    }
}

pub struct FontHelper {
    pub default_layout: Layout,
}

impl FontHelper {
    pub fn new() -> Self {
        Self {
            default_layout: Layout::new(CoordinateSystem::PositiveYDown),
        }
    }
}

pub struct ResourceManager {
    _handle_id: usize,
    _handles: HashSet<usize>, // can probably be a vec and do a binary search since it will always be sorted..?
    _images: Vec<Option<Image>>,
    _fonts: Vec<Option<Font>>,
    _available_image_indexes: Vec<usize>,
    _available_font_indexes: Vec<usize>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            _handle_id: 0,
            _handles: HashSet::new(),
            _images: Vec::new(),
            _fonts: Vec::new(),
            _available_image_indexes: Vec::new(),
            _available_font_indexes: Vec::new(),
        }
    }
    pub fn new_layout() -> Layout {
        Layout::new(CoordinateSystem::PositiveYDown)
    }
    fn create_image_handle(&mut self) -> ImageHandle {
        let handle_id = self._handle_id;
        self._handle_id += 1;
        self._handles.insert(handle_id);
        let index = match self._available_image_indexes.pop() {
            Some(i) => i,
            None => self._images.len(),
        };
        ImageHandle {
            id: handle_id,
            _index: index,
        }
    }
    ///load an image and create a new handle to store it with
    pub fn load_image(&mut self, path: &Path) -> ImageHandle {
        let image_file = match ImageReader::open(path) {
            Err(why) => panic!("Could not open {}: {}", path.display(), why),
            Ok(file) => file,
        };
        let image = match image_file.decode() {
            Err(why) => panic!("Could not decode {}: {}", path.display(), why),
            Ok(result) => result.to_rgba8(),
        };
        let width = image.width();
        let height = image.height();
        let image = Image::new(width, height, image.into_vec());
        let handle = self.create_image_handle();
        if handle._index < self._images.len() {
            self._images[handle._index] = Some(image);
        } else {
            self._images.push(Some(image));
        }
        handle
    }
    pub fn get_image(&mut self, handle: ImageHandle) -> Option<&Image> {
        if self._handles.contains(&handle.id) {
            return Some(self._images[handle._index].as_ref().unwrap());
        }
        None
    }
    pub fn delete_image(&mut self, handle: ImageHandle) {
        if self._handles.remove(&handle.id) {
            self._images[handle._index] = None;
            self._available_image_indexes.push(handle._index);
        }
    }
    fn create_font_handle(&mut self) -> FontHandle {
        let handle_id = self._handle_id;
        self._handle_id += 1;
        self._handles.insert(handle_id);
        let index = match self._available_font_indexes.pop() {
            Some(i) => i,
            None => self._fonts.len(),
        };
        FontHandle {
            id: handle_id,
            _index: index,
        }
    }
    pub fn load_font(&mut self, path: &Path, font_settings: FontSettings) -> FontHandle {
        let font_bytes = match read(path) {
            Err(why) => panic!("Could not open {}: {}", path.display(), why),
            Ok(file) => file,
        };
        let font = match Font::from_bytes(font_bytes, font_settings) {
            Err(why) => panic!("Could not instantiate {}: {}", path.display(), why),
            Ok(result) => result,
        };
        let handle = self.create_font_handle();
        if handle._index < self._fonts.len() {
            self._fonts[handle._index] = Some(font);
        } else {
            self._fonts.push(Some(font));
        }
        handle
    }
    pub fn get_font(&self, handle: FontHandle) -> Option<&Font> {
        if self._handles.contains(&handle.id) {
            return Some(self._fonts[handle._index].as_ref().unwrap());
        }
        None
    }
    pub fn delete_font(&mut self, handle: FontHandle) {
        if self._handles.remove(&handle.id) {
            self._fonts[handle._index] = None;
            self._available_font_indexes.push(handle._index);
        }
    }
}
