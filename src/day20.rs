use git2::{BranchType, Commit, Repository, TreeWalkMode, TreeWalkResult};
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::{post, routes, Route};
use std::fs::File;
use tar::Archive;
use tempfile::TempDir;

#[post("/archive_files", data = "<file>")]
fn archive_files(file: TempFile<'_>) -> Result<String, Status> {
    Day20::archive_files(file)
}

#[post("/archive_files_size", data = "<file>")]
fn archive_files_size(file: TempFile<'_>) -> Result<String, Status> {
    Day20::archive_files_size(file)
}

#[post("/cookie", data = "<file>")]
fn cookie(file: TempFile<'_>) -> Result<String, Status> {
    Day20::cookie(file)
}
pub struct Day20 {}

impl Day20 {
    pub fn routes() -> Vec<Route> {
        routes![archive_files, archive_files_size, cookie]
    }

    fn open_archive(file: TempFile) -> Result<Archive<File>, Status> {
        if file.path().is_none() {
            return Err(Status::BadRequest);
        }
        let file = File::open(file.path().unwrap());
        if file.is_err() {
            eprintln!("Could not open temp file");
            return Err(Status::InternalServerError);
        }
        Ok(Archive::new(file.unwrap()))
    }

    fn archive_files(file: TempFile<'_>) -> Result<String, Status> {
        let mut archive = Self::open_archive(file)?;
        let entries = archive.entries().map_err(|_| Status::BadRequest)?;
        Ok(entries
            .into_iter()
            .filter(|it| it.is_ok())
            .count()
            .to_string())
    }

    fn archive_files_size(file: TempFile<'_>) -> Result<String, Status> {
        let mut archive = Self::open_archive(file)?;
        let entries = archive.entries().map_err(|_| Status::BadRequest)?;

        Ok(entries
            .into_iter()
            .map(|file| {
                file.and_then(|file| file.header().size())
                    .unwrap_or_else(|err| {
                        eprintln!("Could not get size of file: {:?}", err);
                        0
                    })
            })
            .sum::<u64>()
            .to_string())
    }

    fn cookie(file: TempFile) -> Result<String, Status> {
        let mut archive = Day20::open_archive(file)?;
        tempfile::tempdir()
            .map_err(|error| {
                eprintln!("Could not create temporary directory: {:?}", error);
                Status::InternalServerError
            })
            .and_then(|dir| {
                archive
                    .unpack(dir.path())
                    .map_err(|error| {
                        eprintln!("Could not unpack archive {:?}", error);
                        Status::InternalServerError
                    })
                    .map(|_| Day20::find_commit(dir))
            })
    }

    fn find_commit(dir: TempDir) -> String {
        let repo = Repository::open(dir.path()).unwrap();
        let branch = repo.find_branch("christmas", BranchType::Local).unwrap();
        let commit = branch.get().peel_to_commit().unwrap();
        let result = Day20::traverse_commit(&repo, &commit).unwrap_or("".to_string());
        dir.close().unwrap();
        result
    }

    fn traverse_commit(repo: &Repository, commit: &Commit) -> Option<String> {
        if Day20::is_searched_commit(repo, commit) {
            return Some(format!(
                "{} {}",
                commit.author().name().unwrap_or("Unknown Author"),
                commit.id()
            ));
        }

        for commit in commit.parents() {
            let traversed_commit = Day20::traverse_commit(repo, &commit);
            if traversed_commit.is_some() {
                return traversed_commit;
            }
        }
        None
    }

    fn is_searched_commit(repo: &Repository, commit: &Commit) -> bool {
        let tree = commit.tree().unwrap();
        let mut found = false;
        tree.walk(TreeWalkMode::PreOrder, |_, entry| {
            if entry.name() == Some("santa.txt") {
                let object = entry.to_object(repo);
                let object = object.unwrap();
                let blob = object.as_blob();
                if blob.is_none() {
                    println!(
                        "Not a blob: {:?}",
                        entry.name().unwrap_or(entry.id().to_string().as_str())
                    );
                    return TreeWalkResult::Ok;
                }
                let blob = blob.unwrap();
                let content = String::from_utf8(blob.content().to_vec());

                if content.is_err() {
                    println!(
                        "Could not parse santa file {:?}",
                        entry.name().unwrap_or(entry.id().to_string().as_str())
                    );
                    return TreeWalkResult::Ok;
                }

                if content.unwrap().contains("COOKIE") {
                    found = true;
                    return TreeWalkResult::Abort;
                }
            }
            TreeWalkResult::Ok
        })
        .expect("Walk failed");
        found
    }
}
