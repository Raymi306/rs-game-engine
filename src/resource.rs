use std::collections::HashSet;
use std::path::Path;
use image::io::Reader as ImageReader;

#[derive(Copy, Clone, Debug)]
pub struct ResourceHandle {
    pub id: usize,
    _index: usize,
}

pub trait ImageResource {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn get_buf(&self) -> &[u8];
    fn get_buf_mut(&mut self) -> &mut [u8];
}

pub struct ImageResourceVec {
    pub buf: Vec<u8>,
    _width: u32,
    _height: u32,
}

pub struct ImageResourceSlice<'a> {
    pub buf: &'a mut[u8],
    _width: u32,
    _height: u32,
}

impl ImageResourceVec {
    pub fn new(width: u32, height: u32, buf: Vec<u8>) -> Self {
        Self {
            _width: width,
            _height: height,
            buf
        }
    }
}

impl<'a> ImageResourceSlice<'a> {
    pub fn new(width: u32, height: u32, buf: &'a mut [u8]) -> Self {
        Self {
            _width: width,
            _height: height,
            buf
        }
    }
    /// These setters should only be needed to handle when screen is being resized
    pub fn set_width(&mut self, width: u32) {
        self._width = width;
    }
    pub fn set_height(&mut self, height: u32) {
        self._height = height;
    }
}

impl ImageResource for ImageResourceVec {
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
}

impl ImageResource for ImageResourceSlice<'_> {
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
        self.buf
    }
    #[inline]
    fn get_buf_mut(&mut self) -> &mut [u8] {
        self.buf
    }
}

pub struct ResourceManager {
    _handle_id: usize,
    _handles: HashSet<usize>, // can probably be a vec and do a binary search since it will always be sorted..?
    _images: Vec<Option<ImageResourceVec>>,
    _available_image_indexes: Vec<usize>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            _handle_id: 0,
            _handles: HashSet::new(),
            _images: Vec::new(),
            _available_image_indexes: Vec::new(),
        }
    }
    fn create_image_handle(&mut self) -> ResourceHandle {
        let handle_id = self._handle_id;
        self._handle_id += 1;
        self._handles.insert(handle_id);
        let index = match self._available_image_indexes.pop() {
            Some(i) => i,
            None => self._images.len()
        };
        ResourceHandle {
            id: handle_id,
            _index: index,
        }
    }
    ///load an image and create a new handle to store it with
    pub fn load_image(&mut self, path: &Path) -> ResourceHandle {
        let image = ImageReader::open(path)
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();
        let width = image.width();
        let height = image.height();
        let image = ImageResourceVec::new(width, height, image.into_vec());
        let handle = self.create_image_handle();
        if handle._index < self._images.len() {
            self._images[handle._index] = Some(image);
        } else {
            self._images.push(Some(image));
        }
        handle
    }
    pub fn get_image(&mut self, handle: ResourceHandle) -> Option<&ImageResourceVec>{
        if self._handles.contains(&handle.id) {
            return Some(self._images[handle._index].as_ref().unwrap());
        }
        None
    }
    pub fn delete_image(&mut self, handle: ResourceHandle) {
        if self._handles.remove(&handle.id) {
            self._images[handle._index] = None;
            self._available_image_indexes.push(handle._index);
        }
    }
}
