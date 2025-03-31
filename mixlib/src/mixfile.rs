#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

use byteorder::ReadBytesExt;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::rc::Weak;

use byteorder::LittleEndian;

use crate::CRC;

pub struct MixFilename {
    filename: [u8; 12],
    pub crc: u32,
}

impl MixFilename {
    pub const fn new(filename: &str) -> Self {
        let arr = filename.as_bytes();
        Self {
            filename: {
                let mut array = [0; 12];
                let mut i = 0;
                while i < arr.len() && i < 12 {
                    array[i] = arr[i];
                    i += 1;
                }
                array
            },
            crc: CRC::calc_u32_from_str(filename),
        }
    }
}

pub struct MixFileClass {
    pub Buffer: Vec<SubBlock>,
    pub Count: u16,
    pub Data: Vec<u8>,
    pub DataSize: usize,   // Size of raw data.
    pub Filename: PathBuf, // Filename of mixfile.
}

struct FileEntry {
    mix_file: Weak<PathBuf>,
    offset: u32,
    size: u32,
}

pub struct MixFileClasses {
    mixfiles: BTreeMap<Rc<PathBuf>, MixFileClass>,
    files_by_crc: HashMap<CRC, FileEntry>,
}

pub struct OffsetResult {
    mixfile: Rc<PathBuf>,
    offset: u32,
    size: u32,
}

pub enum OffsetError {}

pub enum RetrieveError {
    Offset(OffsetError),
}

impl MixFileClasses {
    pub fn new() -> Self {
        Self {
            files_by_crc: HashMap::new(),
            mixfiles: BTreeMap::new(),
        }
    }

    /// This is the constructor for the mixfile object. It takes a filename and a memory
    /// handler object and registers the mixfile object with the system. The index block is
    /// allocated and loaded from disk by this routine.
    pub fn try_insert(&mut self, filename: &Path) -> Result<&mut MixFileClass, NewError> {
        // // Load in the control block. It always remains resident.
        // Count = 0;

        // if (!Force_CD_Available(RequiredCD)) {
        //     Prog_End();
        //     exit(EXIT_FAILURE);
        // }

        // TODO: Move loading buffer to async processing

        let file = File::open(&filename).map_err(|err| NewError::IO(err))?;
        let fileheader = FileHeader::from_reader(&file)?;

        if fileheader.count == 0 {
            // Non TD mixfile
            return Err(NewError::NotSupportedYet);
        }

        let s = MixFileClass {
            Buffer: {
                let mut buf = Vec::with_capacity(fileheader.count as usize);
                for i in 0..fileheader.count {
                    buf.push(SubBlock::from_reader(&file).map_err(|err| NewError::Read(err))?);
                }
                buf
            },
            Count: fileheader.count,
            // Raw data block starts uncached.
            Data: Vec::with_capacity(0),
            DataSize: fileheader.size as usize,
            Filename: filename.to_path_buf(),
        };

        // Register mixfile.
        self.mixfiles.insert(Rc::new(filename.to_path_buf()), s);

        Ok(self.mixfiles.get_mut(&filename.to_path_buf()).unwrap())
    }

    /// This routine will return with a pointer to the specified data file if the file resides
    /// in memory. Otherwise, this routine returns NULL. Use this routine to access a resident
    /// file directly rather than going through the process of pseudo disk access. This will
    /// save both time and RAM.
    ///
    /// Returns with a pointer to the data file's data. If the file is not in RAM, then
    /// NULL is returned.
    pub fn retrieve(&self, filename: &MixFilename) -> Result<Option<OffsetResult>, RetrieveError> {
        self.offset(filename)
            .map_err(|err| RetrieveError::Offset(err))
        //	if (!ptr) {
        //		errno = ENOENT;
        //		File_Fatal(filename);
        //	}
    }

    /// This routine will scan through all registered mixfiles and return with a pointer to
    /// the matching mixfile. If no mixfile could be found that matches the name specified,
    /// then NULL is returned.
    pub fn finder(&mut self, filename: &Path) -> Option<&MixFileClass> {
        self.mixfiles.get(&filename.to_owned())
    }

    /// This routine will scan through all registered mixfiles and return with a pointer to
    /// the matching mixfile. If no mixfile could be found that matches the name specified,
    /// then NULL is returned.
    pub fn finder_mut(&mut self, filename: &Path) -> Option<&mut MixFileClass> {
        self.mixfiles.get_mut(&filename.to_owned())
    }

