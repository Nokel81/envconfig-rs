use std::{
    convert::TryFrom,
    env, error,
    ffi::{OsStr, OsString},
};

use Error;

/// Load an environment variable by name and parse it into type `T`.
/// The function is used by `envconfig_derive` to implement `init()`.
///
/// Used by types that require starting from an [`OsString`]
///
/// It returns `Error` in the following cases:
/// - Environment variable is not present
/// - Parsing failed
pub fn load_var<T, K>(var_name: K) -> Result<T, Error>
where
    T: TryFrom<OsString>,
    <T as TryFrom<OsString>>::Error: error::Error + 'static,
    K: AsRef<OsStr>,
{
    let var_name = var_name.as_ref();
    match env::var_os(var_name) {
        Some(var) => T::try_from(var).map_err(|e| Error::ParseError {
            name: var_name.to_os_string(),
            info: Box::new(e),
        }),
        None => Err(Error::EnvVarMissing {
            name: var_name.to_os_string(),
        }),
    }
}

pub fn load_var_with_default<T, K>(var_name: K, default: OsString) -> Result<T, Error>
where
    T: TryFrom<OsString>,
    <T as TryFrom<OsString>>::Error: error::Error + 'static,
    K: AsRef<OsStr>,
{
    let var_name = var_name.as_ref();
    let var = env::var_os(var_name).unwrap_or(default);
    T::try_from(var).map_err(|e| Error::ParseError {
        name: var_name.to_os_string(),
        info: Box::new(e),
    })
}

pub fn load_optional_var<T, K>(var_name: K) -> Result<Option<T>, Error>
where
    T: TryFrom<OsString>,
    <T as TryFrom<OsString>>::Error: error::Error + 'static,
    K: AsRef<OsStr>,
{
    let var_name = var_name.as_ref();
    match env::var_os(var_name) {
        Some(var) => T::try_from(var).map(Some).map_err(|e| Error::ParseError {
            name: var_name.to_os_string(),
            info: Box::new(e),
        }),
        None => Ok(None),
    }
}
