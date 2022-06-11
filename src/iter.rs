use std::path::Path;

pub struct LinkPathsArgsIter<P, T>
where
    P: AsRef<Path>,
    T: Iterator<Item = P>,
{
    inner: T,
}

impl<P, T> From<T> for LinkPathsArgsIter<P, T>
where
    P: AsRef<Path>,
    T: Iterator<Item = P>,
{
    #[inline]
    fn from(inner: T) -> Self {
        Self { inner }
    }
}

impl<P, T> Iterator for LinkPathsArgsIter<P, T>
where
    P: AsRef<Path>,
    T: Iterator<Item = P>,
{
    type Item = String;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(format!("-L{}", self.inner.next()?.as_ref().display()))
    }
}

pub struct IncludePathsArgsIter<P, T>
where
    P: AsRef<Path>,
    T: Iterator<Item = P>,
{
    inner: T,
}

impl<P, T> From<T> for IncludePathsArgsIter<P, T>
where
    P: AsRef<Path>,
    T: Iterator<Item = P>,
{
    #[inline]
    fn from(inner: T) -> Self {
        Self { inner }
    }
}

impl<P, T> Iterator for IncludePathsArgsIter<P, T>
where
    P: AsRef<Path>,
    T: Iterator<Item = P>,
{
    type Item = String;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(format!("-I{}", self.inner.next()?.as_ref().display()))
    }
}

pub struct FrameworkPathsArgsIter<P, T>
where
    P: AsRef<Path>,
    T: Iterator<Item = P>,
{
    inner: T,
}

impl<P, T> From<T> for FrameworkPathsArgsIter<P, T>
where
    P: AsRef<Path>,
    T: Iterator<Item = P>,
{
    #[inline]
    fn from(inner: T) -> Self {
        Self { inner }
    }
}

impl<P, T> Iterator for FrameworkPathsArgsIter<P, T>
where
    P: AsRef<Path>,
    T: Iterator<Item = P>,
{
    type Item = String;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(format!("-F{}", self.inner.next()?.as_ref().display()))
    }
}

pub struct FrameworksArgsIter<P, T>
where
    P: AsRef<str>,
    T: Iterator<Item = P>,
{
    inner: T,
    next: Option<P>,
}

impl<P, T> From<T> for FrameworksArgsIter<P, T>
where
    P: AsRef<str>,
    T: Iterator<Item = P>,
{
    #[inline]
    fn from(inner: T) -> Self {
        Self { inner, next: None }
    }
}

impl<P, T> Iterator for FrameworksArgsIter<P, T>
where
    P: AsRef<str>,
    T: Iterator<Item = P>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.next.take() {
            Some(x.as_ref().to_string())
        } else {
            self.next = Some(self.inner.next()?);
            Some("-framework".to_string())
        }
    }
}

pub struct LibsArgsIter<P, T>
where
    P: AsRef<str>,
    T: Iterator<Item = P>,
{
    inner: T,
}

impl<P, T> From<T> for LibsArgsIter<P, T>
where
    P: AsRef<str>,
    T: Iterator<Item = P>,
{
    #[inline]
    fn from(inner: T) -> Self {
        Self { inner }
    }
}

impl<P, T> Iterator for LibsArgsIter<P, T>
where
    P: AsRef<str>,
    T: Iterator<Item = P>,
{
    type Item = String;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(format!("-l{}", self.inner.next()?.as_ref()))
    }
}

pub struct LdArgsArgsIter<P, I, T>
where
    P: AsRef<str>,
    I: Iterator<Item = P>,
    T: Iterator<Item = I>,
{
    inner: T,
}

impl<P, I, T> From<T> for LdArgsArgsIter<P, I, T>
where
    P: AsRef<str>,
    I: Iterator<Item = P>,
    T: Iterator<Item = I>,
{
    #[inline]
    fn from(inner: T) -> Self {
        Self { inner }
    }
}

impl<P, I, T> Iterator for LdArgsArgsIter<P, I, T>
where
    P: AsRef<str>,
    I: Iterator<Item = P>,
    T: Iterator<Item = I>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let arg = self
                .inner
                .next()?
                .fold(String::new(), |a, b| a + b.as_ref());

            if !arg.is_empty() {
                return Some(format!("-Wl,{}", arg));
            }
        }
    }
}

pub struct DefinesArgsIter<'a, S1, S2, T>
where
    S1: AsRef<str> + 'a,
    S2: AsRef<str> + 'a,
    T: Iterator<Item = (S1, &'a Option<S2>)> + 'a,
{
    inner: T,
}

impl<'a, S1, S2, T> From<T> for DefinesArgsIter<'a, S1, S2, T>
where
    S1: AsRef<str> + 'a,
    S2: AsRef<str> + 'a,
    T: Iterator<Item = (S1, &'a Option<S2>)> + 'a,
{
    #[inline]
    fn from(inner: T) -> Self {
        Self { inner }
    }
}

impl<'a, S1, S2, T> Iterator for DefinesArgsIter<'a, S1, S2, T>
where
    S1: AsRef<str> + 'a,
    S2: AsRef<str> + 'a,
    T: Iterator<Item = (S1, &'a Option<S2>)> + 'a,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let (k, v) = self.inner.next()?;
        Some(if let Some(v) = v {
            format!("-D{}={}", k.as_ref(), v.as_ref())
        } else {
            format!("-D{}", k.as_ref())
        })
    }
}
