// TOOD
impl<B: ?Sized + ToOwned> Deref for Cow<'_, B> {
	type Target = B;

	fn deref(&self) -> &B {
			match *self {
					Borrowed(borrowed) => borrowed,
					Owned(ref owned) => owned.borrow(),
			}
	}
}