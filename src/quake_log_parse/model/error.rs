#[derive(Debug)]
pub enum LogError {
    ReadLogError(String),
    ExePathError(String),
    InitGameError(String),
    KillError(String),
    ClientUserinfoChangedError(String),
    MissingKeyword(String),
    EmptyPlayerName(String),
    InsertKillMeanError(String),
    JsonError(String),
}
impl From<serde_json::Error> for LogError {
    fn from(err: serde_json::Error) -> Self {
        LogError::JsonError(err.to_string())
    }
}
