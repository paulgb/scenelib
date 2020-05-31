
#[macro_export]
macro_rules! svg_filename {
    () => {
        format!("{}.svg", std::path::Path::new(file!()).file_stem().unwrap().to_str().unwrap())
    };
}
