#[cfg(feature = "pkg-config")]
pub use pkg_config;

#[cfg(feature = "bindgen")]
pub use bindgen;

#[cfg(feature = "cc")]
pub use cc;

use std::{iter::Chain, path::Path};

#[cfg(feature = "pkg-config")]
use std::path::PathBuf;

pub mod iter;

pub trait CcArgs<'a, P1, P2, P3, S1, S2, S3, S4, S5, I1>
where
    P1: AsRef<Path> + 'a,
    P2: AsRef<Path> + 'a,
    P3: AsRef<Path> + 'a,
    S1: AsRef<str> + 'a,
    S2: AsRef<str> + 'a,
    S3: AsRef<str> + 'a,
    S4: AsRef<str> + 'a,
    S5: AsRef<str> + 'a,
    I1: Iterator<Item = S3> + 'a,
{
    type LinkPathsIter: Iterator<Item = P1>;

    fn link_paths(&'a self) -> Self::LinkPathsIter;

    #[inline]
    fn link_paths_args(&'a self) -> iter::LinkPathsArgsIter<P1, Self::LinkPathsIter> {
        self.link_paths().into()
    }

    type IncludePathsIter: Iterator<Item = P2>;

    fn include_paths(&'a self) -> Self::IncludePathsIter;

    #[inline]
    fn include_paths_args(&'a self) -> iter::IncludePathsArgsIter<P2, Self::IncludePathsIter> {
        self.include_paths().into()
    }

    type FrameworkPathsIter: Iterator<Item = P3>;

    fn framework_paths(&'a self) -> Self::FrameworkPathsIter;

    #[inline]
    fn framework_paths_args(
        &'a self,
    ) -> iter::FrameworkPathsArgsIter<P3, Self::FrameworkPathsIter> {
        self.framework_paths().into()
    }

    type FrameworksIter: Iterator<Item = S1>;

    fn frameworks(&'a self) -> Self::FrameworksIter;

    #[inline]
    fn frameworks_args(&'a self) -> iter::FrameworksArgsIter<S1, Self::FrameworksIter> {
        self.frameworks().into()
    }

    type LibsIter: Iterator<Item = S2>;

    fn libs(&'a self) -> Self::LibsIter;

    #[inline]
    fn libs_args(&'a self) -> iter::LibsArgsIter<S2, Self::LibsIter> {
        self.libs().into()
    }

    type LdArgsIter: Iterator<Item = I1>;

    fn ld_args(&'a self) -> Self::LdArgsIter;

    #[inline]
    fn ld_args_args(&'a self) -> iter::LdArgsArgsIter<S3, I1, Self::LdArgsIter> {
        self.ld_args().into()
    }

    type DefinesIter: Iterator<Item = (S4, &'a Option<S5>)>;

    fn defines(&'a self) -> Self::DefinesIter;

    #[inline]
    fn defines_args(&'a self) -> iter::DefinesArgsIter<'a, S4, S5, Self::DefinesIter> {
        self.defines().into()
    }

    fn cc_args(
        &'a self,
    ) -> Chain<
        Chain<
            Chain<
                Chain<
                    Chain<
                        Chain<
                            iter::LinkPathsArgsIter<P1, Self::LinkPathsIter>,
                            iter::IncludePathsArgsIter<P2, Self::IncludePathsIter>,
                        >,
                        iter::FrameworkPathsArgsIter<P3, Self::FrameworkPathsIter>,
                    >,
                    iter::FrameworksArgsIter<S1, Self::FrameworksIter>,
                >,
                iter::LibsArgsIter<S2, Self::LibsIter>,
            >,
            iter::LdArgsArgsIter<S3, I1, Self::LdArgsIter>,
        >,
        iter::DefinesArgsIter<'a, S4, S5, Self::DefinesIter>,
    > {
        self.link_paths_args()
            .chain(self.include_paths_args())
            .chain(self.framework_paths_args())
            .chain(self.frameworks_args())
            .chain(self.libs_args())
            .chain(self.ld_args_args())
            .chain(self.defines_args())
    }
}

pub trait MergeCcArgs {
    fn merge_cc_args<'a, P1, P2, P3, S1, S2, S3, S4, S5, I1, A>(self, lib: &'a A) -> Self
    where
        P1: AsRef<Path> + 'a,
        P2: AsRef<Path> + 'a,
        P3: AsRef<Path> + 'a,
        S1: AsRef<str> + 'a,
        S2: AsRef<str> + 'a,
        S3: AsRef<str> + 'a,
        S4: AsRef<str> + 'a,
        S5: AsRef<str> + 'a,
        I1: Iterator<Item = S3> + 'a,
        A: CcArgs<'a, P1, P2, P3, S1, S2, S3, S4, S5, I1>;
}

#[cfg(feature = "bindgen")]
impl MergeCcArgs for bindgen::Builder {
    fn merge_cc_args<'a, P1, P2, P3, S1, S2, S3, S4, S5, I1, A>(self, lib: &'a A) -> Self
    where
        P1: AsRef<Path> + 'a,
        P2: AsRef<Path> + 'a,
        P3: AsRef<Path> + 'a,
        S1: AsRef<str> + 'a,
        S2: AsRef<str> + 'a,
        S3: AsRef<str> + 'a,
        S4: AsRef<str> + 'a,
        S5: AsRef<str> + 'a,
        I1: Iterator<Item = S3> + 'a,
        A: CcArgs<'a, P1, P2, P3, S1, S2, S3, S4, S5, I1>,
    {
        self.clang_args(lib.cc_args())
    }
}

#[cfg(feature = "pkg-config")]
#[inline]
pub(crate) fn vec_iter<'a, T>(v: &'a Vec<T>) -> std::slice::Iter<'a, T> {
    v.iter()
}

#[cfg(feature = "pkg-config")]
impl<'a>
    CcArgs<
        'a,
        &'a PathBuf,
        &'a PathBuf,
        &'a PathBuf,
        &'a String,
        &'a String,
        &'a String,
        &'a String,
        String,
        std::slice::Iter<'a, String>,
    > for pkg_config::Library
{
    type LinkPathsIter = std::slice::Iter<'a, PathBuf>;

    fn link_paths(&'a self) -> Self::LinkPathsIter {
        self.link_paths.iter()
    }

    type IncludePathsIter = std::slice::Iter<'a, PathBuf>;

    fn include_paths(&'a self) -> Self::IncludePathsIter {
        self.include_paths.iter()
    }

    type FrameworkPathsIter = std::slice::Iter<'a, PathBuf>;

    fn framework_paths(&'a self) -> Self::FrameworkPathsIter {
        self.framework_paths.iter()
    }

    type FrameworksIter = std::slice::Iter<'a, String>;

    fn frameworks(&'a self) -> Self::FrameworksIter {
        self.frameworks.iter()
    }

    type LibsIter = std::slice::Iter<'a, String>;

    fn libs(&'a self) -> Self::LibsIter {
        self.libs.iter()
    }

    type LdArgsIter = std::iter::Map<
        std::slice::Iter<'a, Vec<String>>,
        fn(&'a Vec<String>) -> std::slice::Iter<'a, String>,
    >;

    fn ld_args(&'a self) -> Self::LdArgsIter {
        self.ld_args.iter().map(vec_iter)
    }

    type DefinesIter = std::collections::hash_map::Iter<'a, String, Option<String>>;

    fn defines(&'a self) -> Self::DefinesIter {
        self.defines.iter()
    }
}

#[cfg(feature = "cc")]
impl MergeCcArgs for cc::Build {
    fn merge_cc_args<'a, P1, P2, P3, S1, S2, S3, S4, S5, I1, A>(mut self, lib: &'a A) -> Self
    where
        P1: AsRef<Path> + 'a,
        P2: AsRef<Path> + 'a,
        P3: AsRef<Path> + 'a,
        S1: AsRef<str> + 'a,
        S2: AsRef<str> + 'a,
        S3: AsRef<str> + 'a,
        S4: AsRef<str> + 'a,
        S5: AsRef<str> + 'a,
        I1: Iterator<Item = S3> + 'a,
        A: CcArgs<'a, P1, P2, P3, S1, S2, S3, S4, S5, I1>,
    {
        for (name, value) in lib.defines() {
            let value = value.as_ref();
            self.define(name.as_ref(), value.map(|x| x.as_ref()));
        }

        for path in lib.include_paths() {
            self.include(path);
        }

        for arg in lib
            .link_paths_args()
            .chain(lib.framework_paths_args())
            .chain(lib.libs_args())
            .chain(lib.frameworks_args())
            .chain(lib.ld_args_args())
        {
            self.flag(&arg);
        }

        self
    }
}
