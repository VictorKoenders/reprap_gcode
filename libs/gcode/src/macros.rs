macro_rules! iter_next {
    ($iter:tt as $t:ty) => {
        match $iter.next() {
            Some(n) => {
                let result: $t = match ::core::str::from_utf8(n).map(|s| s.parse()) {
                    Ok(Ok(n)) => n,
                    Ok(Err(e)) => return ::Result::Error(e.into()),
                    Err(e) => return ::Result::Error(e.into()),
                };
                result
            }
            None => return ::Result::NotEnoughBytes,
        }
    };
    ($iter:tt) => {
        match $iter.next() {
            Some(n) => n,
            None => return ::Result::NotEnoughBytes,
        }
    };
}
