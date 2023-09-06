
#[derive(Debug)]
pub enum LogError {
    ReadLogError(String),
    ExePathError(String),
    InitGameError(String),
    KillError(String),
    ClientUserinfoChangedError(String),
}
