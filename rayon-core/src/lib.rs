#![no_std]

extern crate alloc;
use alloc::boxed::Box;

mod std {
    pub(crate) mod io {
        use crate::Box;
        pub use crate::alloc::fmt;
        pub struct Error {
            repr: Repr,
        }
        enum Repr {
            Os(i32),
            Simple(ErrorKind),
            // &str is a fat pointer, but &&str is a thin pointer.
            SimpleMessage(ErrorKind, &'static &'static str),
            // Custom(Box<Custom>),
        }

impl fmt::Debug for Repr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Repr::Os(code) => fmt
                .debug_struct("Os")
                .field("code", &code)
                //.field("kind", &sys::decode_error_kind(code))
                //.field("message", &sys::os::error_string(code))
                .finish(),
//            Repr::Custom(ref c) => fmt::Debug::fmt(&c, fmt),
            Repr::Simple(kind) => fmt.debug_tuple("Kind").field(&kind).finish(),
            Repr::SimpleMessage(kind, &message) => {
                fmt.debug_struct("Error").field("kind", &kind).field("message", &message).finish()
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ErrorKind {
    /// An entity was not found, often a file.
    NotFound,
    /// The operation lacked the necessary privileges to complete.
    PermissionDenied,
    /// The connection was refused by the remote server.
    ConnectionRefused,
    /// The connection was reset by the remote server.
    ConnectionReset,
    /// The remote host is not reachable.
    HostUnreachable,
    /// The network containing the remote host is not reachable.
    NetworkUnreachable,
    /// The connection was aborted (terminated) by the remote server.
    ConnectionAborted,
    /// The network operation failed because it was not connected yet.
    NotConnected,
    /// A socket address could not be bound because the address is already in
    /// use elsewhere.
    AddrInUse,
    /// A nonexistent interface was requested or the requested address was not
    /// local.
    AddrNotAvailable,
    /// The system's networking is down.
    NetworkDown,
    /// The operation failed because a pipe was closed.
    BrokenPipe,
    /// An entity already exists, often a file.
    AlreadyExists,
    /// The operation needs to block to complete, but the blocking operation was
    /// requested to not occur.
    WouldBlock,
    /// A filesystem object is, unexpectedly, not a directory.
    ///
    /// For example, a filesystem path was specified where one of the intermediate directory
    /// components was, in fact, a plain file.
    NotADirectory,
    /// The filesystem object is, unexpectedly, a directory.
    ///
    /// A directory was specified when a non-directory was expected.
    IsADirectory,
    /// A non-empty directory was specified where an empty directory was expected.
    DirectoryNotEmpty,
    /// The filesystem or storage medium is read-only, but a write operation was attempted.
    ReadOnlyFilesystem,
    /// Loop in the filesystem or IO subsystem; often, too many levels of symbolic links.
    ///
    /// There was a loop (or excessively long chain) resolving a filesystem object
    /// or file IO object.
    ///
    /// On Unix this is usually the result of a symbolic link loop; or, of exceeding the
    /// system-specific limit on the depth of symlink traversal.
    FilesystemLoop,
    /// Stale network file handle.
    ///
    /// With some network filesystems, notably NFS, an open file (or directory) can be invalidated
    /// by problems with the network or server.
    StaleNetworkFileHandle,
    /// A parameter was incorrect.
    InvalidInput,
    /// Data not valid for the operation were encountered.
    ///
    /// Unlike [`InvalidInput`], this typically means that the operation
    /// parameters were valid, however the error was caused by malformed
    /// input data.
    ///
    /// For example, a function that reads a file into a string will error with
    /// `InvalidData` if the file's contents are not valid UTF-8.
    ///
    /// [`InvalidInput`]: ErrorKind::InvalidInput
    InvalidData,
    /// The I/O operation's timeout expired, causing it to be canceled.
    TimedOut,
    /// An error returned when an operation could not be completed because a
    /// call to [`write`] returned [`Ok(0)`].
    ///
    /// This typically means that an operation could only succeed if it wrote a
    /// particular number of bytes but only a smaller number of bytes could be
    /// written.
    ///
    /// [`write`]: crate::io::Write::write
    /// [`Ok(0)`]: Ok
    WriteZero,
    /// The underlying storage (typically, a filesystem) is full.
    ///
    /// This does not include out of quota errors.
    StorageFull,
    /// Seek on unseekable file.
    ///
    /// Seeking was attempted on an open file handle which is not suitable for seeking - for
    /// example, on Unix, a named pipe opened with `File::open`.
    NotSeekable,
    /// Filesystem quota was exceeded.
    FilesystemQuotaExceeded,
    /// File larger than allowed or supported.
    ///
    /// This might arise from a hard limit of the underlying filesystem or file access API, or from
    /// an administratively imposed resource limitation.  Simple disk full, and out of quota, have
    /// their own errors.
    FileTooLarge,
    /// Resource is busy.
    ResourceBusy,
    /// Executable file is busy.
    ///
    /// An attempt was made to write to a file which is also in use as a running program.  (Not all
    /// operating systems detect this situation.)
    ExecutableFileBusy,
    /// Deadlock (avoided).
    ///
    /// A file locking operation would result in deadlock.  This situation is typically detected, if
    /// at all, on a best-effort basis.
    Deadlock,
    /// Cross-device or cross-filesystem (hard) link or rename.
    CrossesDevices,
    /// Too many (hard) links to the same filesystem object.
    ///
    /// The filesystem does not support making so many hardlinks to the same file.
    TooManyLinks,
    /// Filename too long.
    ///
    /// The limit might be from the underlying filesystem or API, or an administratively imposed
    /// resource limit.
    FilenameTooLong,
    /// Program argument list too long.
    ///
    /// When trying to run an external program, a system or process limit on the size of the
    /// arguments would have been exceeded.
    ArgumentListTooLong,
    /// This operation was interrupted.
    ///
    /// Interrupted operations can typically be retried.
    Interrupted,

    /// This operation is unsupported on this platform.
    ///
    /// This means that the operation can never succeed.
    Unsupported,

    // ErrorKinds which are primarily categorisations for OS error
    // codes should be added above.
    //
    /// An error returned when an operation could not be completed because an
    /// "end of file" was reached prematurely.
    ///
    /// This typically means that an operation could only succeed if it read a
    /// particular number of bytes but only a smaller number of bytes could be
    /// read.
    UnexpectedEof,

    /// An operation could not be completed, because it failed
    /// to allocate enough memory.
    OutOfMemory,

    // "Unusual" error kinds which do not correspond simply to (sets
    // of) OS error codes, should be added just above this comment.
    // `Other` and `Uncategorised` should remain at the end:
    //
    /// A custom error that does not fall under any other I/O error kind.
    ///
    /// This can be used to construct your own [`Error`]s that do not match any
    /// [`ErrorKind`].
    ///
    /// This [`ErrorKind`] is not used by the standard library.
    ///
    /// Errors from the standard library that do not fall under any of the I/O
    /// error kinds cannot be `match`ed on, and will only match a wildcard (`_`) pattern.
    /// New [`ErrorKind`]s might be added in the future for some of those.
    Other,

    /// Any I/O error from the standard library that's not part of this list.
    ///
    /// Errors that are `Uncategorized` now may move to a different or a new
    /// [`ErrorKind`] variant in the future. It is not recommended to match
    /// an error against `Uncategorized`; use a wildcard match (`_`) instead.
    #[doc(hidden)]
    Uncategorized,
}
        impl core::fmt::Debug for Error {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                core::fmt::Debug::fmt(&self.repr, f)
            }
        }
    }
    pub(crate) mod error {
        pub trait Error: core::fmt::Debug + core::fmt::Display {
        }
    }
}

fn registry_new(mut builder: impl Sized) -> Result<(), BuildError> { loop { } }

mod tp {
    use super::Box;
    use crate::{BuildError};

    pub struct Pool { }

    pub(super) fn thread_pool_build() -> Result<(), BuildError>
    {
        let registry = crate::registry_new(())?;
        loop { }
    }

    impl Pool {
        pub fn new() {
            let r: Result<_, Box<BuildError>> = thread_pool_build().map_err(Box::from);
            loop { }
        }
    }
}

pub struct BuildError {
    kind: std::io::Error,
}

pub fn build() -> Result<tp::Pool, Box<dyn std::error::Error + 'static>> { loop { } }
