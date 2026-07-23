use appcui::ui::bufferview::BufferAccess;
use filecache::CacheType;
use filecache::FileCache;
use filecache::RandomAccessFile;
use filecache::RandomAccessFlags;
use std::path::Path;

pub struct FileAccess {
    cache: Option<FileCache<RandomAccessFile>>,
}
impl FileAccess {
    pub fn open(path: &Path) -> Result<Self, String> {
        let file = Self::internal_open(path)?;
        let cache = FileCache::new(
            CacheType::SlidingWindow {
                window_size: 0x1000,
                bias_factor: 0.0,
            },
            file,
        ).map_err(|e| e.to_string())?;
        Ok(Self { cache: Some(cache) })
    }
    fn internal_open(path: &Path) -> Result<RandomAccessFile, String> {
        // write & exclusive
        if let Ok(file) = RandomAccessFile::open(
            path,
            RandomAccessFlags::Write | RandomAccessFlags::Exclusive,
        ) {
            return Ok(file);
        }
        // just write
        if let Ok(file) = RandomAccessFile::open(path, RandomAccessFlags::Write) {
            return Ok(file);
        }
        // read only
        RandomAccessFile::open(path, RandomAccessFlags::None).map_err(|e| e.to_string())
    }
}

impl Default for FileAccess {
    fn default() -> Self {
        Self { cache: None }
    }
}

impl BufferAccess for FileAccess {
    fn count(&self) -> u64 {
        if let Some(cache) = &self.cache {
            cache.len()
        } else {
            0
        }
    }

    fn get(&mut self, pos: u64) -> Option<u8> {
        if let Some(cache) = &mut self.cache {
            if let Ok(data) = cache.read(pos, 1) {
                Some(data[0])
            } else {
                None
            }
        } else {
            None
        }
    }

    fn can_write(&self) -> bool {
        if let Some(cache) = &self.cache {
            cache.can_write()
        } else {
            false
        }
    }

    fn set(&mut self, pos: u64, value: u8) -> bool {
        if let Some(cache) = &mut self.cache {
            if let Ok(_) = cache.write(pos, &[value]) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn can_resize(&self) -> bool {
        if let Some(cache) = &self.cache {
            cache.can_resize()
        } else {
            false
        }
    }

    fn resize(&mut self, new_size: u64, fill_byte: u8) -> bool {
        if let Some(cache) = &mut self.cache {
            if let Ok(_) = cache.resize(new_size, fill_byte) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}