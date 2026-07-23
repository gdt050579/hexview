use appcui::ui::bufferview::BufferAccess;
use filecache::CacheType;
use filecache::FileCache;
use filecache::RandomAccessFile;
use filecache::RandomAccessFlags;
use std::path::Path;

pub struct FileAccess {
    cache: FileCache<RandomAccessFile>,
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
        Ok(Self { cache })
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
        Self { cache: FileCache::new(CacheType::Memory, file) }
    }
}

impl BufferAccess for FileAccess {
    fn count(&self) -> u64 {
        self.cache.len()
    }

    fn get(&mut self, pos: u64) -> Option<u8> {
        if let Ok(data) = self.cache.read(pos, 1) {
            Some(data[0])
        } else {
            None
        }
    }

    fn can_write(&self) -> bool {
        self.cache.can_write()
    }

    fn set(&mut self, pos: u64, value: u8) -> bool {
        if let Ok(_) = self.cache.write(pos, &[value]) {
            true
        } else {
            false
        }
    }

    fn can_resize(&self) -> bool {
        self.cache.can_resize()
    }

    fn resize(&mut self, new_size: u64, fill_byte: u8) -> bool {
        if let Ok(_) = self.cache.resize(new_size, fill_byte) {
            true
        } else {
            false
        }
    }
}