    /// This routine will cache the mixfile, specified by name, into RAM.
    pub fn cache(&mut self, filename: &Path) -> Result<&MixFileClass, CacheError> {
        // TODO: replace with get_key_value_mut once this lands in stable
        let rc = match self.mixfiles.get_key_value(&filename.to_owned()) {
            None => Err(CacheError::NotFound)?,
            Some((rc, _)) => rc.clone(),
        };
        let mixer = self.mixfiles.get_mut(&filename.to_owned());

        match mixer {
            None => Err(CacheError::NotFound),
            Some(mixer) => {
                mixer.cache()?;

                for sub in &mixer.Buffer {
                    self.files_by_crc.insert(
                        CRC(sub.CRC),
                        FileEntry {
                            mix_file: Rc::downgrade(&rc),
                            offset: sub.Offset,
                            size: sub.Size,
                        },
                    );
                }
                Ok(mixer)
            }
        }
    }

    pub fn uncache(&mut self, filename: &Path) -> Result<&MixFileClass, CacheError> {
        // TODO: replace with get_key_value_mut once this lands in stable
        let rc = match self.mixfiles.get_key_value(&filename.to_owned()) {
            None => Err(CacheError::NotFound)?,
            Some((rc, _)) => rc.clone(),
        };
        let mixer = self.mixfiles.get_mut(&filename.to_owned());

        match mixer {
            None => Err(CacheError::NotFound),
            Some(mixer) => {
                mixer.cache()?;

                for sub in &mixer.Buffer {
                    self.files_by_crc.remove(&CRC(sub.CRC));
                }
                mixer.uncache();
                Ok(mixer)
            }
        }
    }

    /// This routine will take the filename specified and search through the mixfile system
    /// looking for it. If the file was found, then the mixfile it was found int, the offset
    /// from the start of the mixfile, and the size of the embedded file will be returned.
    /// Using this method it is possible for the CCFileClass system to process it as a normal
    /// file.
    fn offset(
        &self,
        filename: &MixFilename, /*void ** realptr, MixFileClass ** mixfile, long * offset, long * size*/
    ) -> Result<Option<OffsetResult>, OffsetError> {
        /*
         **	Create the key block that will be used to binary search for the file.
         */
        let block = self.files_by_crc.get(&CRC(filename.crc));

        Ok(match block {
            None => None,
            Some(block) => match block.mix_file.upgrade() {
                None => None,
                Some(rc) => Some(OffsetResult {
                    mixfile: rc,
                    offset: block.offset,
                    size: block.size,
                }),
            },
        })
    }
}

#[derive(Clone, Default)]
#[repr(C)]
// Seems like Windows does not align but compile this packed
#[repr(packed)]
pub struct SubBlock {
    pub CRC: u32,    // CRC code for embedded file.
    pub Offset: u32, // Offset from start of data section.
    pub Size: u32,
}

impl SubBlock {
    fn from_reader<R: std::io::Read>(mut reader: R) -> std::io::Result<Self> {
        let crc = reader.read_u32::<LittleEndian>()?;
        let offset = reader.read_u32::<LittleEndian>()?;
        let size = reader.read_u32::<LittleEndian>()?;
        Ok(Self {
            CRC: crc,
            Offset: offset,
            Size: size,
        })
    }
}

#[repr(C)]
// Seems like Windows does not align but compile this packed
#[repr(packed)]
pub struct FileHeader {
    count: u16,
    size: u32,
}

impl FileHeader {
    fn from_reader<R: std::io::Read>(mut reader: R) -> std::io::Result<Self> {
        let count = reader.read_u16::<LittleEndian>()?;
        let size = reader.read_u32::<LittleEndian>()?;
        Ok(FileHeader { count, size })
    }
}

#[derive(Debug)]
pub enum NewError {
    IO(std::io::Error),
    Read(std::io::Error),
    NotSupportedYet,
}

impl From<std::io::Error> for NewError {
    fn from(value: std::io::Error) -> Self {
        NewError::IO(value)
    }
}

#[derive(Debug)]
pub enum CacheError {
    IO(std::io::Error),
    Seek(std::io::Error),
    Read(std::io::Error),
    NotFound,
}

impl MixFileClass {
    /// Loads this particular mixfile's data into RAM.
    /// This load the mixfile data into ram for this mixfile object. This is the counterpart
    /// to the Free() function.
    /// Returns whether the file load successful?  It could fail if there wasn't enough room
    /// to allocate the raw data block.
    fn cache(&mut self) -> Result<(), CacheError> {
        self.Data.reserve(self.DataSize - self.Data.len());

        if self.DataSize == 0 {
            return Ok(());
        }

        let mut file = File::open(&self.Filename).map_err(|err| CacheError::IO(err))?;
        file.seek(std::io::SeekFrom::Start(
            (size_of::<SubBlock>() * self.Count as usize + size_of::<FileHeader>())
                .try_into()
                .unwrap(),
        ))
        .map_err(|err| CacheError::Seek(err))?;
        let c = file
            .read_to_end(&mut self.Data)
            .map_err(|err| CacheError::Read(err))?;
        if c != self.DataSize {
            panic!("Does not match!")
        }
        Ok(())
    }

    fn uncache(&mut self) {
        self.Data.resize(0, 0);
    }
}
