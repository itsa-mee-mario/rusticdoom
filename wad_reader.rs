use std::fs::File;
use std::io::{self, Read};

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

struct Directory {
    filepos: i32, // integer holding a pointer to the start of the lump's data in the file.
    size: i32,    // size of the lump in bytes
    name: [char; 8], // name of the lump
}

struct Lump {
    data: Vec<u8>, // the actual data of the lump
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

    // Load the WAD file and read its header
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
