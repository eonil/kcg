pub trait Lint {
    /// Performs lint on `self`.
    /// - `path`: Path to current linting object. This will be used for reporting.
    /// - `context`: Context to store linting result.
    fn lint<'a>(self: &Self, path: Path, context: &mut Context);
}

/// Lint context.
/// Lint functions are supposed to record linting results here.
#[derive(Default)]
#[derive(Debug)]
pub struct Context {
    pub logs: Vec<Log>,
}
impl Context {
    /// Records an error log.
    pub fn error(self: &mut Self, path: Path, message: &'static str) {
        self.logs.push(Log {
            severity: Severity::Error,
            path: path.clone(),
            message: RCString::from(message.to_owned()),
        })
    }
}
impl Context {
    /// Checks for rejecting errors and returns them.
    pub fn check(&self) -> Result<(), Box<dyn std::error::Error>> {
        let errs = self.logs
            .iter()
            .filter(|x| x.severity == Severity::Error)
            .map(|x| x.clone())
            .collect::<Vec<Log>>();
        if errs.is_empty() {
            Ok(())
        }
        else {
            Err(Box::new(LintError(errs)))
        }
    }
}
impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        for x in self.logs.iter() {
            f.write_fmt(format_args!("- {}\n", x))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct LintError(Vec<Log>);
impl std::fmt::Display for LintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        for x in self.0.iter() {
            f.write_fmt(format_args!("- {}\n", x))?;
        }
        Ok(())
    }
}
impl std::error::Error for LintError {
}


/// Path to lint target object.
#[derive(Default,Clone)]
#[derive(Eq,PartialEq)]
pub struct Path {
    segments: RCVec<RCString>,
}
impl Path {
    pub fn appending<'a>(self: &Self, segment: &'a str) -> Path {
        let mut x = self.clone();
        x.segments.push_back(RCString::from(segment.to_owned()));
        x
    }
    fn to_string(&self) -> String {
        let mut z = String::new();
        for x in self.segments.iter() {
            z.push_str(&*x);
            z.push_str("/");
        }
        z.pop();
        z
    }
}
impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        f.write_str(&self.to_string())?;
        Ok(())
    }
}
impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        f.write_str(&self.to_string())?;
        Ok(())
    }
}

#[derive(Clone)]
#[derive(Eq,PartialEq)]
#[derive(Debug)]
pub struct Log {
    pub severity: Severity,
    pub path: Path,
    pub message: RCString,
}
impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        f.write_fmt(format_args!("{:#?}({}): {}", self.severity, self.path, self.message))?;
        Ok(())
    }
}

#[derive(Clone,Copy)]
#[derive(Eq,PartialEq)]
#[derive(Debug)]
pub enum Severity {
    Info,
    Warning,
    Error,
}
pub type RCString = std::rc::Rc<String>;
pub type RCVec<T> = im_rc::vector::Vector<T>;


