#![allow(unused)]
use git2::Repository;
use std::fs;
fn main() {
    makelfolder();
    // The repositories to clone are passed to the function here, I will have some more repo's, and they might not all be on GitHub, in future, I hope to let it read from a list and clone them automatically.
    clonelrepo_github("Marnix0810", "ClaudscapeAI");

    makedfolder();
    // The repositories to clone are passed to the function here, I will have some more repo's, and they might not all be on GitHub, in future, I hope to let it read from a list and clone them automatically.
    clonedrepo_github("Marnix0810", "cio-datacore-init");
}

fn makelfolder() -> std::io::Result<()> {
       fs::create_dir("../linked-repositories")?;
       Ok(())
}

fn makedfolder() -> std::io::Result<()> {
       fs::create_dir("../Dependency-repositories")?;
       Ok(())
}

fn clonelrepo_github(u: &str, r: &str){
  let url = format!("https://github.com/{}/{}", u, r);
  let repo = match Repository::clone(&url, format!("../linked-repositories/{}", r)) {
    Ok(r) => r,
    Err(e) => panic!("failed to clone: {}", e),
  };
}

fn clonedrepo_github(u: &str, r: &str){
  let url = format!("https://github.com/{}/{}", u, r);
  let repo = match Repository::clone(&url, format!("../Dependency-repositories/{}", r)) {
    Ok(r) => r,
    Err(e) => panic!("failed to clone: {}", e),
  };
}
