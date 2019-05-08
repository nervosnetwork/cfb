/// Uses 0, 64 - 113 as exit code.
#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum ExitCode {
    Cli = 64,
}

impl ExitCode {
    pub fn exit(self) -> ! {
        ::std::process::exit(self as i32);
    }
}

#[derive(Clone, Debug)]
pub struct ExitCodeWithMessage {
    code: ExitCode,
    message: String,
}

impl ExitCodeWithMessage {
    pub fn new(code: ExitCode, message: String) -> Self {
        ExitCodeWithMessage { code, message }
    }

    pub fn cli(message: String) -> Self {
        Self::new(ExitCode::Cli, message)
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<ExitCodeWithMessage> for ExitCode {
    fn from(err: ExitCodeWithMessage) -> ExitCode {
        eprintln!("{}", err.message());
        err.code
    }
}
