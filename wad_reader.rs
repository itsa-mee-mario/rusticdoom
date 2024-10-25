use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

struct WADHeader {
    identification: [char; 4], // should be "IWAD" or "PWAD"
    numlumps: i32,             // number of lumps in the WAD
    infotableofs: i32,         // pointer to location of directory
}

impl WADHeader {
    // Constructor
    fn new() -> WADHeader {
        WADHeader {
            identification: [' '; 4],
            numlumps: 0,
            infotableofs: 0,
        }
    }

    // Read the WAD header from a file
    fn read_header(&mut self, file: &mut File) -> io::Result<()> {
        // Buffer for reading the 4 characters of identification
        let mut id_buffer = [0u8; 4];
        file.read_exact(&mut id_buffer)?;

        // Convert bytes to characters and store in identification
        for (i, &byte) in id_buffer.iter().enumerate() {
            self.identification[i] = byte as char;
        }

        // Buffer for reading i32 values
        let mut int_buffer = [0u8; 4];

        // Read the number of lumps
        file.read_exact(&mut int_buffer)?;
        self.numlumps = i32::from_le_bytes(int_buffer);

        // Read the offset to the directory
        file.read_exact(&mut int_buffer)?;
        self.infotableofs = i32::from_le_bytes(int_buffer);

        Ok(())
    }
}

struct DirectoryEntry {
    filepos: i32,
    size: i32,
    name: [char; 8],
}

impl DirectoryEntry {
    fn new() -> DirectoryEntry {
        DirectoryEntry {
            filepos: 0,
            size: 0,
            name: [' '; 8],
        }
    }

    fn read_entry(&mut self, file: &mut File) -> io::Result<()> {
        // Buffer for reading i32 values
        let mut int_buffer = [0u8; 4];

        // Read the file position
        file.read_exact(&mut int_buffer)?;
        self.filepos = i32::from_le_bytes(int_buffer);

        // Read the size
        file.read_exact(&mut int_buffer)?;
        self.size = i32::from_le_bytes(int_buffer);

        // Buffer for reading the 8 characters of the name
        let mut name_buffer = [0u8; 8];
        file.read_exact(&mut name_buffer)?;

        // Convert bytes to characters and store in name
        for (i, &byte) in name_buffer.iter().enumerate() {
            self.name[i] = byte as char;
        }

        Ok(())
    }
}

struct Directory {
    entries: Vec<DirectoryEntry>,
}

impl Directory {
    fn new(num_entries: usize) -> Directory {
        Directory {
            entries: Vec::with_capacity(num_entries),
        }
    }

    fn read_entries(&mut self, file: &mut File, num_entries: usize) -> io::Result<()> {
        for _ in 0..num_entries {
            let mut entry = DirectoryEntry::new();
            entry.read_entry(file)?;
            self.entries.push(entry);
        }
        Ok(())
    }
}

struct DoomEngine {
    wad_path: String,
}

impl DoomEngine {
    // Constructor
    fn new(wad_path: &str) -> DoomEngine {
        DoomEngine {
            wad_path: wad_path.to_string(),
        }
    }

    // Load the WAD file, read its header, and directory
    fn load_wad(&self) -> io::Result<()> {
        // Open the file
        let mut file = File::open(&self.wad_path)?;

        // Create a WADHeader and read the header
        let mut header = WADHeader::new();
        header.read_header(&mut file)?;

        // Print the header information
        println!(
            "WAD identification: {:?}, num lumps: {}, info table offset: {}",
            header.identification, header.numlumps, header.infotableofs
        );

        // Seek to the directory location using infotableofs
        file.seek(SeekFrom::Start(header.infotableofs as u64))?;

        // Create the directory and read all the entries
        let mut directory = Directory::new(header.numlumps as usize);
        directory.read_entries(&mut file, header.numlumps as usize)?;

        // Print directory entries
        for (i, entry) in directory.entries.iter().enumerate() {
            println!(
                "Entry {}: filepos: {}, size: {}, name: {:?}",
                i, entry.filepos, entry.size, entry.name
            );
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let doomengine = DoomEngine::new("wad/doom1.wad");
    println!("Loading WAD file: {}", doomengine.wad_path);

    // Load and process the WAD file
    doomengine.load_wad()?;

    Ok(())
}
