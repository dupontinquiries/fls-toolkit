// example adapted from extract example in zip crate

use std::fs;
use std::io;

fn main() {
    std::process::exit(real_main());
}


// hint! run with
// $ cargo run --example extract_zip -- "examples/test_data/example.zip" --no-capture

// $ cargo run --example extract_zip -- "full_path_to/fls-toolkit/examples/test_data/example.zip" --no-capture


fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }
    let fname = std::path::Path::new(&*args[1]);
    let file = fs::File::open(fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            // Some(path) => path.to_owned(),
            Some(path) => {
                let pp = path.to_owned();
                let ppp = pp.parent().unwrap().join(
                    "examples/tmp_data"
                ).join(
                    pp.file_name().unwrap()
                );
                ppp
            },
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            // // TODO inject directory switch into examples folder to keep the main directory tidy
            // let injected_outpath = outpath.parent().unwrap().join(
            //     "examples/tmp_exports"
            // ).join(
            //     outpath.file_name().unwrap()
            // ).unwrap();
            // // !TODO
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    0
}
