use std::path::Path;

use tokio::fs;

pub async fn read_lines(path: impl AsRef<Path>) -> Option<Vec<String>> {
    let path_chars = path.as_ref().to_str()?.chars();
    let total_path = "assets/".chars()
        .chain(path_chars)
        .collect::<String>();
    
    let contents = fs::read_to_string(total_path).await.ok()?;
    let mut all_lines = contents
        .split('\n')
        .map(|c| c.into()).collect::<Vec<String>>();

    while all_lines.len() >= 1 && (all_lines[0].len() == 0) {
        all_lines.remove(0);
    }
    let mut index_to_cut = all_lines.len() - 1;
    while all_lines.len() >= 1 && (all_lines[index_to_cut].len() == 0) {
        all_lines.remove(index_to_cut);
        index_to_cut = index_to_cut - 1;
    }
    Some(all_lines)
}
