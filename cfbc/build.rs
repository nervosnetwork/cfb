use includedir_codegen::Compression;
use walkdir::WalkDir;

fn main() {
    let mut templates = includedir_codegen::start("TEMPLATES");

    for entry in WalkDir::new("templates").into_iter() {
        match entry {
            Ok(ref e)
                if !e.file_type().is_dir() && !e.file_name().to_string_lossy().starts_with(".") =>
            {
                templates
                    .add_file(e.path(), Compression::Gzip)
                    .expect("add files to templates");
            }
            _ => (),
        }
    }

    templates.build("templates.rs").expect("build templates");
}
