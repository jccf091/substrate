#[frame_support::pallet(ExamplePallet)]
mod pallet {
	pub use frame_support::pallet_prelude::*;
	pub use frame_system::pallet_prelude::*;

	#[pallet::trait_]
	pub trait Trait<I: Instance = DefaultInstance>: frame_system::Trait {
		#[pallet::const_]
		type Too: Get<u32>;
		type Balance: frame_support::dispatch::Parameter + Default;
	}

	#[pallet::module]
	pub struct Module<T, I = DefaultInstance>(core::marker::PhantomData::<(T, I)>);

	#[pallet::module_interface]
	impl<T: Trait<I>, I: Instance> ModuleInterface<BlockNumberFor<T>> for Module<T, I> {
	}

	#[pallet::call]
	impl<T: Trait<I>, I: Instance> Call for Module<T, I> {
		#[pallet::weight = 0]
		fn toto(origin: OriginFor<T>, #[pallet::compact] toto: u32) -> DispatchResultWithPostInfo {
			<MyStorage<I>>::insert(3u32, 3u32);
			let _ = origin;
			let _ = toto;
			Ok(().into())
		}
	}

	#[pallet::storage]
	pub type MyStorageValue<T: Trait<I>, I = DefaultInstance> = StorageValueType<MyStorageValueP<I>, T::Balance, ValueQuery>;

	#[pallet::storage]
	pub type MyStorage<I = DefaultInstance> = StorageMapType<MyStorageP<I>, Blake2_128Concat, u32, u32>;

	#[pallet::error]
	pub enum Error<T, I = DefaultInstance> {
		/// E
		/// E
		E,
		///
		B,
	}

	#[pallet::event]
	pub enum Event<T: Trait<I>, I: Instance = DefaultInstance> {
		/// A
		A(T::Balance, T::Balance, u32),
		/// B
		/// B2
		B { aa: u32, bb: T::Balance },
	}

	#[pallet::origin]
	pub struct Origin<T, I = DefaultInstance>(core::marker::PhantomData<(T, I)>);

	#[pallet::inherent]
	impl<T: Trait<I>, I: Instance> ProvideInherent for Module<T, I> {
		type Call = Call<T, I>;
		type Error = InherentError;

		const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

		fn create_inherent(_data: &InherentData) -> Option<Self::Call> {
			unimplemented!();
		}
	}

	#[derive(codec::Encode, sp_runtime::RuntimeDebug)]
	#[cfg_attr(feature = "std", derive(codec::Decode))]
	pub enum InherentError {
	}

	impl sp_inherents::IsFatalError for InherentError {
		fn is_fatal_error(&self) -> bool {
			unimplemented!();
		}
	}

	pub const INHERENT_IDENTIFIER: sp_inherents::InherentIdentifier = *b"testpall";
}
