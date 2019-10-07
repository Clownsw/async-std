use std::collections::VecDeque;
use std::pin::Pin;

use crate::prelude::*;
use crate::stream::{Extend, IntoStream};

impl<T> Extend<T> for VecDeque<T> {
    fn stream_extend<'a, S: IntoStream<Item = T> + 'a>(
        &'a mut self,
        stream: S,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        let stream = stream.into_stream();

        self.reserve(stream.size_hint().0);

        Box::pin(stream.for_each(move |item| self.push_back(item)))
    }
}
