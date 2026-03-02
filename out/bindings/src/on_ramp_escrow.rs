///Module containing a contract's types and functions.
/**

```solidity
library IOnRampEscrow {
    type OrderStatus is uint8;
    struct OnRampOrder { address buyer; address lp; address token; uint256 tokenAmount; string fiatCurrency; uint256 fiatAmount; OrderStatus status; uint256 createdAt; uint256 fundsLockedAt; bytes32 userPaymentProof; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IOnRampEscrow {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OrderStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<OrderStatus> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl OrderStatus {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u8> for OrderStatus {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<OrderStatus> for u8 {
            fn from(value: OrderStatus) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for OrderStatus {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OrderStatus {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct OnRampOrder { address buyer; address lp; address token; uint256 tokenAmount; string fiatCurrency; uint256 fiatAmount; OrderStatus status; uint256 createdAt; uint256 fundsLockedAt; bytes32 userPaymentProof; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnRampOrder {
        #[allow(missing_docs)]
        pub buyer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub lp: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fiatCurrency: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub fiatAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub status: <OrderStatus as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub createdAt: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fundsLockedAt: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub userPaymentProof: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Uint<256>,
            OrderStatus,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::String,
            alloy::sol_types::private::primitives::aliases::U256,
            <OrderStatus as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::FixedBytes<32>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnRampOrder> for UnderlyingRustTuple<'_> {
            fn from(value: OnRampOrder) -> Self {
                (
                    value.buyer,
                    value.lp,
                    value.token,
                    value.tokenAmount,
                    value.fiatCurrency,
                    value.fiatAmount,
                    value.status,
                    value.createdAt,
                    value.fundsLockedAt,
                    value.userPaymentProof,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnRampOrder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    buyer: tuple.0,
                    lp: tuple.1,
                    token: tuple.2,
                    tokenAmount: tuple.3,
                    fiatCurrency: tuple.4,
                    fiatAmount: tuple.5,
                    status: tuple.6,
                    createdAt: tuple.7,
                    fundsLockedAt: tuple.8,
                    userPaymentProof: tuple.9,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OnRampOrder {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OnRampOrder {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.buyer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.lp,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenAmount),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.fiatCurrency,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fiatAmount),
                    <OrderStatus as alloy_sol_types::SolType>::tokenize(&self.status),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.createdAt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fundsLockedAt),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.userPaymentProof),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for OnRampOrder {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for OnRampOrder {
            const NAME: &'static str = "OnRampOrder";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OnRampOrder(address buyer,address lp,address token,uint256 tokenAmount,string fiatCurrency,uint256 fiatAmount,uint8 status,uint256 createdAt,uint256 fundsLockedAt,bytes32 userPaymentProof)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.buyer,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.lp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tokenAmount)
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.fiatCurrency,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.fiatAmount)
                        .0,
                    <OrderStatus as alloy_sol_types::SolType>::eip712_data_word(
                            &self.status,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.createdAt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.fundsLockedAt)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.userPaymentProof,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OnRampOrder {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.buyer,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.lp,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenAmount,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.fiatCurrency,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.fiatAmount,
                    )
                    + <OrderStatus as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.status,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.createdAt,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.fundsLockedAt,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.userPaymentProof,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.buyer,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.lp,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenAmount,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.fiatCurrency,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.fiatAmount,
                    out,
                );
                <OrderStatus as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.status,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.createdAt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.fundsLockedAt,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.userPaymentProof,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IOnRampEscrow`](self) contract instance.

See the [wrapper's documentation](`IOnRampEscrowInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> IOnRampEscrowInstance<P, N> {
        IOnRampEscrowInstance::<P, N>::new(address, __provider)
    }
    /**A [`IOnRampEscrow`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IOnRampEscrow`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IOnRampEscrowInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IOnRampEscrowInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IOnRampEscrowInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IOnRampEscrowInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IOnRampEscrow`](self) contract instance.

See the [wrapper's documentation](`IOnRampEscrowInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> IOnRampEscrowInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IOnRampEscrowInstance<P, N> {
            IOnRampEscrowInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IOnRampEscrowInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IOnRampEscrowInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IOnRampEscrow {
    type OrderStatus is uint8;
    struct OnRampOrder {
        address buyer;
        address lp;
        address token;
        uint256 tokenAmount;
        string fiatCurrency;
        uint256 fiatAmount;
        OrderStatus status;
        uint256 createdAt;
        uint256 fundsLockedAt;
        bytes32 userPaymentProof;
    }
}

interface OnRampEscrow {
    error AccessControlBadConfirmation();
    error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
    error EnforcedPause();
    error ExpectedPause();
    error ReentrancyGuardReentrantCall();

    event DisputeResolved(bytes32 indexed orderId, address indexed resolver, address winner);
    event FiatSent(bytes32 indexed orderId, bytes32 proofHash);
    event FundsLocked(bytes32 indexed orderId, address indexed lp, uint256 amount);
    event LpCapSet(address indexed lp, address indexed token, uint256 cap);
    event LpOutstandingUpdated(address indexed lp, address indexed token, uint256 newOutstanding);
    event OrderCancelled(bytes32 indexed orderId, string reason);
    event OrderCompleted(bytes32 indexed orderId, address indexed buyer);
    event OrderCreated(bytes32 indexed orderId, address indexed buyer, address token, uint256 tokenAmount, uint256 fiatAmount, string fiatCurrency);
    event OrderDisputed(bytes32 indexed orderId, address indexed reporter);
    event Paused(address account);
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    event Unpaused(address account);

    constructor();

    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    function LP_ROLE() external view returns (bytes32);
    function RELAYER_ROLE() external view returns (bytes32);
    function cancelOrder(bytes32 orderId) external;
    function confirmFiatSent(bytes32 orderId, bytes32 proofHash) external;
    function createOnRampOrder(address token, uint256 tokenAmount, uint256 fiatAmount, string memory fiatCurrency) external returns (bytes32 orderId);
    function disputeOrder(bytes32 orderId) external;
    function getOrder(bytes32 orderId) external view returns (IOnRampEscrow.OnRampOrder memory);
    function getRoleAdmin(bytes32 role) external view returns (bytes32);
    function grantRole(bytes32 role, address account) external;
    function hasRole(bytes32 role, address account) external view returns (bool);
    function lockDeadline() external view returns (uint256);
    function lockFunds(bytes32 orderId) external;
    function lockFundsByRelayer(bytes32 orderId, address lpAddress) external;
    function lpCapByToken(address, address) external view returns (uint256);
    function lpOutstandingByToken(address, address) external view returns (uint256);
    function orders(bytes32) external view returns (address buyer, address lp, address token, uint256 tokenAmount, string memory fiatCurrency, uint256 fiatAmount, IOnRampEscrow.OrderStatus status, uint256 createdAt, uint256 fundsLockedAt, bytes32 userPaymentProof);
    function pause() external;
    function paused() external view returns (bool);
    function paymentDeadline() external view returns (uint256);
    function reclaimLockedFunds(bytes32 orderId) external;
    function releaseFunds(bytes32 orderId) external;
    function renounceRole(bytes32 role, address callerConfirmation) external;
    function resolveDispute(bytes32 orderId, bool releaseToBuyer) external;
    function revokeRole(bytes32 role, address account) external;
    function setLockDeadline(uint256 newDeadline) external;
    function setLpCap(address lp, address token, uint256 cap) external;
    function setPaymentDeadline(uint256 newDeadline) external;
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function unpause() external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "DEFAULT_ADMIN_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "LP_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "RELAYER_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "cancelOrder",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "confirmFiatSent",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "proofHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createOnRampOrder",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "fiatAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "fiatCurrency",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "disputeOrder",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getOrder",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IOnRampEscrow.OnRampOrder",
        "components": [
          {
            "name": "buyer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "lp",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "token",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "tokenAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "fiatCurrency",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "fiatAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum IOnRampEscrow.OrderStatus"
          },
          {
            "name": "createdAt",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "fundsLockedAt",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "userPaymentProof",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRoleAdmin",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "grantRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "hasRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "lockDeadline",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "lockFunds",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "lockFundsByRelayer",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "lpAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "lpCapByToken",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "lpOutstandingByToken",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "orders",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "buyer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "lp",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "fiatCurrency",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "fiatAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "status",
        "type": "uint8",
        "internalType": "enum IOnRampEscrow.OrderStatus"
      },
      {
        "name": "createdAt",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "fundsLockedAt",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "userPaymentProof",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pause",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "paymentDeadline",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "reclaimLockedFunds",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "releaseFunds",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "callerConfirmation",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "resolveDispute",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "releaseToBuyer",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "revokeRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setLockDeadline",
    "inputs": [
      {
        "name": "newDeadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setLpCap",
    "inputs": [
      {
        "name": "lp",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "cap",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setPaymentDeadline",
    "inputs": [
      {
        "name": "newDeadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "supportsInterface",
    "inputs": [
      {
        "name": "interfaceId",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "unpause",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "DisputeResolved",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "resolver",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "winner",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "FiatSent",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "proofHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "FundsLocked",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "lp",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "LpCapSet",
    "inputs": [
      {
        "name": "lp",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "token",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "cap",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "LpOutstandingUpdated",
    "inputs": [
      {
        "name": "lp",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "token",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOutstanding",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OrderCancelled",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "reason",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OrderCompleted",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "buyer",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OrderCreated",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "buyer",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "token",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "tokenAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "fiatAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "fiatCurrency",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OrderDisputed",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "reporter",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Paused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleAdminChanged",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "previousAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "newAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleGranted",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleRevoked",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Unpaused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AccessControlBadConfirmation",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AccessControlUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "neededRole",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "EnforcedPause",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExpectedPause",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ReentrancyGuardReentrantCall",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod OnRampEscrow {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405261025860035561070860045534801561001b575f5ffd5b50600161003a61002f6100ba60201b60201c565b6100e360201b60201c565b5f01819055506100525f5f1b336100ec60201b60201c565b506100837fb0296ea8dd3227371927b1c1cea2b12ea394743ddf2f32f58024ce26f83a24a6336100ec60201b60201c565b506100b47fe2b7fb3b832174769106daebcfd6d1970523240dda11281102db9363b83b0dc4336100ec60201b60201c565b5061024c565b5f7f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005f1b905090565b5f819050919050565b5f6100fd83836101e160201b60201c565b6101d7576001805f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555061017461024560201b60201c565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a4600190506101db565b5f90505b92915050565b5f60015f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f33905090565b6144cd806102595f395ff3fe608060405234801561000f575f5ffd5b50600436106101d8575f3560e01c8063896a2ff4116101025780639e49f7e4116100a0578063d547741f1161006f578063d547741f1461052d578063e1d3945014610549578063fa4d0c3c14610567578063fc46d8b814610585576101d8565b80639e49f7e4146104a7578063a217fddf146104c3578063c92ee043146104e1578063ce9bc15e146104fd576101d8565b806391d14854116100dc57806391d1485414610404578063926d7d7f146104345780639c3f1e90146104525780639cbeb6c11461048b576101d8565b8063896a2ff4146103b05780638cfd919e146103cc5780638fa4a641146103e8576101d8565b80633f4ba83a1161017a5780636d929f23116101495780636d929f23146103505780637489ec231461036e5780638456cb591461038a57806384a5ce5314610394576101d8565b80633f4ba83a146102dc57806343a0e3e6146102e65780635778472a146103025780635c975abb14610332576101d8565b8063305df56f116101b6578063305df56f1461025857806336568abe146102885780633e4e3792146102a45780633e88e0b6146102c0576101d8565b806301ffc9a7146101dc578063248a9ca31461020c5780632f2ff15d1461023c575b5f5ffd5b6101f660048036038101906101f19190612e51565b6105b5565b6040516102039190612e96565b60405180910390f35b61022660048036038101906102219190612ee2565b61062e565b6040516102339190612f1c565b60405180910390f35b61025660048036038101906102519190612f8f565b61064b565b005b610272600480360381019061026d9190613061565b61066d565b60405161027f9190612f1c565b60405180910390f35b6102a2600480360381019061029d9190612f8f565b610983565b005b6102be60048036038101906102b99190612ee2565b6109fe565b005b6102da60048036038101906102d59190612ee2565b610be6565b005b6102e4610f24565b005b61030060048036038101906102fb919061310f565b610f3b565b005b61031c60048036038101906103179190612ee2565b6113df565b6040516103299190613334565b60405180910390f35b61033a611605565b6040516103479190612e96565b60405180910390f35b610358611619565b6040516103659190613363565b60405180910390f35b61038860048036038101906103839190612ee2565b61161f565b005b6103926117f2565b005b6103ae60048036038101906103a9919061337c565b611809565b005b6103ca60048036038101906103c591906133a7565b611820565b005b6103e660048036038101906103e1919061337c565b611914565b005b61040260048036038101906103fd91906133f7565b61192b565b005b61041e60048036038101906104199190612f8f565b611b0c565b60405161042b9190612e96565b60405180910390f35b61043c611b70565b6040516104499190612f1c565b60405180910390f35b61046c60048036038101906104679190612ee2565b611b94565b6040516104829a9998979695949392919061349b565b60405180910390f35b6104a560048036038101906104a09190612ee2565b611cd3565b005b6104c160048036038101906104bc9190612f8f565b611d1b565b005b6104cb611d64565b6040516104d89190612f1c565b60405180910390f35b6104fb60048036038101906104f69190612ee2565b611d6a565b005b6105176004803603810190610512919061353c565b612084565b6040516105249190613363565b60405180910390f35b61054760048036038101906105429190612f8f565b6120a4565b005b6105516120c6565b60405161055e9190612f1c565b60405180910390f35b61056f6120ea565b60405161057c9190613363565b60405180910390f35b61059f600480360381019061059a919061353c565b6120f0565b6040516105ac9190613363565b60405180910390f35b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19161480610627575061062682612110565b5b9050919050565b5f60015f8381526020019081526020015f20600101549050919050565b6106548261062e565b61065d81612179565b610667838361218d565b50505050565b5f610676612276565b5f85116106b8576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016106af906135c4565b60405180910390fd5b338686426040516020016106cf9493929190613647565b6040516020818303038152906040528051906020012090506040518061014001604052803373ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020018773ffffffffffffffffffffffffffffffffffffffff16815260200186815260200184848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081526020018581526020015f60058111156107af576107ae6131db565b5b81526020014281526020015f81526020015f5f1b81525060025f8381526020019081526020015f205f820151815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506020820151816001015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506040820151816002015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506060820151816003015560808201518160040190816108c691906138be565b5060a0820151816005015560c0820151816006015f6101000a81548160ff021916908360058111156108fb576108fa6131db565b5b021790555060e08201518160070155610100820151816008015561012082015181600901559050503373ffffffffffffffffffffffffffffffffffffffff16817ff2b2765edf82fc07e26cea1350aecc843b2e6bbc95c7b52076d65c97c430115588888888886040516109729594939291906139c7565b60405180910390a395945050505050565b61098b6122b7565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16146109ef576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6109f982826122be565b505050565b5f60025f8381526020019081526020015f209050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161480610abc5750806001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16145b610afb576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610af290613a5d565b60405180910390fd5b60026005811115610b0f57610b0e6131db565b5b816006015f9054906101000a900460ff166005811115610b3257610b316131db565b5b14610b72576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610b6990613aeb565b60405180910390fd5b6005816006015f6101000a81548160ff02191690836005811115610b9957610b986131db565b5b02179055503373ffffffffffffffffffffffffffffffffffffffff16827f9809df6c9d63642a6f1e33a698d415ba2d882a4071894da748ab5482975c8e6360405160405180910390a35050565b610bee6123a8565b5f60025f8381526020019081526020015f2090503373ffffffffffffffffffffffffffffffffffffffff16816001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614610c93576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610c8a90613b53565b60405180910390fd5b60016005811115610ca757610ca66131db565b5b816006015f9054906101000a900460ff166005811115610cca57610cc96131db565b5b14610d0a576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610d0190613bbb565b60405180910390fd5b6004548160080154610d1c9190613c06565b4211610d5d576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610d5490613c83565b60405180910390fd5b6004816006015f6101000a81548160ff02191690836005811115610d8457610d836131db565b5b0217905550806002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663a9059cbb826001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600301546040518363ffffffff1660e01b8152600401610e0e929190613ca1565b6020604051808303815f875af1158015610e2a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e4e9190613cdc565b610e8d576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e8490613d51565b60405180910390fd5b610ee2816001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16826002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600301546123ca565b817f386c4fc2fe51e17dd0f7b5bb6d38fca3a652d4e6fbcfa1a93283034acdc5843a604051610f1090613db9565b60405180910390a250610f21612539565b50565b5f5f1b610f3081612179565b610f38612553565b50565b5f5f1b610f4781612179565b610f4f6123a8565b5f60025f8581526020019081526020015f209050600580811115610f7657610f756131db565b5b816006015f9054906101000a900460ff166005811115610f9957610f986131db565b5b14610fd9576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610fd090613e21565b60405180910390fd5b82156111d9576003816006015f6101000a81548160ff02191690836005811115611006576110056131db565b5b0217905550806002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663a9059cbb825f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600301546040518363ffffffff1660e01b815260040161108f929190613ca1565b6020604051808303815f875af11580156110ab573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110cf9190613cdc565b61110e576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161110590613e89565b60405180910390fd5b611163816001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16826002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600301546123ca565b3373ffffffffffffffffffffffffffffffffffffffff16847f41f67fe3f67f1dce68769486352c3bc7c5c5acda8d3c0e6628f918c255e34a0b835f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040516111cc9190613ea7565b60405180910390a36113d1565b6004816006015f6101000a81548160ff02191690836005811115611200576111ff6131db565b5b0217905550806002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663a9059cbb826001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600301546040518363ffffffff1660e01b815260040161128a929190613ca1565b6020604051808303815f875af11580156112a6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112ca9190613cdc565b611309576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161130090613f0a565b60405180910390fd5b61135e816001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16826002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600301546123ca565b3373ffffffffffffffffffffffffffffffffffffffff16847f41f67fe3f67f1dce68769486352c3bc7c5c5acda8d3c0e6628f918c255e34a0b836001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040516113c89190613ea7565b60405180910390a35b506113da612539565b505050565b6113e7612d56565b60025f8381526020019081526020015f20604051806101400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020016003820154815260200160048201805461151c906136ee565b80601f0160208091040260200160405190810160405280929190818152602001828054611548906136ee565b80156115935780601f1061156a57610100808354040283529160200191611593565b820191905f5260205f20905b81548152906001019060200180831161157657829003601f168201915b5050505050815260200160058201548152602001600682015f9054906101000a900460ff1660058111156115ca576115c96131db565b5b60058111156115dc576115db6131db565b5b815260200160078201548152602001600882015481526020016009820154815250509050919050565b5f5f5f9054906101000a900460ff16905090565b60045481565b5f60025f8381526020019081526020015f2090503373ffffffffffffffffffffffffffffffffffffffff16815f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16146116c3576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016116ba90613f72565b60405180910390fd5b5f60058111156116d6576116d56131db565b5b816006015f9054906101000a900460ff1660058111156116f9576116f86131db565b5b14611739576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161173090613fda565b60405180910390fd5b600354816007015461174b9190613c06565b421161178c576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161178390614042565b60405180910390fd5b6004816006015f6101000a81548160ff021916908360058111156117b3576117b26131db565b5b0217905550817f386c4fc2fe51e17dd0f7b5bb6d38fca3a652d4e6fbcfa1a93283034acdc5843a6040516117e6906140aa565b60405180910390a25050565b5f5f1b6117fe81612179565b6118066125b3565b50565b5f5f1b61181581612179565b816003819055505050565b5f5f1b61182c81612179565b8160055f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20819055508273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff167f64d19fedce9351d7c194985163e6b400555c186ba785881239156413948a8bca846040516119069190613363565b60405180910390a350505050565b5f5f1b61192081612179565b816004819055505050565b5f60025f8481526020019081526020015f2090503373ffffffffffffffffffffffffffffffffffffffff16815f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16146119cf576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016119c690613f72565b60405180910390fd5b600160058111156119e3576119e26131db565b5b816006015f9054906101000a900460ff166005811115611a0657611a056131db565b5b14611a46576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611a3d90614112565b60405180910390fd5b6004548160080154611a589190613c06565b421115611a9a576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611a919061417a565b60405180910390fd5b6002816006015f6101000a81548160ff02191690836005811115611ac157611ac06131db565b5b0217905550818160090181905550827fd8e25e58ea6ac5e1dff31cc8c341922c4d1ccd427c84e0bf339c510b4b56106283604051611aff9190612f1c565b60405180910390a2505050565b5f60015f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b7fe2b7fb3b832174769106daebcfd6d1970523240dda11281102db9363b83b0dc481565b6002602052805f5260405f205f91509050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690806001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690806002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690806003015490806004018054611c28906136ee565b80601f0160208091040260200160405190810160405280929190818152602001828054611c54906136ee565b8015611c9f5780601f10611c7657610100808354040283529160200191611c9f565b820191905f5260205f20905b815481529060010190602001808311611c8257829003601f168201915b505050505090806005015490806006015f9054906101000a900460ff1690806007015490806008015490806009015490508a565b611cdb6123a8565b7fb0296ea8dd3227371927b1c1cea2b12ea394743ddf2f32f58024ce26f83a24a6611d0581612179565b611d0f8233612614565b50611d18612539565b50565b611d236123a8565b7fe2b7fb3b832174769106daebcfd6d1970523240dda11281102db9363b83b0dc4611d4d81612179565b611d578383612614565b50611d60612539565b5050565b5f5f1b81565b611d726123a8565b5f60025f8381526020019081526020015f2090503373ffffffffffffffffffffffffffffffffffffffff16816001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614611e17576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611e0e90613b53565b60405180910390fd5b60026005811115611e2b57611e2a6131db565b5b816006015f9054906101000a900460ff166005811115611e4e57611e4d6131db565b5b14611e8e576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611e85906141e2565b60405180910390fd5b6003816006015f6101000a81548160ff02191690836005811115611eb557611eb46131db565b5b0217905550806002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663a9059cbb825f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600301546040518363ffffffff1660e01b8152600401611f3e929190613ca1565b6020604051808303815f875af1158015611f5a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f7e9190613cdc565b611fbd576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611fb49061424a565b60405180910390fd5b612012816001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16826002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600301546123ca565b805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16827fa13a9d77a6fbd690b8000b14cc8f4ba7602b329937638e708937fd9faa6069cd60405160405180910390a350612081612539565b50565b6006602052815f5260405f20602052805f5260405f205f91509150505481565b6120ad8261062e565b6120b681612179565b6120c083836122be565b50505050565b7fb0296ea8dd3227371927b1c1cea2b12ea394743ddf2f32f58024ce26f83a24a681565b60035481565b6005602052815f5260405f20602052805f5260405f205f91509150505481565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b61218a816121856122b7565b612971565b50565b5f6121988383611b0c565b61226c576001805f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506122096122b7565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a460019050612270565b5f90505b92915050565b61227e611605565b156122b5576040517fd93c066500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f33905090565b5f6122c98383611b0c565b1561239e575f60015f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555061233b6122b7565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a4600190506123a2565b5f90505b92915050565b6123b06129c2565b60026123c26123bd612a03565b612a2c565b5f0181905550565b5f8160065f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205461244f9190614268565b90508060065f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20819055508273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff167f08118319557e9998588247d46837aea5c77de48967936343b6c923fb1f7813e38360405161252b9190613363565b60405180910390a350505050565b600161254b612546612a03565b612a2c565b5f0181905550565b61255b612a35565b5f5f5f6101000a81548160ff0219169083151502179055507f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa61259c6122b7565b6040516125a99190613ea7565b60405180910390a1565b6125bb612276565b60015f5f6101000a81548160ff0219169083151502179055507f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586125fd6122b7565b60405161260a9190613ea7565b60405180910390a1565b5f60025f8481526020019081526020015f2090505f600581111561263b5761263a6131db565b5b816006015f9054906101000a900460ff16600581111561265e5761265d6131db565b5b1461269e576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161269590613fda565b60405180910390fd5b60035481600701546126b09190613c06565b4211156126f2576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016126e9906142e5565b60405180910390fd5b61271c7fb0296ea8dd3227371927b1c1cea2b12ea394743ddf2f32f58024ce26f83a24a683611b0c565b61275b576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016127529061434d565b60405180910390fd5b61278d82826002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff168360030154612a75565b81816001015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506001816006015f6101000a81548160ff021916908360058111156127f6576127f56131db565b5b0217905550428160080181905550806002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166323b872dd833084600301546040518463ffffffff1660e01b81526004016128689392919061436b565b6020604051808303815f875af1158015612884573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906128a89190613cdc565b6128e7576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016128de906143ea565b60405180910390fd5b61291982826002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff168360030154612bcb565b8173ffffffffffffffffffffffffffffffffffffffff16837f15144217405064631de951752111334f6d9db4be6cfff45346f7068ea857fcad83600301546040516129649190613363565b60405180910390a3505050565b61297b8282611b0c565b6129be5780826040517fe2517d3f0000000000000000000000000000000000000000000000000000000081526004016129b5929190614408565b60405180910390fd5b5050565b6129ca612d3a565b15612a01576040517f3ee5aeb500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005f1b905090565b5f819050919050565b612a3d611605565b612a73576040517f8dfc202b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f60055f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205490505f8103612afe5750612bc6565b808260065f8773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f2054612b839190613c06565b1115612bc4576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612bbb90614479565b60405180910390fd5b505b505050565b5f8160065f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f2054612c509190613c06565b90508060065f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20819055508273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff167f08118319557e9998588247d46837aea5c77de48967936343b6c923fb1f7813e383604051612d2c9190613363565b60405180910390a350505050565b5f6002612d4d612d48612a03565b612a2c565b5f015414905090565b6040518061014001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f8152602001606081526020015f81526020015f6005811115612ddc57612ddb6131db565b5b81526020015f81526020015f81526020015f81525090565b5f5ffd5b5f5ffd5b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b612e3081612dfc565b8114612e3a575f5ffd5b50565b5f81359050612e4b81612e27565b92915050565b5f60208284031215612e6657612e65612df4565b5b5f612e7384828501612e3d565b91505092915050565b5f8115159050919050565b612e9081612e7c565b82525050565b5f602082019050612ea95f830184612e87565b92915050565b5f819050919050565b612ec181612eaf565b8114612ecb575f5ffd5b50565b5f81359050612edc81612eb8565b92915050565b5f60208284031215612ef757612ef6612df4565b5b5f612f0484828501612ece565b91505092915050565b612f1681612eaf565b82525050565b5f602082019050612f2f5f830184612f0d565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f612f5e82612f35565b9050919050565b612f6e81612f54565b8114612f78575f5ffd5b50565b5f81359050612f8981612f65565b92915050565b5f5f60408385031215612fa557612fa4612df4565b5b5f612fb285828601612ece565b9250506020612fc385828601612f7b565b9150509250929050565b5f819050919050565b612fdf81612fcd565b8114612fe9575f5ffd5b50565b5f81359050612ffa81612fd6565b92915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f84011261302157613020613000565b5b8235905067ffffffffffffffff81111561303e5761303d613004565b5b60208301915083600182028301111561305a57613059613008565b5b9250929050565b5f5f5f5f5f6080868803121561307a57613079612df4565b5b5f61308788828901612f7b565b955050602061309888828901612fec565b94505060406130a988828901612fec565b935050606086013567ffffffffffffffff8111156130ca576130c9612df8565b5b6130d68882890161300c565b92509250509295509295909350565b6130ee81612e7c565b81146130f8575f5ffd5b50565b5f81359050613109816130e5565b92915050565b5f5f6040838503121561312557613124612df4565b5b5f61313285828601612ece565b9250506020613143858286016130fb565b9150509250929050565b61315681612f54565b82525050565b61316581612fcd565b82525050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f601f19601f8301169050919050565b5f6131ad8261316b565b6131b78185613175565b93506131c7818560208601613185565b6131d081613193565b840191505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60068110613219576132186131db565b5b50565b5f81905061322982613208565b919050565b5f6132388261321c565b9050919050565b6132488161322e565b82525050565b61325781612eaf565b82525050565b5f61014083015f8301516132735f86018261314d565b506020830151613286602086018261314d565b506040830151613299604086018261314d565b5060608301516132ac606086018261315c565b50608083015184820360808601526132c482826131a3565b91505060a08301516132d960a086018261315c565b5060c08301516132ec60c086018261323f565b5060e08301516132ff60e086018261315c565b5061010083015161331461010086018261315c565b5061012083015161332961012086018261324e565b508091505092915050565b5f6020820190508181035f83015261334c818461325d565b905092915050565b61335d81612fcd565b82525050565b5f6020820190506133765f830184613354565b92915050565b5f6020828403121561339157613390612df4565b5b5f61339e84828501612fec565b91505092915050565b5f5f5f606084860312156133be576133bd612df4565b5b5f6133cb86828701612f7b565b93505060206133dc86828701612f7b565b92505060406133ed86828701612fec565b9150509250925092565b5f5f6040838503121561340d5761340c612df4565b5b5f61341a85828601612ece565b925050602061342b85828601612ece565b9150509250929050565b61343e81612f54565b82525050565b5f82825260208201905092915050565b5f61345e8261316b565b6134688185613444565b9350613478818560208601613185565b61348181613193565b840191505092915050565b6134958161322e565b82525050565b5f610140820190506134af5f83018d613435565b6134bc602083018c613435565b6134c9604083018b613435565b6134d6606083018a613354565b81810360808301526134e88189613454565b90506134f760a0830188613354565b61350460c083018761348c565b61351160e0830186613354565b61351f610100830185613354565b61352d610120830184612f0d565b9b9a5050505050505050505050565b5f5f6040838503121561355257613551612df4565b5b5f61355f85828601612f7b565b925050602061357085828601612f7b565b9150509250929050565b7f546f6b656e20616d6f756e74206d757374206265203e203000000000000000005f82015250565b5f6135ae601883613444565b91506135b98261357a565b602082019050919050565b5f6020820190508181035f8301526135db816135a2565b9050919050565b5f8160601b9050919050565b5f6135f8826135e2565b9050919050565b5f613609826135ee565b9050919050565b61362161361c82612f54565b6135ff565b82525050565b5f819050919050565b61364161363c82612fcd565b613627565b82525050565b5f6136528287613610565b6014820191506136628286613610565b6014820191506136728285613630565b6020820191506136828284613630565b60208201915081905095945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061370557607f821691505b602082108103613718576137176136c1565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f6008830261377a7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8261373f565b613784868361373f565b95508019841693508086168417925050509392505050565b5f819050919050565b5f6137bf6137ba6137b584612fcd565b61379c565b612fcd565b9050919050565b5f819050919050565b6137d8836137a5565b6137ec6137e4826137c6565b84845461374b565b825550505050565b5f5f905090565b6138036137f4565b61380e8184846137cf565b505050565b5b81811015613831576138265f826137fb565b600181019050613814565b5050565b601f821115613876576138478161371e565b61385084613730565b8101602085101561385f578190505b61387361386b85613730565b830182613813565b50505b505050565b5f82821c905092915050565b5f6138965f198460080261387b565b1980831691505092915050565b5f6138ae8383613887565b9150826002028217905092915050565b6138c78261316b565b67ffffffffffffffff8111156138e0576138df613694565b5b6138ea82546136ee565b6138f5828285613835565b5f60209050601f831160018114613926575f8415613914578287015190505b61391e85826138a3565b865550613985565b601f1984166139348661371e565b5f5b8281101561395b57848901518255600182019150602085019450602081019050613936565b868310156139785784890151613974601f891682613887565b8355505b6001600288020188555050505b505050505050565b828183375f83830152505050565b5f6139a68385613444565b93506139b383858461398d565b6139bc83613193565b840190509392505050565b5f6080820190506139da5f830188613435565b6139e76020830187613354565b6139f46040830186613354565b8181036060830152613a0781848661399b565b90509695505050505050565b7f4e6f74206120706172747920746f20746865206f7264657200000000000000005f82015250565b5f613a47601883613444565b9150613a5282613a13565b602082019050919050565b5f6020820190508181035f830152613a7481613a3b565b9050919050565b7f43616e206f6e6c7920646973707574652061667465722066696174206973206d5f8201527f61726b65642073656e7400000000000000000000000000000000000000000000602082015250565b5f613ad5602a83613444565b9150613ae082613a7b565b604082019050919050565b5f6020820190508181035f830152613b0281613ac9565b9050919050565b7f4e6f74207468652061737369676e6564204c50000000000000000000000000005f82015250565b5f613b3d601383613444565b9150613b4882613b09565b602082019050919050565b5f6020820190508181035f830152613b6a81613b31565b9050919050565b7f4f726465722073746174757320696e76616c69640000000000000000000000005f82015250565b5f613ba5601483613444565b9150613bb082613b71565b602082019050919050565b5f6020820190508181035f830152613bd281613b99565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f613c1082612fcd565b9150613c1b83612fcd565b9250828201905080821115613c3357613c32613bd9565b5b92915050565b7f5061796d656e7420646561646c696e65206e6f742070617373656400000000005f82015250565b5f613c6d601b83613444565b9150613c7882613c39565b602082019050919050565b5f6020820190508181035f830152613c9a81613c61565b9050919050565b5f604082019050613cb45f830185613435565b613cc16020830184613354565b9392505050565b5f81519050613cd6816130e5565b92915050565b5f60208284031215613cf157613cf0612df4565b5b5f613cfe84828501613cc8565b91505092915050565b7f5245434c41494d5f5452414e534645525f4641494c45440000000000000000005f82015250565b5f613d3b601783613444565b9150613d4682613d07565b602082019050919050565b5f6020820190508181035f830152613d6881613d2f565b9050919050565b7f55736572207061796d656e742074696d6564206f7574000000000000000000005f82015250565b5f613da3601683613444565b9150613dae82613d6f565b602082019050919050565b5f6020820190508181035f830152613dd081613d97565b9050919050565b7f4f72646572206e6f7420696e20646973707574650000000000000000000000005f82015250565b5f613e0b601483613444565b9150613e1682613dd7565b602082019050919050565b5f6020820190508181035f830152613e3881613dff565b9050919050565b7f444953505554455f42555945525f5452414e534645525f4641494c45440000005f82015250565b5f613e73601d83613444565b9150613e7e82613e3f565b602082019050919050565b5f6020820190508181035f830152613ea081613e67565b9050919050565b5f602082019050613eba5f830184613435565b92915050565b7f444953505554455f4c505f5452414e534645525f4641494c45440000000000005f82015250565b5f613ef4601a83613444565b9150613eff82613ec0565b602082019050919050565b5f6020820190508181035f830152613f2181613ee8565b9050919050565b7f4e6f7420746865206275796572000000000000000000000000000000000000005f82015250565b5f613f5c600d83613444565b9150613f6782613f28565b602082019050919050565b5f6020820190508181035f830152613f8981613f50565b9050919050565b7f4f72646572206e6f742070656e64696e670000000000000000000000000000005f82015250565b5f613fc4601183613444565b9150613fcf82613f90565b602082019050919050565b5f6020820190508181035f830152613ff181613fb8565b9050919050565b7f4c6f636b20646561646c696e65206e6f742070617373656400000000000000005f82015250565b5f61402c601883613444565b915061403782613ff8565b602082019050919050565b5f6020820190508181035f83015261405981614020565b9050919050565b7f4c50206661696c656420746f206c6f636b2066756e64730000000000000000005f82015250565b5f614094601783613444565b915061409f82614060565b602082019050919050565b5f6020820190508181035f8301526140c181614088565b9050919050565b7f46756e6473206e6f74206c6f636b6564000000000000000000000000000000005f82015250565b5f6140fc601083613444565b9150614107826140c8565b602082019050919050565b5f6020820190508181035f830152614129816140f0565b9050919050565b7f5061796d656e7420646561646c696e65207061737365640000000000000000005f82015250565b5f614164601783613444565b915061416f82614130565b602082019050919050565b5f6020820190508181035f83015261419181614158565b9050919050565b7f46696174206e6f74206d61726b65642061732073656e740000000000000000005f82015250565b5f6141cc601783613444565b91506141d782614198565b602082019050919050565b5f6020820190508181035f8301526141f9816141c0565b9050919050565b7f52454c454153455f5452414e534645525f4641494c45440000000000000000005f82015250565b5f614234601783613444565b915061423f82614200565b602082019050919050565b5f6020820190508181035f83015261426181614228565b9050919050565b5f61427282612fcd565b915061427d83612fcd565b925082820390508181111561429557614294613bd9565b5b92915050565b7f4c6f636b20646561646c696e65207061737365640000000000000000000000005f82015250565b5f6142cf601483613444565b91506142da8261429b565b602082019050919050565b5f6020820190508181035f8301526142fc816142c3565b9050919050565b7f4c505f524f4c455f5245515549524544000000000000000000000000000000005f82015250565b5f614337601083613444565b915061434282614303565b602082019050919050565b5f6020820190508181035f8301526143648161432b565b9050919050565b5f60608201905061437e5f830186613435565b61438b6020830185613435565b6143986040830184613354565b949350505050565b7f4c4f434b5f5452414e534645525f4641494c45440000000000000000000000005f82015250565b5f6143d4601483613444565b91506143df826143a0565b602082019050919050565b5f6020820190508181035f830152614401816143c8565b9050919050565b5f60408201905061441b5f830185613435565b6144286020830184612f0d565b9392505050565b7f4c505f4341505f455843454544454400000000000000000000000000000000005f82015250565b5f614463600f83613444565b915061446e8261442f565b602082019050919050565b5f6020820190508181035f83015261449081614457565b905091905056fea2646970667358221220113cb55e0710cab3041e0ab0b52aee69c9507363f0e9feec9ce225ddd872dacd64736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@Ra\x02X`\x03Ua\x07\x08`\x04U4\x80\x15a\0\x1BW__\xFD[P`\x01a\0:a\0/a\0\xBA` \x1B` \x1CV[a\0\xE3` \x1B` \x1CV[_\x01\x81\x90UPa\0R__\x1B3a\0\xEC` \x1B` \x1CV[Pa\0\x83\x7F\xB0)n\xA8\xDD2'7\x19'\xB1\xC1\xCE\xA2\xB1.\xA3\x94t=\xDF/2\xF5\x80$\xCE&\xF8:$\xA63a\0\xEC` \x1B` \x1CV[Pa\0\xB4\x7F\xE2\xB7\xFB;\x83!tv\x91\x06\xDA\xEB\xCF\xD6\xD1\x97\x05#$\r\xDA\x11(\x11\x02\xDB\x93c\xB8;\r\xC43a\0\xEC` \x1B` \x1CV[Pa\x02LV[_\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0_\x1B\x90P\x90V[_\x81\x90P\x91\x90PV[_a\0\xFD\x83\x83a\x01\xE1` \x1B` \x1CV[a\x01\xD7W`\x01\x80_\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x01ta\x02E` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x01\xDBV[_\x90P[\x92\x91PPV[_`\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_3\x90P\x90V[aD\xCD\x80a\x02Y_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xD8W_5`\xE0\x1C\x80c\x89j/\xF4\x11a\x01\x02W\x80c\x9EI\xF7\xE4\x11a\0\xA0W\x80c\xD5Gt\x1F\x11a\0oW\x80c\xD5Gt\x1F\x14a\x05-W\x80c\xE1\xD3\x94P\x14a\x05IW\x80c\xFAM\x0C<\x14a\x05gW\x80c\xFCF\xD8\xB8\x14a\x05\x85Wa\x01\xD8V[\x80c\x9EI\xF7\xE4\x14a\x04\xA7W\x80c\xA2\x17\xFD\xDF\x14a\x04\xC3W\x80c\xC9.\xE0C\x14a\x04\xE1W\x80c\xCE\x9B\xC1^\x14a\x04\xFDWa\x01\xD8V[\x80c\x91\xD1HT\x11a\0\xDCW\x80c\x91\xD1HT\x14a\x04\x04W\x80c\x92m}\x7F\x14a\x044W\x80c\x9C?\x1E\x90\x14a\x04RW\x80c\x9C\xBE\xB6\xC1\x14a\x04\x8BWa\x01\xD8V[\x80c\x89j/\xF4\x14a\x03\xB0W\x80c\x8C\xFD\x91\x9E\x14a\x03\xCCW\x80c\x8F\xA4\xA6A\x14a\x03\xE8Wa\x01\xD8V[\x80c?K\xA8:\x11a\x01zW\x80cm\x92\x9F#\x11a\x01IW\x80cm\x92\x9F#\x14a\x03PW\x80ct\x89\xEC#\x14a\x03nW\x80c\x84V\xCBY\x14a\x03\x8AW\x80c\x84\xA5\xCES\x14a\x03\x94Wa\x01\xD8V[\x80c?K\xA8:\x14a\x02\xDCW\x80cC\xA0\xE3\xE6\x14a\x02\xE6W\x80cWxG*\x14a\x03\x02W\x80c\\\x97Z\xBB\x14a\x032Wa\x01\xD8V[\x80c0]\xF5o\x11a\x01\xB6W\x80c0]\xF5o\x14a\x02XW\x80c6V\x8A\xBE\x14a\x02\x88W\x80c>N7\x92\x14a\x02\xA4W\x80c>\x88\xE0\xB6\x14a\x02\xC0Wa\x01\xD8V[\x80c\x01\xFF\xC9\xA7\x14a\x01\xDCW\x80c$\x8A\x9C\xA3\x14a\x02\x0CW\x80c//\xF1]\x14a\x02<W[__\xFD[a\x01\xF6`\x04\x806\x03\x81\x01\x90a\x01\xF1\x91\x90a.QV[a\x05\xB5V[`@Qa\x02\x03\x91\x90a.\x96V[`@Q\x80\x91\x03\x90\xF3[a\x02&`\x04\x806\x03\x81\x01\x90a\x02!\x91\x90a.\xE2V[a\x06.V[`@Qa\x023\x91\x90a/\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x02V`\x04\x806\x03\x81\x01\x90a\x02Q\x91\x90a/\x8FV[a\x06KV[\0[a\x02r`\x04\x806\x03\x81\x01\x90a\x02m\x91\x90a0aV[a\x06mV[`@Qa\x02\x7F\x91\x90a/\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x02\xA2`\x04\x806\x03\x81\x01\x90a\x02\x9D\x91\x90a/\x8FV[a\t\x83V[\0[a\x02\xBE`\x04\x806\x03\x81\x01\x90a\x02\xB9\x91\x90a.\xE2V[a\t\xFEV[\0[a\x02\xDA`\x04\x806\x03\x81\x01\x90a\x02\xD5\x91\x90a.\xE2V[a\x0B\xE6V[\0[a\x02\xE4a\x0F$V[\0[a\x03\0`\x04\x806\x03\x81\x01\x90a\x02\xFB\x91\x90a1\x0FV[a\x0F;V[\0[a\x03\x1C`\x04\x806\x03\x81\x01\x90a\x03\x17\x91\x90a.\xE2V[a\x13\xDFV[`@Qa\x03)\x91\x90a34V[`@Q\x80\x91\x03\x90\xF3[a\x03:a\x16\x05V[`@Qa\x03G\x91\x90a.\x96V[`@Q\x80\x91\x03\x90\xF3[a\x03Xa\x16\x19V[`@Qa\x03e\x91\x90a3cV[`@Q\x80\x91\x03\x90\xF3[a\x03\x88`\x04\x806\x03\x81\x01\x90a\x03\x83\x91\x90a.\xE2V[a\x16\x1FV[\0[a\x03\x92a\x17\xF2V[\0[a\x03\xAE`\x04\x806\x03\x81\x01\x90a\x03\xA9\x91\x90a3|V[a\x18\tV[\0[a\x03\xCA`\x04\x806\x03\x81\x01\x90a\x03\xC5\x91\x90a3\xA7V[a\x18 V[\0[a\x03\xE6`\x04\x806\x03\x81\x01\x90a\x03\xE1\x91\x90a3|V[a\x19\x14V[\0[a\x04\x02`\x04\x806\x03\x81\x01\x90a\x03\xFD\x91\x90a3\xF7V[a\x19+V[\0[a\x04\x1E`\x04\x806\x03\x81\x01\x90a\x04\x19\x91\x90a/\x8FV[a\x1B\x0CV[`@Qa\x04+\x91\x90a.\x96V[`@Q\x80\x91\x03\x90\xF3[a\x04<a\x1BpV[`@Qa\x04I\x91\x90a/\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x04l`\x04\x806\x03\x81\x01\x90a\x04g\x91\x90a.\xE2V[a\x1B\x94V[`@Qa\x04\x82\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a4\x9BV[`@Q\x80\x91\x03\x90\xF3[a\x04\xA5`\x04\x806\x03\x81\x01\x90a\x04\xA0\x91\x90a.\xE2V[a\x1C\xD3V[\0[a\x04\xC1`\x04\x806\x03\x81\x01\x90a\x04\xBC\x91\x90a/\x8FV[a\x1D\x1BV[\0[a\x04\xCBa\x1DdV[`@Qa\x04\xD8\x91\x90a/\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x04\xFB`\x04\x806\x03\x81\x01\x90a\x04\xF6\x91\x90a.\xE2V[a\x1DjV[\0[a\x05\x17`\x04\x806\x03\x81\x01\x90a\x05\x12\x91\x90a5<V[a \x84V[`@Qa\x05$\x91\x90a3cV[`@Q\x80\x91\x03\x90\xF3[a\x05G`\x04\x806\x03\x81\x01\x90a\x05B\x91\x90a/\x8FV[a \xA4V[\0[a\x05Qa \xC6V[`@Qa\x05^\x91\x90a/\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x05oa \xEAV[`@Qa\x05|\x91\x90a3cV[`@Q\x80\x91\x03\x90\xF3[a\x05\x9F`\x04\x806\x03\x81\x01\x90a\x05\x9A\x91\x90a5<V[a \xF0V[`@Qa\x05\xAC\x91\x90a3cV[`@Q\x80\x91\x03\x90\xF3[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06'WPa\x06&\x82a!\x10V[[\x90P\x91\x90PV[_`\x01_\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[a\x06T\x82a\x06.V[a\x06]\x81a!yV[a\x06g\x83\x83a!\x8DV[PPPPV[_a\x06va\"vV[_\x85\x11a\x06\xB8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xAF\x90a5\xC4V[`@Q\x80\x91\x03\x90\xFD[3\x86\x86B`@Q` \x01a\x06\xCF\x94\x93\x92\x91\x90a6GV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`@Q\x80a\x01@\x01`@R\x803s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x85\x81R` \x01_`\x05\x81\x11\x15a\x07\xAFWa\x07\xAEa1\xDBV[[\x81R` \x01B\x81R` \x01_\x81R` \x01__\x1B\x81RP`\x02_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x02\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01\x90\x81a\x08\xC6\x91\x90a8\xBEV[P`\xA0\x82\x01Q\x81`\x05\x01U`\xC0\x82\x01Q\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x08\xFBWa\x08\xFAa1\xDBV[[\x02\x17\x90UP`\xE0\x82\x01Q\x81`\x07\x01Ua\x01\0\x82\x01Q\x81`\x08\x01Ua\x01 \x82\x01Q\x81`\t\x01U\x90PP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x7F\xF2\xB2v^\xDF\x82\xFC\x07\xE2l\xEA\x13P\xAE\xCC\x84;.k\xBC\x95\xC7\xB5 v\xD6\\\x97\xC40\x11U\x88\x88\x88\x88\x88`@Qa\tr\x95\x94\x93\x92\x91\x90a9\xC7V[`@Q\x80\x91\x03\x90\xA3\x95\x94PPPPPV[a\t\x8Ba\"\xB7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\t\xEFW`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xF9\x82\x82a\"\xBEV[PPPV[_`\x02_\x83\x81R` \x01\x90\x81R` \x01_ \x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\n\xBCWP\x80`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\n\xFBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\xF2\x90a:]V[`@Q\x80\x91\x03\x90\xFD[`\x02`\x05\x81\x11\x15a\x0B\x0FWa\x0B\x0Ea1\xDBV[[\x81`\x06\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x0B2Wa\x0B1a1\xDBV[[\x14a\x0BrW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Bi\x90a:\xEBV[`@Q\x80\x91\x03\x90\xFD[`\x05\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x0B\x99Wa\x0B\x98a1\xDBV[[\x02\x17\x90UP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x7F\x98\t\xDFl\x9Dcd*o\x1E3\xA6\x98\xD4\x15\xBA-\x88*@q\x89M\xA7H\xABT\x82\x97\\\x8Ec`@Q`@Q\x80\x91\x03\x90\xA3PPV[a\x0B\xEEa#\xA8V[_`\x02_\x83\x81R` \x01\x90\x81R` \x01_ \x90P3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C\x93W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x8A\x90a;SV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x05\x81\x11\x15a\x0C\xA7Wa\x0C\xA6a1\xDBV[[\x81`\x06\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x0C\xCAWa\x0C\xC9a1\xDBV[[\x14a\r\nW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\x01\x90a;\xBBV[`@Q\x80\x91\x03\x90\xFD[`\x04T\x81`\x08\x01Ta\r\x1C\x91\x90a<\x06V[B\x11a\r]W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\rT\x90a<\x83V[`@Q\x80\x91\x03\x90\xFD[`\x04\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\r\x84Wa\r\x83a1\xDBV[[\x02\x17\x90UP\x80`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x82`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01T`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\x0E\x92\x91\x90a<\xA1V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E*W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EN\x91\x90a<\xDCV[a\x0E\x8DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\x84\x90a=QV[`@Q\x80\x91\x03\x90\xFD[a\x0E\xE2\x81`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01Ta#\xCAV[\x81\x7F8lO\xC2\xFEQ\xE1}\xD0\xF7\xB5\xBBm8\xFC\xA3\xA6R\xD4\xE6\xFB\xCF\xA1\xA92\x83\x03J\xCD\xC5\x84:`@Qa\x0F\x10\x90a=\xB9V[`@Q\x80\x91\x03\x90\xA2Pa\x0F!a%9V[PV[__\x1Ba\x0F0\x81a!yV[a\x0F8a%SV[PV[__\x1Ba\x0FG\x81a!yV[a\x0FOa#\xA8V[_`\x02_\x85\x81R` \x01\x90\x81R` \x01_ \x90P`\x05\x80\x81\x11\x15a\x0FvWa\x0Fua1\xDBV[[\x81`\x06\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x0F\x99Wa\x0F\x98a1\xDBV[[\x14a\x0F\xD9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xD0\x90a>!V[`@Q\x80\x91\x03\x90\xFD[\x82\x15a\x11\xD9W`\x03\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x10\x06Wa\x10\x05a1\xDBV[[\x02\x17\x90UP\x80`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x82_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01T`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\x8F\x92\x91\x90a<\xA1V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10\xABW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xCF\x91\x90a<\xDCV[a\x11\x0EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\x05\x90a>\x89V[`@Q\x80\x91\x03\x90\xFD[a\x11c\x81`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01Ta#\xCAV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7FA\xF6\x7F\xE3\xF6\x7F\x1D\xCEhv\x94\x865,;\xC7\xC5\xC5\xAC\xDA\x8D<\x0Ef(\xF9\x18\xC2U\xE3J\x0B\x83_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Qa\x11\xCC\x91\x90a>\xA7V[`@Q\x80\x91\x03\x90\xA3a\x13\xD1V[`\x04\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x12\0Wa\x11\xFFa1\xDBV[[\x02\x17\x90UP\x80`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x82`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01T`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\x8A\x92\x91\x90a<\xA1V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12\xA6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xCA\x91\x90a<\xDCV[a\x13\tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\0\x90a?\nV[`@Q\x80\x91\x03\x90\xFD[a\x13^\x81`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01Ta#\xCAV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7FA\xF6\x7F\xE3\xF6\x7F\x1D\xCEhv\x94\x865,;\xC7\xC5\xC5\xAC\xDA\x8D<\x0Ef(\xF9\x18\xC2U\xE3J\x0B\x83`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Qa\x13\xC8\x91\x90a>\xA7V[`@Q\x80\x91\x03\x90\xA3[Pa\x13\xDAa%9V[PPPV[a\x13\xE7a-VV[`\x02_\x83\x81R` \x01\x90\x81R` \x01_ `@Q\x80a\x01@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01\x80Ta\x15\x1C\x90a6\xEEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15H\x90a6\xEEV[\x80\x15a\x15\x93W\x80`\x1F\x10a\x15jWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\x93V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15vW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x05\x82\x01T\x81R` \x01`\x06\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x15\xCAWa\x15\xC9a1\xDBV[[`\x05\x81\x11\x15a\x15\xDCWa\x15\xDBa1\xDBV[[\x81R` \x01`\x07\x82\x01T\x81R` \x01`\x08\x82\x01T\x81R` \x01`\t\x82\x01T\x81RPP\x90P\x91\x90PV[___\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[`\x04T\x81V[_`\x02_\x83\x81R` \x01\x90\x81R` \x01_ \x90P3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x16\xC3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16\xBA\x90a?rV[`@Q\x80\x91\x03\x90\xFD[_`\x05\x81\x11\x15a\x16\xD6Wa\x16\xD5a1\xDBV[[\x81`\x06\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x16\xF9Wa\x16\xF8a1\xDBV[[\x14a\x179W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x170\x90a?\xDAV[`@Q\x80\x91\x03\x90\xFD[`\x03T\x81`\x07\x01Ta\x17K\x91\x90a<\x06V[B\x11a\x17\x8CW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17\x83\x90a@BV[`@Q\x80\x91\x03\x90\xFD[`\x04\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x17\xB3Wa\x17\xB2a1\xDBV[[\x02\x17\x90UP\x81\x7F8lO\xC2\xFEQ\xE1}\xD0\xF7\xB5\xBBm8\xFC\xA3\xA6R\xD4\xE6\xFB\xCF\xA1\xA92\x83\x03J\xCD\xC5\x84:`@Qa\x17\xE6\x90a@\xAAV[`@Q\x80\x91\x03\x90\xA2PPV[__\x1Ba\x17\xFE\x81a!yV[a\x18\x06a%\xB3V[PV[__\x1Ba\x18\x15\x81a!yV[\x81`\x03\x81\x90UPPPV[__\x1Ba\x18,\x81a!yV[\x81`\x05_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fd\xD1\x9F\xED\xCE\x93Q\xD7\xC1\x94\x98Qc\xE6\xB4\0U\\\x18k\xA7\x85\x88\x129\x15d\x13\x94\x8A\x8B\xCA\x84`@Qa\x19\x06\x91\x90a3cV[`@Q\x80\x91\x03\x90\xA3PPPPV[__\x1Ba\x19 \x81a!yV[\x81`\x04\x81\x90UPPPV[_`\x02_\x84\x81R` \x01\x90\x81R` \x01_ \x90P3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x19\xCFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\xC6\x90a?rV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x05\x81\x11\x15a\x19\xE3Wa\x19\xE2a1\xDBV[[\x81`\x06\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x1A\x06Wa\x1A\x05a1\xDBV[[\x14a\x1AFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A=\x90aA\x12V[`@Q\x80\x91\x03\x90\xFD[`\x04T\x81`\x08\x01Ta\x1AX\x91\x90a<\x06V[B\x11\x15a\x1A\x9AW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\x91\x90aAzV[`@Q\x80\x91\x03\x90\xFD[`\x02\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x1A\xC1Wa\x1A\xC0a1\xDBV[[\x02\x17\x90UP\x81\x81`\t\x01\x81\x90UP\x82\x7F\xD8\xE2^X\xEAj\xC5\xE1\xDF\xF3\x1C\xC8\xC3A\x92,M\x1C\xCDB|\x84\xE0\xBF3\x9CQ\x0BKV\x10b\x83`@Qa\x1A\xFF\x91\x90a/\x1CV[`@Q\x80\x91\x03\x90\xA2PPPV[_`\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[\x7F\xE2\xB7\xFB;\x83!tv\x91\x06\xDA\xEB\xCF\xD6\xD1\x97\x05#$\r\xDA\x11(\x11\x02\xDB\x93c\xB8;\r\xC4\x81V[`\x02` R\x80_R`@_ _\x91P\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80`\x03\x01T\x90\x80`\x04\x01\x80Ta\x1C(\x90a6\xEEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1CT\x90a6\xEEV[\x80\x15a\x1C\x9FW\x80`\x1F\x10a\x1CvWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C\x9FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\x82W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x05\x01T\x90\x80`\x06\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90\x80`\x07\x01T\x90\x80`\x08\x01T\x90\x80`\t\x01T\x90P\x8AV[a\x1C\xDBa#\xA8V[\x7F\xB0)n\xA8\xDD2'7\x19'\xB1\xC1\xCE\xA2\xB1.\xA3\x94t=\xDF/2\xF5\x80$\xCE&\xF8:$\xA6a\x1D\x05\x81a!yV[a\x1D\x0F\x823a&\x14V[Pa\x1D\x18a%9V[PV[a\x1D#a#\xA8V[\x7F\xE2\xB7\xFB;\x83!tv\x91\x06\xDA\xEB\xCF\xD6\xD1\x97\x05#$\r\xDA\x11(\x11\x02\xDB\x93c\xB8;\r\xC4a\x1DM\x81a!yV[a\x1DW\x83\x83a&\x14V[Pa\x1D`a%9V[PPV[__\x1B\x81V[a\x1Dra#\xA8V[_`\x02_\x83\x81R` \x01\x90\x81R` \x01_ \x90P3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1E\x17W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E\x0E\x90a;SV[`@Q\x80\x91\x03\x90\xFD[`\x02`\x05\x81\x11\x15a\x1E+Wa\x1E*a1\xDBV[[\x81`\x06\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x1ENWa\x1EMa1\xDBV[[\x14a\x1E\x8EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E\x85\x90aA\xE2V[`@Q\x80\x91\x03\x90\xFD[`\x03\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x1E\xB5Wa\x1E\xB4a1\xDBV[[\x02\x17\x90UP\x80`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x82_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01T`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F>\x92\x91\x90a<\xA1V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1FZW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F~\x91\x90a<\xDCV[a\x1F\xBDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1F\xB4\x90aBJV[`@Q\x80\x91\x03\x90\xFD[a \x12\x81`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01Ta#\xCAV[\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x7F\xA1:\x9Dw\xA6\xFB\xD6\x90\xB8\0\x0B\x14\xCC\x8FK\xA7`+2\x997c\x8Ep\x897\xFD\x9F\xAA`i\xCD`@Q`@Q\x80\x91\x03\x90\xA3Pa \x81a%9V[PV[`\x06` R\x81_R`@_ ` R\x80_R`@_ _\x91P\x91PPT\x81V[a \xAD\x82a\x06.V[a \xB6\x81a!yV[a \xC0\x83\x83a\"\xBEV[PPPPV[\x7F\xB0)n\xA8\xDD2'7\x19'\xB1\xC1\xCE\xA2\xB1.\xA3\x94t=\xDF/2\xF5\x80$\xCE&\xF8:$\xA6\x81V[`\x03T\x81V[`\x05` R\x81_R`@_ ` R\x80_R`@_ _\x91P\x91PPT\x81V[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[a!\x8A\x81a!\x85a\"\xB7V[a)qV[PV[_a!\x98\x83\x83a\x1B\x0CV[a\"lW`\x01\x80_\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\"\ta\"\xB7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\"pV[_\x90P[\x92\x91PPV[a\"~a\x16\x05V[\x15a\"\xB5W`@Q\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_3\x90P\x90V[_a\"\xC9\x83\x83a\x1B\x0CV[\x15a#\x9EW_`\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa#;a\"\xB7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa#\xA2V[_\x90P[\x92\x91PPV[a#\xB0a)\xC2V[`\x02a#\xC2a#\xBDa*\x03V[a*,V[_\x01\x81\x90UPV[_\x81`\x06_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ Ta$O\x91\x90aBhV[\x90P\x80`\x06_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x08\x11\x83\x19U~\x99\x98X\x82G\xD4h7\xAE\xA5\xC7}\xE4\x89g\x93cC\xB6\xC9#\xFB\x1Fx\x13\xE3\x83`@Qa%+\x91\x90a3cV[`@Q\x80\x91\x03\x90\xA3PPPPV[`\x01a%Ka%Fa*\x03V[a*,V[_\x01\x81\x90UPV[a%[a*5V[___a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAAa%\x9Ca\"\xB7V[`@Qa%\xA9\x91\x90a>\xA7V[`@Q\x80\x91\x03\x90\xA1V[a%\xBBa\"vV[`\x01__a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa%\xFDa\"\xB7V[`@Qa&\n\x91\x90a>\xA7V[`@Q\x80\x91\x03\x90\xA1V[_`\x02_\x84\x81R` \x01\x90\x81R` \x01_ \x90P_`\x05\x81\x11\x15a&;Wa&:a1\xDBV[[\x81`\x06\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a&^Wa&]a1\xDBV[[\x14a&\x9EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\x95\x90a?\xDAV[`@Q\x80\x91\x03\x90\xFD[`\x03T\x81`\x07\x01Ta&\xB0\x91\x90a<\x06V[B\x11\x15a&\xF2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\xE9\x90aB\xE5V[`@Q\x80\x91\x03\x90\xFD[a'\x1C\x7F\xB0)n\xA8\xDD2'7\x19'\xB1\xC1\xCE\xA2\xB1.\xA3\x94t=\xDF/2\xF5\x80$\xCE&\xF8:$\xA6\x83a\x1B\x0CV[a'[W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a'R\x90aCMV[`@Q\x80\x91\x03\x90\xFD[a'\x8D\x82\x82`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01Ta*uV[\x81\x81`\x01\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a'\xF6Wa'\xF5a1\xDBV[[\x02\x17\x90UPB\x81`\x08\x01\x81\x90UP\x80`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD\x830\x84`\x03\x01T`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a(h\x93\x92\x91\x90aCkV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a(\x84W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xA8\x91\x90a<\xDCV[a(\xE7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a(\xDE\x90aC\xEAV[`@Q\x80\x91\x03\x90\xFD[a)\x19\x82\x82`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01Ta+\xCBV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\x15\x14B\x17@Pdc\x1D\xE9Qu!\x113Om\x9D\xB4\xBEl\xFF\xF4SF\xF7\x06\x8E\xA8W\xFC\xAD\x83`\x03\x01T`@Qa)d\x91\x90a3cV[`@Q\x80\x91\x03\x90\xA3PPPV[a){\x82\x82a\x1B\x0CV[a)\xBEW\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)\xB5\x92\x91\x90aD\x08V[`@Q\x80\x91\x03\x90\xFD[PPV[a)\xCAa-:V[\x15a*\x01W`@Q\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0_\x1B\x90P\x90V[_\x81\x90P\x91\x90PV[a*=a\x16\x05V[a*sW`@Q\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_`\x05_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x03a*\xFEWPa+\xC6V[\x80\x82`\x06_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ Ta+\x83\x91\x90a<\x06V[\x11\x15a+\xC4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a+\xBB\x90aDyV[`@Q\x80\x91\x03\x90\xFD[P[PPPV[_\x81`\x06_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ Ta,P\x91\x90a<\x06V[\x90P\x80`\x06_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x08\x11\x83\x19U~\x99\x98X\x82G\xD4h7\xAE\xA5\xC7}\xE4\x89g\x93cC\xB6\xC9#\xFB\x1Fx\x13\xE3\x83`@Qa-,\x91\x90a3cV[`@Q\x80\x91\x03\x90\xA3PPPPV[_`\x02a-Ma-Ha*\x03V[a*,V[_\x01T\x14\x90P\x90V[`@Q\x80a\x01@\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81R` \x01``\x81R` \x01_\x81R` \x01_`\x05\x81\x11\x15a-\xDCWa-\xDBa1\xDBV[[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[__\xFD[__\xFD[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a.0\x81a-\xFCV[\x81\x14a.:W__\xFD[PV[_\x815\x90Pa.K\x81a.'V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a.fWa.ea-\xF4V[[_a.s\x84\x82\x85\x01a.=V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a.\x90\x81a.|V[\x82RPPV[_` \x82\x01\x90Pa.\xA9_\x83\x01\x84a.\x87V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a.\xC1\x81a.\xAFV[\x81\x14a.\xCBW__\xFD[PV[_\x815\x90Pa.\xDC\x81a.\xB8V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a.\xF7Wa.\xF6a-\xF4V[[_a/\x04\x84\x82\x85\x01a.\xCEV[\x91PP\x92\x91PPV[a/\x16\x81a.\xAFV[\x82RPPV[_` \x82\x01\x90Pa//_\x83\x01\x84a/\rV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a/^\x82a/5V[\x90P\x91\x90PV[a/n\x81a/TV[\x81\x14a/xW__\xFD[PV[_\x815\x90Pa/\x89\x81a/eV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a/\xA5Wa/\xA4a-\xF4V[[_a/\xB2\x85\x82\x86\x01a.\xCEV[\x92PP` a/\xC3\x85\x82\x86\x01a/{V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a/\xDF\x81a/\xCDV[\x81\x14a/\xE9W__\xFD[PV[_\x815\x90Pa/\xFA\x81a/\xD6V[\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a0!Wa0 a0\0V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0>Wa0=a0\x04V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a0ZWa0Ya0\x08V[[\x92P\x92\x90PV[_____`\x80\x86\x88\x03\x12\x15a0zWa0ya-\xF4V[[_a0\x87\x88\x82\x89\x01a/{V[\x95PP` a0\x98\x88\x82\x89\x01a/\xECV[\x94PP`@a0\xA9\x88\x82\x89\x01a/\xECV[\x93PP``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xCAWa0\xC9a-\xF8V[[a0\xD6\x88\x82\x89\x01a0\x0CV[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[a0\xEE\x81a.|V[\x81\x14a0\xF8W__\xFD[PV[_\x815\x90Pa1\t\x81a0\xE5V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a1%Wa1$a-\xF4V[[_a12\x85\x82\x86\x01a.\xCEV[\x92PP` a1C\x85\x82\x86\x01a0\xFBV[\x91PP\x92P\x92\x90PV[a1V\x81a/TV[\x82RPPV[a1e\x81a/\xCDV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a1\xAD\x82a1kV[a1\xB7\x81\x85a1uV[\x93Pa1\xC7\x81\x85` \x86\x01a1\x85V[a1\xD0\x81a1\x93V[\x84\x01\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x06\x81\x10a2\x19Wa2\x18a1\xDBV[[PV[_\x81\x90Pa2)\x82a2\x08V[\x91\x90PV[_a28\x82a2\x1CV[\x90P\x91\x90PV[a2H\x81a2.V[\x82RPPV[a2W\x81a.\xAFV[\x82RPPV[_a\x01@\x83\x01_\x83\x01Qa2s_\x86\x01\x82a1MV[P` \x83\x01Qa2\x86` \x86\x01\x82a1MV[P`@\x83\x01Qa2\x99`@\x86\x01\x82a1MV[P``\x83\x01Qa2\xAC``\x86\x01\x82a1\\V[P`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra2\xC4\x82\x82a1\xA3V[\x91PP`\xA0\x83\x01Qa2\xD9`\xA0\x86\x01\x82a1\\V[P`\xC0\x83\x01Qa2\xEC`\xC0\x86\x01\x82a2?V[P`\xE0\x83\x01Qa2\xFF`\xE0\x86\x01\x82a1\\V[Pa\x01\0\x83\x01Qa3\x14a\x01\0\x86\x01\x82a1\\V[Pa\x01 \x83\x01Qa3)a\x01 \x86\x01\x82a2NV[P\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra3L\x81\x84a2]V[\x90P\x92\x91PPV[a3]\x81a/\xCDV[\x82RPPV[_` \x82\x01\x90Pa3v_\x83\x01\x84a3TV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a3\x91Wa3\x90a-\xF4V[[_a3\x9E\x84\x82\x85\x01a/\xECV[\x91PP\x92\x91PPV[___``\x84\x86\x03\x12\x15a3\xBEWa3\xBDa-\xF4V[[_a3\xCB\x86\x82\x87\x01a/{V[\x93PP` a3\xDC\x86\x82\x87\x01a/{V[\x92PP`@a3\xED\x86\x82\x87\x01a/\xECV[\x91PP\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15a4\rWa4\x0Ca-\xF4V[[_a4\x1A\x85\x82\x86\x01a.\xCEV[\x92PP` a4+\x85\x82\x86\x01a.\xCEV[\x91PP\x92P\x92\x90PV[a4>\x81a/TV[\x82RPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a4^\x82a1kV[a4h\x81\x85a4DV[\x93Pa4x\x81\x85` \x86\x01a1\x85V[a4\x81\x81a1\x93V[\x84\x01\x91PP\x92\x91PPV[a4\x95\x81a2.V[\x82RPPV[_a\x01@\x82\x01\x90Pa4\xAF_\x83\x01\x8Da45V[a4\xBC` \x83\x01\x8Ca45V[a4\xC9`@\x83\x01\x8Ba45V[a4\xD6``\x83\x01\x8Aa3TV[\x81\x81\x03`\x80\x83\x01Ra4\xE8\x81\x89a4TV[\x90Pa4\xF7`\xA0\x83\x01\x88a3TV[a5\x04`\xC0\x83\x01\x87a4\x8CV[a5\x11`\xE0\x83\x01\x86a3TV[a5\x1Fa\x01\0\x83\x01\x85a3TV[a5-a\x01 \x83\x01\x84a/\rV[\x9B\x9APPPPPPPPPPPV[__`@\x83\x85\x03\x12\x15a5RWa5Qa-\xF4V[[_a5_\x85\x82\x86\x01a/{V[\x92PP` a5p\x85\x82\x86\x01a/{V[\x91PP\x92P\x92\x90PV[\x7FToken amount must be > 0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a5\xAE`\x18\x83a4DV[\x91Pa5\xB9\x82a5zV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra5\xDB\x81a5\xA2V[\x90P\x91\x90PV[_\x81``\x1B\x90P\x91\x90PV[_a5\xF8\x82a5\xE2V[\x90P\x91\x90PV[_a6\t\x82a5\xEEV[\x90P\x91\x90PV[a6!a6\x1C\x82a/TV[a5\xFFV[\x82RPPV[_\x81\x90P\x91\x90PV[a6Aa6<\x82a/\xCDV[a6'V[\x82RPPV[_a6R\x82\x87a6\x10V[`\x14\x82\x01\x91Pa6b\x82\x86a6\x10V[`\x14\x82\x01\x91Pa6r\x82\x85a60V[` \x82\x01\x91Pa6\x82\x82\x84a60V[` \x82\x01\x91P\x81\x90P\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a7\x05W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a7\x18Wa7\x17a6\xC1V[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a7z\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a7?V[a7\x84\x86\x83a7?V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a7\xBFa7\xBAa7\xB5\x84a/\xCDV[a7\x9CV[a/\xCDV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a7\xD8\x83a7\xA5V[a7\xECa7\xE4\x82a7\xC6V[\x84\x84Ta7KV[\x82UPPPPV[__\x90P\x90V[a8\x03a7\xF4V[a8\x0E\x81\x84\x84a7\xCFV[PPPV[[\x81\x81\x10\x15a81Wa8&_\x82a7\xFBV[`\x01\x81\x01\x90Pa8\x14V[PPV[`\x1F\x82\x11\x15a8vWa8G\x81a7\x1EV[a8P\x84a70V[\x81\x01` \x85\x10\x15a8_W\x81\x90P[a8sa8k\x85a70V[\x83\x01\x82a8\x13V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a8\x96_\x19\x84`\x08\x02a8{V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a8\xAE\x83\x83a8\x87V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a8\xC7\x82a1kV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\xE0Wa8\xDFa6\x94V[[a8\xEA\x82Ta6\xEEV[a8\xF5\x82\x82\x85a85V[_` \x90P`\x1F\x83\x11`\x01\x81\x14a9&W_\x84\x15a9\x14W\x82\x87\x01Q\x90P[a9\x1E\x85\x82a8\xA3V[\x86UPa9\x85V[`\x1F\x19\x84\x16a94\x86a7\x1EV[_[\x82\x81\x10\x15a9[W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa96V[\x86\x83\x10\x15a9xW\x84\x89\x01Qa9t`\x1F\x89\x16\x82a8\x87V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[\x82\x81\x837_\x83\x83\x01RPPPV[_a9\xA6\x83\x85a4DV[\x93Pa9\xB3\x83\x85\x84a9\x8DV[a9\xBC\x83a1\x93V[\x84\x01\x90P\x93\x92PPPV[_`\x80\x82\x01\x90Pa9\xDA_\x83\x01\x88a45V[a9\xE7` \x83\x01\x87a3TV[a9\xF4`@\x83\x01\x86a3TV[\x81\x81\x03``\x83\x01Ra:\x07\x81\x84\x86a9\x9BV[\x90P\x96\x95PPPPPPV[\x7FNot a party to the order\0\0\0\0\0\0\0\0_\x82\x01RPV[_a:G`\x18\x83a4DV[\x91Pa:R\x82a:\x13V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra:t\x81a:;V[\x90P\x91\x90PV[\x7FCan only dispute after fiat is m_\x82\x01R\x7Farked sent\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a:\xD5`*\x83a4DV[\x91Pa:\xE0\x82a:{V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;\x02\x81a:\xC9V[\x90P\x91\x90PV[\x7FNot the assigned LP\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a;=`\x13\x83a4DV[\x91Pa;H\x82a;\tV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;j\x81a;1V[\x90P\x91\x90PV[\x7FOrder status invalid\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a;\xA5`\x14\x83a4DV[\x91Pa;\xB0\x82a;qV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;\xD2\x81a;\x99V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a<\x10\x82a/\xCDV[\x91Pa<\x1B\x83a/\xCDV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a<3Wa<2a;\xD9V[[\x92\x91PPV[\x7FPayment deadline not passed\0\0\0\0\0_\x82\x01RPV[_a<m`\x1B\x83a4DV[\x91Pa<x\x82a<9V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra<\x9A\x81a<aV[\x90P\x91\x90PV[_`@\x82\x01\x90Pa<\xB4_\x83\x01\x85a45V[a<\xC1` \x83\x01\x84a3TV[\x93\x92PPPV[_\x81Q\x90Pa<\xD6\x81a0\xE5V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a<\xF1Wa<\xF0a-\xF4V[[_a<\xFE\x84\x82\x85\x01a<\xC8V[\x91PP\x92\x91PPV[\x7FRECLAIM_TRANSFER_FAILED\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a=;`\x17\x83a4DV[\x91Pa=F\x82a=\x07V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra=h\x81a=/V[\x90P\x91\x90PV[\x7FUser payment timed out\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a=\xA3`\x16\x83a4DV[\x91Pa=\xAE\x82a=oV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra=\xD0\x81a=\x97V[\x90P\x91\x90PV[\x7FOrder not in dispute\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a>\x0B`\x14\x83a4DV[\x91Pa>\x16\x82a=\xD7V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra>8\x81a=\xFFV[\x90P\x91\x90PV[\x7FDISPUTE_BUYER_TRANSFER_FAILED\0\0\0_\x82\x01RPV[_a>s`\x1D\x83a4DV[\x91Pa>~\x82a>?V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra>\xA0\x81a>gV[\x90P\x91\x90PV[_` \x82\x01\x90Pa>\xBA_\x83\x01\x84a45V[\x92\x91PPV[\x7FDISPUTE_LP_TRANSFER_FAILED\0\0\0\0\0\0_\x82\x01RPV[_a>\xF4`\x1A\x83a4DV[\x91Pa>\xFF\x82a>\xC0V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra?!\x81a>\xE8V[\x90P\x91\x90PV[\x7FNot the buyer\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a?\\`\r\x83a4DV[\x91Pa?g\x82a?(V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra?\x89\x81a?PV[\x90P\x91\x90PV[\x7FOrder not pending\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a?\xC4`\x11\x83a4DV[\x91Pa?\xCF\x82a?\x90V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra?\xF1\x81a?\xB8V[\x90P\x91\x90PV[\x7FLock deadline not passed\0\0\0\0\0\0\0\0_\x82\x01RPV[_a@,`\x18\x83a4DV[\x91Pa@7\x82a?\xF8V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra@Y\x81a@ V[\x90P\x91\x90PV[\x7FLP failed to lock funds\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a@\x94`\x17\x83a4DV[\x91Pa@\x9F\x82a@`V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra@\xC1\x81a@\x88V[\x90P\x91\x90PV[\x7FFunds not locked\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a@\xFC`\x10\x83a4DV[\x91PaA\x07\x82a@\xC8V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaA)\x81a@\xF0V[\x90P\x91\x90PV[\x7FPayment deadline passed\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aAd`\x17\x83a4DV[\x91PaAo\x82aA0V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaA\x91\x81aAXV[\x90P\x91\x90PV[\x7FFiat not marked as sent\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aA\xCC`\x17\x83a4DV[\x91PaA\xD7\x82aA\x98V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaA\xF9\x81aA\xC0V[\x90P\x91\x90PV[\x7FRELEASE_TRANSFER_FAILED\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aB4`\x17\x83a4DV[\x91PaB?\x82aB\0V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaBa\x81aB(V[\x90P\x91\x90PV[_aBr\x82a/\xCDV[\x91PaB}\x83a/\xCDV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aB\x95WaB\x94a;\xD9V[[\x92\x91PPV[\x7FLock deadline passed\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aB\xCF`\x14\x83a4DV[\x91PaB\xDA\x82aB\x9BV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaB\xFC\x81aB\xC3V[\x90P\x91\x90PV[\x7FLP_ROLE_REQUIRED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aC7`\x10\x83a4DV[\x91PaCB\x82aC\x03V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaCd\x81aC+V[\x90P\x91\x90PV[_``\x82\x01\x90PaC~_\x83\x01\x86a45V[aC\x8B` \x83\x01\x85a45V[aC\x98`@\x83\x01\x84a3TV[\x94\x93PPPPV[\x7FLOCK_TRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aC\xD4`\x14\x83a4DV[\x91PaC\xDF\x82aC\xA0V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaD\x01\x81aC\xC8V[\x90P\x91\x90PV[_`@\x82\x01\x90PaD\x1B_\x83\x01\x85a45V[aD(` \x83\x01\x84a/\rV[\x93\x92PPPV[\x7FLP_CAP_EXCEEDED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aDc`\x0F\x83a4DV[\x91PaDn\x82aD/V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaD\x90\x81aDWV[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \x11<\xB5^\x07\x10\xCA\xB3\x04\x1E\n\xB0\xB5*\xEEi\xC9Psc\xF0\xE9\xFE\xEC\x9C\xE2%\xDD\xD8r\xDA\xCDdsolcC\0\x08\x1E\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106101d8575f3560e01c8063896a2ff4116101025780639e49f7e4116100a0578063d547741f1161006f578063d547741f1461052d578063e1d3945014610549578063fa4d0c3c14610567578063fc46d8b814610585576101d8565b80639e49f7e4146104a7578063a217fddf146104c3578063c92ee043146104e1578063ce9bc15e146104fd576101d8565b806391d14854116100dc57806391d1485414610404578063926d7d7f146104345780639c3f1e90146104525780639cbeb6c11461048b576101d8565b8063896a2ff4146103b05780638cfd919e146103cc5780638fa4a641146103e8576101d8565b80633f4ba83a1161017a5780636d929f23116101495780636d929f23146103505780637489ec231461036e5780638456cb591461038a57806384a5ce5314610394576101d8565b80633f4ba83a146102dc57806343a0e3e6146102e65780635778472a146103025780635c975abb14610332576101d8565b8063305df56f116101b6578063305df56f1461025857806336568abe146102885780633e4e3792146102a45780633e88e0b6146102c0576101d8565b806301ffc9a7146101dc578063248a9ca31461020c5780632f2ff15d1461023c575b5f5ffd5b6101f660048036038101906101f19190612e51565b6105b5565b6040516102039190612e96565b60405180910390f35b61022660048036038101906102219190612ee2565b61062e565b6040516102339190612f1c565b60405180910390f35b61025660048036038101906102519190612f8f565b61064b565b005b610272600480360381019061026d9190613061565b61066d565b60405161027f9190612f1c565b60405180910390f35b6102a2600480360381019061029d9190612f8f565b610983565b005b6102be60048036038101906102b99190612ee2565b6109fe565b005b6102da60048036038101906102d59190612ee2565b610be6565b005b6102e4610f24565b005b61030060048036038101906102fb919061310f565b610f3b565b005b61031c60048036038101906103179190612ee2565b6113df565b6040516103299190613334565b60405180910390f35b61033a611605565b6040516103479190612e96565b60405180910390f35b610358611619565b6040516103659190613363565b60405180910390f35b61038860048036038101906103839190612ee2565b61161f565b005b6103926117f2565b005b6103ae60048036038101906103a9919061337c565b611809565b005b6103ca60048036038101906103c591906133a7565b611820565b005b6103e660048036038101906103e1919061337c565b611914565b005b61040260048036038101906103fd91906133f7565b61192b565b005b61041e60048036038101906104199190612f8f565b611b0c565b60405161042b9190612e96565b60405180910390f35b61043c611b70565b6040516104499190612f1c565b60405180910390f35b61046c60048036038101906104679190612ee2565b611b94565b6040516104829a9998979695949392919061349b565b60405180910390f35b6104a560048036038101906104a09190612ee2565b611cd3565b005b6104c160048036038101906104bc9190612f8f565b611d1b565b005b6104cb611d64565b6040516104d89190612f1c565b60405180910390f35b6104fb60048036038101906104f69190612ee2565b611d6a565b005b6105176004803603810190610512919061353c565b612084565b6040516105249190613363565b60405180910390f35b61054760048036038101906105429190612f8f565b6120a4565b005b6105516120c6565b60405161055e9190612f1c565b60405180910390f35b61056f6120ea565b60405161057c9190613363565b60405180910390f35b61059f600480360381019061059a919061353c565b6120f0565b6040516105ac9190613363565b60405180910390f35b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19161480610627575061062682612110565b5b9050919050565b5f60015f8381526020019081526020015f20600101549050919050565b6106548261062e565b61065d81612179565b610667838361218d565b50505050565b5f610676612276565b5f85116106b8576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016106af906135c4565b60405180910390fd5b338686426040516020016106cf9493929190613647565b6040516020818303038152906040528051906020012090506040518061014001604052803373ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020018773ffffffffffffffffffffffffffffffffffffffff16815260200186815260200184848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081526020018581526020015f60058111156107af576107ae6131db565b5b81526020014281526020015f81526020015f5f1b81525060025f8381526020019081526020015f205f820151815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506020820151816001015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506040820151816002015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506060820151816003015560808201518160040190816108c691906138be565b5060a0820151816005015560c0820151816006015f6101000a81548160ff021916908360058111156108fb576108fa6131db565b5b021790555060e08201518160070155610100820151816008015561012082015181600901559050503373ffffffffffffffffffffffffffffffffffffffff16817ff2b2765edf82fc07e26cea1350aecc843b2e6bbc95c7b52076d65c97c430115588888888886040516109729594939291906139c7565b60405180910390a395945050505050565b61098b6122b7565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16146109ef576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6109f982826122be565b505050565b5f60025f8381526020019081526020015f209050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161480610abc5750806001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16145b610afb576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610af290613a5d565b60405180910390fd5b60026005811115610b0f57610b0e6131db565b5b816006015f9054906101000a900460ff166005811115610b3257610b316131db565b5b14610b72576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610b6990613aeb565b60405180910390fd5b6005816006015f6101000a81548160ff02191690836005811115610b9957610b986131db565b5b02179055503373ffffffffffffffffffffffffffffffffffffffff16827f9809df6c9d63642a6f1e33a698d415ba2d882a4071894da748ab5482975c8e6360405160405180910390a35050565b610bee6123a8565b5f60025f8381526020019081526020015f2090503373ffffffffffffffffffffffffffffffffffffffff16816001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614610c93576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610c8a90613b53565b60405180910390fd5b60016005811115610ca757610ca66131db565b5b816006015f9054906101000a900460ff166005811115610cca57610cc96131db565b5b14610d0a576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610d0190613bbb565b60405180910390fd5b6004548160080154610d1c9190613c06565b4211610d5d576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610d5490613c83565b60405180910390fd5b6004816006015f6101000a81548160ff02191690836005811115610d8457610d836131db565b5b0217905550806002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663a9059cbb826001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600301546040518363ffffffff1660e01b8152600401610e0e929190613ca1565b6020604051808303815f875af1158015610e2a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e4e9190613cdc565b610e8d576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e8490613d51565b60405180910390fd5b610ee2816001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16826002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600301546123ca565b817f386c4fc2fe51e17dd0f7b5bb6d38fca3a652d4e6fbcfa1a93283034acdc5843a604051610f1090613db9565b60405180910390a250610f21612539565b50565b5f5f1b610f3081612179565b610f38612553565b50565b5f5f1b610f4781612179565b610f4f6123a8565b5f60025f8581526020019081526020015f209050600580811115610f7657610f756131db565b5b816006015f9054906101000a900460ff166005811115610f9957610f986131db565b5b14610fd9576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610fd090613e21565b60405180910390fd5b82156111d9576003816006015f6101000a81548160ff02191690836005811115611006576110056131db565b5b0217905550806002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663a9059cbb825f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600301546040518363ffffffff1660e01b815260040161108f929190613ca1565b6020604051808303815f875af11580156110ab573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110cf9190613cdc565b61110e576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161110590613e89565b60405180910390fd5b611163816001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16826002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600301546123ca565b3373ffffffffffffffffffffffffffffffffffffffff16847f41f67fe3f67f1dce68769486352c3bc7c5c5acda8d3c0e6628f918c255e34a0b835f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040516111cc9190613ea7565b60405180910390a36113d1565b6004816006015f6101000a81548160ff02191690836005811115611200576111ff6131db565b5b0217905550806002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663a9059cbb826001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600301546040518363ffffffff1660e01b815260040161128a929190613ca1565b6020604051808303815f875af11580156112a6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112ca9190613cdc565b611309576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161130090613f0a565b60405180910390fd5b61135e816001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16826002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600301546123ca565b3373ffffffffffffffffffffffffffffffffffffffff16847f41f67fe3f67f1dce68769486352c3bc7c5c5acda8d3c0e6628f918c255e34a0b836001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040516113c89190613ea7565b60405180910390a35b506113da612539565b505050565b6113e7612d56565b60025f8381526020019081526020015f20604051806101400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020016003820154815260200160048201805461151c906136ee565b80601f0160208091040260200160405190810160405280929190818152602001828054611548906136ee565b80156115935780601f1061156a57610100808354040283529160200191611593565b820191905f5260205f20905b81548152906001019060200180831161157657829003601f168201915b5050505050815260200160058201548152602001600682015f9054906101000a900460ff1660058111156115ca576115c96131db565b5b60058111156115dc576115db6131db565b5b815260200160078201548152602001600882015481526020016009820154815250509050919050565b5f5f5f9054906101000a900460ff16905090565b60045481565b5f60025f8381526020019081526020015f2090503373ffffffffffffffffffffffffffffffffffffffff16815f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16146116c3576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016116ba90613f72565b60405180910390fd5b5f60058111156116d6576116d56131db565b5b816006015f9054906101000a900460ff1660058111156116f9576116f86131db565b5b14611739576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161173090613fda565b60405180910390fd5b600354816007015461174b9190613c06565b421161178c576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161178390614042565b60405180910390fd5b6004816006015f6101000a81548160ff021916908360058111156117b3576117b26131db565b5b0217905550817f386c4fc2fe51e17dd0f7b5bb6d38fca3a652d4e6fbcfa1a93283034acdc5843a6040516117e6906140aa565b60405180910390a25050565b5f5f1b6117fe81612179565b6118066125b3565b50565b5f5f1b61181581612179565b816003819055505050565b5f5f1b61182c81612179565b8160055f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20819055508273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff167f64d19fedce9351d7c194985163e6b400555c186ba785881239156413948a8bca846040516119069190613363565b60405180910390a350505050565b5f5f1b61192081612179565b816004819055505050565b5f60025f8481526020019081526020015f2090503373ffffffffffffffffffffffffffffffffffffffff16815f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16146119cf576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016119c690613f72565b60405180910390fd5b600160058111156119e3576119e26131db565b5b816006015f9054906101000a900460ff166005811115611a0657611a056131db565b5b14611a46576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611a3d90614112565b60405180910390fd5b6004548160080154611a589190613c06565b421115611a9a576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611a919061417a565b60405180910390fd5b6002816006015f6101000a81548160ff02191690836005811115611ac157611ac06131db565b5b0217905550818160090181905550827fd8e25e58ea6ac5e1dff31cc8c341922c4d1ccd427c84e0bf339c510b4b56106283604051611aff9190612f1c565b60405180910390a2505050565b5f60015f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b7fe2b7fb3b832174769106daebcfd6d1970523240dda11281102db9363b83b0dc481565b6002602052805f5260405f205f91509050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690806001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690806002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690806003015490806004018054611c28906136ee565b80601f0160208091040260200160405190810160405280929190818152602001828054611c54906136ee565b8015611c9f5780601f10611c7657610100808354040283529160200191611c9f565b820191905f5260205f20905b815481529060010190602001808311611c8257829003601f168201915b505050505090806005015490806006015f9054906101000a900460ff1690806007015490806008015490806009015490508a565b611cdb6123a8565b7fb0296ea8dd3227371927b1c1cea2b12ea394743ddf2f32f58024ce26f83a24a6611d0581612179565b611d0f8233612614565b50611d18612539565b50565b611d236123a8565b7fe2b7fb3b832174769106daebcfd6d1970523240dda11281102db9363b83b0dc4611d4d81612179565b611d578383612614565b50611d60612539565b5050565b5f5f1b81565b611d726123a8565b5f60025f8381526020019081526020015f2090503373ffffffffffffffffffffffffffffffffffffffff16816001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614611e17576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611e0e90613b53565b60405180910390fd5b60026005811115611e2b57611e2a6131db565b5b816006015f9054906101000a900460ff166005811115611e4e57611e4d6131db565b5b14611e8e576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611e85906141e2565b60405180910390fd5b6003816006015f6101000a81548160ff02191690836005811115611eb557611eb46131db565b5b0217905550806002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663a9059cbb825f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600301546040518363ffffffff1660e01b8152600401611f3e929190613ca1565b6020604051808303815f875af1158015611f5a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f7e9190613cdc565b611fbd576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611fb49061424a565b60405180910390fd5b612012816001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16826002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600301546123ca565b805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16827fa13a9d77a6fbd690b8000b14cc8f4ba7602b329937638e708937fd9faa6069cd60405160405180910390a350612081612539565b50565b6006602052815f5260405f20602052805f5260405f205f91509150505481565b6120ad8261062e565b6120b681612179565b6120c083836122be565b50505050565b7fb0296ea8dd3227371927b1c1cea2b12ea394743ddf2f32f58024ce26f83a24a681565b60035481565b6005602052815f5260405f20602052805f5260405f205f91509150505481565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b61218a816121856122b7565b612971565b50565b5f6121988383611b0c565b61226c576001805f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506122096122b7565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a460019050612270565b5f90505b92915050565b61227e611605565b156122b5576040517fd93c066500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f33905090565b5f6122c98383611b0c565b1561239e575f60015f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555061233b6122b7565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a4600190506123a2565b5f90505b92915050565b6123b06129c2565b60026123c26123bd612a03565b612a2c565b5f0181905550565b5f8160065f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205461244f9190614268565b90508060065f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20819055508273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff167f08118319557e9998588247d46837aea5c77de48967936343b6c923fb1f7813e38360405161252b9190613363565b60405180910390a350505050565b600161254b612546612a03565b612a2c565b5f0181905550565b61255b612a35565b5f5f5f6101000a81548160ff0219169083151502179055507f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa61259c6122b7565b6040516125a99190613ea7565b60405180910390a1565b6125bb612276565b60015f5f6101000a81548160ff0219169083151502179055507f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586125fd6122b7565b60405161260a9190613ea7565b60405180910390a1565b5f60025f8481526020019081526020015f2090505f600581111561263b5761263a6131db565b5b816006015f9054906101000a900460ff16600581111561265e5761265d6131db565b5b1461269e576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161269590613fda565b60405180910390fd5b60035481600701546126b09190613c06565b4211156126f2576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016126e9906142e5565b60405180910390fd5b61271c7fb0296ea8dd3227371927b1c1cea2b12ea394743ddf2f32f58024ce26f83a24a683611b0c565b61275b576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016127529061434d565b60405180910390fd5b61278d82826002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff168360030154612a75565b81816001015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506001816006015f6101000a81548160ff021916908360058111156127f6576127f56131db565b5b0217905550428160080181905550806002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166323b872dd833084600301546040518463ffffffff1660e01b81526004016128689392919061436b565b6020604051808303815f875af1158015612884573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906128a89190613cdc565b6128e7576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016128de906143ea565b60405180910390fd5b61291982826002015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff168360030154612bcb565b8173ffffffffffffffffffffffffffffffffffffffff16837f15144217405064631de951752111334f6d9db4be6cfff45346f7068ea857fcad83600301546040516129649190613363565b60405180910390a3505050565b61297b8282611b0c565b6129be5780826040517fe2517d3f0000000000000000000000000000000000000000000000000000000081526004016129b5929190614408565b60405180910390fd5b5050565b6129ca612d3a565b15612a01576040517f3ee5aeb500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005f1b905090565b5f819050919050565b612a3d611605565b612a73576040517f8dfc202b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f60055f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205490505f8103612afe5750612bc6565b808260065f8773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f2054612b839190613c06565b1115612bc4576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612bbb90614479565b60405180910390fd5b505b505050565b5f8160065f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f2054612c509190613c06565b90508060065f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20819055508273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff167f08118319557e9998588247d46837aea5c77de48967936343b6c923fb1f7813e383604051612d2c9190613363565b60405180910390a350505050565b5f6002612d4d612d48612a03565b612a2c565b5f015414905090565b6040518061014001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f8152602001606081526020015f81526020015f6005811115612ddc57612ddb6131db565b5b81526020015f81526020015f81526020015f81525090565b5f5ffd5b5f5ffd5b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b612e3081612dfc565b8114612e3a575f5ffd5b50565b5f81359050612e4b81612e27565b92915050565b5f60208284031215612e6657612e65612df4565b5b5f612e7384828501612e3d565b91505092915050565b5f8115159050919050565b612e9081612e7c565b82525050565b5f602082019050612ea95f830184612e87565b92915050565b5f819050919050565b612ec181612eaf565b8114612ecb575f5ffd5b50565b5f81359050612edc81612eb8565b92915050565b5f60208284031215612ef757612ef6612df4565b5b5f612f0484828501612ece565b91505092915050565b612f1681612eaf565b82525050565b5f602082019050612f2f5f830184612f0d565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f612f5e82612f35565b9050919050565b612f6e81612f54565b8114612f78575f5ffd5b50565b5f81359050612f8981612f65565b92915050565b5f5f60408385031215612fa557612fa4612df4565b5b5f612fb285828601612ece565b9250506020612fc385828601612f7b565b9150509250929050565b5f819050919050565b612fdf81612fcd565b8114612fe9575f5ffd5b50565b5f81359050612ffa81612fd6565b92915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f84011261302157613020613000565b5b8235905067ffffffffffffffff81111561303e5761303d613004565b5b60208301915083600182028301111561305a57613059613008565b5b9250929050565b5f5f5f5f5f6080868803121561307a57613079612df4565b5b5f61308788828901612f7b565b955050602061309888828901612fec565b94505060406130a988828901612fec565b935050606086013567ffffffffffffffff8111156130ca576130c9612df8565b5b6130d68882890161300c565b92509250509295509295909350565b6130ee81612e7c565b81146130f8575f5ffd5b50565b5f81359050613109816130e5565b92915050565b5f5f6040838503121561312557613124612df4565b5b5f61313285828601612ece565b9250506020613143858286016130fb565b9150509250929050565b61315681612f54565b82525050565b61316581612fcd565b82525050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f601f19601f8301169050919050565b5f6131ad8261316b565b6131b78185613175565b93506131c7818560208601613185565b6131d081613193565b840191505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60068110613219576132186131db565b5b50565b5f81905061322982613208565b919050565b5f6132388261321c565b9050919050565b6132488161322e565b82525050565b61325781612eaf565b82525050565b5f61014083015f8301516132735f86018261314d565b506020830151613286602086018261314d565b506040830151613299604086018261314d565b5060608301516132ac606086018261315c565b50608083015184820360808601526132c482826131a3565b91505060a08301516132d960a086018261315c565b5060c08301516132ec60c086018261323f565b5060e08301516132ff60e086018261315c565b5061010083015161331461010086018261315c565b5061012083015161332961012086018261324e565b508091505092915050565b5f6020820190508181035f83015261334c818461325d565b905092915050565b61335d81612fcd565b82525050565b5f6020820190506133765f830184613354565b92915050565b5f6020828403121561339157613390612df4565b5b5f61339e84828501612fec565b91505092915050565b5f5f5f606084860312156133be576133bd612df4565b5b5f6133cb86828701612f7b565b93505060206133dc86828701612f7b565b92505060406133ed86828701612fec565b9150509250925092565b5f5f6040838503121561340d5761340c612df4565b5b5f61341a85828601612ece565b925050602061342b85828601612ece565b9150509250929050565b61343e81612f54565b82525050565b5f82825260208201905092915050565b5f61345e8261316b565b6134688185613444565b9350613478818560208601613185565b61348181613193565b840191505092915050565b6134958161322e565b82525050565b5f610140820190506134af5f83018d613435565b6134bc602083018c613435565b6134c9604083018b613435565b6134d6606083018a613354565b81810360808301526134e88189613454565b90506134f760a0830188613354565b61350460c083018761348c565b61351160e0830186613354565b61351f610100830185613354565b61352d610120830184612f0d565b9b9a5050505050505050505050565b5f5f6040838503121561355257613551612df4565b5b5f61355f85828601612f7b565b925050602061357085828601612f7b565b9150509250929050565b7f546f6b656e20616d6f756e74206d757374206265203e203000000000000000005f82015250565b5f6135ae601883613444565b91506135b98261357a565b602082019050919050565b5f6020820190508181035f8301526135db816135a2565b9050919050565b5f8160601b9050919050565b5f6135f8826135e2565b9050919050565b5f613609826135ee565b9050919050565b61362161361c82612f54565b6135ff565b82525050565b5f819050919050565b61364161363c82612fcd565b613627565b82525050565b5f6136528287613610565b6014820191506136628286613610565b6014820191506136728285613630565b6020820191506136828284613630565b60208201915081905095945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061370557607f821691505b602082108103613718576137176136c1565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f6008830261377a7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8261373f565b613784868361373f565b95508019841693508086168417925050509392505050565b5f819050919050565b5f6137bf6137ba6137b584612fcd565b61379c565b612fcd565b9050919050565b5f819050919050565b6137d8836137a5565b6137ec6137e4826137c6565b84845461374b565b825550505050565b5f5f905090565b6138036137f4565b61380e8184846137cf565b505050565b5b81811015613831576138265f826137fb565b600181019050613814565b5050565b601f821115613876576138478161371e565b61385084613730565b8101602085101561385f578190505b61387361386b85613730565b830182613813565b50505b505050565b5f82821c905092915050565b5f6138965f198460080261387b565b1980831691505092915050565b5f6138ae8383613887565b9150826002028217905092915050565b6138c78261316b565b67ffffffffffffffff8111156138e0576138df613694565b5b6138ea82546136ee565b6138f5828285613835565b5f60209050601f831160018114613926575f8415613914578287015190505b61391e85826138a3565b865550613985565b601f1984166139348661371e565b5f5b8281101561395b57848901518255600182019150602085019450602081019050613936565b868310156139785784890151613974601f891682613887565b8355505b6001600288020188555050505b505050505050565b828183375f83830152505050565b5f6139a68385613444565b93506139b383858461398d565b6139bc83613193565b840190509392505050565b5f6080820190506139da5f830188613435565b6139e76020830187613354565b6139f46040830186613354565b8181036060830152613a0781848661399b565b90509695505050505050565b7f4e6f74206120706172747920746f20746865206f7264657200000000000000005f82015250565b5f613a47601883613444565b9150613a5282613a13565b602082019050919050565b5f6020820190508181035f830152613a7481613a3b565b9050919050565b7f43616e206f6e6c7920646973707574652061667465722066696174206973206d5f8201527f61726b65642073656e7400000000000000000000000000000000000000000000602082015250565b5f613ad5602a83613444565b9150613ae082613a7b565b604082019050919050565b5f6020820190508181035f830152613b0281613ac9565b9050919050565b7f4e6f74207468652061737369676e6564204c50000000000000000000000000005f82015250565b5f613b3d601383613444565b9150613b4882613b09565b602082019050919050565b5f6020820190508181035f830152613b6a81613b31565b9050919050565b7f4f726465722073746174757320696e76616c69640000000000000000000000005f82015250565b5f613ba5601483613444565b9150613bb082613b71565b602082019050919050565b5f6020820190508181035f830152613bd281613b99565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f613c1082612fcd565b9150613c1b83612fcd565b9250828201905080821115613c3357613c32613bd9565b5b92915050565b7f5061796d656e7420646561646c696e65206e6f742070617373656400000000005f82015250565b5f613c6d601b83613444565b9150613c7882613c39565b602082019050919050565b5f6020820190508181035f830152613c9a81613c61565b9050919050565b5f604082019050613cb45f830185613435565b613cc16020830184613354565b9392505050565b5f81519050613cd6816130e5565b92915050565b5f60208284031215613cf157613cf0612df4565b5b5f613cfe84828501613cc8565b91505092915050565b7f5245434c41494d5f5452414e534645525f4641494c45440000000000000000005f82015250565b5f613d3b601783613444565b9150613d4682613d07565b602082019050919050565b5f6020820190508181035f830152613d6881613d2f565b9050919050565b7f55736572207061796d656e742074696d6564206f7574000000000000000000005f82015250565b5f613da3601683613444565b9150613dae82613d6f565b602082019050919050565b5f6020820190508181035f830152613dd081613d97565b9050919050565b7f4f72646572206e6f7420696e20646973707574650000000000000000000000005f82015250565b5f613e0b601483613444565b9150613e1682613dd7565b602082019050919050565b5f6020820190508181035f830152613e3881613dff565b9050919050565b7f444953505554455f42555945525f5452414e534645525f4641494c45440000005f82015250565b5f613e73601d83613444565b9150613e7e82613e3f565b602082019050919050565b5f6020820190508181035f830152613ea081613e67565b9050919050565b5f602082019050613eba5f830184613435565b92915050565b7f444953505554455f4c505f5452414e534645525f4641494c45440000000000005f82015250565b5f613ef4601a83613444565b9150613eff82613ec0565b602082019050919050565b5f6020820190508181035f830152613f2181613ee8565b9050919050565b7f4e6f7420746865206275796572000000000000000000000000000000000000005f82015250565b5f613f5c600d83613444565b9150613f6782613f28565b602082019050919050565b5f6020820190508181035f830152613f8981613f50565b9050919050565b7f4f72646572206e6f742070656e64696e670000000000000000000000000000005f82015250565b5f613fc4601183613444565b9150613fcf82613f90565b602082019050919050565b5f6020820190508181035f830152613ff181613fb8565b9050919050565b7f4c6f636b20646561646c696e65206e6f742070617373656400000000000000005f82015250565b5f61402c601883613444565b915061403782613ff8565b602082019050919050565b5f6020820190508181035f83015261405981614020565b9050919050565b7f4c50206661696c656420746f206c6f636b2066756e64730000000000000000005f82015250565b5f614094601783613444565b915061409f82614060565b602082019050919050565b5f6020820190508181035f8301526140c181614088565b9050919050565b7f46756e6473206e6f74206c6f636b6564000000000000000000000000000000005f82015250565b5f6140fc601083613444565b9150614107826140c8565b602082019050919050565b5f6020820190508181035f830152614129816140f0565b9050919050565b7f5061796d656e7420646561646c696e65207061737365640000000000000000005f82015250565b5f614164601783613444565b915061416f82614130565b602082019050919050565b5f6020820190508181035f83015261419181614158565b9050919050565b7f46696174206e6f74206d61726b65642061732073656e740000000000000000005f82015250565b5f6141cc601783613444565b91506141d782614198565b602082019050919050565b5f6020820190508181035f8301526141f9816141c0565b9050919050565b7f52454c454153455f5452414e534645525f4641494c45440000000000000000005f82015250565b5f614234601783613444565b915061423f82614200565b602082019050919050565b5f6020820190508181035f83015261426181614228565b9050919050565b5f61427282612fcd565b915061427d83612fcd565b925082820390508181111561429557614294613bd9565b5b92915050565b7f4c6f636b20646561646c696e65207061737365640000000000000000000000005f82015250565b5f6142cf601483613444565b91506142da8261429b565b602082019050919050565b5f6020820190508181035f8301526142fc816142c3565b9050919050565b7f4c505f524f4c455f5245515549524544000000000000000000000000000000005f82015250565b5f614337601083613444565b915061434282614303565b602082019050919050565b5f6020820190508181035f8301526143648161432b565b9050919050565b5f60608201905061437e5f830186613435565b61438b6020830185613435565b6143986040830184613354565b949350505050565b7f4c4f434b5f5452414e534645525f4641494c45440000000000000000000000005f82015250565b5f6143d4601483613444565b91506143df826143a0565b602082019050919050565b5f6020820190508181035f830152614401816143c8565b9050919050565b5f60408201905061441b5f830185613435565b6144286020830184612f0d565b9392505050565b7f4c505f4341505f455843454544454400000000000000000000000000000000005f82015250565b5f614463600f83613444565b915061446e8261442f565b602082019050919050565b5f6020820190508181035f83015261449081614457565b905091905056fea2646970667358221220113cb55e0710cab3041e0ab0b52aee69c9507363f0e9feec9ce225ddd872dacd64736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xD8W_5`\xE0\x1C\x80c\x89j/\xF4\x11a\x01\x02W\x80c\x9EI\xF7\xE4\x11a\0\xA0W\x80c\xD5Gt\x1F\x11a\0oW\x80c\xD5Gt\x1F\x14a\x05-W\x80c\xE1\xD3\x94P\x14a\x05IW\x80c\xFAM\x0C<\x14a\x05gW\x80c\xFCF\xD8\xB8\x14a\x05\x85Wa\x01\xD8V[\x80c\x9EI\xF7\xE4\x14a\x04\xA7W\x80c\xA2\x17\xFD\xDF\x14a\x04\xC3W\x80c\xC9.\xE0C\x14a\x04\xE1W\x80c\xCE\x9B\xC1^\x14a\x04\xFDWa\x01\xD8V[\x80c\x91\xD1HT\x11a\0\xDCW\x80c\x91\xD1HT\x14a\x04\x04W\x80c\x92m}\x7F\x14a\x044W\x80c\x9C?\x1E\x90\x14a\x04RW\x80c\x9C\xBE\xB6\xC1\x14a\x04\x8BWa\x01\xD8V[\x80c\x89j/\xF4\x14a\x03\xB0W\x80c\x8C\xFD\x91\x9E\x14a\x03\xCCW\x80c\x8F\xA4\xA6A\x14a\x03\xE8Wa\x01\xD8V[\x80c?K\xA8:\x11a\x01zW\x80cm\x92\x9F#\x11a\x01IW\x80cm\x92\x9F#\x14a\x03PW\x80ct\x89\xEC#\x14a\x03nW\x80c\x84V\xCBY\x14a\x03\x8AW\x80c\x84\xA5\xCES\x14a\x03\x94Wa\x01\xD8V[\x80c?K\xA8:\x14a\x02\xDCW\x80cC\xA0\xE3\xE6\x14a\x02\xE6W\x80cWxG*\x14a\x03\x02W\x80c\\\x97Z\xBB\x14a\x032Wa\x01\xD8V[\x80c0]\xF5o\x11a\x01\xB6W\x80c0]\xF5o\x14a\x02XW\x80c6V\x8A\xBE\x14a\x02\x88W\x80c>N7\x92\x14a\x02\xA4W\x80c>\x88\xE0\xB6\x14a\x02\xC0Wa\x01\xD8V[\x80c\x01\xFF\xC9\xA7\x14a\x01\xDCW\x80c$\x8A\x9C\xA3\x14a\x02\x0CW\x80c//\xF1]\x14a\x02<W[__\xFD[a\x01\xF6`\x04\x806\x03\x81\x01\x90a\x01\xF1\x91\x90a.QV[a\x05\xB5V[`@Qa\x02\x03\x91\x90a.\x96V[`@Q\x80\x91\x03\x90\xF3[a\x02&`\x04\x806\x03\x81\x01\x90a\x02!\x91\x90a.\xE2V[a\x06.V[`@Qa\x023\x91\x90a/\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x02V`\x04\x806\x03\x81\x01\x90a\x02Q\x91\x90a/\x8FV[a\x06KV[\0[a\x02r`\x04\x806\x03\x81\x01\x90a\x02m\x91\x90a0aV[a\x06mV[`@Qa\x02\x7F\x91\x90a/\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x02\xA2`\x04\x806\x03\x81\x01\x90a\x02\x9D\x91\x90a/\x8FV[a\t\x83V[\0[a\x02\xBE`\x04\x806\x03\x81\x01\x90a\x02\xB9\x91\x90a.\xE2V[a\t\xFEV[\0[a\x02\xDA`\x04\x806\x03\x81\x01\x90a\x02\xD5\x91\x90a.\xE2V[a\x0B\xE6V[\0[a\x02\xE4a\x0F$V[\0[a\x03\0`\x04\x806\x03\x81\x01\x90a\x02\xFB\x91\x90a1\x0FV[a\x0F;V[\0[a\x03\x1C`\x04\x806\x03\x81\x01\x90a\x03\x17\x91\x90a.\xE2V[a\x13\xDFV[`@Qa\x03)\x91\x90a34V[`@Q\x80\x91\x03\x90\xF3[a\x03:a\x16\x05V[`@Qa\x03G\x91\x90a.\x96V[`@Q\x80\x91\x03\x90\xF3[a\x03Xa\x16\x19V[`@Qa\x03e\x91\x90a3cV[`@Q\x80\x91\x03\x90\xF3[a\x03\x88`\x04\x806\x03\x81\x01\x90a\x03\x83\x91\x90a.\xE2V[a\x16\x1FV[\0[a\x03\x92a\x17\xF2V[\0[a\x03\xAE`\x04\x806\x03\x81\x01\x90a\x03\xA9\x91\x90a3|V[a\x18\tV[\0[a\x03\xCA`\x04\x806\x03\x81\x01\x90a\x03\xC5\x91\x90a3\xA7V[a\x18 V[\0[a\x03\xE6`\x04\x806\x03\x81\x01\x90a\x03\xE1\x91\x90a3|V[a\x19\x14V[\0[a\x04\x02`\x04\x806\x03\x81\x01\x90a\x03\xFD\x91\x90a3\xF7V[a\x19+V[\0[a\x04\x1E`\x04\x806\x03\x81\x01\x90a\x04\x19\x91\x90a/\x8FV[a\x1B\x0CV[`@Qa\x04+\x91\x90a.\x96V[`@Q\x80\x91\x03\x90\xF3[a\x04<a\x1BpV[`@Qa\x04I\x91\x90a/\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x04l`\x04\x806\x03\x81\x01\x90a\x04g\x91\x90a.\xE2V[a\x1B\x94V[`@Qa\x04\x82\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a4\x9BV[`@Q\x80\x91\x03\x90\xF3[a\x04\xA5`\x04\x806\x03\x81\x01\x90a\x04\xA0\x91\x90a.\xE2V[a\x1C\xD3V[\0[a\x04\xC1`\x04\x806\x03\x81\x01\x90a\x04\xBC\x91\x90a/\x8FV[a\x1D\x1BV[\0[a\x04\xCBa\x1DdV[`@Qa\x04\xD8\x91\x90a/\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x04\xFB`\x04\x806\x03\x81\x01\x90a\x04\xF6\x91\x90a.\xE2V[a\x1DjV[\0[a\x05\x17`\x04\x806\x03\x81\x01\x90a\x05\x12\x91\x90a5<V[a \x84V[`@Qa\x05$\x91\x90a3cV[`@Q\x80\x91\x03\x90\xF3[a\x05G`\x04\x806\x03\x81\x01\x90a\x05B\x91\x90a/\x8FV[a \xA4V[\0[a\x05Qa \xC6V[`@Qa\x05^\x91\x90a/\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x05oa \xEAV[`@Qa\x05|\x91\x90a3cV[`@Q\x80\x91\x03\x90\xF3[a\x05\x9F`\x04\x806\x03\x81\x01\x90a\x05\x9A\x91\x90a5<V[a \xF0V[`@Qa\x05\xAC\x91\x90a3cV[`@Q\x80\x91\x03\x90\xF3[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06'WPa\x06&\x82a!\x10V[[\x90P\x91\x90PV[_`\x01_\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[a\x06T\x82a\x06.V[a\x06]\x81a!yV[a\x06g\x83\x83a!\x8DV[PPPPV[_a\x06va\"vV[_\x85\x11a\x06\xB8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xAF\x90a5\xC4V[`@Q\x80\x91\x03\x90\xFD[3\x86\x86B`@Q` \x01a\x06\xCF\x94\x93\x92\x91\x90a6GV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`@Q\x80a\x01@\x01`@R\x803s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x85\x81R` \x01_`\x05\x81\x11\x15a\x07\xAFWa\x07\xAEa1\xDBV[[\x81R` \x01B\x81R` \x01_\x81R` \x01__\x1B\x81RP`\x02_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x02\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01\x90\x81a\x08\xC6\x91\x90a8\xBEV[P`\xA0\x82\x01Q\x81`\x05\x01U`\xC0\x82\x01Q\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x08\xFBWa\x08\xFAa1\xDBV[[\x02\x17\x90UP`\xE0\x82\x01Q\x81`\x07\x01Ua\x01\0\x82\x01Q\x81`\x08\x01Ua\x01 \x82\x01Q\x81`\t\x01U\x90PP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x7F\xF2\xB2v^\xDF\x82\xFC\x07\xE2l\xEA\x13P\xAE\xCC\x84;.k\xBC\x95\xC7\xB5 v\xD6\\\x97\xC40\x11U\x88\x88\x88\x88\x88`@Qa\tr\x95\x94\x93\x92\x91\x90a9\xC7V[`@Q\x80\x91\x03\x90\xA3\x95\x94PPPPPV[a\t\x8Ba\"\xB7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\t\xEFW`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xF9\x82\x82a\"\xBEV[PPPV[_`\x02_\x83\x81R` \x01\x90\x81R` \x01_ \x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\n\xBCWP\x80`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\n\xFBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\xF2\x90a:]V[`@Q\x80\x91\x03\x90\xFD[`\x02`\x05\x81\x11\x15a\x0B\x0FWa\x0B\x0Ea1\xDBV[[\x81`\x06\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x0B2Wa\x0B1a1\xDBV[[\x14a\x0BrW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Bi\x90a:\xEBV[`@Q\x80\x91\x03\x90\xFD[`\x05\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x0B\x99Wa\x0B\x98a1\xDBV[[\x02\x17\x90UP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x7F\x98\t\xDFl\x9Dcd*o\x1E3\xA6\x98\xD4\x15\xBA-\x88*@q\x89M\xA7H\xABT\x82\x97\\\x8Ec`@Q`@Q\x80\x91\x03\x90\xA3PPV[a\x0B\xEEa#\xA8V[_`\x02_\x83\x81R` \x01\x90\x81R` \x01_ \x90P3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C\x93W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x8A\x90a;SV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x05\x81\x11\x15a\x0C\xA7Wa\x0C\xA6a1\xDBV[[\x81`\x06\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x0C\xCAWa\x0C\xC9a1\xDBV[[\x14a\r\nW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\x01\x90a;\xBBV[`@Q\x80\x91\x03\x90\xFD[`\x04T\x81`\x08\x01Ta\r\x1C\x91\x90a<\x06V[B\x11a\r]W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\rT\x90a<\x83V[`@Q\x80\x91\x03\x90\xFD[`\x04\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\r\x84Wa\r\x83a1\xDBV[[\x02\x17\x90UP\x80`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x82`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01T`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\x0E\x92\x91\x90a<\xA1V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E*W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EN\x91\x90a<\xDCV[a\x0E\x8DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\x84\x90a=QV[`@Q\x80\x91\x03\x90\xFD[a\x0E\xE2\x81`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01Ta#\xCAV[\x81\x7F8lO\xC2\xFEQ\xE1}\xD0\xF7\xB5\xBBm8\xFC\xA3\xA6R\xD4\xE6\xFB\xCF\xA1\xA92\x83\x03J\xCD\xC5\x84:`@Qa\x0F\x10\x90a=\xB9V[`@Q\x80\x91\x03\x90\xA2Pa\x0F!a%9V[PV[__\x1Ba\x0F0\x81a!yV[a\x0F8a%SV[PV[__\x1Ba\x0FG\x81a!yV[a\x0FOa#\xA8V[_`\x02_\x85\x81R` \x01\x90\x81R` \x01_ \x90P`\x05\x80\x81\x11\x15a\x0FvWa\x0Fua1\xDBV[[\x81`\x06\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x0F\x99Wa\x0F\x98a1\xDBV[[\x14a\x0F\xD9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xD0\x90a>!V[`@Q\x80\x91\x03\x90\xFD[\x82\x15a\x11\xD9W`\x03\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x10\x06Wa\x10\x05a1\xDBV[[\x02\x17\x90UP\x80`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x82_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01T`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\x8F\x92\x91\x90a<\xA1V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10\xABW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xCF\x91\x90a<\xDCV[a\x11\x0EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\x05\x90a>\x89V[`@Q\x80\x91\x03\x90\xFD[a\x11c\x81`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01Ta#\xCAV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7FA\xF6\x7F\xE3\xF6\x7F\x1D\xCEhv\x94\x865,;\xC7\xC5\xC5\xAC\xDA\x8D<\x0Ef(\xF9\x18\xC2U\xE3J\x0B\x83_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Qa\x11\xCC\x91\x90a>\xA7V[`@Q\x80\x91\x03\x90\xA3a\x13\xD1V[`\x04\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x12\0Wa\x11\xFFa1\xDBV[[\x02\x17\x90UP\x80`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x82`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01T`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\x8A\x92\x91\x90a<\xA1V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12\xA6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xCA\x91\x90a<\xDCV[a\x13\tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\0\x90a?\nV[`@Q\x80\x91\x03\x90\xFD[a\x13^\x81`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01Ta#\xCAV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7FA\xF6\x7F\xE3\xF6\x7F\x1D\xCEhv\x94\x865,;\xC7\xC5\xC5\xAC\xDA\x8D<\x0Ef(\xF9\x18\xC2U\xE3J\x0B\x83`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Qa\x13\xC8\x91\x90a>\xA7V[`@Q\x80\x91\x03\x90\xA3[Pa\x13\xDAa%9V[PPPV[a\x13\xE7a-VV[`\x02_\x83\x81R` \x01\x90\x81R` \x01_ `@Q\x80a\x01@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01\x80Ta\x15\x1C\x90a6\xEEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15H\x90a6\xEEV[\x80\x15a\x15\x93W\x80`\x1F\x10a\x15jWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\x93V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15vW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x05\x82\x01T\x81R` \x01`\x06\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x15\xCAWa\x15\xC9a1\xDBV[[`\x05\x81\x11\x15a\x15\xDCWa\x15\xDBa1\xDBV[[\x81R` \x01`\x07\x82\x01T\x81R` \x01`\x08\x82\x01T\x81R` \x01`\t\x82\x01T\x81RPP\x90P\x91\x90PV[___\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[`\x04T\x81V[_`\x02_\x83\x81R` \x01\x90\x81R` \x01_ \x90P3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x16\xC3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16\xBA\x90a?rV[`@Q\x80\x91\x03\x90\xFD[_`\x05\x81\x11\x15a\x16\xD6Wa\x16\xD5a1\xDBV[[\x81`\x06\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x16\xF9Wa\x16\xF8a1\xDBV[[\x14a\x179W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x170\x90a?\xDAV[`@Q\x80\x91\x03\x90\xFD[`\x03T\x81`\x07\x01Ta\x17K\x91\x90a<\x06V[B\x11a\x17\x8CW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17\x83\x90a@BV[`@Q\x80\x91\x03\x90\xFD[`\x04\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x17\xB3Wa\x17\xB2a1\xDBV[[\x02\x17\x90UP\x81\x7F8lO\xC2\xFEQ\xE1}\xD0\xF7\xB5\xBBm8\xFC\xA3\xA6R\xD4\xE6\xFB\xCF\xA1\xA92\x83\x03J\xCD\xC5\x84:`@Qa\x17\xE6\x90a@\xAAV[`@Q\x80\x91\x03\x90\xA2PPV[__\x1Ba\x17\xFE\x81a!yV[a\x18\x06a%\xB3V[PV[__\x1Ba\x18\x15\x81a!yV[\x81`\x03\x81\x90UPPPV[__\x1Ba\x18,\x81a!yV[\x81`\x05_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fd\xD1\x9F\xED\xCE\x93Q\xD7\xC1\x94\x98Qc\xE6\xB4\0U\\\x18k\xA7\x85\x88\x129\x15d\x13\x94\x8A\x8B\xCA\x84`@Qa\x19\x06\x91\x90a3cV[`@Q\x80\x91\x03\x90\xA3PPPPV[__\x1Ba\x19 \x81a!yV[\x81`\x04\x81\x90UPPPV[_`\x02_\x84\x81R` \x01\x90\x81R` \x01_ \x90P3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x19\xCFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\xC6\x90a?rV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x05\x81\x11\x15a\x19\xE3Wa\x19\xE2a1\xDBV[[\x81`\x06\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x1A\x06Wa\x1A\x05a1\xDBV[[\x14a\x1AFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A=\x90aA\x12V[`@Q\x80\x91\x03\x90\xFD[`\x04T\x81`\x08\x01Ta\x1AX\x91\x90a<\x06V[B\x11\x15a\x1A\x9AW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\x91\x90aAzV[`@Q\x80\x91\x03\x90\xFD[`\x02\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x1A\xC1Wa\x1A\xC0a1\xDBV[[\x02\x17\x90UP\x81\x81`\t\x01\x81\x90UP\x82\x7F\xD8\xE2^X\xEAj\xC5\xE1\xDF\xF3\x1C\xC8\xC3A\x92,M\x1C\xCDB|\x84\xE0\xBF3\x9CQ\x0BKV\x10b\x83`@Qa\x1A\xFF\x91\x90a/\x1CV[`@Q\x80\x91\x03\x90\xA2PPPV[_`\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[\x7F\xE2\xB7\xFB;\x83!tv\x91\x06\xDA\xEB\xCF\xD6\xD1\x97\x05#$\r\xDA\x11(\x11\x02\xDB\x93c\xB8;\r\xC4\x81V[`\x02` R\x80_R`@_ _\x91P\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80`\x03\x01T\x90\x80`\x04\x01\x80Ta\x1C(\x90a6\xEEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1CT\x90a6\xEEV[\x80\x15a\x1C\x9FW\x80`\x1F\x10a\x1CvWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C\x9FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\x82W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x05\x01T\x90\x80`\x06\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90\x80`\x07\x01T\x90\x80`\x08\x01T\x90\x80`\t\x01T\x90P\x8AV[a\x1C\xDBa#\xA8V[\x7F\xB0)n\xA8\xDD2'7\x19'\xB1\xC1\xCE\xA2\xB1.\xA3\x94t=\xDF/2\xF5\x80$\xCE&\xF8:$\xA6a\x1D\x05\x81a!yV[a\x1D\x0F\x823a&\x14V[Pa\x1D\x18a%9V[PV[a\x1D#a#\xA8V[\x7F\xE2\xB7\xFB;\x83!tv\x91\x06\xDA\xEB\xCF\xD6\xD1\x97\x05#$\r\xDA\x11(\x11\x02\xDB\x93c\xB8;\r\xC4a\x1DM\x81a!yV[a\x1DW\x83\x83a&\x14V[Pa\x1D`a%9V[PPV[__\x1B\x81V[a\x1Dra#\xA8V[_`\x02_\x83\x81R` \x01\x90\x81R` \x01_ \x90P3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1E\x17W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E\x0E\x90a;SV[`@Q\x80\x91\x03\x90\xFD[`\x02`\x05\x81\x11\x15a\x1E+Wa\x1E*a1\xDBV[[\x81`\x06\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x1ENWa\x1EMa1\xDBV[[\x14a\x1E\x8EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E\x85\x90aA\xE2V[`@Q\x80\x91\x03\x90\xFD[`\x03\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x1E\xB5Wa\x1E\xB4a1\xDBV[[\x02\x17\x90UP\x80`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x82_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01T`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F>\x92\x91\x90a<\xA1V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1FZW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F~\x91\x90a<\xDCV[a\x1F\xBDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1F\xB4\x90aBJV[`@Q\x80\x91\x03\x90\xFD[a \x12\x81`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01Ta#\xCAV[\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x7F\xA1:\x9Dw\xA6\xFB\xD6\x90\xB8\0\x0B\x14\xCC\x8FK\xA7`+2\x997c\x8Ep\x897\xFD\x9F\xAA`i\xCD`@Q`@Q\x80\x91\x03\x90\xA3Pa \x81a%9V[PV[`\x06` R\x81_R`@_ ` R\x80_R`@_ _\x91P\x91PPT\x81V[a \xAD\x82a\x06.V[a \xB6\x81a!yV[a \xC0\x83\x83a\"\xBEV[PPPPV[\x7F\xB0)n\xA8\xDD2'7\x19'\xB1\xC1\xCE\xA2\xB1.\xA3\x94t=\xDF/2\xF5\x80$\xCE&\xF8:$\xA6\x81V[`\x03T\x81V[`\x05` R\x81_R`@_ ` R\x80_R`@_ _\x91P\x91PPT\x81V[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[a!\x8A\x81a!\x85a\"\xB7V[a)qV[PV[_a!\x98\x83\x83a\x1B\x0CV[a\"lW`\x01\x80_\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\"\ta\"\xB7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\"pV[_\x90P[\x92\x91PPV[a\"~a\x16\x05V[\x15a\"\xB5W`@Q\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_3\x90P\x90V[_a\"\xC9\x83\x83a\x1B\x0CV[\x15a#\x9EW_`\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa#;a\"\xB7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa#\xA2V[_\x90P[\x92\x91PPV[a#\xB0a)\xC2V[`\x02a#\xC2a#\xBDa*\x03V[a*,V[_\x01\x81\x90UPV[_\x81`\x06_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ Ta$O\x91\x90aBhV[\x90P\x80`\x06_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x08\x11\x83\x19U~\x99\x98X\x82G\xD4h7\xAE\xA5\xC7}\xE4\x89g\x93cC\xB6\xC9#\xFB\x1Fx\x13\xE3\x83`@Qa%+\x91\x90a3cV[`@Q\x80\x91\x03\x90\xA3PPPPV[`\x01a%Ka%Fa*\x03V[a*,V[_\x01\x81\x90UPV[a%[a*5V[___a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAAa%\x9Ca\"\xB7V[`@Qa%\xA9\x91\x90a>\xA7V[`@Q\x80\x91\x03\x90\xA1V[a%\xBBa\"vV[`\x01__a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa%\xFDa\"\xB7V[`@Qa&\n\x91\x90a>\xA7V[`@Q\x80\x91\x03\x90\xA1V[_`\x02_\x84\x81R` \x01\x90\x81R` \x01_ \x90P_`\x05\x81\x11\x15a&;Wa&:a1\xDBV[[\x81`\x06\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a&^Wa&]a1\xDBV[[\x14a&\x9EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\x95\x90a?\xDAV[`@Q\x80\x91\x03\x90\xFD[`\x03T\x81`\x07\x01Ta&\xB0\x91\x90a<\x06V[B\x11\x15a&\xF2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\xE9\x90aB\xE5V[`@Q\x80\x91\x03\x90\xFD[a'\x1C\x7F\xB0)n\xA8\xDD2'7\x19'\xB1\xC1\xCE\xA2\xB1.\xA3\x94t=\xDF/2\xF5\x80$\xCE&\xF8:$\xA6\x83a\x1B\x0CV[a'[W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a'R\x90aCMV[`@Q\x80\x91\x03\x90\xFD[a'\x8D\x82\x82`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01Ta*uV[\x81\x81`\x01\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81`\x06\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a'\xF6Wa'\xF5a1\xDBV[[\x02\x17\x90UPB\x81`\x08\x01\x81\x90UP\x80`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD\x830\x84`\x03\x01T`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a(h\x93\x92\x91\x90aCkV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a(\x84W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xA8\x91\x90a<\xDCV[a(\xE7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a(\xDE\x90aC\xEAV[`@Q\x80\x91\x03\x90\xFD[a)\x19\x82\x82`\x02\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x03\x01Ta+\xCBV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\x15\x14B\x17@Pdc\x1D\xE9Qu!\x113Om\x9D\xB4\xBEl\xFF\xF4SF\xF7\x06\x8E\xA8W\xFC\xAD\x83`\x03\x01T`@Qa)d\x91\x90a3cV[`@Q\x80\x91\x03\x90\xA3PPPV[a){\x82\x82a\x1B\x0CV[a)\xBEW\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)\xB5\x92\x91\x90aD\x08V[`@Q\x80\x91\x03\x90\xFD[PPV[a)\xCAa-:V[\x15a*\x01W`@Q\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0_\x1B\x90P\x90V[_\x81\x90P\x91\x90PV[a*=a\x16\x05V[a*sW`@Q\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_`\x05_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x03a*\xFEWPa+\xC6V[\x80\x82`\x06_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ Ta+\x83\x91\x90a<\x06V[\x11\x15a+\xC4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a+\xBB\x90aDyV[`@Q\x80\x91\x03\x90\xFD[P[PPPV[_\x81`\x06_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ Ta,P\x91\x90a<\x06V[\x90P\x80`\x06_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x08\x11\x83\x19U~\x99\x98X\x82G\xD4h7\xAE\xA5\xC7}\xE4\x89g\x93cC\xB6\xC9#\xFB\x1Fx\x13\xE3\x83`@Qa-,\x91\x90a3cV[`@Q\x80\x91\x03\x90\xA3PPPPV[_`\x02a-Ma-Ha*\x03V[a*,V[_\x01T\x14\x90P\x90V[`@Q\x80a\x01@\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81R` \x01``\x81R` \x01_\x81R` \x01_`\x05\x81\x11\x15a-\xDCWa-\xDBa1\xDBV[[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[__\xFD[__\xFD[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a.0\x81a-\xFCV[\x81\x14a.:W__\xFD[PV[_\x815\x90Pa.K\x81a.'V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a.fWa.ea-\xF4V[[_a.s\x84\x82\x85\x01a.=V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a.\x90\x81a.|V[\x82RPPV[_` \x82\x01\x90Pa.\xA9_\x83\x01\x84a.\x87V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a.\xC1\x81a.\xAFV[\x81\x14a.\xCBW__\xFD[PV[_\x815\x90Pa.\xDC\x81a.\xB8V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a.\xF7Wa.\xF6a-\xF4V[[_a/\x04\x84\x82\x85\x01a.\xCEV[\x91PP\x92\x91PPV[a/\x16\x81a.\xAFV[\x82RPPV[_` \x82\x01\x90Pa//_\x83\x01\x84a/\rV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a/^\x82a/5V[\x90P\x91\x90PV[a/n\x81a/TV[\x81\x14a/xW__\xFD[PV[_\x815\x90Pa/\x89\x81a/eV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a/\xA5Wa/\xA4a-\xF4V[[_a/\xB2\x85\x82\x86\x01a.\xCEV[\x92PP` a/\xC3\x85\x82\x86\x01a/{V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a/\xDF\x81a/\xCDV[\x81\x14a/\xE9W__\xFD[PV[_\x815\x90Pa/\xFA\x81a/\xD6V[\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a0!Wa0 a0\0V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0>Wa0=a0\x04V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a0ZWa0Ya0\x08V[[\x92P\x92\x90PV[_____`\x80\x86\x88\x03\x12\x15a0zWa0ya-\xF4V[[_a0\x87\x88\x82\x89\x01a/{V[\x95PP` a0\x98\x88\x82\x89\x01a/\xECV[\x94PP`@a0\xA9\x88\x82\x89\x01a/\xECV[\x93PP``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xCAWa0\xC9a-\xF8V[[a0\xD6\x88\x82\x89\x01a0\x0CV[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[a0\xEE\x81a.|V[\x81\x14a0\xF8W__\xFD[PV[_\x815\x90Pa1\t\x81a0\xE5V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a1%Wa1$a-\xF4V[[_a12\x85\x82\x86\x01a.\xCEV[\x92PP` a1C\x85\x82\x86\x01a0\xFBV[\x91PP\x92P\x92\x90PV[a1V\x81a/TV[\x82RPPV[a1e\x81a/\xCDV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a1\xAD\x82a1kV[a1\xB7\x81\x85a1uV[\x93Pa1\xC7\x81\x85` \x86\x01a1\x85V[a1\xD0\x81a1\x93V[\x84\x01\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x06\x81\x10a2\x19Wa2\x18a1\xDBV[[PV[_\x81\x90Pa2)\x82a2\x08V[\x91\x90PV[_a28\x82a2\x1CV[\x90P\x91\x90PV[a2H\x81a2.V[\x82RPPV[a2W\x81a.\xAFV[\x82RPPV[_a\x01@\x83\x01_\x83\x01Qa2s_\x86\x01\x82a1MV[P` \x83\x01Qa2\x86` \x86\x01\x82a1MV[P`@\x83\x01Qa2\x99`@\x86\x01\x82a1MV[P``\x83\x01Qa2\xAC``\x86\x01\x82a1\\V[P`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra2\xC4\x82\x82a1\xA3V[\x91PP`\xA0\x83\x01Qa2\xD9`\xA0\x86\x01\x82a1\\V[P`\xC0\x83\x01Qa2\xEC`\xC0\x86\x01\x82a2?V[P`\xE0\x83\x01Qa2\xFF`\xE0\x86\x01\x82a1\\V[Pa\x01\0\x83\x01Qa3\x14a\x01\0\x86\x01\x82a1\\V[Pa\x01 \x83\x01Qa3)a\x01 \x86\x01\x82a2NV[P\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra3L\x81\x84a2]V[\x90P\x92\x91PPV[a3]\x81a/\xCDV[\x82RPPV[_` \x82\x01\x90Pa3v_\x83\x01\x84a3TV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a3\x91Wa3\x90a-\xF4V[[_a3\x9E\x84\x82\x85\x01a/\xECV[\x91PP\x92\x91PPV[___``\x84\x86\x03\x12\x15a3\xBEWa3\xBDa-\xF4V[[_a3\xCB\x86\x82\x87\x01a/{V[\x93PP` a3\xDC\x86\x82\x87\x01a/{V[\x92PP`@a3\xED\x86\x82\x87\x01a/\xECV[\x91PP\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15a4\rWa4\x0Ca-\xF4V[[_a4\x1A\x85\x82\x86\x01a.\xCEV[\x92PP` a4+\x85\x82\x86\x01a.\xCEV[\x91PP\x92P\x92\x90PV[a4>\x81a/TV[\x82RPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a4^\x82a1kV[a4h\x81\x85a4DV[\x93Pa4x\x81\x85` \x86\x01a1\x85V[a4\x81\x81a1\x93V[\x84\x01\x91PP\x92\x91PPV[a4\x95\x81a2.V[\x82RPPV[_a\x01@\x82\x01\x90Pa4\xAF_\x83\x01\x8Da45V[a4\xBC` \x83\x01\x8Ca45V[a4\xC9`@\x83\x01\x8Ba45V[a4\xD6``\x83\x01\x8Aa3TV[\x81\x81\x03`\x80\x83\x01Ra4\xE8\x81\x89a4TV[\x90Pa4\xF7`\xA0\x83\x01\x88a3TV[a5\x04`\xC0\x83\x01\x87a4\x8CV[a5\x11`\xE0\x83\x01\x86a3TV[a5\x1Fa\x01\0\x83\x01\x85a3TV[a5-a\x01 \x83\x01\x84a/\rV[\x9B\x9APPPPPPPPPPPV[__`@\x83\x85\x03\x12\x15a5RWa5Qa-\xF4V[[_a5_\x85\x82\x86\x01a/{V[\x92PP` a5p\x85\x82\x86\x01a/{V[\x91PP\x92P\x92\x90PV[\x7FToken amount must be > 0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a5\xAE`\x18\x83a4DV[\x91Pa5\xB9\x82a5zV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra5\xDB\x81a5\xA2V[\x90P\x91\x90PV[_\x81``\x1B\x90P\x91\x90PV[_a5\xF8\x82a5\xE2V[\x90P\x91\x90PV[_a6\t\x82a5\xEEV[\x90P\x91\x90PV[a6!a6\x1C\x82a/TV[a5\xFFV[\x82RPPV[_\x81\x90P\x91\x90PV[a6Aa6<\x82a/\xCDV[a6'V[\x82RPPV[_a6R\x82\x87a6\x10V[`\x14\x82\x01\x91Pa6b\x82\x86a6\x10V[`\x14\x82\x01\x91Pa6r\x82\x85a60V[` \x82\x01\x91Pa6\x82\x82\x84a60V[` \x82\x01\x91P\x81\x90P\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a7\x05W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a7\x18Wa7\x17a6\xC1V[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a7z\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a7?V[a7\x84\x86\x83a7?V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a7\xBFa7\xBAa7\xB5\x84a/\xCDV[a7\x9CV[a/\xCDV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a7\xD8\x83a7\xA5V[a7\xECa7\xE4\x82a7\xC6V[\x84\x84Ta7KV[\x82UPPPPV[__\x90P\x90V[a8\x03a7\xF4V[a8\x0E\x81\x84\x84a7\xCFV[PPPV[[\x81\x81\x10\x15a81Wa8&_\x82a7\xFBV[`\x01\x81\x01\x90Pa8\x14V[PPV[`\x1F\x82\x11\x15a8vWa8G\x81a7\x1EV[a8P\x84a70V[\x81\x01` \x85\x10\x15a8_W\x81\x90P[a8sa8k\x85a70V[\x83\x01\x82a8\x13V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a8\x96_\x19\x84`\x08\x02a8{V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a8\xAE\x83\x83a8\x87V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a8\xC7\x82a1kV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\xE0Wa8\xDFa6\x94V[[a8\xEA\x82Ta6\xEEV[a8\xF5\x82\x82\x85a85V[_` \x90P`\x1F\x83\x11`\x01\x81\x14a9&W_\x84\x15a9\x14W\x82\x87\x01Q\x90P[a9\x1E\x85\x82a8\xA3V[\x86UPa9\x85V[`\x1F\x19\x84\x16a94\x86a7\x1EV[_[\x82\x81\x10\x15a9[W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa96V[\x86\x83\x10\x15a9xW\x84\x89\x01Qa9t`\x1F\x89\x16\x82a8\x87V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[\x82\x81\x837_\x83\x83\x01RPPPV[_a9\xA6\x83\x85a4DV[\x93Pa9\xB3\x83\x85\x84a9\x8DV[a9\xBC\x83a1\x93V[\x84\x01\x90P\x93\x92PPPV[_`\x80\x82\x01\x90Pa9\xDA_\x83\x01\x88a45V[a9\xE7` \x83\x01\x87a3TV[a9\xF4`@\x83\x01\x86a3TV[\x81\x81\x03``\x83\x01Ra:\x07\x81\x84\x86a9\x9BV[\x90P\x96\x95PPPPPPV[\x7FNot a party to the order\0\0\0\0\0\0\0\0_\x82\x01RPV[_a:G`\x18\x83a4DV[\x91Pa:R\x82a:\x13V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra:t\x81a:;V[\x90P\x91\x90PV[\x7FCan only dispute after fiat is m_\x82\x01R\x7Farked sent\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a:\xD5`*\x83a4DV[\x91Pa:\xE0\x82a:{V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;\x02\x81a:\xC9V[\x90P\x91\x90PV[\x7FNot the assigned LP\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a;=`\x13\x83a4DV[\x91Pa;H\x82a;\tV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;j\x81a;1V[\x90P\x91\x90PV[\x7FOrder status invalid\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a;\xA5`\x14\x83a4DV[\x91Pa;\xB0\x82a;qV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;\xD2\x81a;\x99V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a<\x10\x82a/\xCDV[\x91Pa<\x1B\x83a/\xCDV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a<3Wa<2a;\xD9V[[\x92\x91PPV[\x7FPayment deadline not passed\0\0\0\0\0_\x82\x01RPV[_a<m`\x1B\x83a4DV[\x91Pa<x\x82a<9V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra<\x9A\x81a<aV[\x90P\x91\x90PV[_`@\x82\x01\x90Pa<\xB4_\x83\x01\x85a45V[a<\xC1` \x83\x01\x84a3TV[\x93\x92PPPV[_\x81Q\x90Pa<\xD6\x81a0\xE5V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a<\xF1Wa<\xF0a-\xF4V[[_a<\xFE\x84\x82\x85\x01a<\xC8V[\x91PP\x92\x91PPV[\x7FRECLAIM_TRANSFER_FAILED\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a=;`\x17\x83a4DV[\x91Pa=F\x82a=\x07V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra=h\x81a=/V[\x90P\x91\x90PV[\x7FUser payment timed out\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a=\xA3`\x16\x83a4DV[\x91Pa=\xAE\x82a=oV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra=\xD0\x81a=\x97V[\x90P\x91\x90PV[\x7FOrder not in dispute\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a>\x0B`\x14\x83a4DV[\x91Pa>\x16\x82a=\xD7V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra>8\x81a=\xFFV[\x90P\x91\x90PV[\x7FDISPUTE_BUYER_TRANSFER_FAILED\0\0\0_\x82\x01RPV[_a>s`\x1D\x83a4DV[\x91Pa>~\x82a>?V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra>\xA0\x81a>gV[\x90P\x91\x90PV[_` \x82\x01\x90Pa>\xBA_\x83\x01\x84a45V[\x92\x91PPV[\x7FDISPUTE_LP_TRANSFER_FAILED\0\0\0\0\0\0_\x82\x01RPV[_a>\xF4`\x1A\x83a4DV[\x91Pa>\xFF\x82a>\xC0V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra?!\x81a>\xE8V[\x90P\x91\x90PV[\x7FNot the buyer\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a?\\`\r\x83a4DV[\x91Pa?g\x82a?(V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra?\x89\x81a?PV[\x90P\x91\x90PV[\x7FOrder not pending\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a?\xC4`\x11\x83a4DV[\x91Pa?\xCF\x82a?\x90V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra?\xF1\x81a?\xB8V[\x90P\x91\x90PV[\x7FLock deadline not passed\0\0\0\0\0\0\0\0_\x82\x01RPV[_a@,`\x18\x83a4DV[\x91Pa@7\x82a?\xF8V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra@Y\x81a@ V[\x90P\x91\x90PV[\x7FLP failed to lock funds\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a@\x94`\x17\x83a4DV[\x91Pa@\x9F\x82a@`V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra@\xC1\x81a@\x88V[\x90P\x91\x90PV[\x7FFunds not locked\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a@\xFC`\x10\x83a4DV[\x91PaA\x07\x82a@\xC8V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaA)\x81a@\xF0V[\x90P\x91\x90PV[\x7FPayment deadline passed\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aAd`\x17\x83a4DV[\x91PaAo\x82aA0V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaA\x91\x81aAXV[\x90P\x91\x90PV[\x7FFiat not marked as sent\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aA\xCC`\x17\x83a4DV[\x91PaA\xD7\x82aA\x98V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaA\xF9\x81aA\xC0V[\x90P\x91\x90PV[\x7FRELEASE_TRANSFER_FAILED\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aB4`\x17\x83a4DV[\x91PaB?\x82aB\0V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaBa\x81aB(V[\x90P\x91\x90PV[_aBr\x82a/\xCDV[\x91PaB}\x83a/\xCDV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aB\x95WaB\x94a;\xD9V[[\x92\x91PPV[\x7FLock deadline passed\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aB\xCF`\x14\x83a4DV[\x91PaB\xDA\x82aB\x9BV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaB\xFC\x81aB\xC3V[\x90P\x91\x90PV[\x7FLP_ROLE_REQUIRED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aC7`\x10\x83a4DV[\x91PaCB\x82aC\x03V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaCd\x81aC+V[\x90P\x91\x90PV[_``\x82\x01\x90PaC~_\x83\x01\x86a45V[aC\x8B` \x83\x01\x85a45V[aC\x98`@\x83\x01\x84a3TV[\x94\x93PPPPV[\x7FLOCK_TRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aC\xD4`\x14\x83a4DV[\x91PaC\xDF\x82aC\xA0V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaD\x01\x81aC\xC8V[\x90P\x91\x90PV[_`@\x82\x01\x90PaD\x1B_\x83\x01\x85a45V[aD(` \x83\x01\x84a/\rV[\x93\x92PPPV[\x7FLP_CAP_EXCEEDED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aDc`\x0F\x83a4DV[\x91PaDn\x82aD/V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaD\x90\x81aDWV[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \x11<\xB5^\x07\x10\xCA\xB3\x04\x1E\n\xB0\xB5*\xEEi\xC9Psc\xF0\xE9\xFE\xEC\x9C\xE2%\xDD\xD8r\xDA\xCDdsolcC\0\x08\x1E\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AccessControlBadConfirmation()` and selector `0x6697b232`.
```solidity
error AccessControlBadConfirmation();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccessControlBadConfirmation;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AccessControlBadConfirmation>
        for UnderlyingRustTuple<'_> {
            fn from(value: AccessControlBadConfirmation) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AccessControlBadConfirmation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccessControlBadConfirmation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccessControlBadConfirmation()";
            const SELECTOR: [u8; 4] = [102u8, 151u8, 178u8, 50u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AccessControlUnauthorizedAccount(address,bytes32)` and selector `0xe2517d3f`.
```solidity
error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccessControlUnauthorizedAccount {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub neededRole: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::FixedBytes<32>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AccessControlUnauthorizedAccount>
        for UnderlyingRustTuple<'_> {
            fn from(value: AccessControlUnauthorizedAccount) -> Self {
                (value.account, value.neededRole)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AccessControlUnauthorizedAccount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    account: tuple.0,
                    neededRole: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccessControlUnauthorizedAccount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccessControlUnauthorizedAccount(address,bytes32)";
            const SELECTOR: [u8; 4] = [226u8, 81u8, 125u8, 63u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.neededRole),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `EnforcedPause()` and selector `0xd93c0665`.
```solidity
error EnforcedPause();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EnforcedPause;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EnforcedPause> for UnderlyingRustTuple<'_> {
            fn from(value: EnforcedPause) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EnforcedPause {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EnforcedPause {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EnforcedPause()";
            const SELECTOR: [u8; 4] = [217u8, 60u8, 6u8, 101u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ExpectedPause()` and selector `0x8dfc202b`.
```solidity
error ExpectedPause();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpectedPause;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ExpectedPause> for UnderlyingRustTuple<'_> {
            fn from(value: ExpectedPause) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpectedPause {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExpectedPause {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExpectedPause()";
            const SELECTOR: [u8; 4] = [141u8, 252u8, 32u8, 43u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ReentrancyGuardReentrantCall()` and selector `0x3ee5aeb5`.
```solidity
error ReentrancyGuardReentrantCall();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ReentrancyGuardReentrantCall;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ReentrancyGuardReentrantCall>
        for UnderlyingRustTuple<'_> {
            fn from(value: ReentrancyGuardReentrantCall) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ReentrancyGuardReentrantCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ReentrancyGuardReentrantCall {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ReentrancyGuardReentrantCall()";
            const SELECTOR: [u8; 4] = [62u8, 229u8, 174u8, 181u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `DisputeResolved(bytes32,address,address)` and selector `0x41f67fe3f67f1dce68769486352c3bc7c5c5acda8d3c0e6628f918c255e34a0b`.
```solidity
event DisputeResolved(bytes32 indexed orderId, address indexed resolver, address winner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DisputeResolved {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub resolver: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub winner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for DisputeResolved {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "DisputeResolved(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                65u8, 246u8, 127u8, 227u8, 246u8, 127u8, 29u8, 206u8, 104u8, 118u8,
                148u8, 134u8, 53u8, 44u8, 59u8, 199u8, 197u8, 197u8, 172u8, 218u8, 141u8,
                60u8, 14u8, 102u8, 40u8, 249u8, 24u8, 194u8, 85u8, 227u8, 74u8, 11u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    orderId: topics.1,
                    resolver: topics.2,
                    winner: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.winner,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.orderId.clone(),
                    self.resolver.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.orderId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.resolver,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DisputeResolved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DisputeResolved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DisputeResolved) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `FiatSent(bytes32,bytes32)` and selector `0xd8e25e58ea6ac5e1dff31cc8c341922c4d1ccd427c84e0bf339c510b4b561062`.
```solidity
event FiatSent(bytes32 indexed orderId, bytes32 proofHash);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct FiatSent {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub proofHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for FiatSent {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "FiatSent(bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                216u8, 226u8, 94u8, 88u8, 234u8, 106u8, 197u8, 225u8, 223u8, 243u8, 28u8,
                200u8, 195u8, 65u8, 146u8, 44u8, 77u8, 28u8, 205u8, 66u8, 124u8, 132u8,
                224u8, 191u8, 51u8, 156u8, 81u8, 11u8, 75u8, 86u8, 16u8, 98u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    orderId: topics.1,
                    proofHash: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.proofHash),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.orderId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.orderId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for FiatSent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&FiatSent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &FiatSent) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `FundsLocked(bytes32,address,uint256)` and selector `0x15144217405064631de951752111334f6d9db4be6cfff45346f7068ea857fcad`.
```solidity
event FundsLocked(bytes32 indexed orderId, address indexed lp, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct FundsLocked {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub lp: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for FundsLocked {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "FundsLocked(bytes32,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                21u8, 20u8, 66u8, 23u8, 64u8, 80u8, 100u8, 99u8, 29u8, 233u8, 81u8,
                117u8, 33u8, 17u8, 51u8, 79u8, 109u8, 157u8, 180u8, 190u8, 108u8, 255u8,
                244u8, 83u8, 70u8, 247u8, 6u8, 142u8, 168u8, 87u8, 252u8, 173u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    orderId: topics.1,
                    lp: topics.2,
                    amount: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.orderId.clone(), self.lp.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.orderId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.lp,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for FundsLocked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&FundsLocked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &FundsLocked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `LpCapSet(address,address,uint256)` and selector `0x64d19fedce9351d7c194985163e6b400555c186ba785881239156413948a8bca`.
```solidity
event LpCapSet(address indexed lp, address indexed token, uint256 cap);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct LpCapSet {
        #[allow(missing_docs)]
        pub lp: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub cap: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for LpCapSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "LpCapSet(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                100u8, 209u8, 159u8, 237u8, 206u8, 147u8, 81u8, 215u8, 193u8, 148u8,
                152u8, 81u8, 99u8, 230u8, 180u8, 0u8, 85u8, 92u8, 24u8, 107u8, 167u8,
                133u8, 136u8, 18u8, 57u8, 21u8, 100u8, 19u8, 148u8, 138u8, 139u8, 202u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    lp: topics.1,
                    token: topics.2,
                    cap: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.cap),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.lp.clone(), self.token.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.lp,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.token,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for LpCapSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&LpCapSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &LpCapSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `LpOutstandingUpdated(address,address,uint256)` and selector `0x08118319557e9998588247d46837aea5c77de48967936343b6c923fb1f7813e3`.
```solidity
event LpOutstandingUpdated(address indexed lp, address indexed token, uint256 newOutstanding);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct LpOutstandingUpdated {
        #[allow(missing_docs)]
        pub lp: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOutstanding: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for LpOutstandingUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "LpOutstandingUpdated(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                8u8, 17u8, 131u8, 25u8, 85u8, 126u8, 153u8, 152u8, 88u8, 130u8, 71u8,
                212u8, 104u8, 55u8, 174u8, 165u8, 199u8, 125u8, 228u8, 137u8, 103u8,
                147u8, 99u8, 67u8, 182u8, 201u8, 35u8, 251u8, 31u8, 120u8, 19u8, 227u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    lp: topics.1,
                    token: topics.2,
                    newOutstanding: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newOutstanding),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.lp.clone(), self.token.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.lp,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.token,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for LpOutstandingUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&LpOutstandingUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &LpOutstandingUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OrderCancelled(bytes32,string)` and selector `0x386c4fc2fe51e17dd0f7b5bb6d38fca3a652d4e6fbcfa1a93283034acdc5843a`.
```solidity
event OrderCancelled(bytes32 indexed orderId, string reason);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OrderCancelled {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub reason: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OrderCancelled {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OrderCancelled(bytes32,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                56u8, 108u8, 79u8, 194u8, 254u8, 81u8, 225u8, 125u8, 208u8, 247u8, 181u8,
                187u8, 109u8, 56u8, 252u8, 163u8, 166u8, 82u8, 212u8, 230u8, 251u8,
                207u8, 161u8, 169u8, 50u8, 131u8, 3u8, 74u8, 205u8, 197u8, 132u8, 58u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    orderId: topics.1,
                    reason: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.reason,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.orderId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.orderId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OrderCancelled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OrderCancelled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OrderCancelled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OrderCompleted(bytes32,address)` and selector `0xa13a9d77a6fbd690b8000b14cc8f4ba7602b329937638e708937fd9faa6069cd`.
```solidity
event OrderCompleted(bytes32 indexed orderId, address indexed buyer);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OrderCompleted {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub buyer: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OrderCompleted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OrderCompleted(bytes32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                161u8, 58u8, 157u8, 119u8, 166u8, 251u8, 214u8, 144u8, 184u8, 0u8, 11u8,
                20u8, 204u8, 143u8, 75u8, 167u8, 96u8, 43u8, 50u8, 153u8, 55u8, 99u8,
                142u8, 112u8, 137u8, 55u8, 253u8, 159u8, 170u8, 96u8, 105u8, 205u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    orderId: topics.1,
                    buyer: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.orderId.clone(), self.buyer.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.orderId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.buyer,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OrderCompleted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OrderCompleted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OrderCompleted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OrderCreated(bytes32,address,address,uint256,uint256,string)` and selector `0xf2b2765edf82fc07e26cea1350aecc843b2e6bbc95c7b52076d65c97c4301155`.
```solidity
event OrderCreated(bytes32 indexed orderId, address indexed buyer, address token, uint256 tokenAmount, uint256 fiatAmount, string fiatCurrency);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OrderCreated {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub buyer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fiatAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fiatCurrency: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OrderCreated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OrderCreated(bytes32,address,address,uint256,uint256,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                242u8, 178u8, 118u8, 94u8, 223u8, 130u8, 252u8, 7u8, 226u8, 108u8, 234u8,
                19u8, 80u8, 174u8, 204u8, 132u8, 59u8, 46u8, 107u8, 188u8, 149u8, 199u8,
                181u8, 32u8, 118u8, 214u8, 92u8, 151u8, 196u8, 48u8, 17u8, 85u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    orderId: topics.1,
                    buyer: topics.2,
                    token: data.0,
                    tokenAmount: data.1,
                    fiatAmount: data.2,
                    fiatCurrency: data.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fiatAmount),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.fiatCurrency,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.orderId.clone(), self.buyer.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.orderId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.buyer,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OrderCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OrderCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OrderCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OrderDisputed(bytes32,address)` and selector `0x9809df6c9d63642a6f1e33a698d415ba2d882a4071894da748ab5482975c8e63`.
```solidity
event OrderDisputed(bytes32 indexed orderId, address indexed reporter);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OrderDisputed {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub reporter: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OrderDisputed {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OrderDisputed(bytes32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                152u8, 9u8, 223u8, 108u8, 157u8, 99u8, 100u8, 42u8, 111u8, 30u8, 51u8,
                166u8, 152u8, 212u8, 21u8, 186u8, 45u8, 136u8, 42u8, 64u8, 113u8, 137u8,
                77u8, 167u8, 72u8, 171u8, 84u8, 130u8, 151u8, 92u8, 142u8, 99u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    orderId: topics.1,
                    reporter: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.orderId.clone(),
                    self.reporter.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.orderId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.reporter,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OrderDisputed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OrderDisputed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OrderDisputed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Paused(address)` and selector `0x62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258`.
```solidity
event Paused(address account);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Paused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Paused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Paused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                98u8, 231u8, 140u8, 234u8, 1u8, 190u8, 227u8, 32u8, 205u8, 78u8, 66u8,
                2u8, 112u8, 181u8, 234u8, 116u8, 0u8, 13u8, 17u8, 176u8, 201u8, 247u8,
                71u8, 84u8, 235u8, 219u8, 252u8, 84u8, 75u8, 5u8, 162u8, 88u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Paused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Paused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Paused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RoleAdminChanged(bytes32,bytes32,bytes32)` and selector `0xbd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff`.
```solidity
event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleAdminChanged {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub previousAdminRole: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub newAdminRole: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleAdminChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "RoleAdminChanged(bytes32,bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8,
                81u8, 66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8,
                71u8, 92u8, 58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    previousAdminRole: topics.2,
                    newAdminRole: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.previousAdminRole.clone(),
                    self.newAdminRole.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.previousAdminRole);
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.newAdminRole);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleAdminChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleAdminChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleAdminChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RoleGranted(bytes32,address,address)` and selector `0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d`.
```solidity
event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleGranted {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleGranted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleGranted(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8,
                236u8, 121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8,
                64u8, 48u8, 69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    account: topics.2,
                    sender: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleGranted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleGranted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleGranted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RoleRevoked(bytes32,address,address)` and selector `0xf6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b`.
```solidity
event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleRevoked {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleRevoked {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleRevoked(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8,
                103u8, 11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8,
                253u8, 100u8, 235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    account: topics.2,
                    sender: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleRevoked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleRevoked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleRevoked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Unpaused(address)` and selector `0x5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa`.
```solidity
event Unpaused(address account);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Unpaused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Unpaused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Unpaused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8, 185u8, 238u8, 10u8, 73u8, 91u8, 242u8, 230u8, 255u8, 156u8, 145u8,
                167u8, 131u8, 76u8, 27u8, 164u8, 253u8, 210u8, 68u8, 165u8, 232u8, 170u8,
                78u8, 83u8, 123u8, 211u8, 138u8, 234u8, 228u8, 176u8, 115u8, 170u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Unpaused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Unpaused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Unpaused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {}
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`.
```solidity
function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`DEFAULT_ADMIN_ROLE()`](DEFAULT_ADMIN_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLEReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DEFAULT_ADMIN_ROLECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_ADMIN_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DEFAULT_ADMIN_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_ADMIN_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DEFAULT_ADMIN_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DEFAULT_ADMIN_ROLE()";
            const SELECTOR: [u8; 4] = [162u8, 23u8, 253u8, 223u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: DEFAULT_ADMIN_ROLEReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: DEFAULT_ADMIN_ROLEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `LP_ROLE()` and selector `0xe1d39450`.
```solidity
function LP_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LP_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`LP_ROLE()`](LP_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LP_ROLEReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<LP_ROLECall> for UnderlyingRustTuple<'_> {
                fn from(value: LP_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for LP_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<LP_ROLEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: LP_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for LP_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for LP_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LP_ROLE()";
            const SELECTOR: [u8; 4] = [225u8, 211u8, 148u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: LP_ROLEReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: LP_ROLEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `RELAYER_ROLE()` and selector `0x926d7d7f`.
```solidity
function RELAYER_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RELAYER_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`RELAYER_ROLE()`](RELAYER_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RELAYER_ROLEReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<RELAYER_ROLECall> for UnderlyingRustTuple<'_> {
                fn from(value: RELAYER_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for RELAYER_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<RELAYER_ROLEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: RELAYER_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for RELAYER_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for RELAYER_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RELAYER_ROLE()";
            const SELECTOR: [u8; 4] = [146u8, 109u8, 125u8, 127u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: RELAYER_ROLEReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: RELAYER_ROLEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `cancelOrder(bytes32)` and selector `0x7489ec23`.
```solidity
function cancelOrder(bytes32 orderId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelOrderCall {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`cancelOrder(bytes32)`](cancelOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelOrderReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<cancelOrderCall> for UnderlyingRustTuple<'_> {
                fn from(value: cancelOrderCall) -> Self {
                    (value.orderId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cancelOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { orderId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<cancelOrderReturn> for UnderlyingRustTuple<'_> {
                fn from(value: cancelOrderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cancelOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl cancelOrderReturn {
            fn _tokenize(
                &self,
            ) -> <cancelOrderCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cancelOrderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cancelOrderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancelOrder(bytes32)";
            const SELECTOR: [u8; 4] = [116u8, 137u8, 236u8, 35u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                cancelOrderReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `confirmFiatSent(bytes32,bytes32)` and selector `0x8fa4a641`.
```solidity
function confirmFiatSent(bytes32 orderId, bytes32 proofHash) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmFiatSentCall {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub proofHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`confirmFiatSent(bytes32,bytes32)`](confirmFiatSentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmFiatSentReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<confirmFiatSentCall> for UnderlyingRustTuple<'_> {
                fn from(value: confirmFiatSentCall) -> Self {
                    (value.orderId, value.proofHash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for confirmFiatSentCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        orderId: tuple.0,
                        proofHash: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<confirmFiatSentReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: confirmFiatSentReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for confirmFiatSentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl confirmFiatSentReturn {
            fn _tokenize(
                &self,
            ) -> <confirmFiatSentCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for confirmFiatSentCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = confirmFiatSentReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "confirmFiatSent(bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [143u8, 164u8, 166u8, 65u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.proofHash),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                confirmFiatSentReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `createOnRampOrder(address,uint256,uint256,string)` and selector `0x305df56f`.
```solidity
function createOnRampOrder(address token, uint256 tokenAmount, uint256 fiatAmount, string memory fiatCurrency) external returns (bytes32 orderId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOnRampOrderCall {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fiatAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fiatCurrency: alloy::sol_types::private::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`createOnRampOrder(address,uint256,uint256,string)`](createOnRampOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOnRampOrderReturn {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::String,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createOnRampOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createOnRampOrderCall) -> Self {
                    (
                        value.token,
                        value.tokenAmount,
                        value.fiatAmount,
                        value.fiatCurrency,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOnRampOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        token: tuple.0,
                        tokenAmount: tuple.1,
                        fiatAmount: tuple.2,
                        fiatCurrency: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createOnRampOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createOnRampOrderReturn) -> Self {
                    (value.orderId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOnRampOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { orderId: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createOnRampOrderCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createOnRampOrder(address,uint256,uint256,string)";
            const SELECTOR: [u8; 4] = [48u8, 93u8, 245u8, 111u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fiatAmount),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.fiatCurrency,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: createOnRampOrderReturn = r.into();
                        r.orderId
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: createOnRampOrderReturn = r.into();
                        r.orderId
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `disputeOrder(bytes32)` and selector `0x3e4e3792`.
```solidity
function disputeOrder(bytes32 orderId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disputeOrderCall {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`disputeOrder(bytes32)`](disputeOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disputeOrderReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<disputeOrderCall> for UnderlyingRustTuple<'_> {
                fn from(value: disputeOrderCall) -> Self {
                    (value.orderId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disputeOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { orderId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<disputeOrderReturn> for UnderlyingRustTuple<'_> {
                fn from(value: disputeOrderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disputeOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl disputeOrderReturn {
            fn _tokenize(
                &self,
            ) -> <disputeOrderCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disputeOrderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = disputeOrderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disputeOrder(bytes32)";
            const SELECTOR: [u8; 4] = [62u8, 78u8, 55u8, 146u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                disputeOrderReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOrder(bytes32)` and selector `0x5778472a`.
```solidity
function getOrder(bytes32 orderId) external view returns (IOnRampEscrow.OnRampOrder memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOrderCall {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    ///Container type for the return parameters of the [`getOrder(bytes32)`](getOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOrderReturn {
        #[allow(missing_docs)]
        pub _0: <IOnRampEscrow::OnRampOrder as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOrderCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOrderCall) -> Self {
                    (value.orderId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { orderId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (IOnRampEscrow::OnRampOrder,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IOnRampEscrow::OnRampOrder as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOrderReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOrderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOrderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IOnRampEscrow::OnRampOrder as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IOnRampEscrow::OnRampOrder,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOrder(bytes32)";
            const SELECTOR: [u8; 4] = [87u8, 120u8, 71u8, 42u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <IOnRampEscrow::OnRampOrder as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getOrderReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getOrderReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`.
```solidity
function getRoleAdmin(bytes32 role) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getRoleAdmin(bytes32)`](getRoleAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRoleAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminCall) -> Self {
                    (value.role,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRoleAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRoleAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRoleAdmin(bytes32)";
            const SELECTOR: [u8; 4] = [36u8, 138u8, 156u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getRoleAdminReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getRoleAdminReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`.
```solidity
function grantRole(bytes32 role, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`grantRole(bytes32,address)`](grantRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<grantRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<grantRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl grantRoleReturn {
            fn _tokenize(
                &self,
            ) -> <grantRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for grantRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = grantRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "grantRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [47u8, 47u8, 241u8, 93u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                grantRoleReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `hasRole(bytes32,address)` and selector `0x91d14854`.
```solidity
function hasRole(bytes32 role, address account) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`hasRole(bytes32,address)`](hasRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hasRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hasRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hasRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hasRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [145u8, 209u8, 72u8, 84u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: hasRoleReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: hasRoleReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `lockDeadline()` and selector `0xfa4d0c3c`.
```solidity
function lockDeadline() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lockDeadlineCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`lockDeadline()`](lockDeadlineCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lockDeadlineReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lockDeadlineCall> for UnderlyingRustTuple<'_> {
                fn from(value: lockDeadlineCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lockDeadlineCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lockDeadlineReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lockDeadlineReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lockDeadlineReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lockDeadlineCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lockDeadline()";
            const SELECTOR: [u8; 4] = [250u8, 77u8, 12u8, 60u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: lockDeadlineReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: lockDeadlineReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `lockFunds(bytes32)` and selector `0x9cbeb6c1`.
```solidity
function lockFunds(bytes32 orderId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lockFundsCall {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`lockFunds(bytes32)`](lockFundsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lockFundsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lockFundsCall> for UnderlyingRustTuple<'_> {
                fn from(value: lockFundsCall) -> Self {
                    (value.orderId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lockFundsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { orderId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lockFundsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lockFundsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lockFundsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl lockFundsReturn {
            fn _tokenize(
                &self,
            ) -> <lockFundsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lockFundsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = lockFundsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lockFunds(bytes32)";
            const SELECTOR: [u8; 4] = [156u8, 190u8, 182u8, 193u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                lockFundsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `lockFundsByRelayer(bytes32,address)` and selector `0x9e49f7e4`.
```solidity
function lockFundsByRelayer(bytes32 orderId, address lpAddress) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lockFundsByRelayerCall {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub lpAddress: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`lockFundsByRelayer(bytes32,address)`](lockFundsByRelayerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lockFundsByRelayerReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lockFundsByRelayerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: lockFundsByRelayerCall) -> Self {
                    (value.orderId, value.lpAddress)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lockFundsByRelayerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        orderId: tuple.0,
                        lpAddress: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lockFundsByRelayerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: lockFundsByRelayerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lockFundsByRelayerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl lockFundsByRelayerReturn {
            fn _tokenize(
                &self,
            ) -> <lockFundsByRelayerCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lockFundsByRelayerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = lockFundsByRelayerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lockFundsByRelayer(bytes32,address)";
            const SELECTOR: [u8; 4] = [158u8, 73u8, 247u8, 228u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.lpAddress,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                lockFundsByRelayerReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `lpCapByToken(address,address)` and selector `0xfc46d8b8`.
```solidity
function lpCapByToken(address, address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lpCapByTokenCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`lpCapByToken(address,address)`](lpCapByTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lpCapByTokenReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lpCapByTokenCall> for UnderlyingRustTuple<'_> {
                fn from(value: lpCapByTokenCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lpCapByTokenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lpCapByTokenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lpCapByTokenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lpCapByTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lpCapByTokenCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lpCapByToken(address,address)";
            const SELECTOR: [u8; 4] = [252u8, 70u8, 216u8, 184u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: lpCapByTokenReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: lpCapByTokenReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `lpOutstandingByToken(address,address)` and selector `0xce9bc15e`.
```solidity
function lpOutstandingByToken(address, address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lpOutstandingByTokenCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`lpOutstandingByToken(address,address)`](lpOutstandingByTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lpOutstandingByTokenReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lpOutstandingByTokenCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: lpOutstandingByTokenCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lpOutstandingByTokenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lpOutstandingByTokenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: lpOutstandingByTokenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lpOutstandingByTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lpOutstandingByTokenCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lpOutstandingByToken(address,address)";
            const SELECTOR: [u8; 4] = [206u8, 155u8, 193u8, 94u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: lpOutstandingByTokenReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: lpOutstandingByTokenReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `orders(bytes32)` and selector `0x9c3f1e90`.
```solidity
function orders(bytes32) external view returns (address buyer, address lp, address token, uint256 tokenAmount, string memory fiatCurrency, uint256 fiatAmount, IOnRampEscrow.OrderStatus status, uint256 createdAt, uint256 fundsLockedAt, bytes32 userPaymentProof);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ordersCall(pub alloy::sol_types::private::FixedBytes<32>);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`orders(bytes32)`](ordersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ordersReturn {
        #[allow(missing_docs)]
        pub buyer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub lp: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fiatCurrency: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub fiatAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub status: <IOnRampEscrow::OrderStatus as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub createdAt: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fundsLockedAt: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub userPaymentProof: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ordersCall> for UnderlyingRustTuple<'_> {
                fn from(value: ordersCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ordersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                IOnRampEscrow::OrderStatus,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                <IOnRampEscrow::OrderStatus as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ordersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ordersReturn) -> Self {
                    (
                        value.buyer,
                        value.lp,
                        value.token,
                        value.tokenAmount,
                        value.fiatCurrency,
                        value.fiatAmount,
                        value.status,
                        value.createdAt,
                        value.fundsLockedAt,
                        value.userPaymentProof,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ordersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        buyer: tuple.0,
                        lp: tuple.1,
                        token: tuple.2,
                        tokenAmount: tuple.3,
                        fiatCurrency: tuple.4,
                        fiatAmount: tuple.5,
                        status: tuple.6,
                        createdAt: tuple.7,
                        fundsLockedAt: tuple.8,
                        userPaymentProof: tuple.9,
                    }
                }
            }
        }
        impl ordersReturn {
            fn _tokenize(
                &self,
            ) -> <ordersCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.buyer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.lp,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenAmount),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.fiatCurrency,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fiatAmount),
                    <IOnRampEscrow::OrderStatus as alloy_sol_types::SolType>::tokenize(
                        &self.status,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.createdAt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fundsLockedAt),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.userPaymentProof),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ordersCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ordersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                IOnRampEscrow::OrderStatus,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "orders(bytes32)";
            const SELECTOR: [u8; 4] = [156u8, 63u8, 30u8, 144u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                ordersReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `pause()` and selector `0x8456cb59`.
```solidity
function pause() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseCall;
    ///Container type for the return parameters of the [`pause()`](pauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl pauseReturn {
            fn _tokenize(
                &self,
            ) -> <pauseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pause()";
            const SELECTOR: [u8; 4] = [132u8, 86u8, 203u8, 89u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                pauseReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `paused()` and selector `0x5c975abb`.
```solidity
function paused() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`paused()`](pausedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pausedCall> for UnderlyingRustTuple<'_> {
                fn from(value: pausedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pausedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pausedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pausedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: pausedReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: pausedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `paymentDeadline()` and selector `0x6d929f23`.
```solidity
function paymentDeadline() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paymentDeadlineCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`paymentDeadline()`](paymentDeadlineCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paymentDeadlineReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<paymentDeadlineCall> for UnderlyingRustTuple<'_> {
                fn from(value: paymentDeadlineCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paymentDeadlineCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<paymentDeadlineReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: paymentDeadlineReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for paymentDeadlineReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paymentDeadlineCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paymentDeadline()";
            const SELECTOR: [u8; 4] = [109u8, 146u8, 159u8, 35u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: paymentDeadlineReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: paymentDeadlineReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `reclaimLockedFunds(bytes32)` and selector `0x3e88e0b6`.
```solidity
function reclaimLockedFunds(bytes32 orderId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reclaimLockedFundsCall {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`reclaimLockedFunds(bytes32)`](reclaimLockedFundsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reclaimLockedFundsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<reclaimLockedFundsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: reclaimLockedFundsCall) -> Self {
                    (value.orderId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for reclaimLockedFundsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { orderId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<reclaimLockedFundsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: reclaimLockedFundsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for reclaimLockedFundsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl reclaimLockedFundsReturn {
            fn _tokenize(
                &self,
            ) -> <reclaimLockedFundsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for reclaimLockedFundsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reclaimLockedFundsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reclaimLockedFunds(bytes32)";
            const SELECTOR: [u8; 4] = [62u8, 136u8, 224u8, 182u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                reclaimLockedFundsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `releaseFunds(bytes32)` and selector `0xc92ee043`.
```solidity
function releaseFunds(bytes32 orderId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct releaseFundsCall {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`releaseFunds(bytes32)`](releaseFundsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct releaseFundsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<releaseFundsCall> for UnderlyingRustTuple<'_> {
                fn from(value: releaseFundsCall) -> Self {
                    (value.orderId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for releaseFundsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { orderId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<releaseFundsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: releaseFundsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for releaseFundsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl releaseFundsReturn {
            fn _tokenize(
                &self,
            ) -> <releaseFundsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for releaseFundsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = releaseFundsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "releaseFunds(bytes32)";
            const SELECTOR: [u8; 4] = [201u8, 46u8, 224u8, 67u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                releaseFundsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`.
```solidity
function renounceRole(bytes32 role, address callerConfirmation) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub callerConfirmation: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`renounceRole(bytes32,address)`](renounceRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleCall) -> Self {
                    (value.role, value.callerConfirmation)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        callerConfirmation: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl renounceRoleReturn {
            fn _tokenize(
                &self,
            ) -> <renounceRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [54u8, 86u8, 138u8, 190u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.callerConfirmation,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                renounceRoleReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `resolveDispute(bytes32,bool)` and selector `0x43a0e3e6`.
```solidity
function resolveDispute(bytes32 orderId, bool releaseToBuyer) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolveDisputeCall {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub releaseToBuyer: bool,
    }
    ///Container type for the return parameters of the [`resolveDispute(bytes32,bool)`](resolveDisputeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resolveDisputeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                bool,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<resolveDisputeCall> for UnderlyingRustTuple<'_> {
                fn from(value: resolveDisputeCall) -> Self {
                    (value.orderId, value.releaseToBuyer)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for resolveDisputeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        orderId: tuple.0,
                        releaseToBuyer: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<resolveDisputeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: resolveDisputeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resolveDisputeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl resolveDisputeReturn {
            fn _tokenize(
                &self,
            ) -> <resolveDisputeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for resolveDisputeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = resolveDisputeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resolveDispute(bytes32,bool)";
            const SELECTOR: [u8; 4] = [67u8, 160u8, 227u8, 230u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.releaseToBuyer,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                resolveDisputeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`.
```solidity
function revokeRole(bytes32 role, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`revokeRole(bytes32,address)`](revokeRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revokeRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revokeRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl revokeRoleReturn {
            fn _tokenize(
                &self,
            ) -> <revokeRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for revokeRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = revokeRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "revokeRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [213u8, 71u8, 116u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                revokeRoleReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setLockDeadline(uint256)` and selector `0x84a5ce53`.
```solidity
function setLockDeadline(uint256 newDeadline) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setLockDeadlineCall {
        #[allow(missing_docs)]
        pub newDeadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setLockDeadline(uint256)`](setLockDeadlineCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setLockDeadlineReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setLockDeadlineCall> for UnderlyingRustTuple<'_> {
                fn from(value: setLockDeadlineCall) -> Self {
                    (value.newDeadline,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setLockDeadlineCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newDeadline: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setLockDeadlineReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setLockDeadlineReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setLockDeadlineReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setLockDeadlineReturn {
            fn _tokenize(
                &self,
            ) -> <setLockDeadlineCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setLockDeadlineCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setLockDeadlineReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setLockDeadline(uint256)";
            const SELECTOR: [u8; 4] = [132u8, 165u8, 206u8, 83u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newDeadline),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setLockDeadlineReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setLpCap(address,address,uint256)` and selector `0x896a2ff4`.
```solidity
function setLpCap(address lp, address token, uint256 cap) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setLpCapCall {
        #[allow(missing_docs)]
        pub lp: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub cap: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setLpCap(address,address,uint256)`](setLpCapCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setLpCapReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setLpCapCall> for UnderlyingRustTuple<'_> {
                fn from(value: setLpCapCall) -> Self {
                    (value.lp, value.token, value.cap)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setLpCapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        lp: tuple.0,
                        token: tuple.1,
                        cap: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setLpCapReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setLpCapReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setLpCapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setLpCapReturn {
            fn _tokenize(
                &self,
            ) -> <setLpCapCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setLpCapCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setLpCapReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setLpCap(address,address,uint256)";
            const SELECTOR: [u8; 4] = [137u8, 106u8, 47u8, 244u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.lp,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.cap),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setLpCapReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setPaymentDeadline(uint256)` and selector `0x8cfd919e`.
```solidity
function setPaymentDeadline(uint256 newDeadline) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPaymentDeadlineCall {
        #[allow(missing_docs)]
        pub newDeadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setPaymentDeadline(uint256)`](setPaymentDeadlineCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPaymentDeadlineReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setPaymentDeadlineCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPaymentDeadlineCall) -> Self {
                    (value.newDeadline,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPaymentDeadlineCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newDeadline: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setPaymentDeadlineReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPaymentDeadlineReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPaymentDeadlineReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setPaymentDeadlineReturn {
            fn _tokenize(
                &self,
            ) -> <setPaymentDeadlineCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setPaymentDeadlineCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setPaymentDeadlineReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setPaymentDeadline(uint256)";
            const SELECTOR: [u8; 4] = [140u8, 253u8, 145u8, 158u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newDeadline),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setPaymentDeadlineReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`.
```solidity
function supportsInterface(bytes4 interfaceId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceCall {
        #[allow(missing_docs)]
        pub interfaceId: alloy::sol_types::private::FixedBytes<4>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`supportsInterface(bytes4)`](supportsInterfaceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceCall) -> Self {
                    (value.interfaceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { interfaceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportsInterfaceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supportsInterface(bytes4)";
            const SELECTOR: [u8; 4] = [1u8, 255u8, 201u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.interfaceId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: supportsInterfaceReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: supportsInterfaceReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `unpause()` and selector `0x3f4ba83a`.
```solidity
function unpause() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall;
    ///Container type for the return parameters of the [`unpause()`](unpauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl unpauseReturn {
            fn _tokenize(
                &self,
            ) -> <unpauseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unpauseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpause()";
            const SELECTOR: [u8; 4] = [63u8, 75u8, 168u8, 58u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                unpauseReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`OnRampEscrow`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum OnRampEscrowCalls {
        #[allow(missing_docs)]
        DEFAULT_ADMIN_ROLE(DEFAULT_ADMIN_ROLECall),
        #[allow(missing_docs)]
        LP_ROLE(LP_ROLECall),
        #[allow(missing_docs)]
        RELAYER_ROLE(RELAYER_ROLECall),
        #[allow(missing_docs)]
        cancelOrder(cancelOrderCall),
        #[allow(missing_docs)]
        confirmFiatSent(confirmFiatSentCall),
        #[allow(missing_docs)]
        createOnRampOrder(createOnRampOrderCall),
        #[allow(missing_docs)]
        disputeOrder(disputeOrderCall),
        #[allow(missing_docs)]
        getOrder(getOrderCall),
        #[allow(missing_docs)]
        getRoleAdmin(getRoleAdminCall),
        #[allow(missing_docs)]
        grantRole(grantRoleCall),
        #[allow(missing_docs)]
        hasRole(hasRoleCall),
        #[allow(missing_docs)]
        lockDeadline(lockDeadlineCall),
        #[allow(missing_docs)]
        lockFunds(lockFundsCall),
        #[allow(missing_docs)]
        lockFundsByRelayer(lockFundsByRelayerCall),
        #[allow(missing_docs)]
        lpCapByToken(lpCapByTokenCall),
        #[allow(missing_docs)]
        lpOutstandingByToken(lpOutstandingByTokenCall),
        #[allow(missing_docs)]
        orders(ordersCall),
        #[allow(missing_docs)]
        pause(pauseCall),
        #[allow(missing_docs)]
        paused(pausedCall),
        #[allow(missing_docs)]
        paymentDeadline(paymentDeadlineCall),
        #[allow(missing_docs)]
        reclaimLockedFunds(reclaimLockedFundsCall),
        #[allow(missing_docs)]
        releaseFunds(releaseFundsCall),
        #[allow(missing_docs)]
        renounceRole(renounceRoleCall),
        #[allow(missing_docs)]
        resolveDispute(resolveDisputeCall),
        #[allow(missing_docs)]
        revokeRole(revokeRoleCall),
        #[allow(missing_docs)]
        setLockDeadline(setLockDeadlineCall),
        #[allow(missing_docs)]
        setLpCap(setLpCapCall),
        #[allow(missing_docs)]
        setPaymentDeadline(setPaymentDeadlineCall),
        #[allow(missing_docs)]
        supportsInterface(supportsInterfaceCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
    }
    impl OnRampEscrowCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 255u8, 201u8, 167u8],
            [36u8, 138u8, 156u8, 163u8],
            [47u8, 47u8, 241u8, 93u8],
            [48u8, 93u8, 245u8, 111u8],
            [54u8, 86u8, 138u8, 190u8],
            [62u8, 78u8, 55u8, 146u8],
            [62u8, 136u8, 224u8, 182u8],
            [63u8, 75u8, 168u8, 58u8],
            [67u8, 160u8, 227u8, 230u8],
            [87u8, 120u8, 71u8, 42u8],
            [92u8, 151u8, 90u8, 187u8],
            [109u8, 146u8, 159u8, 35u8],
            [116u8, 137u8, 236u8, 35u8],
            [132u8, 86u8, 203u8, 89u8],
            [132u8, 165u8, 206u8, 83u8],
            [137u8, 106u8, 47u8, 244u8],
            [140u8, 253u8, 145u8, 158u8],
            [143u8, 164u8, 166u8, 65u8],
            [145u8, 209u8, 72u8, 84u8],
            [146u8, 109u8, 125u8, 127u8],
            [156u8, 63u8, 30u8, 144u8],
            [156u8, 190u8, 182u8, 193u8],
            [158u8, 73u8, 247u8, 228u8],
            [162u8, 23u8, 253u8, 223u8],
            [201u8, 46u8, 224u8, 67u8],
            [206u8, 155u8, 193u8, 94u8],
            [213u8, 71u8, 116u8, 31u8],
            [225u8, 211u8, 148u8, 80u8],
            [250u8, 77u8, 12u8, 60u8],
            [252u8, 70u8, 216u8, 184u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(supportsInterface),
            ::core::stringify!(getRoleAdmin),
            ::core::stringify!(grantRole),
            ::core::stringify!(createOnRampOrder),
            ::core::stringify!(renounceRole),
            ::core::stringify!(disputeOrder),
            ::core::stringify!(reclaimLockedFunds),
            ::core::stringify!(unpause),
            ::core::stringify!(resolveDispute),
            ::core::stringify!(getOrder),
            ::core::stringify!(paused),
            ::core::stringify!(paymentDeadline),
            ::core::stringify!(cancelOrder),
            ::core::stringify!(pause),
            ::core::stringify!(setLockDeadline),
            ::core::stringify!(setLpCap),
            ::core::stringify!(setPaymentDeadline),
            ::core::stringify!(confirmFiatSent),
            ::core::stringify!(hasRole),
            ::core::stringify!(RELAYER_ROLE),
            ::core::stringify!(orders),
            ::core::stringify!(lockFunds),
            ::core::stringify!(lockFundsByRelayer),
            ::core::stringify!(DEFAULT_ADMIN_ROLE),
            ::core::stringify!(releaseFunds),
            ::core::stringify!(lpOutstandingByToken),
            ::core::stringify!(revokeRole),
            ::core::stringify!(LP_ROLE),
            ::core::stringify!(lockDeadline),
            ::core::stringify!(lpCapByToken),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <supportsInterfaceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <grantRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <createOnRampOrderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <disputeOrderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <reclaimLockedFundsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <unpauseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resolveDisputeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getOrderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pausedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <paymentDeadlineCall as alloy_sol_types::SolCall>::SIGNATURE,
            <cancelOrderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pauseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setLockDeadlineCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setLpCapCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setPaymentDeadlineCall as alloy_sol_types::SolCall>::SIGNATURE,
            <confirmFiatSentCall as alloy_sol_types::SolCall>::SIGNATURE,
            <hasRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <RELAYER_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <ordersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <lockFundsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <lockFundsByRelayerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <releaseFundsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <lpOutstandingByTokenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <revokeRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <LP_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <lockDeadlineCall as alloy_sol_types::SolCall>::SIGNATURE,
            <lpCapByTokenCall as alloy_sol_types::SolCall>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for OnRampEscrowCalls {
        const NAME: &'static str = "OnRampEscrowCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 30usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DEFAULT_ADMIN_ROLE(_) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::LP_ROLE(_) => <LP_ROLECall as alloy_sol_types::SolCall>::SELECTOR,
                Self::RELAYER_ROLE(_) => {
                    <RELAYER_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cancelOrder(_) => {
                    <cancelOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::confirmFiatSent(_) => {
                    <confirmFiatSentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createOnRampOrder(_) => {
                    <createOnRampOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::disputeOrder(_) => {
                    <disputeOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOrder(_) => <getOrderCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getRoleAdmin(_) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::grantRole(_) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasRole(_) => <hasRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::lockDeadline(_) => {
                    <lockDeadlineCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lockFunds(_) => {
                    <lockFundsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lockFundsByRelayer(_) => {
                    <lockFundsByRelayerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lpCapByToken(_) => {
                    <lpCapByTokenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lpOutstandingByToken(_) => {
                    <lpOutstandingByTokenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::orders(_) => <ordersCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused(_) => <pausedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paymentDeadline(_) => {
                    <paymentDeadlineCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reclaimLockedFunds(_) => {
                    <reclaimLockedFundsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::releaseFunds(_) => {
                    <releaseFundsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceRole(_) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::resolveDispute(_) => {
                    <resolveDisputeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::revokeRole(_) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setLockDeadline(_) => {
                    <setLockDeadlineCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setLpCap(_) => <setLpCapCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setPaymentDeadline(_) => {
                    <setPaymentDeadlineCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<OnRampEscrowCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn grantRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OnRampEscrowCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn createOnRampOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <createOnRampOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::createOnRampOrder)
                    }
                    createOnRampOrder
                },
                {
                    fn renounceRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn disputeOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <disputeOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::disputeOrder)
                    }
                    disputeOrder
                },
                {
                    fn reclaimLockedFunds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <reclaimLockedFundsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::reclaimLockedFunds)
                    }
                    reclaimLockedFunds
                },
                {
                    fn unpause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OnRampEscrowCalls::unpause)
                    }
                    unpause
                },
                {
                    fn resolveDispute(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <resolveDisputeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::resolveDispute)
                    }
                    resolveDispute
                },
                {
                    fn getOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <getOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OnRampEscrowCalls::getOrder)
                    }
                    getOrder
                },
                {
                    fn paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OnRampEscrowCalls::paused)
                    }
                    paused
                },
                {
                    fn paymentDeadline(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <paymentDeadlineCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::paymentDeadline)
                    }
                    paymentDeadline
                },
                {
                    fn cancelOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <cancelOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::cancelOrder)
                    }
                    cancelOrder
                },
                {
                    fn pause(data: &[u8]) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OnRampEscrowCalls::pause)
                    }
                    pause
                },
                {
                    fn setLockDeadline(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <setLockDeadlineCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::setLockDeadline)
                    }
                    setLockDeadline
                },
                {
                    fn setLpCap(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <setLpCapCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OnRampEscrowCalls::setLpCap)
                    }
                    setLpCap
                },
                {
                    fn setPaymentDeadline(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <setPaymentDeadlineCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::setPaymentDeadline)
                    }
                    setPaymentDeadline
                },
                {
                    fn confirmFiatSent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <confirmFiatSentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::confirmFiatSent)
                    }
                    confirmFiatSent
                },
                {
                    fn hasRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OnRampEscrowCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn RELAYER_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <RELAYER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::RELAYER_ROLE)
                    }
                    RELAYER_ROLE
                },
                {
                    fn orders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <ordersCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OnRampEscrowCalls::orders)
                    }
                    orders
                },
                {
                    fn lockFunds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <lockFundsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OnRampEscrowCalls::lockFunds)
                    }
                    lockFunds
                },
                {
                    fn lockFundsByRelayer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <lockFundsByRelayerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::lockFundsByRelayer)
                    }
                    lockFundsByRelayer
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn releaseFunds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <releaseFundsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::releaseFunds)
                    }
                    releaseFunds
                },
                {
                    fn lpOutstandingByToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <lpOutstandingByTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::lpOutstandingByToken)
                    }
                    lpOutstandingByToken
                },
                {
                    fn revokeRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn LP_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <LP_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OnRampEscrowCalls::LP_ROLE)
                    }
                    LP_ROLE
                },
                {
                    fn lockDeadline(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <lockDeadlineCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::lockDeadline)
                    }
                    lockDeadline
                },
                {
                    fn lpCapByToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <lpCapByTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowCalls::lpCapByToken)
                    }
                    lpCapByToken
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<OnRampEscrowCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn grantRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn createOnRampOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <createOnRampOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::createOnRampOrder)
                    }
                    createOnRampOrder
                },
                {
                    fn renounceRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn disputeOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <disputeOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::disputeOrder)
                    }
                    disputeOrder
                },
                {
                    fn reclaimLockedFunds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <reclaimLockedFundsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::reclaimLockedFunds)
                    }
                    reclaimLockedFunds
                },
                {
                    fn unpause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::unpause)
                    }
                    unpause
                },
                {
                    fn resolveDispute(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <resolveDisputeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::resolveDispute)
                    }
                    resolveDispute
                },
                {
                    fn getOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <getOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::getOrder)
                    }
                    getOrder
                },
                {
                    fn paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::paused)
                    }
                    paused
                },
                {
                    fn paymentDeadline(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <paymentDeadlineCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::paymentDeadline)
                    }
                    paymentDeadline
                },
                {
                    fn cancelOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <cancelOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::cancelOrder)
                    }
                    cancelOrder
                },
                {
                    fn pause(data: &[u8]) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::pause)
                    }
                    pause
                },
                {
                    fn setLockDeadline(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <setLockDeadlineCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::setLockDeadline)
                    }
                    setLockDeadline
                },
                {
                    fn setLpCap(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <setLpCapCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::setLpCap)
                    }
                    setLpCap
                },
                {
                    fn setPaymentDeadline(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <setPaymentDeadlineCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::setPaymentDeadline)
                    }
                    setPaymentDeadline
                },
                {
                    fn confirmFiatSent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <confirmFiatSentCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::confirmFiatSent)
                    }
                    confirmFiatSent
                },
                {
                    fn hasRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn RELAYER_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <RELAYER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::RELAYER_ROLE)
                    }
                    RELAYER_ROLE
                },
                {
                    fn orders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <ordersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::orders)
                    }
                    orders
                },
                {
                    fn lockFunds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <lockFundsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::lockFunds)
                    }
                    lockFunds
                },
                {
                    fn lockFundsByRelayer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <lockFundsByRelayerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::lockFundsByRelayer)
                    }
                    lockFundsByRelayer
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn releaseFunds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <releaseFundsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::releaseFunds)
                    }
                    releaseFunds
                },
                {
                    fn lpOutstandingByToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <lpOutstandingByTokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::lpOutstandingByToken)
                    }
                    lpOutstandingByToken
                },
                {
                    fn revokeRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn LP_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <LP_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::LP_ROLE)
                    }
                    LP_ROLE
                },
                {
                    fn lockDeadline(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <lockDeadlineCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::lockDeadline)
                    }
                    lockDeadline
                },
                {
                    fn lpCapByToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowCalls> {
                        <lpCapByTokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowCalls::lpCapByToken)
                    }
                    lpCapByToken
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LP_ROLE(inner) => {
                    <LP_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::RELAYER_ROLE(inner) => {
                    <RELAYER_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cancelOrder(inner) => {
                    <cancelOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::confirmFiatSent(inner) => {
                    <confirmFiatSentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createOnRampOrder(inner) => {
                    <createOnRampOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::disputeOrder(inner) => {
                    <disputeOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOrder(inner) => {
                    <getOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::lockDeadline(inner) => {
                    <lockDeadlineCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lockFunds(inner) => {
                    <lockFundsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::lockFundsByRelayer(inner) => {
                    <lockFundsByRelayerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lpCapByToken(inner) => {
                    <lpCapByTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lpOutstandingByToken(inner) => {
                    <lpOutstandingByTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::orders(inner) => {
                    <ordersCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paymentDeadline(inner) => {
                    <paymentDeadlineCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::reclaimLockedFunds(inner) => {
                    <reclaimLockedFundsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::releaseFunds(inner) => {
                    <releaseFundsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::resolveDispute(inner) => {
                    <resolveDisputeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setLockDeadline(inner) => {
                    <setLockDeadlineCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setLpCap(inner) => {
                    <setLpCapCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setPaymentDeadline(inner) => {
                    <setPaymentDeadlineCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LP_ROLE(inner) => {
                    <LP_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::RELAYER_ROLE(inner) => {
                    <RELAYER_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cancelOrder(inner) => {
                    <cancelOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::confirmFiatSent(inner) => {
                    <confirmFiatSentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createOnRampOrder(inner) => {
                    <createOnRampOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::disputeOrder(inner) => {
                    <disputeOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOrder(inner) => {
                    <getOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::lockDeadline(inner) => {
                    <lockDeadlineCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lockFunds(inner) => {
                    <lockFundsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lockFundsByRelayer(inner) => {
                    <lockFundsByRelayerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lpCapByToken(inner) => {
                    <lpCapByTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lpOutstandingByToken(inner) => {
                    <lpOutstandingByTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::orders(inner) => {
                    <ordersCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paymentDeadline(inner) => {
                    <paymentDeadlineCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::reclaimLockedFunds(inner) => {
                    <reclaimLockedFundsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::releaseFunds(inner) => {
                    <releaseFundsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::resolveDispute(inner) => {
                    <resolveDisputeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setLockDeadline(inner) => {
                    <setLockDeadlineCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setLpCap(inner) => {
                    <setLpCapCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setPaymentDeadline(inner) => {
                    <setPaymentDeadlineCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`OnRampEscrow`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum OnRampEscrowErrors {
        #[allow(missing_docs)]
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        #[allow(missing_docs)]
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        #[allow(missing_docs)]
        EnforcedPause(EnforcedPause),
        #[allow(missing_docs)]
        ExpectedPause(ExpectedPause),
        #[allow(missing_docs)]
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
    }
    impl OnRampEscrowErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [62u8, 229u8, 174u8, 181u8],
            [102u8, 151u8, 178u8, 50u8],
            [141u8, 252u8, 32u8, 43u8],
            [217u8, 60u8, 6u8, 101u8],
            [226u8, 81u8, 125u8, 63u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(ReentrancyGuardReentrantCall),
            ::core::stringify!(AccessControlBadConfirmation),
            ::core::stringify!(ExpectedPause),
            ::core::stringify!(EnforcedPause),
            ::core::stringify!(AccessControlUnauthorizedAccount),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::SIGNATURE,
            <AccessControlBadConfirmation as alloy_sol_types::SolError>::SIGNATURE,
            <ExpectedPause as alloy_sol_types::SolError>::SIGNATURE,
            <EnforcedPause as alloy_sol_types::SolError>::SIGNATURE,
            <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for OnRampEscrowErrors {
        const NAME: &'static str = "OnRampEscrowErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 5usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AccessControlBadConfirmation(_) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AccessControlUnauthorizedAccount(_) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EnforcedPause(_) => {
                    <EnforcedPause as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExpectedPause(_) => {
                    <ExpectedPause as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ReentrancyGuardReentrantCall(_) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<OnRampEscrowErrors>] = &[
                {
                    fn ReentrancyGuardReentrantCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowErrors> {
                        <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowErrors::ReentrancyGuardReentrantCall)
                    }
                    ReentrancyGuardReentrantCall
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OnRampEscrowErrors::AccessControlUnauthorizedAccount)
                    }
                    AccessControlUnauthorizedAccount
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<OnRampEscrowErrors>] = &[
                {
                    fn ReentrancyGuardReentrantCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowErrors> {
                        <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowErrors::ReentrancyGuardReentrantCall)
                    }
                    ReentrancyGuardReentrantCall
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OnRampEscrowErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OnRampEscrowErrors::AccessControlUnauthorizedAccount)
                    }
                    AccessControlUnauthorizedAccount
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AccessControlBadConfirmation(inner) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AccessControlUnauthorizedAccount(inner) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EnforcedPause(inner) => {
                    <EnforcedPause as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ExpectedPause(inner) => {
                    <ExpectedPause as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ReentrancyGuardReentrantCall(inner) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AccessControlBadConfirmation(inner) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AccessControlUnauthorizedAccount(inner) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EnforcedPause(inner) => {
                    <EnforcedPause as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ExpectedPause(inner) => {
                    <ExpectedPause as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ReentrancyGuardReentrantCall(inner) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`OnRampEscrow`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum OnRampEscrowEvents {
        #[allow(missing_docs)]
        DisputeResolved(DisputeResolved),
        #[allow(missing_docs)]
        FiatSent(FiatSent),
        #[allow(missing_docs)]
        FundsLocked(FundsLocked),
        #[allow(missing_docs)]
        LpCapSet(LpCapSet),
        #[allow(missing_docs)]
        LpOutstandingUpdated(LpOutstandingUpdated),
        #[allow(missing_docs)]
        OrderCancelled(OrderCancelled),
        #[allow(missing_docs)]
        OrderCompleted(OrderCompleted),
        #[allow(missing_docs)]
        OrderCreated(OrderCreated),
        #[allow(missing_docs)]
        OrderDisputed(OrderDisputed),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        RoleAdminChanged(RoleAdminChanged),
        #[allow(missing_docs)]
        RoleGranted(RoleGranted),
        #[allow(missing_docs)]
        RoleRevoked(RoleRevoked),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
    }
    impl OnRampEscrowEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                8u8, 17u8, 131u8, 25u8, 85u8, 126u8, 153u8, 152u8, 88u8, 130u8, 71u8,
                212u8, 104u8, 55u8, 174u8, 165u8, 199u8, 125u8, 228u8, 137u8, 103u8,
                147u8, 99u8, 67u8, 182u8, 201u8, 35u8, 251u8, 31u8, 120u8, 19u8, 227u8,
            ],
            [
                21u8, 20u8, 66u8, 23u8, 64u8, 80u8, 100u8, 99u8, 29u8, 233u8, 81u8,
                117u8, 33u8, 17u8, 51u8, 79u8, 109u8, 157u8, 180u8, 190u8, 108u8, 255u8,
                244u8, 83u8, 70u8, 247u8, 6u8, 142u8, 168u8, 87u8, 252u8, 173u8,
            ],
            [
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8,
                236u8, 121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8,
                64u8, 48u8, 69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
            ],
            [
                56u8, 108u8, 79u8, 194u8, 254u8, 81u8, 225u8, 125u8, 208u8, 247u8, 181u8,
                187u8, 109u8, 56u8, 252u8, 163u8, 166u8, 82u8, 212u8, 230u8, 251u8,
                207u8, 161u8, 169u8, 50u8, 131u8, 3u8, 74u8, 205u8, 197u8, 132u8, 58u8,
            ],
            [
                65u8, 246u8, 127u8, 227u8, 246u8, 127u8, 29u8, 206u8, 104u8, 118u8,
                148u8, 134u8, 53u8, 44u8, 59u8, 199u8, 197u8, 197u8, 172u8, 218u8, 141u8,
                60u8, 14u8, 102u8, 40u8, 249u8, 24u8, 194u8, 85u8, 227u8, 74u8, 11u8,
            ],
            [
                93u8, 185u8, 238u8, 10u8, 73u8, 91u8, 242u8, 230u8, 255u8, 156u8, 145u8,
                167u8, 131u8, 76u8, 27u8, 164u8, 253u8, 210u8, 68u8, 165u8, 232u8, 170u8,
                78u8, 83u8, 123u8, 211u8, 138u8, 234u8, 228u8, 176u8, 115u8, 170u8,
            ],
            [
                98u8, 231u8, 140u8, 234u8, 1u8, 190u8, 227u8, 32u8, 205u8, 78u8, 66u8,
                2u8, 112u8, 181u8, 234u8, 116u8, 0u8, 13u8, 17u8, 176u8, 201u8, 247u8,
                71u8, 84u8, 235u8, 219u8, 252u8, 84u8, 75u8, 5u8, 162u8, 88u8,
            ],
            [
                100u8, 209u8, 159u8, 237u8, 206u8, 147u8, 81u8, 215u8, 193u8, 148u8,
                152u8, 81u8, 99u8, 230u8, 180u8, 0u8, 85u8, 92u8, 24u8, 107u8, 167u8,
                133u8, 136u8, 18u8, 57u8, 21u8, 100u8, 19u8, 148u8, 138u8, 139u8, 202u8,
            ],
            [
                152u8, 9u8, 223u8, 108u8, 157u8, 99u8, 100u8, 42u8, 111u8, 30u8, 51u8,
                166u8, 152u8, 212u8, 21u8, 186u8, 45u8, 136u8, 42u8, 64u8, 113u8, 137u8,
                77u8, 167u8, 72u8, 171u8, 84u8, 130u8, 151u8, 92u8, 142u8, 99u8,
            ],
            [
                161u8, 58u8, 157u8, 119u8, 166u8, 251u8, 214u8, 144u8, 184u8, 0u8, 11u8,
                20u8, 204u8, 143u8, 75u8, 167u8, 96u8, 43u8, 50u8, 153u8, 55u8, 99u8,
                142u8, 112u8, 137u8, 55u8, 253u8, 159u8, 170u8, 96u8, 105u8, 205u8,
            ],
            [
                189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8,
                81u8, 66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8,
                71u8, 92u8, 58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
            ],
            [
                216u8, 226u8, 94u8, 88u8, 234u8, 106u8, 197u8, 225u8, 223u8, 243u8, 28u8,
                200u8, 195u8, 65u8, 146u8, 44u8, 77u8, 28u8, 205u8, 66u8, 124u8, 132u8,
                224u8, 191u8, 51u8, 156u8, 81u8, 11u8, 75u8, 86u8, 16u8, 98u8,
            ],
            [
                242u8, 178u8, 118u8, 94u8, 223u8, 130u8, 252u8, 7u8, 226u8, 108u8, 234u8,
                19u8, 80u8, 174u8, 204u8, 132u8, 59u8, 46u8, 107u8, 188u8, 149u8, 199u8,
                181u8, 32u8, 118u8, 214u8, 92u8, 151u8, 196u8, 48u8, 17u8, 85u8,
            ],
            [
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8,
                103u8, 11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8,
                253u8, 100u8, 235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(LpOutstandingUpdated),
            ::core::stringify!(FundsLocked),
            ::core::stringify!(RoleGranted),
            ::core::stringify!(OrderCancelled),
            ::core::stringify!(DisputeResolved),
            ::core::stringify!(Unpaused),
            ::core::stringify!(Paused),
            ::core::stringify!(LpCapSet),
            ::core::stringify!(OrderDisputed),
            ::core::stringify!(OrderCompleted),
            ::core::stringify!(RoleAdminChanged),
            ::core::stringify!(FiatSent),
            ::core::stringify!(OrderCreated),
            ::core::stringify!(RoleRevoked),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <LpOutstandingUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <FundsLocked as alloy_sol_types::SolEvent>::SIGNATURE,
            <RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE,
            <OrderCancelled as alloy_sol_types::SolEvent>::SIGNATURE,
            <DisputeResolved as alloy_sol_types::SolEvent>::SIGNATURE,
            <Unpaused as alloy_sol_types::SolEvent>::SIGNATURE,
            <Paused as alloy_sol_types::SolEvent>::SIGNATURE,
            <LpCapSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <OrderDisputed as alloy_sol_types::SolEvent>::SIGNATURE,
            <OrderCompleted as alloy_sol_types::SolEvent>::SIGNATURE,
            <RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE,
            <FiatSent as alloy_sol_types::SolEvent>::SIGNATURE,
            <OrderCreated as alloy_sol_types::SolEvent>::SIGNATURE,
            <RoleRevoked as alloy_sol_types::SolEvent>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for OnRampEscrowEvents {
        const NAME: &'static str = "OnRampEscrowEvents";
        const COUNT: usize = 14usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<DisputeResolved as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <DisputeResolved as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::DisputeResolved)
                }
                Some(<FiatSent as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <FiatSent as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::FiatSent)
                }
                Some(<FundsLocked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <FundsLocked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::FundsLocked)
                }
                Some(<LpCapSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <LpCapSet as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::LpCapSet)
                }
                Some(
                    <LpOutstandingUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <LpOutstandingUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::LpOutstandingUpdated)
                }
                Some(<OrderCancelled as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OrderCancelled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OrderCancelled)
                }
                Some(<OrderCompleted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OrderCompleted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OrderCompleted)
                }
                Some(<OrderCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OrderCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OrderCreated)
                }
                Some(<OrderDisputed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OrderDisputed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OrderDisputed)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Paused)
                }
                Some(<RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleAdminChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleAdminChanged)
                }
                Some(<RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleGranted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleGranted)
                }
                Some(<RoleRevoked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleRevoked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleRevoked)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Unpaused)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for OnRampEscrowEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DisputeResolved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::FiatSent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::FundsLocked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::LpCapSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::LpOutstandingUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OrderCancelled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OrderCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OrderCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OrderDisputed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DisputeResolved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::FiatSent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::FundsLocked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::LpCapSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::LpOutstandingUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OrderCancelled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OrderCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OrderCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OrderDisputed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`OnRampEscrow`](self) contract instance.

See the [wrapper's documentation](`OnRampEscrowInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> OnRampEscrowInstance<P, N> {
        OnRampEscrowInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<OnRampEscrowInstance<P, N>>,
    > {
        OnRampEscrowInstance::<P, N>::deploy(__provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
        OnRampEscrowInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`OnRampEscrow`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OnRampEscrow`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OnRampEscrowInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for OnRampEscrowInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OnRampEscrowInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > OnRampEscrowInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`OnRampEscrow`](self) contract instance.

See the [wrapper's documentation](`OnRampEscrowInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            __provider: P,
        ) -> alloy_contract::Result<OnRampEscrowInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> OnRampEscrowInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OnRampEscrowInstance<P, N> {
            OnRampEscrowInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > OnRampEscrowInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`DEFAULT_ADMIN_ROLE`] function.
        pub fn DEFAULT_ADMIN_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, DEFAULT_ADMIN_ROLECall, N> {
            self.call_builder(&DEFAULT_ADMIN_ROLECall)
        }
        ///Creates a new call builder for the [`LP_ROLE`] function.
        pub fn LP_ROLE(&self) -> alloy_contract::SolCallBuilder<&P, LP_ROLECall, N> {
            self.call_builder(&LP_ROLECall)
        }
        ///Creates a new call builder for the [`RELAYER_ROLE`] function.
        pub fn RELAYER_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, RELAYER_ROLECall, N> {
            self.call_builder(&RELAYER_ROLECall)
        }
        ///Creates a new call builder for the [`cancelOrder`] function.
        pub fn cancelOrder(
            &self,
            orderId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, cancelOrderCall, N> {
            self.call_builder(&cancelOrderCall { orderId })
        }
        ///Creates a new call builder for the [`confirmFiatSent`] function.
        pub fn confirmFiatSent(
            &self,
            orderId: alloy::sol_types::private::FixedBytes<32>,
            proofHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, confirmFiatSentCall, N> {
            self.call_builder(
                &confirmFiatSentCall {
                    orderId,
                    proofHash,
                },
            )
        }
        ///Creates a new call builder for the [`createOnRampOrder`] function.
        pub fn createOnRampOrder(
            &self,
            token: alloy::sol_types::private::Address,
            tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
            fiatAmount: alloy::sol_types::private::primitives::aliases::U256,
            fiatCurrency: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<&P, createOnRampOrderCall, N> {
            self.call_builder(
                &createOnRampOrderCall {
                    token,
                    tokenAmount,
                    fiatAmount,
                    fiatCurrency,
                },
            )
        }
        ///Creates a new call builder for the [`disputeOrder`] function.
        pub fn disputeOrder(
            &self,
            orderId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, disputeOrderCall, N> {
            self.call_builder(&disputeOrderCall { orderId })
        }
        ///Creates a new call builder for the [`getOrder`] function.
        pub fn getOrder(
            &self,
            orderId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getOrderCall, N> {
            self.call_builder(&getOrderCall { orderId })
        }
        ///Creates a new call builder for the [`getRoleAdmin`] function.
        pub fn getRoleAdmin(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getRoleAdminCall, N> {
            self.call_builder(&getRoleAdminCall { role })
        }
        ///Creates a new call builder for the [`grantRole`] function.
        pub fn grantRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, grantRoleCall, N> {
            self.call_builder(&grantRoleCall { role, account })
        }
        ///Creates a new call builder for the [`hasRole`] function.
        pub fn hasRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, hasRoleCall, N> {
            self.call_builder(&hasRoleCall { role, account })
        }
        ///Creates a new call builder for the [`lockDeadline`] function.
        pub fn lockDeadline(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, lockDeadlineCall, N> {
            self.call_builder(&lockDeadlineCall)
        }
        ///Creates a new call builder for the [`lockFunds`] function.
        pub fn lockFunds(
            &self,
            orderId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, lockFundsCall, N> {
            self.call_builder(&lockFundsCall { orderId })
        }
        ///Creates a new call builder for the [`lockFundsByRelayer`] function.
        pub fn lockFundsByRelayer(
            &self,
            orderId: alloy::sol_types::private::FixedBytes<32>,
            lpAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, lockFundsByRelayerCall, N> {
            self.call_builder(
                &lockFundsByRelayerCall {
                    orderId,
                    lpAddress,
                },
            )
        }
        ///Creates a new call builder for the [`lpCapByToken`] function.
        pub fn lpCapByToken(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, lpCapByTokenCall, N> {
            self.call_builder(&lpCapByTokenCall { _0, _1 })
        }
        ///Creates a new call builder for the [`lpOutstandingByToken`] function.
        pub fn lpOutstandingByToken(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, lpOutstandingByTokenCall, N> {
            self.call_builder(&lpOutstandingByTokenCall { _0, _1 })
        }
        ///Creates a new call builder for the [`orders`] function.
        pub fn orders(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, ordersCall, N> {
            self.call_builder(&ordersCall(_0))
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(&self) -> alloy_contract::SolCallBuilder<&P, pauseCall, N> {
            self.call_builder(&pauseCall)
        }
        ///Creates a new call builder for the [`paused`] function.
        pub fn paused(&self) -> alloy_contract::SolCallBuilder<&P, pausedCall, N> {
            self.call_builder(&pausedCall)
        }
        ///Creates a new call builder for the [`paymentDeadline`] function.
        pub fn paymentDeadline(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, paymentDeadlineCall, N> {
            self.call_builder(&paymentDeadlineCall)
        }
        ///Creates a new call builder for the [`reclaimLockedFunds`] function.
        pub fn reclaimLockedFunds(
            &self,
            orderId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, reclaimLockedFundsCall, N> {
            self.call_builder(&reclaimLockedFundsCall { orderId })
        }
        ///Creates a new call builder for the [`releaseFunds`] function.
        pub fn releaseFunds(
            &self,
            orderId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, releaseFundsCall, N> {
            self.call_builder(&releaseFundsCall { orderId })
        }
        ///Creates a new call builder for the [`renounceRole`] function.
        pub fn renounceRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            callerConfirmation: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, renounceRoleCall, N> {
            self.call_builder(
                &renounceRoleCall {
                    role,
                    callerConfirmation,
                },
            )
        }
        ///Creates a new call builder for the [`resolveDispute`] function.
        pub fn resolveDispute(
            &self,
            orderId: alloy::sol_types::private::FixedBytes<32>,
            releaseToBuyer: bool,
        ) -> alloy_contract::SolCallBuilder<&P, resolveDisputeCall, N> {
            self.call_builder(
                &resolveDisputeCall {
                    orderId,
                    releaseToBuyer,
                },
            )
        }
        ///Creates a new call builder for the [`revokeRole`] function.
        pub fn revokeRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, revokeRoleCall, N> {
            self.call_builder(&revokeRoleCall { role, account })
        }
        ///Creates a new call builder for the [`setLockDeadline`] function.
        pub fn setLockDeadline(
            &self,
            newDeadline: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, setLockDeadlineCall, N> {
            self.call_builder(&setLockDeadlineCall { newDeadline })
        }
        ///Creates a new call builder for the [`setLpCap`] function.
        pub fn setLpCap(
            &self,
            lp: alloy::sol_types::private::Address,
            token: alloy::sol_types::private::Address,
            cap: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, setLpCapCall, N> {
            self.call_builder(&setLpCapCall { lp, token, cap })
        }
        ///Creates a new call builder for the [`setPaymentDeadline`] function.
        pub fn setPaymentDeadline(
            &self,
            newDeadline: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, setPaymentDeadlineCall, N> {
            self.call_builder(
                &setPaymentDeadlineCall {
                    newDeadline,
                },
            )
        }
        ///Creates a new call builder for the [`supportsInterface`] function.
        pub fn supportsInterface(
            &self,
            interfaceId: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<&P, supportsInterfaceCall, N> {
            self.call_builder(
                &supportsInterfaceCall {
                    interfaceId,
                },
            )
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(&self) -> alloy_contract::SolCallBuilder<&P, unpauseCall, N> {
            self.call_builder(&unpauseCall)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > OnRampEscrowInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`DisputeResolved`] event.
        pub fn DisputeResolved_filter(
            &self,
        ) -> alloy_contract::Event<&P, DisputeResolved, N> {
            self.event_filter::<DisputeResolved>()
        }
        ///Creates a new event filter for the [`FiatSent`] event.
        pub fn FiatSent_filter(&self) -> alloy_contract::Event<&P, FiatSent, N> {
            self.event_filter::<FiatSent>()
        }
        ///Creates a new event filter for the [`FundsLocked`] event.
        pub fn FundsLocked_filter(&self) -> alloy_contract::Event<&P, FundsLocked, N> {
            self.event_filter::<FundsLocked>()
        }
        ///Creates a new event filter for the [`LpCapSet`] event.
        pub fn LpCapSet_filter(&self) -> alloy_contract::Event<&P, LpCapSet, N> {
            self.event_filter::<LpCapSet>()
        }
        ///Creates a new event filter for the [`LpOutstandingUpdated`] event.
        pub fn LpOutstandingUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, LpOutstandingUpdated, N> {
            self.event_filter::<LpOutstandingUpdated>()
        }
        ///Creates a new event filter for the [`OrderCancelled`] event.
        pub fn OrderCancelled_filter(
            &self,
        ) -> alloy_contract::Event<&P, OrderCancelled, N> {
            self.event_filter::<OrderCancelled>()
        }
        ///Creates a new event filter for the [`OrderCompleted`] event.
        pub fn OrderCompleted_filter(
            &self,
        ) -> alloy_contract::Event<&P, OrderCompleted, N> {
            self.event_filter::<OrderCompleted>()
        }
        ///Creates a new event filter for the [`OrderCreated`] event.
        pub fn OrderCreated_filter(&self) -> alloy_contract::Event<&P, OrderCreated, N> {
            self.event_filter::<OrderCreated>()
        }
        ///Creates a new event filter for the [`OrderDisputed`] event.
        pub fn OrderDisputed_filter(
            &self,
        ) -> alloy_contract::Event<&P, OrderDisputed, N> {
            self.event_filter::<OrderDisputed>()
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<&P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`RoleAdminChanged`] event.
        pub fn RoleAdminChanged_filter(
            &self,
        ) -> alloy_contract::Event<&P, RoleAdminChanged, N> {
            self.event_filter::<RoleAdminChanged>()
        }
        ///Creates a new event filter for the [`RoleGranted`] event.
        pub fn RoleGranted_filter(&self) -> alloy_contract::Event<&P, RoleGranted, N> {
            self.event_filter::<RoleGranted>()
        }
        ///Creates a new event filter for the [`RoleRevoked`] event.
        pub fn RoleRevoked_filter(&self) -> alloy_contract::Event<&P, RoleRevoked, N> {
            self.event_filter::<RoleRevoked>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<&P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
