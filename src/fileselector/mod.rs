use std::fs;

use slint::{ModelRc, Rgb8Pixel, SharedPixelBuffer};

/// Retrieve all image files from folder given as String
/// Must also retrieve size of images and mime type
///
/// # Arguments
///
/// * `folder` - A string slice that holds the path to the folder
///
/// # Returns
///
/// A vector of tuples containing the image file path, file_size and mime type
fn get_images_from_folder(folder: &str) -> Vec<(String, u64, String)> {
    let mut images = Vec::new();
    let paths = fs::read_dir(folder).unwrap();
    for path in paths {
        let path = path.unwrap().path();

        // Filter files
        if !path.is_file() {
            continue;
        }

        let path_str = path.to_str().unwrap().to_string();
        let file_size = fs::metadata(&path_str).unwrap().len();
        let mime = mime_guess::from_path(&path_str).first_raw();
        if mime.is_none() {
            continue;
        }
        let mime = mime.unwrap().to_string();

        // Filter mime for image types
        if !mime.starts_with("image") {
            continue;
        }

        // Extract filename from path
        let path_str = String::from(path_str.split(std::path::MAIN_SEPARATOR).last().unwrap());
        images.push((path_str, file_size, mime));
    }
    images
}

/// Retrieve all folders from folder given as String
///
/// # Arguments
///
/// * `folder` - A string slice that holds the path to the folder
///
/// # Returns
///
/// A vector of string slices containing the folder names
fn get_folders_from_folder(folder: &str) -> Vec<String> {
    let mut folders = Vec::new();
    let paths = fs::read_dir(folder).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap().to_string();
        if path.is_dir() {
            // Extract folder name from path
            let path_str = String::from(path_str.split(std::path::MAIN_SEPARATOR).last().unwrap());
            folders.push(path_str);
        }
    }
    folders
}

/// format a byte size into a human readable format
///
/// # Arguments
///
/// * `size` - A u64 that holds the size of the file in bytes
///
/// # Returns
///
/// A string slice that holds the human readable file size
fn format_size(size: u64) -> String {
    let kb = 1024;
    let mb = kb * 1024;
    let gb = mb * 1024;
    if size < kb {
        format!("{} B", size)
    } else if size < mb {
        format!("{:.2} KiB", size as f64 / kb as f64)
    } else if size < gb {
        format!("{:.2} MiB", size as f64 / mb as f64)
    } else {
        format!("{:.2} GiB", size as f64 / gb as f64)
    }
}

pub fn get_slint_files_from_folder(
    folder: &str,
) -> slint::ModelRc<ModelRc<slint::StandardListViewItem>> {
    let images = get_images_from_folder(folder);
    let files = slint::VecModel::from(
        images
            .iter()
            .map(|image| {
                let filename = format!("{}", image.0);
                let filename = slint::SharedString::from(filename.as_str());
                let size = format_size(image.1);
                let size = slint::SharedString::from(size.as_str());
                let mime = format!("{}", image.2);
                let mime = slint::SharedString::from(mime.as_str());

                slint::ModelRc::new(slint::VecModel::from(vec![
                    slint::StandardListViewItem::from(filename),
                    slint::StandardListViewItem::from(size),
                    slint::StandardListViewItem::from(mime),
                ]))
            })
            .collect::<Vec<slint::ModelRc<slint::StandardListViewItem>>>(),
    );

    let files = slint::VecModel::from(files);
    let files = slint::ModelRc::new(files);

    files
}

pub fn get_slint_folders_from_folder(path: &str) -> slint::ModelRc<slint::StandardListViewItem> {
    let mut folders = vec![slint::StandardListViewItem::from(slint::SharedString::from(".."))];
    let folder_lists = get_folders_from_folder(path);
    let folder_lists = folder_lists
        .iter()
        .map(|folder| slint::StandardListViewItem::from(slint::SharedString::from(folder.as_str())))
        .collect::<Vec<slint::StandardListViewItem>>();
    folders.extend(folder_lists);
    let folders = slint::VecModel::from(folders);
    let folders = slint::ModelRc::new(folders);

    folders
}
