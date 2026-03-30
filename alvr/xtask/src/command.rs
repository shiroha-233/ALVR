use std::{
    path::{Path, PathBuf},
    thread,
    time::Duration,
};
use xshell::{Shell, cmd};

pub fn zip(sh: &Shell, source: &Path) -> Result<(), xshell::Error> {
    if cfg!(windows) {
        let archive_path = PathBuf::from(format!("{}.zip", source.display()));
        let source_glob = source.join("*");

        cmd!(
            sh,
            "powershell -NoProfile -Command Compress-Archive -Path {source_glob} -DestinationPath {archive_path} -Force"
        )
        .run()
    } else {
        let _push_guard = sh.push_dir(source);
        cmd!(sh, "zip -r9X {source} .").run()
    }
}

pub fn unzip(sh: &Shell, source: &Path, destination: &Path) -> Result<(), xshell::Error> {
    if cfg!(windows) {
        cmd!(
            sh,
            "powershell -NoProfile -Command Expand-Archive -LiteralPath {source} -DestinationPath {destination} -Force"
        )
        .run()
    } else {
        cmd!(sh, "unzip {source} -d {destination}").run()
    }
}

pub fn untar(sh: &Shell, source: &Path, destination: &Path) -> Result<(), xshell::Error> {
    cmd!(sh, "tar -xvf {source} -C {destination}").run()
}

pub fn targz(sh: &Shell, source: &Path) -> Result<(), xshell::Error> {
    let parent_dir = source.parent().unwrap();
    let file_name = source.file_name().unwrap();

    cmd!(sh, "tar -czvf {source}.tar.gz -C {parent_dir} {file_name}").run()
}

pub fn download(sh: &Shell, url: &str, destination: &Path) -> Result<(), xshell::Error> {
    const MAX_ATTEMPTS: usize = 3;

    for attempt in 1..=MAX_ATTEMPTS {
        let result = if cfg!(windows) {
            cmd!(sh, "curl --ssl-no-revoke -L -o {destination} --url {url}").run()
        } else {
            cmd!(sh, "curl -L -o {destination} --url {url}").run()
        };

        match result {
            Ok(()) => return Ok(()),
            Err(err) if attempt == MAX_ATTEMPTS => return Err(err),
            Err(err) => {
                eprintln!(
                    "Download attempt {attempt}/{MAX_ATTEMPTS} failed for {url}: {err}. Retrying..."
                );
                thread::sleep(Duration::from_secs(2));
            }
        }
    }

    unreachable!()
}

pub fn download_and_extract_zip(url: &str, destination: &Path) -> Result<(), xshell::Error> {
    let sh = Shell::new().unwrap();
    let temp_dir_guard = sh.create_temp_dir()?;

    let zip_file = temp_dir_guard.path().join("temp_download.zip");
    download(&sh, url, &zip_file)?;

    unzip(&sh, &zip_file, destination)
}

pub fn download_and_extract_tar(url: &str, destination: &Path) -> Result<(), xshell::Error> {
    let sh = Shell::new().unwrap();
    let temp_dir_guard = sh.create_temp_dir()?;

    let tar_file = temp_dir_guard.path().join("temp_download.tar");
    download(&sh, url, &tar_file)?;

    untar(&sh, &tar_file, destination)
}

pub fn date_utc_yyyymmdd(sh: &Shell) -> Result<String, xshell::Error> {
    if cfg!(windows) {
        cmd!(
            sh,
            "powershell (Get-Date).ToUniversalTime().ToString(\"yyyy.MM.dd\")"
        )
        .read()
    } else {
        cmd!(sh, "date -u +%Y.%m.%d").read()
    }
}
