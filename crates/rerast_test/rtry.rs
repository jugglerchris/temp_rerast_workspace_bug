fn rule1<T,E,X: From<E>>(r: Result<T,E>) -> Result<T,X> {
    replace!(r#try!(r) => r?);
    unreachable!()
}
