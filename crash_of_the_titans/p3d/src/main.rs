#![allow(unused)]
use std::fs::{File, OpenOptions, create_dir};
use std::io::{self, prelude::*, Read, Write};
use std::path::PathBuf;

mod chunks; mod col; mod obj; mod utils; mod weird_chunks;
use chunks::{Chunks, chunk_paris, get_chunks};
use obj::Obj;

fn main() -> io::Result<()> {
    let path = PathBuf::from(
        // "/tmp/cott/thing2.p3d",
        "/tmp/cott/default/package/L1_E1/L1_E1_R1_ext.p3d"
    );
    let mut buf: Vec<u8> = vec![];
    let mut file = OpenOptions::new()
        .read(true).open(&path)?.read_to_end(&mut buf).unwrap();
    let input = buf.as_slice();
    // let input = &input[0x227548..];
    
    let chunks = get_chunks(
        &chunk_paris::<()>(input).unwrap().1,
        &mut vec![]
    );

    let mut obj = Obj::new();
    for c in chunks.iter() {
        if let Chunks::Col(root, x20s) = c {
            // _ = create_dir(&dir);

            for x20 in x20s.iter() {
                println!("{:?}", &x20.name.replace("\x00", ""));

                obj.comment(&root.name);
                obj.x20(x20);
            }
            // println!("{:?}", &dir);
        }
    }
    obj.obj_write(PathBuf::from(
        path.file_stem().unwrap().to_string_lossy().to_string()
    ).with_extension("obj"));

    Ok(())
}
