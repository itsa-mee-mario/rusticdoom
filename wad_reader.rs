use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct WADHeader {
    identification: [char; 4], // should be "IWAD" or "PWAD"
    numlumps: i32,             // number of lumps in the WAD
    infotableofs: i32,         // pointer to location of directory
}

impl WADHeader {
    // Constructor
    pub fn new() -> WADHeader {
        WADHeader {
            identification: [' '; 4],
            numlumps: 0,
            infotableofs: 0,
        }
    }

    // Read the WAD header from a file
    pub fn read_header(&mut self, file: &mut File) -> io::Result<()> {
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

#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    filepos: i32,
    size: i32,
    name: String, // Changed from [char; 8] to String for better handling
}

impl DirectoryEntry {
    pub fn new() -> DirectoryEntry {
        DirectoryEntry {
            filepos: 0,
            size: 0,
            name: String::new(),
        }
    }

    pub fn read_entry(&mut self, file: &mut File) -> io::Result<()> {
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

        // Convert name buffer to string, trimming null bytes and whitespace
        self.name = name_buffer
            .iter()
            .take_while(|&&b| b != 0)
            .map(|&b| b as char)
            .collect::<String>()
            .trim()
            .to_string();

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Directory {
    entries: Vec<DirectoryEntry>,
    entry_map: HashMap<String, (i32, i32)>, // name -> (filepos, size)
}

impl Directory {
    pub fn new(num_entries: usize) -> Directory {
        Directory {
            entries: Vec::with_capacity(num_entries),
            entry_map: HashMap::new(),
        }
    }

    pub fn read_entries(&mut self, file: &mut File, num_entries: usize) -> io::Result<()> {
        for _ in 0..num_entries {
            let mut entry = DirectoryEntry::new();
            entry.read_entry(file)?;

            // Store in hashmap for quick lookup
            self.entry_map
                .insert(entry.name.clone(), (entry.filepos, entry.size));

            self.entries.push(entry);
        }
        Ok(())
    }

    pub fn get_entry(&self, name: &str) -> Option<&DirectoryEntry> {
        self.entries.iter().find(|e| e.name == name)
    }
}

pub struct DoomEngine {
    pub wad_path: String,
    pub directory: Directory, // Changed to lowercase for Rust conventions
}

impl DoomEngine {
    // Constructor
    pub fn new(wad_path: &str) -> DoomEngine {
        DoomEngine {
            wad_path: wad_path.to_string(),
            directory: Directory::new(0),
        }
    }

    pub fn load_wad(&mut self) -> io::Result<()> {
        let mut file = File::open(&self.wad_path)?;

        // Read WAD header
        let mut header = WADHeader::new();
        header.read_header(&mut file)?;

        // Print the header information
        println!(
            "WAD identification: {:?}, num lumps: {}, info table offset: {}",
            header.identification, header.numlumps, header.infotableofs
        );

        // Seek to directory location
        file.seek(SeekFrom::Start(header.infotableofs as u64))?;

        // Create and read directory
        let mut directory = Directory::new(header.numlumps as usize);
        directory.read_entries(&mut file, header.numlumps as usize)?;

        // Store directory in engine
        self.directory = directory;

        // Print directory entries
        for (i, entry) in self.directory.entries.iter().enumerate() {
            println!(
                "Entry {}: filepos: {}, size: {}, name: {}",
                i, entry.filepos, entry.size, entry.name
            );
        }

        Ok(())
    }

    pub fn read_vertex(&self, offset: i32) -> io::Result<(f32, f32)> {
        let mut file = File::open(&self.wad_path)?;
        let mut x_bytes = [0u8; 2];
        let mut y_bytes = [0u8; 2];

        file.seek(SeekFrom::Start(offset as u64))?;
        file.read_exact(&mut x_bytes)?;
        file.read_exact(&mut y_bytes)?;

        let x = i16::from_le_bytes(x_bytes);
        let y = i16::from_le_bytes(y_bytes);

        // convert x, y to f32 for renderer
        let x = x as f32;
        let y = y as f32;

        println!("Vertex: x: {}, y: {}", x, y);
        Ok((x, y))
    }
}

pub struct WadData {
    wad: DoomEngine,
}

impl WadData {
    pub fn new(wad: DoomEngine) -> WadData {
        WadData { wad }
    }

    pub fn read_vertexes(&self) -> io::Result<Vec<(f32, f32)>> {
        // Look up the VERTEXES lump in the directory
        let vertexes_entry = self
            .wad
            .directory
            .get_entry("VERTEXES")
            .ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "VERTEXES lump not found",
            ))?;

        println!("VERTEXES entry: {:?}", vertexes_entry);

        let mut file = File::open(&self.wad.wad_path)?;

        // Seek to the start of the VERTEXES lump
        file.seek(SeekFrom::Start(vertexes_entry.filepos as u64))?;

        // Calculate number of vertices (each vertex is 4 bytes - 2 for x, 2 for y)
        let num_vertices = vertexes_entry.size / 4;
        let mut vertices = Vec::with_capacity(num_vertices as usize);

        // Read all vertices
        for _ in 0..num_vertices {
            let mut x_bytes = [0u8; 2];
            let mut y_bytes = [0u8; 2];

            file.read_exact(&mut x_bytes)?;
            file.read_exact(&mut y_bytes)?;

            let x = i16::from_le_bytes(x_bytes);
            let y = i16::from_le_bytes(y_bytes);

            // convert x, y to f32 for renderer
            let x = x as f32;
            let y = y as f32;

            vertices.push((x, y));
        }

        // print vertices
        for (i, vertex) in vertices.iter().enumerate() {
            println!("Vertex {}: x: {}, y: {}", i, vertex.0, vertex.1);
        }
        Ok(vertices)
    }
}
