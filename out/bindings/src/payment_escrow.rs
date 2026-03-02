///Module containing a contract's types and functions.
/**

```solidity
library IPaymentEscrow {
    type PaymentStatus is uint8;
    struct Payment { address sender; address token; uint256 amount; string fiatCurrency; uint256 fiatAmount; uint256 exchangeRate; bytes32 recipientHash; uint256 createdAt; uint256 deadline; PaymentStatus status; string bankReference; address operator; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IPaymentEscrow {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PaymentStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<PaymentStatus> for u8 {
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
        impl PaymentStatus {
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
        impl From<u8> for PaymentStatus {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<PaymentStatus> for u8 {
            fn from(value: PaymentStatus) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for PaymentStatus {
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
        impl alloy_sol_types::EventTopic for PaymentStatus {
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
struct Payment { address sender; address token; uint256 amount; string fiatCurrency; uint256 fiatAmount; uint256 exchangeRate; bytes32 recipientHash; uint256 createdAt; uint256 deadline; PaymentStatus status; string bankReference; address operator; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Payment {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fiatCurrency: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub fiatAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub exchangeRate: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub recipientHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub createdAt: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub status: <PaymentStatus as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub bankReference: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            PaymentStatus,
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::String,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            <PaymentStatus as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::String,
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
        impl ::core::convert::From<Payment> for UnderlyingRustTuple<'_> {
            fn from(value: Payment) -> Self {
                (
                    value.sender,
                    value.token,
                    value.amount,
                    value.fiatCurrency,
                    value.fiatAmount,
                    value.exchangeRate,
                    value.recipientHash,
                    value.createdAt,
                    value.deadline,
                    value.status,
                    value.bankReference,
                    value.operator,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Payment {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    sender: tuple.0,
                    token: tuple.1,
                    amount: tuple.2,
                    fiatCurrency: tuple.3,
                    fiatAmount: tuple.4,
                    exchangeRate: tuple.5,
                    recipientHash: tuple.6,
                    createdAt: tuple.7,
                    deadline: tuple.8,
                    status: tuple.9,
                    bankReference: tuple.10,
                    operator: tuple.11,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Payment {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Payment {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.fiatCurrency,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fiatAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.exchangeRate),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.recipientHash),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.createdAt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                    <PaymentStatus as alloy_sol_types::SolType>::tokenize(&self.status),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.bankReference,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
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
        impl alloy_sol_types::SolType for Payment {
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
        impl alloy_sol_types::SolStruct for Payment {
            const NAME: &'static str = "Payment";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Payment(address sender,address token,uint256 amount,string fiatCurrency,uint256 fiatAmount,uint256 exchangeRate,bytes32 recipientHash,uint256 createdAt,uint256 deadline,uint8 status,string bankReference,address operator)",
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
                            &self.sender,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.fiatCurrency,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.fiatAmount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.exchangeRate)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.recipientHash)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.createdAt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.deadline)
                        .0,
                    <PaymentStatus as alloy_sol_types::SolType>::eip712_data_word(
                            &self.status,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.bankReference,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operator,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Payment {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sender,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.fiatCurrency,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.fiatAmount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.exchangeRate,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.recipientHash,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.createdAt,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.deadline,
                    )
                    + <PaymentStatus as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.status,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.bankReference,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
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
                    &rust.sender,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.exchangeRate,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.recipientHash,
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
                    &rust.deadline,
                    out,
                );
                <PaymentStatus as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.status,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.bankReference,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operator,
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
    /**Creates a new wrapper around an on-chain [`IPaymentEscrow`](self) contract instance.

See the [wrapper's documentation](`IPaymentEscrowInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> IPaymentEscrowInstance<P, N> {
        IPaymentEscrowInstance::<P, N>::new(address, __provider)
    }
    /**A [`IPaymentEscrow`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IPaymentEscrow`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IPaymentEscrowInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IPaymentEscrowInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IPaymentEscrowInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IPaymentEscrowInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IPaymentEscrow`](self) contract instance.

See the [wrapper's documentation](`IPaymentEscrowInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> IPaymentEscrowInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IPaymentEscrowInstance<P, N> {
            IPaymentEscrowInstance {
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
    > IPaymentEscrowInstance<P, N> {
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
    > IPaymentEscrowInstance<P, N> {
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
library IPaymentEscrow {
    type PaymentStatus is uint8;
    struct Payment {
        address sender;
        address token;
        uint256 amount;
        string fiatCurrency;
        uint256 fiatAmount;
        uint256 exchangeRate;
        bytes32 recipientHash;
        uint256 createdAt;
        uint256 deadline;
        PaymentStatus status;
        string bankReference;
        address operator;
    }
}

interface PaymentEscrow {
    error AccessControlBadConfirmation();
    error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error EnforcedPause();
    error ExpectedPause();
    error ReentrancyGuardReentrantCall();

    event Paused(address account);
    event PaymentCompleted(bytes32 indexed paymentId, bytes32 proofHash);
    event PaymentCreated(bytes32 indexed paymentId, address indexed sender, address token, uint256 amount, string fiatCurrency, uint256 fiatAmount);
    event PaymentProcessing(bytes32 indexed paymentId, address indexed operator, string bankReference);
    event PaymentRefunded(bytes32 indexed paymentId, string reason);
    event PlatformFeeUpdated(uint256 newFeePercent);
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    event SupportedTokenAdded(address indexed token);
    event SupportedTokenRemoved(address indexed token);
    event Unpaused(address account);

    constructor(address _feeCollector, address _lpRegistryAddress, address _oracleWallet, address _permissionSlipSigner);

    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    function PAYMENT_DEADLINE() external view returns (uint256);
    function addSupportedToken(address token) external;
    function claimPayment(bytes32 paymentId, bytes memory permissionSlip) external;
    function claimRefund(bytes32 paymentId) external;
    function confirmSettlement(bytes32 paymentId) external;
    function createPayment(address token, uint256 amount, uint256 fiatAmount, string memory fiatCurrency, bytes32 recipientHash) external returns (bytes32 paymentId);
    function feeCollector() external view returns (address);
    function getPayment(bytes32 paymentId) external view returns (IPaymentEscrow.Payment memory);
    function getRoleAdmin(bytes32 role) external view returns (bytes32);
    function grantRole(bytes32 role, address account) external;
    function hasRole(bytes32 role, address account) external view returns (bool);
    function lpRegistry() external view returns (address);
    function oracleWallet() external view returns (address);
    function pause() external;
    function paused() external view returns (bool);
    function payments(bytes32) external view returns (address sender, address token, uint256 amount, string memory fiatCurrency, uint256 fiatAmount, uint256 exchangeRate, bytes32 recipientHash, uint256 createdAt, uint256 deadline, IPaymentEscrow.PaymentStatus status, string memory bankReference, address operator);
    function permissionSlipSigner() external view returns (address);
    function platformFeePercent() external view returns (uint256);
    function removeSupportedToken(address token) external;
    function renounceRole(bytes32 role, address callerConfirmation) external;
    function revokeRole(bytes32 role, address account) external;
    function setOracleWallet(address _newOracle) external;
    function setPermissionSlipSigner(address _newSigner) external;
    function setPlatformFee(uint256 newFee) external;
    function supportedTokens(address) external view returns (bool);
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function unpause() external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_feeCollector",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_lpRegistryAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_oracleWallet",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_permissionSlipSigner",
        "type": "address",
        "internalType": "address"
      }
    ],
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
    "name": "PAYMENT_DEADLINE",
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
    "name": "addSupportedToken",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "claimPayment",
    "inputs": [
      {
        "name": "paymentId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "permissionSlip",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "claimRefund",
    "inputs": [
      {
        "name": "paymentId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "confirmSettlement",
    "inputs": [
      {
        "name": "paymentId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createPayment",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
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
      },
      {
        "name": "recipientHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "paymentId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "feeCollector",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPayment",
    "inputs": [
      {
        "name": "paymentId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IPaymentEscrow.Payment",
        "components": [
          {
            "name": "sender",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "token",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "amount",
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
            "name": "exchangeRate",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "recipientHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "createdAt",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "deadline",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum IPaymentEscrow.PaymentStatus"
          },
          {
            "name": "bankReference",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "operator",
            "type": "address",
            "internalType": "address"
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
    "name": "lpRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ILPRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "oracleWallet",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
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
    "name": "payments",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
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
        "name": "exchangeRate",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "recipientHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "createdAt",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "status",
        "type": "uint8",
        "internalType": "enum IPaymentEscrow.PaymentStatus"
      },
      {
        "name": "bankReference",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "permissionSlipSigner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "platformFeePercent",
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
    "name": "removeSupportedToken",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
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
    "name": "setOracleWallet",
    "inputs": [
      {
        "name": "_newOracle",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setPermissionSlipSigner",
    "inputs": [
      {
        "name": "_newSigner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setPlatformFee",
    "inputs": [
      {
        "name": "newFee",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "supportedTokens",
    "inputs": [
      {
        "name": "",
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
    "name": "PaymentCompleted",
    "inputs": [
      {
        "name": "paymentId",
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
    "name": "PaymentCreated",
    "inputs": [
      {
        "name": "paymentId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "sender",
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
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "fiatCurrency",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "fiatAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PaymentProcessing",
    "inputs": [
      {
        "name": "paymentId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "bankReference",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PaymentRefunded",
    "inputs": [
      {
        "name": "paymentId",
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
    "name": "PlatformFeeUpdated",
    "inputs": [
      {
        "name": "newFeePercent",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "SupportedTokenAdded",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SupportedTokenRemoved",
    "inputs": [
      {
        "name": "token",
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
    "name": "ECDSAInvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureLength",
    "inputs": [
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureS",
    "inputs": [
      {
        "name": "s",
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
pub mod PaymentEscrow {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60806040526032600755348015610014575f5ffd5b506040516144623803806144628339818101604052810190610036919061051e565b600161005461004961032e60201b60201c565b61035760201b60201c565b5f01819055505f73ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff16036100c8576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016100bf906105dc565b60405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff1603610136576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161012d906105dc565b60405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16036101a4576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161019b906105dc565b60405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603610212576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610209906105dc565b60405180910390fd5b6102245f5f1b3361036060201b60201c565b508360085f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508260025f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508160035f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508060045f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550505050506105fa565b5f7f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005f1b905090565b5f819050919050565b5f610371838361045560201b60201c565b61044b576001805f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506103e86104b960201b60201c565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a46001905061044f565b5f90505b92915050565b5f60015f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f33905090565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6104ed826104c4565b9050919050565b6104fd816104e3565b8114610507575f5ffd5b50565b5f81519050610518816104f4565b92915050565b5f5f5f5f60808587031215610536576105356104c0565b5b5f6105438782880161050a565b94505060206105548782880161050a565b93505060406105658782880161050a565b92505060606105768782880161050a565b91505092959194509250565b5f82825260208201905092915050565b7f5a45524f5f4144445245535300000000000000000000000000000000000000005f82015250565b5f6105c6600c83610582565b91506105d182610592565b602082019050919050565b5f6020820190508181035f8301526105f3816105ba565b9050919050565b613e5b806106075f395ff3fe608060405234801561000f575f5ffd5b50600436106101c2575f3560e01c80636d69fcaf116100f757806391d1485411610095578063c415b95c1161006f578063c415b95c146104d1578063d547741f146104ef578063e66eefc81461050b578063f2e9e4181461053b576101c2565b806391d148541461046557806397da034a14610495578063a217fddf146104b3576101c2565b806376319190116100d157806376319190146104035780637994d1a41461041f5780638456cb591461043d5780638c639a8514610447576101c2565b80636d69fcaf146103ad57806371de2ffc146103c957806372f3e8e1146103e5576101c2565b80632f2ff15d116101645780634dd0301f1161013e5780634dd0301f14610327578063550ebe28146103435780635c975abb1461035f57806368c4ac261461037d576101c2565b80632f2ff15d146102e557806336568abe146103015780633f4ba83a1461031d576101c2565b806312e8e2c3116101a057806312e8e2c31461024d5780631e0a505d14610269578063248a9ca3146102995780632bd735ab146102c9576101c2565b806301e8a6bb146101c657806301ffc9a7146101e25780630716326d14610212575b5f5ffd5b6101e060048036038101906101db919061284f565b610559565b005b6101fc60048036038101906101f791906128cf565b610617565b6040516102099190612914565b60405180910390f35b61022c60048036038101906102279190612960565b610690565b6040516102449c9b9a99989796959493929190612aa4565b60405180910390f35b61026760048036038101906102629190612b94565b610861565b005b610283600480360381019061027e9190612c20565b6108f3565b6040516102909190612cb6565b60405180910390f35b6102b360048036038101906102ae9190612960565b610ebb565b6040516102c09190612cb6565b60405180910390f35b6102e360048036038101906102de9190612960565b610ed8565b005b6102ff60048036038101906102fa9190612ccf565b611201565b005b61031b60048036038101906103169190612ccf565b611223565b005b61032561129e565b005b610341600480360381019061033c9190612d62565b6112b5565b005b61035d6004803603810190610358919061284f565b61163d565b005b6103676116fb565b6040516103749190612914565b60405180910390f35b6103976004803603810190610392919061284f565b61170f565b6040516103a49190612914565b60405180910390f35b6103c760048036038101906103c2919061284f565b61172c565b005b6103e360048036038101906103de9190612960565b611842565b005b6103ed611b1b565b6040516103fa9190612dbf565b60405180910390f35b61041d6004803603810190610418919061284f565b611b40565b005b610427611be7565b6040516104349190612dd8565b60405180910390f35b610445611bed565b005b61044f611c04565b60405161045c9190612dd8565b60405180910390f35b61047f600480360381019061047a9190612ccf565b611c0a565b60405161048c9190612914565b60405180910390f35b61049d611c6e565b6040516104aa9190612dbf565b60405180910390f35b6104bb611c93565b6040516104c89190612cb6565b60405180910390f35b6104d9611c99565b6040516104e69190612dbf565b60405180910390f35b61050960048036038101906105049190612ccf565b611cbe565b005b61052560048036038101906105209190612960565b611ce0565b6040516105329190612f7d565b60405180910390f35b610543611fa0565b6040516105509190612ff8565b60405180910390f35b5f5f1b61056581611fc5565b5f73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16036105d3576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016105ca9061305b565b60405180910390fd5b8160035f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505050565b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19161480610689575061068882611fd9565b5b9050919050565b6006602052805f5260405f205f91509050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690806001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16908060020154908060030180546106ff906130a6565b80601f016020809104026020016040519081016040528092919081815260200182805461072b906130a6565b80156107765780601f1061074d57610100808354040283529160200191610776565b820191905f5260205f20905b81548152906001019060200180831161075957829003601f168201915b505050505090806004015490806005015490806006015490806007015490806008015490806009015f9054906101000a900460ff169080600a0180546107bb906130a6565b80601f01602080910402602001604051908101604052809291908181526020018280546107e7906130a6565b80156108325780601f1061080957610100808354040283529160200191610832565b820191905f5260205f20905b81548152906001019060200180831161081557829003601f168201915b50505050509080600b015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690508c565b5f5f1b61086d81611fc5565b60c88211156108b1576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016108a890613120565b60405180910390fd5b816007819055507f45610d581145924dd7090a5017e5f2b1d6f42213bb2e95707ff86846bbfcb1ca826040516108e79190612dd8565b60405180910390a15050565b5f6108fc612042565b610904612064565b60055f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1661098d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161098490613188565b60405180910390fd5b5f86116109cf576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016109c6906131f0565b60405180910390fd5b8673ffffffffffffffffffffffffffffffffffffffff166323b872dd3330896040518463ffffffff1660e01b8152600401610a0c9392919061320e565b6020604051808303815f875af1158015610a28573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a4c919061326d565b610a8b576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610a82906132e2565b60405180910390fd5b5f61271060075488610a9d919061332d565b610aa7919061339b565b90505f8188610ab691906133cb565b90503389898642604051602001610ad1959493929190613483565b6040516020818303038152906040528051906020012092506040518061018001604052803373ffffffffffffffffffffffffffffffffffffffff1681526020018a73ffffffffffffffffffffffffffffffffffffffff16815260200182815260200187878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081526020018881526020015f8152602001858152602001428152602001611c2042610ba291906134e1565b81526020015f6003811115610bba57610bb9612a31565b5b815260200160405180602001604052805f81525081526020015f73ffffffffffffffffffffffffffffffffffffffff1681525060065f8581526020019081526020015f205f820151815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506020820151816001015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550604082015181600201556060820151816003019081610ca791906136d8565b506080820151816004015560a0820151816005015560c0820151816006015560e082015181600701556101008201518160080155610120820151816009015f6101000a81548160ff02191690836003811115610d0657610d05612a31565b5b021790555061014082015181600a019081610d2191906136d8565b5061016082015181600b015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055509050505f821115610e50578873ffffffffffffffffffffffffffffffffffffffff1663a9059cbb60085f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16846040518363ffffffff1660e01b8152600401610dd09291906137a7565b6020604051808303815f875af1158015610dec573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e10919061326d565b610e4f576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e4690613818565b60405180910390fd5b5b3373ffffffffffffffffffffffffffffffffffffffff16837fad3c6549dd317555f3e8872a10664c9a3312a268f0c9c873a80b9c52f180a07c8b848a8a8d604051610e9f959493929190613870565b60405180910390a35050610eb16120a5565b9695505050505050565b5f60015f8381526020019081526020015f20600101549050919050565b610ee0612042565b60035f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610f6f576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610f6690613906565b60405180910390fd5b5f60065f8381526020019081526020015f20905060016003811115610f9757610f96612a31565b5b816009015f9054906101000a900460ff166003811115610fba57610fb9612a31565b5b14610ffa576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610ff19061396e565b60405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff1681600b015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff160361108b576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611082906139d6565b60405180910390fd5b806001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663a9059cbb82600b015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600201546040518363ffffffff1660e01b81526004016111109291906137a7565b6020604051808303815f875af115801561112c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611150919061326d565b61118f576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611186906132e2565b60405180910390fd5b6002816009015f6101000a81548160ff021916908360038111156111b6576111b5612a31565b5b0217905550817f912edf360d10ba8006466028db26d840a68a22b8db84ef0deb3a7fa9c268eef25f5f1b6040516111ed9190612cb6565b60405180910390a2506111fe6120a5565b50565b61120a82610ebb565b61121381611fc5565b61121d83836120bf565b50505050565b61122b6121a8565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161461128f576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61129982826121af565b505050565b5f5f1b6112aa81611fc5565b6112b2612299565b50565b6112bd612064565b5f60065f8581526020019081526020015f2090505f60038111156112e4576112e3612a31565b5b816009015f9054906101000a900460ff16600381111561130757611306612a31565b5b14611347576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161133e90613a3e565b60405180910390fd5b806008015442111561138e576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161138590613aa6565b60405180910390fd5b60025f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663b5eb4539336040518263ffffffff1660e01b81526004016113e89190612dbf565b602060405180830381865afa158015611403573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611427919061326d565b611466576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161145d90613b0e565b60405180910390fd5b5f843360405160200161147a929190613b2c565b6040516020818303038152906040528051906020012090505f6114e985858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050836122f990919063ffffffff16565b905060045f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161461157a576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161157190613ba1565b60405180910390fd5b6001836009015f6101000a81548160ff021916908360038111156115a1576115a0612a31565b5b02179055503383600b015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055503373ffffffffffffffffffffffffffffffffffffffff16867f9663413acc8b4a21ea9ca7c900fd3bb1fbe4e1ab6e7e60cab595b1a54027025160405161162d90613be2565b60405180910390a3505050505050565b5f5f1b61164981611fc5565b5f73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16036116b7576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016116ae9061305b565b60405180910390fd5b8160045f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505050565b5f5f5f9054906101000a900460ff16905090565b6005602052805f5260405f205f915054906101000a900460ff1681565b5f5f1b61173881611fc5565b5f73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16036117a6576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161179d9061305b565b60405180910390fd5b600160055f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055508173ffffffffffffffffffffffffffffffffffffffff167fd1be2e90bd3d24839d9dd94ad871068e1f9688b02fa43f2a62c9975dfa9de2d760405160405180910390a25050565b61184a612042565b5f60065f8381526020019081526020015f2090503373ffffffffffffffffffffffffffffffffffffffff16815f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16146118ee576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016118e590613c4a565b60405180910390fd5b5f600381111561190157611900612a31565b5b816009015f9054906101000a900460ff16600381111561192457611923612a31565b5b14611964576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161195b90613a3e565b60405180910390fd5b806008015442116119aa576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016119a190613cb2565b60405180910390fd5b806001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663a9059cbb825f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600201546040518363ffffffff1660e01b8152600401611a2e9291906137a7565b6020604051808303815f875af1158015611a4a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a6e919061326d565b611aad576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611aa490613d1a565b60405180910390fd5b6003816009015f6101000a81548160ff02191690836003811115611ad457611ad3612a31565b5b0217905550817fa2c0cfcfdd46ca4b148dde16939db4dbf0481430d552d486f78e076410689be9604051611b0790613d82565b60405180910390a250611b186120a5565b50565b60045f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b5f5f1b611b4c81611fc5565b5f60055f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055508173ffffffffffffffffffffffffffffffffffffffff167fbea12876694c4055c71f74308f752b9027cf3d554194000a366abddfc239a30660405160405180910390a25050565b611c2081565b5f5f1b611bf981611fc5565b611c01612323565b50565b60075481565b5f60015f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b60035f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b5f5f1b81565b60085f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b611cc782610ebb565b611cd081611fc5565b611cda83836121af565b50505050565b611ce8612742565b60065f8381526020019081526020015f20604051806101800160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160028201548152602001600382018054611dc8906130a6565b80601f0160208091040260200160405190810160405280929190818152602001828054611df4906130a6565b8015611e3f5780601f10611e1657610100808354040283529160200191611e3f565b820191905f5260205f20905b815481529060010190602001808311611e2257829003601f168201915b505050505081526020016004820154815260200160058201548152602001600682015481526020016007820154815260200160088201548152602001600982015f9054906101000a900460ff166003811115611e9e57611e9d612a31565b5b6003811115611eb057611eaf612a31565b5b8152602001600a82018054611ec4906130a6565b80601f0160208091040260200160405190810160405280929190818152602001828054611ef0906130a6565b8015611f3b5780601f10611f1257610100808354040283529160200191611f3b565b820191905f5260205f20905b815481529060010190602001808311611f1e57829003601f168201915b50505050508152602001600b82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815250509050919050565b60025f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b611fd681611fd16121a8565b612384565b50565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b61204a6123d5565b600261205c612057612416565b61243f565b5f0181905550565b61206c6116fb565b156120a3576040517fd93c066500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b60016120b76120b2612416565b61243f565b5f0181905550565b5f6120ca8383611c0a565b61219e576001805f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555061213b6121a8565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a4600190506121a2565b5f90505b92915050565b5f33905090565b5f6121ba8383611c0a565b1561228f575f60015f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555061222c6121a8565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a460019050612293565b5f90505b92915050565b6122a1612448565b5f5f5f6101000a81548160ff0219169083151502179055507f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa6122e26121a8565b6040516122ef9190612dbf565b60405180910390a1565b5f5f5f5f6123078686612488565b92509250925061231782826124dd565b82935050505092915050565b61232b612064565b60015f5f6101000a81548160ff0219169083151502179055507f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a25861236d6121a8565b60405161237a9190612dbf565b60405180910390a1565b61238e8282611c0a565b6123d15780826040517fe2517d3f0000000000000000000000000000000000000000000000000000000081526004016123c8929190613da0565b60405180910390fd5b5050565b6123dd61263f565b15612414576040517f3ee5aeb500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005f1b905090565b5f819050919050565b6124506116fb565b612486576040517f8dfc202b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f5f5f60418451036124c8575f5f5f602087015192506040870151915060608701515f1a90506124ba8882858561265b565b9550955095505050506124d6565b5f600285515f1b9250925092505b9250925092565b5f60038111156124f0576124ef612a31565b5b82600381111561250357612502612a31565b5b031561263b576001600381111561251d5761251c612a31565b5b8260038111156125305761252f612a31565b5b03612567576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6002600381111561257b5761257a612a31565b5b82600381111561258e5761258d612a31565b5b036125d257805f1c6040517ffce698f70000000000000000000000000000000000000000000000000000000081526004016125c99190612dd8565b60405180910390fd5b6003808111156125e5576125e4612a31565b5b8260038111156125f8576125f7612a31565b5b0361263a57806040517fd78bce0c0000000000000000000000000000000000000000000000000000000081526004016126319190612cb6565b60405180910390fd5b5b5050565b5f600261265261264d612416565b61243f565b5f015414905090565b5f5f5f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115612697575f600385925092509250612738565b5f6001888888886040515f81526020016040526040516126ba9493929190613de2565b6020604051602081039080840390855afa1580156126da573d5f5f3e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff160361272b575f60015f5f1b93509350935050612738565b805f5f5f1b935093509350505b9450945094915050565b6040518061018001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f8152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f60038111156127c4576127c3612a31565b5b8152602001606081526020015f73ffffffffffffffffffffffffffffffffffffffff1681525090565b5f5ffd5b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61281e826127f5565b9050919050565b61282e81612814565b8114612838575f5ffd5b50565b5f8135905061284981612825565b92915050565b5f60208284031215612864576128636127ed565b5b5f6128718482850161283b565b91505092915050565b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b6128ae8161287a565b81146128b8575f5ffd5b50565b5f813590506128c9816128a5565b92915050565b5f602082840312156128e4576128e36127ed565b5b5f6128f1848285016128bb565b91505092915050565b5f8115159050919050565b61290e816128fa565b82525050565b5f6020820190506129275f830184612905565b92915050565b5f819050919050565b61293f8161292d565b8114612949575f5ffd5b50565b5f8135905061295a81612936565b92915050565b5f60208284031215612975576129746127ed565b5b5f6129828482850161294c565b91505092915050565b61299481612814565b82525050565b5f819050919050565b6129ac8161299a565b82525050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f601f19601f8301169050919050565b5f6129f4826129b2565b6129fe81856129bc565b9350612a0e8185602086016129cc565b612a17816129da565b840191505092915050565b612a2b8161292d565b82525050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60048110612a6f57612a6e612a31565b5b50565b5f819050612a7f82612a5e565b919050565b5f612a8e82612a72565b9050919050565b612a9e81612a84565b82525050565b5f61018082019050612ab85f83018f61298b565b612ac5602083018e61298b565b612ad2604083018d6129a3565b8181036060830152612ae4818c6129ea565b9050612af3608083018b6129a3565b612b0060a083018a6129a3565b612b0d60c0830189612a22565b612b1a60e08301886129a3565b612b286101008301876129a3565b612b36610120830186612a95565b818103610140830152612b4981856129ea565b9050612b5961016083018461298b565b9d9c50505050505050505050505050565b612b738161299a565b8114612b7d575f5ffd5b50565b5f81359050612b8e81612b6a565b92915050565b5f60208284031215612ba957612ba86127ed565b5b5f612bb684828501612b80565b91505092915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f840112612be057612bdf612bbf565b5b8235905067ffffffffffffffff811115612bfd57612bfc612bc3565b5b602083019150836001820283011115612c1957612c18612bc7565b5b9250929050565b5f5f5f5f5f5f60a08789031215612c3a57612c396127ed565b5b5f612c4789828a0161283b565b9650506020612c5889828a01612b80565b9550506040612c6989828a01612b80565b945050606087013567ffffffffffffffff811115612c8a57612c896127f1565b5b612c9689828a01612bcb565b93509350506080612ca989828a0161294c565b9150509295509295509295565b5f602082019050612cc95f830184612a22565b92915050565b5f5f60408385031215612ce557612ce46127ed565b5b5f612cf28582860161294c565b9250506020612d038582860161283b565b9150509250929050565b5f5f83601f840112612d2257612d21612bbf565b5b8235905067ffffffffffffffff811115612d3f57612d3e612bc3565b5b602083019150836001820283011115612d5b57612d5a612bc7565b5b9250929050565b5f5f5f60408486031215612d7957612d786127ed565b5b5f612d868682870161294c565b935050602084013567ffffffffffffffff811115612da757612da66127f1565b5b612db386828701612d0d565b92509250509250925092565b5f602082019050612dd25f83018461298b565b92915050565b5f602082019050612deb5f8301846129a3565b92915050565b612dfa81612814565b82525050565b612e098161299a565b82525050565b5f82825260208201905092915050565b5f612e29826129b2565b612e338185612e0f565b9350612e438185602086016129cc565b612e4c816129da565b840191505092915050565b612e608161292d565b82525050565b612e6f81612a84565b82525050565b5f61018083015f830151612e8b5f860182612df1565b506020830151612e9e6020860182612df1565b506040830151612eb16040860182612e00565b5060608301518482036060860152612ec98282612e1f565b9150506080830151612ede6080860182612e00565b5060a0830151612ef160a0860182612e00565b5060c0830151612f0460c0860182612e57565b5060e0830151612f1760e0860182612e00565b50610100830151612f2c610100860182612e00565b50610120830151612f41610120860182612e66565b50610140830151848203610140860152612f5b8282612e1f565b915050610160830151612f72610160860182612df1565b508091505092915050565b5f6020820190508181035f830152612f958184612e75565b905092915050565b5f819050919050565b5f612fc0612fbb612fb6846127f5565b612f9d565b6127f5565b9050919050565b5f612fd182612fa6565b9050919050565b5f612fe282612fc7565b9050919050565b612ff281612fd8565b82525050565b5f60208201905061300b5f830184612fe9565b92915050565b7f5a45524f5f4144445245535300000000000000000000000000000000000000005f82015250565b5f613045600c836129bc565b915061305082613011565b602082019050919050565b5f6020820190508181035f83015261307281613039565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f60028204905060018216806130bd57607f821691505b6020821081036130d0576130cf613079565b5b50919050565b7f4645455f544f4f5f4849474800000000000000000000000000000000000000005f82015250565b5f61310a600c836129bc565b9150613115826130d6565b602082019050919050565b5f6020820190508181035f830152613137816130fe565b9050919050565b7f544f4b454e5f4e4f545f535550504f52544544000000000000000000000000005f82015250565b5f6131726013836129bc565b915061317d8261313e565b602082019050919050565b5f6020820190508181035f83015261319f81613166565b9050919050565b7f414d4f554e545f49535f5a45524f0000000000000000000000000000000000005f82015250565b5f6131da600e836129bc565b91506131e5826131a6565b602082019050919050565b5f6020820190508181035f830152613207816131ce565b9050919050565b5f6060820190506132215f83018661298b565b61322e602083018561298b565b61323b60408301846129a3565b949350505050565b61324c816128fa565b8114613256575f5ffd5b50565b5f8151905061326781613243565b92915050565b5f60208284031215613282576132816127ed565b5b5f61328f84828501613259565b91505092915050565b7f5452414e534645525f4641494c454400000000000000000000000000000000005f82015250565b5f6132cc600f836129bc565b91506132d782613298565b602082019050919050565b5f6020820190508181035f8301526132f9816132c0565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6133378261299a565b91506133428361299a565b92508282026133508161299a565b9150828204841483151761336757613366613300565b5b5092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f6133a58261299a565b91506133b08361299a565b9250826133c0576133bf61336e565b5b828204905092915050565b5f6133d58261299a565b91506133e08361299a565b92508282039050818111156133f8576133f7613300565b5b92915050565b5f8160601b9050919050565b5f613414826133fe565b9050919050565b5f6134258261340a565b9050919050565b61343d61343882612814565b61341b565b82525050565b5f819050919050565b61345d6134588261299a565b613443565b82525050565b5f819050919050565b61347d6134788261292d565b613463565b82525050565b5f61348e828861342c565b60148201915061349e828761342c565b6014820191506134ae828661344c565b6020820191506134be828561346c565b6020820191506134ce828461344c565b6020820191508190509695505050505050565b5f6134eb8261299a565b91506134f68361299a565b925082820190508082111561350e5761350d613300565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f6008830261359d7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82613562565b6135a78683613562565b95508019841693508086168417925050509392505050565b5f6135d96135d46135cf8461299a565b612f9d565b61299a565b9050919050565b5f819050919050565b6135f2836135bf565b6136066135fe826135e0565b84845461356e565b825550505050565b5f5f905090565b61361d61360e565b6136288184846135e9565b505050565b5b8181101561364b576136405f82613615565b60018101905061362e565b5050565b601f8211156136905761366181613541565b61366a84613553565b81016020851015613679578190505b61368d61368585613553565b83018261362d565b50505b505050565b5f82821c905092915050565b5f6136b05f1984600802613695565b1980831691505092915050565b5f6136c883836136a1565b9150826002028217905092915050565b6136e1826129b2565b67ffffffffffffffff8111156136fa576136f9613514565b5b61370482546130a6565b61370f82828561364f565b5f60209050601f831160018114613740575f841561372e578287015190505b61373885826136bd565b86555061379f565b601f19841661374e86613541565b5f5b8281101561377557848901518255600182019150602085019450602081019050613750565b86831015613792578489015161378e601f8916826136a1565b8355505b6001600288020188555050505b505050505050565b5f6040820190506137ba5f83018561298b565b6137c760208301846129a3565b9392505050565b7f4645455f5452414e534645525f4641494c4544000000000000000000000000005f82015250565b5f6138026013836129bc565b915061380d826137ce565b602082019050919050565b5f6020820190508181035f83015261382f816137f6565b9050919050565b828183375f83830152505050565b5f61384f83856129bc565b935061385c838584613836565b613865836129da565b840190509392505050565b5f6080820190506138835f83018861298b565b61389060208301876129a3565b81810360408301526138a3818587613844565b90506138b260608301846129a3565b9695505050505050565b7f554e415554484f52495a45445f4f5241434c45000000000000000000000000005f82015250565b5f6138f06013836129bc565b91506138fb826138bc565b602082019050919050565b5f6020820190508181035f83015261391d816138e4565b9050919050565b7f4e4f545f50524f43455353494e470000000000000000000000000000000000005f82015250565b5f613958600e836129bc565b915061396382613924565b602082019050919050565b5f6020820190508181035f8301526139858161394c565b9050919050565b7f4f50455241544f525f4e4f545f41535349474e454400000000000000000000005f82015250565b5f6139c06015836129bc565b91506139cb8261398c565b602082019050919050565b5f6020820190508181035f8301526139ed816139b4565b9050919050565b7f4e4f545f50454e44494e470000000000000000000000000000000000000000005f82015250565b5f613a28600b836129bc565b9150613a33826139f4565b602082019050919050565b5f6020820190508181035f830152613a5581613a1c565b9050919050565b7f5041594d454e545f4558504952454400000000000000000000000000000000005f82015250565b5f613a90600f836129bc565b9150613a9b82613a5c565b602082019050919050565b5f6020820190508181035f830152613abd81613a84565b9050919050565b7f4e4f545f4143544956455f4c50000000000000000000000000000000000000005f82015250565b5f613af8600d836129bc565b9150613b0382613ac4565b602082019050919050565b5f6020820190508181035f830152613b2581613aec565b9050919050565b5f613b37828561346c565b602082019150613b47828461342c565b6014820191508190509392505050565b7f494e56414c49445f534c495000000000000000000000000000000000000000005f82015250565b5f613b8b600c836129bc565b9150613b9682613b57565b602082019050919050565b5f6020820190508181035f830152613bb881613b7f565b9050919050565b50565b5f613bcd5f836129bc565b9150613bd882613bbf565b5f82019050919050565b5f6020820190508181035f830152613bf981613bc2565b9050919050565b7f4e4f545f53454e444552000000000000000000000000000000000000000000005f82015250565b5f613c34600a836129bc565b9150613c3f82613c00565b602082019050919050565b5f6020820190508181035f830152613c6181613c28565b9050919050565b7f444541444c494e455f4e4f545f504153534544000000000000000000000000005f82015250565b5f613c9c6013836129bc565b9150613ca782613c68565b602082019050919050565b5f6020820190508181035f830152613cc981613c90565b9050919050565b7f524546554e445f5452414e534645525f4641494c4544000000000000000000005f82015250565b5f613d046016836129bc565b9150613d0f82613cd0565b602082019050919050565b5f6020820190508181035f830152613d3181613cf8565b9050919050565b7f446561646c696e652065787069726564000000000000000000000000000000005f82015250565b5f613d6c6010836129bc565b9150613d7782613d38565b602082019050919050565b5f6020820190508181035f830152613d9981613d60565b9050919050565b5f604082019050613db35f83018561298b565b613dc06020830184612a22565b9392505050565b5f60ff82169050919050565b613ddc81613dc7565b82525050565b5f608082019050613df55f830187612a22565b613e026020830186613dd3565b613e0f6040830185612a22565b613e1c6060830184612a22565b9594505050505056fea2646970667358221220f0bc58f671ac9e3380e01e920bd1d974d20cd65b1d5d71101842d42487082d2a64736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`2`\x07U4\x80\x15a\0\x14W__\xFD[P`@QaDb8\x03\x80aDb\x839\x81\x81\x01`@R\x81\x01\x90a\x006\x91\x90a\x05\x1EV[`\x01a\0Ta\0Ia\x03.` \x1B` \x1CV[a\x03W` \x1B` \x1CV[_\x01\x81\x90UP_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\0\xC8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\xBF\x90a\x05\xDCV[`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x016W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x01-\x90a\x05\xDCV[`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x01\xA4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x01\x9B\x90a\x05\xDCV[`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x02\x12W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02\t\x90a\x05\xDCV[`@Q\x80\x91\x03\x90\xFD[a\x02$__\x1B3a\x03`` \x1B` \x1CV[P\x83`\x08_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82`\x02_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\x03_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x04_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPa\x05\xFAV[_\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0_\x1B\x90P\x90V[_\x81\x90P\x91\x90PV[_a\x03q\x83\x83a\x04U` \x1B` \x1CV[a\x04KW`\x01\x80_\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x03\xE8a\x04\xB9` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x04OV[_\x90P[\x92\x91PPV[_`\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_3\x90P\x90V[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x04\xED\x82a\x04\xC4V[\x90P\x91\x90PV[a\x04\xFD\x81a\x04\xE3V[\x81\x14a\x05\x07W__\xFD[PV[_\x81Q\x90Pa\x05\x18\x81a\x04\xF4V[\x92\x91PPV[____`\x80\x85\x87\x03\x12\x15a\x056Wa\x055a\x04\xC0V[[_a\x05C\x87\x82\x88\x01a\x05\nV[\x94PP` a\x05T\x87\x82\x88\x01a\x05\nV[\x93PP`@a\x05e\x87\x82\x88\x01a\x05\nV[\x92PP``a\x05v\x87\x82\x88\x01a\x05\nV[\x91PP\x92\x95\x91\x94P\x92PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FZERO_ADDRESS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a\x05\xC6`\x0C\x83a\x05\x82V[\x91Pa\x05\xD1\x82a\x05\x92V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x05\xF3\x81a\x05\xBAV[\x90P\x91\x90PV[a>[\x80a\x06\x07_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xC2W_5`\xE0\x1C\x80cmi\xFC\xAF\x11a\0\xF7W\x80c\x91\xD1HT\x11a\0\x95W\x80c\xC4\x15\xB9\\\x11a\0oW\x80c\xC4\x15\xB9\\\x14a\x04\xD1W\x80c\xD5Gt\x1F\x14a\x04\xEFW\x80c\xE6n\xEF\xC8\x14a\x05\x0BW\x80c\xF2\xE9\xE4\x18\x14a\x05;Wa\x01\xC2V[\x80c\x91\xD1HT\x14a\x04eW\x80c\x97\xDA\x03J\x14a\x04\x95W\x80c\xA2\x17\xFD\xDF\x14a\x04\xB3Wa\x01\xC2V[\x80cv1\x91\x90\x11a\0\xD1W\x80cv1\x91\x90\x14a\x04\x03W\x80cy\x94\xD1\xA4\x14a\x04\x1FW\x80c\x84V\xCBY\x14a\x04=W\x80c\x8Cc\x9A\x85\x14a\x04GWa\x01\xC2V[\x80cmi\xFC\xAF\x14a\x03\xADW\x80cq\xDE/\xFC\x14a\x03\xC9W\x80cr\xF3\xE8\xE1\x14a\x03\xE5Wa\x01\xC2V[\x80c//\xF1]\x11a\x01dW\x80cM\xD00\x1F\x11a\x01>W\x80cM\xD00\x1F\x14a\x03'W\x80cU\x0E\xBE(\x14a\x03CW\x80c\\\x97Z\xBB\x14a\x03_W\x80ch\xC4\xAC&\x14a\x03}Wa\x01\xC2V[\x80c//\xF1]\x14a\x02\xE5W\x80c6V\x8A\xBE\x14a\x03\x01W\x80c?K\xA8:\x14a\x03\x1DWa\x01\xC2V[\x80c\x12\xE8\xE2\xC3\x11a\x01\xA0W\x80c\x12\xE8\xE2\xC3\x14a\x02MW\x80c\x1E\nP]\x14a\x02iW\x80c$\x8A\x9C\xA3\x14a\x02\x99W\x80c+\xD75\xAB\x14a\x02\xC9Wa\x01\xC2V[\x80c\x01\xE8\xA6\xBB\x14a\x01\xC6W\x80c\x01\xFF\xC9\xA7\x14a\x01\xE2W\x80c\x07\x162m\x14a\x02\x12W[__\xFD[a\x01\xE0`\x04\x806\x03\x81\x01\x90a\x01\xDB\x91\x90a(OV[a\x05YV[\0[a\x01\xFC`\x04\x806\x03\x81\x01\x90a\x01\xF7\x91\x90a(\xCFV[a\x06\x17V[`@Qa\x02\t\x91\x90a)\x14V[`@Q\x80\x91\x03\x90\xF3[a\x02,`\x04\x806\x03\x81\x01\x90a\x02'\x91\x90a)`V[a\x06\x90V[`@Qa\x02D\x9C\x9B\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a*\xA4V[`@Q\x80\x91\x03\x90\xF3[a\x02g`\x04\x806\x03\x81\x01\x90a\x02b\x91\x90a+\x94V[a\x08aV[\0[a\x02\x83`\x04\x806\x03\x81\x01\x90a\x02~\x91\x90a, V[a\x08\xF3V[`@Qa\x02\x90\x91\x90a,\xB6V[`@Q\x80\x91\x03\x90\xF3[a\x02\xB3`\x04\x806\x03\x81\x01\x90a\x02\xAE\x91\x90a)`V[a\x0E\xBBV[`@Qa\x02\xC0\x91\x90a,\xB6V[`@Q\x80\x91\x03\x90\xF3[a\x02\xE3`\x04\x806\x03\x81\x01\x90a\x02\xDE\x91\x90a)`V[a\x0E\xD8V[\0[a\x02\xFF`\x04\x806\x03\x81\x01\x90a\x02\xFA\x91\x90a,\xCFV[a\x12\x01V[\0[a\x03\x1B`\x04\x806\x03\x81\x01\x90a\x03\x16\x91\x90a,\xCFV[a\x12#V[\0[a\x03%a\x12\x9EV[\0[a\x03A`\x04\x806\x03\x81\x01\x90a\x03<\x91\x90a-bV[a\x12\xB5V[\0[a\x03]`\x04\x806\x03\x81\x01\x90a\x03X\x91\x90a(OV[a\x16=V[\0[a\x03ga\x16\xFBV[`@Qa\x03t\x91\x90a)\x14V[`@Q\x80\x91\x03\x90\xF3[a\x03\x97`\x04\x806\x03\x81\x01\x90a\x03\x92\x91\x90a(OV[a\x17\x0FV[`@Qa\x03\xA4\x91\x90a)\x14V[`@Q\x80\x91\x03\x90\xF3[a\x03\xC7`\x04\x806\x03\x81\x01\x90a\x03\xC2\x91\x90a(OV[a\x17,V[\0[a\x03\xE3`\x04\x806\x03\x81\x01\x90a\x03\xDE\x91\x90a)`V[a\x18BV[\0[a\x03\xEDa\x1B\x1BV[`@Qa\x03\xFA\x91\x90a-\xBFV[`@Q\x80\x91\x03\x90\xF3[a\x04\x1D`\x04\x806\x03\x81\x01\x90a\x04\x18\x91\x90a(OV[a\x1B@V[\0[a\x04'a\x1B\xE7V[`@Qa\x044\x91\x90a-\xD8V[`@Q\x80\x91\x03\x90\xF3[a\x04Ea\x1B\xEDV[\0[a\x04Oa\x1C\x04V[`@Qa\x04\\\x91\x90a-\xD8V[`@Q\x80\x91\x03\x90\xF3[a\x04\x7F`\x04\x806\x03\x81\x01\x90a\x04z\x91\x90a,\xCFV[a\x1C\nV[`@Qa\x04\x8C\x91\x90a)\x14V[`@Q\x80\x91\x03\x90\xF3[a\x04\x9Da\x1CnV[`@Qa\x04\xAA\x91\x90a-\xBFV[`@Q\x80\x91\x03\x90\xF3[a\x04\xBBa\x1C\x93V[`@Qa\x04\xC8\x91\x90a,\xB6V[`@Q\x80\x91\x03\x90\xF3[a\x04\xD9a\x1C\x99V[`@Qa\x04\xE6\x91\x90a-\xBFV[`@Q\x80\x91\x03\x90\xF3[a\x05\t`\x04\x806\x03\x81\x01\x90a\x05\x04\x91\x90a,\xCFV[a\x1C\xBEV[\0[a\x05%`\x04\x806\x03\x81\x01\x90a\x05 \x91\x90a)`V[a\x1C\xE0V[`@Qa\x052\x91\x90a/}V[`@Q\x80\x91\x03\x90\xF3[a\x05Ca\x1F\xA0V[`@Qa\x05P\x91\x90a/\xF8V[`@Q\x80\x91\x03\x90\xF3[__\x1Ba\x05e\x81a\x1F\xC5V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x05\xD3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xCA\x90a0[V[`@Q\x80\x91\x03\x90\xFD[\x81`\x03_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\x89WPa\x06\x88\x82a\x1F\xD9V[[\x90P\x91\x90PV[`\x06` R\x80_R`@_ _\x91P\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80`\x02\x01T\x90\x80`\x03\x01\x80Ta\x06\xFF\x90a0\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07+\x90a0\xA6V[\x80\x15a\x07vW\x80`\x1F\x10a\x07MWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07vV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07YW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x04\x01T\x90\x80`\x05\x01T\x90\x80`\x06\x01T\x90\x80`\x07\x01T\x90\x80`\x08\x01T\x90\x80`\t\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90\x80`\n\x01\x80Ta\x07\xBB\x90a0\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xE7\x90a0\xA6V[\x80\x15a\x082W\x80`\x1F\x10a\x08\tWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x082V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\x15W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x0B\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x8CV[__\x1Ba\x08m\x81a\x1F\xC5V[`\xC8\x82\x11\x15a\x08\xB1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xA8\x90a1 V[`@Q\x80\x91\x03\x90\xFD[\x81`\x07\x81\x90UP\x7FEa\rX\x11E\x92M\xD7\t\nP\x17\xE5\xF2\xB1\xD6\xF4\"\x13\xBB.\x95p\x7F\xF8hF\xBB\xFC\xB1\xCA\x82`@Qa\x08\xE7\x91\x90a-\xD8V[`@Q\x80\x91\x03\x90\xA1PPV[_a\x08\xFCa BV[a\t\x04a dV[`\x05_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\t\x8DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\x84\x90a1\x88V[`@Q\x80\x91\x03\x90\xFD[_\x86\x11a\t\xCFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xC6\x90a1\xF0V[`@Q\x80\x91\x03\x90\xFD[\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD30\x89`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x0C\x93\x92\x91\x90a2\x0EV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n(W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nL\x91\x90a2mV[a\n\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\x82\x90a2\xE2V[`@Q\x80\x91\x03\x90\xFD[_a'\x10`\x07T\x88a\n\x9D\x91\x90a3-V[a\n\xA7\x91\x90a3\x9BV[\x90P_\x81\x88a\n\xB6\x91\x90a3\xCBV[\x90P3\x89\x89\x86B`@Q` \x01a\n\xD1\x95\x94\x93\x92\x91\x90a4\x83V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92P`@Q\x80a\x01\x80\x01`@R\x803s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81R` \x01\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x88\x81R` \x01_\x81R` \x01\x85\x81R` \x01B\x81R` \x01a\x1C Ba\x0B\xA2\x91\x90a4\xE1V[\x81R` \x01_`\x03\x81\x11\x15a\x0B\xBAWa\x0B\xB9a*1V[[\x81R` \x01`@Q\x80` \x01`@R\x80_\x81RP\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP`\x06_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01\x90\x81a\x0C\xA7\x91\x90a6\xD8V[P`\x80\x82\x01Q\x81`\x04\x01U`\xA0\x82\x01Q\x81`\x05\x01U`\xC0\x82\x01Q\x81`\x06\x01U`\xE0\x82\x01Q\x81`\x07\x01Ua\x01\0\x82\x01Q\x81`\x08\x01Ua\x01 \x82\x01Q\x81`\t\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\r\x06Wa\r\x05a*1V[[\x02\x17\x90UPa\x01@\x82\x01Q\x81`\n\x01\x90\x81a\r!\x91\x90a6\xD8V[Pa\x01`\x82\x01Q\x81`\x0B\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x90PP_\x82\x11\x15a\x0EPW\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB`\x08_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xD0\x92\x91\x90a7\xA7V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\r\xECW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x10\x91\x90a2mV[a\x0EOW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0EF\x90a8\x18V[`@Q\x80\x91\x03\x90\xFD[[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xAD<eI\xDD1uU\xF3\xE8\x87*\x10fL\x9A3\x12\xA2h\xF0\xC9\xC8s\xA8\x0B\x9CR\xF1\x80\xA0|\x8B\x84\x8A\x8A\x8D`@Qa\x0E\x9F\x95\x94\x93\x92\x91\x90a8pV[`@Q\x80\x91\x03\x90\xA3PPa\x0E\xB1a \xA5V[\x96\x95PPPPPPV[_`\x01_\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[a\x0E\xE0a BV[`\x03_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0FoW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Ff\x90a9\x06V[`@Q\x80\x91\x03\x90\xFD[_`\x06_\x83\x81R` \x01\x90\x81R` \x01_ \x90P`\x01`\x03\x81\x11\x15a\x0F\x97Wa\x0F\x96a*1V[[\x81`\t\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x0F\xBAWa\x0F\xB9a*1V[[\x14a\x0F\xFAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xF1\x90a9nV[`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x0B\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x10\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x82\x90a9\xD6V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x82`\x0B\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x02\x01T`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\x10\x92\x91\x90a7\xA7V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x11,W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11P\x91\x90a2mV[a\x11\x8FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\x86\x90a2\xE2V[`@Q\x80\x91\x03\x90\xFD[`\x02\x81`\t\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\x11\xB6Wa\x11\xB5a*1V[[\x02\x17\x90UP\x81\x7F\x91.\xDF6\r\x10\xBA\x80\x06F`(\xDB&\xD8@\xA6\x8A\"\xB8\xDB\x84\xEF\r\xEB:\x7F\xA9\xC2h\xEE\xF2__\x1B`@Qa\x11\xED\x91\x90a,\xB6V[`@Q\x80\x91\x03\x90\xA2Pa\x11\xFEa \xA5V[PV[a\x12\n\x82a\x0E\xBBV[a\x12\x13\x81a\x1F\xC5V[a\x12\x1D\x83\x83a \xBFV[PPPPV[a\x12+a!\xA8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x12\x8FW`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\x99\x82\x82a!\xAFV[PPPV[__\x1Ba\x12\xAA\x81a\x1F\xC5V[a\x12\xB2a\"\x99V[PV[a\x12\xBDa dV[_`\x06_\x85\x81R` \x01\x90\x81R` \x01_ \x90P_`\x03\x81\x11\x15a\x12\xE4Wa\x12\xE3a*1V[[\x81`\t\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x13\x07Wa\x13\x06a*1V[[\x14a\x13GW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13>\x90a:>V[`@Q\x80\x91\x03\x90\xFD[\x80`\x08\x01TB\x11\x15a\x13\x8EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\x85\x90a:\xA6V[`@Q\x80\x91\x03\x90\xFD[`\x02_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xB5\xEBE93`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xE8\x91\x90a-\xBFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\x03W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14'\x91\x90a2mV[a\x14fW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14]\x90a;\x0EV[`@Q\x80\x91\x03\x90\xFD[_\x843`@Q` \x01a\x14z\x92\x91\x90a;,V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_a\x14\xE9\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x83a\"\xF9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\x04_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x15zW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15q\x90a;\xA1V[`@Q\x80\x91\x03\x90\xFD[`\x01\x83`\t\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\x15\xA1Wa\x15\xA0a*1V[[\x02\x17\x90UP3\x83`\x0B\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x7F\x96cA:\xCC\x8BJ!\xEA\x9C\xA7\xC9\0\xFD;\xB1\xFB\xE4\xE1\xABn~`\xCA\xB5\x95\xB1\xA5@'\x02Q`@Qa\x16-\x90a;\xE2V[`@Q\x80\x91\x03\x90\xA3PPPPPPV[__\x1Ba\x16I\x81a\x1F\xC5V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x16\xB7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16\xAE\x90a0[V[`@Q\x80\x91\x03\x90\xFD[\x81`\x04_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[___\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[`\x05` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[__\x1Ba\x178\x81a\x1F\xC5V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x17\xA6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17\x9D\x90a0[V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x05_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xD1\xBE.\x90\xBD=$\x83\x9D\x9D\xD9J\xD8q\x06\x8E\x1F\x96\x88\xB0/\xA4?*b\xC9\x97]\xFA\x9D\xE2\xD7`@Q`@Q\x80\x91\x03\x90\xA2PPV[a\x18Ja BV[_`\x06_\x83\x81R` \x01\x90\x81R` \x01_ \x90P3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x18\xEEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xE5\x90a<JV[`@Q\x80\x91\x03\x90\xFD[_`\x03\x81\x11\x15a\x19\x01Wa\x19\0a*1V[[\x81`\t\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x19$Wa\x19#a*1V[[\x14a\x19dW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19[\x90a:>V[`@Q\x80\x91\x03\x90\xFD[\x80`\x08\x01TB\x11a\x19\xAAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\xA1\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x82_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x02\x01T`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A.\x92\x91\x90a7\xA7V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1AJW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1An\x91\x90a2mV[a\x1A\xADW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\xA4\x90a=\x1AV[`@Q\x80\x91\x03\x90\xFD[`\x03\x81`\t\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\x1A\xD4Wa\x1A\xD3a*1V[[\x02\x17\x90UP\x81\x7F\xA2\xC0\xCF\xCF\xDDF\xCAK\x14\x8D\xDE\x16\x93\x9D\xB4\xDB\xF0H\x140\xD5R\xD4\x86\xF7\x8E\x07d\x10h\x9B\xE9`@Qa\x1B\x07\x90a=\x82V[`@Q\x80\x91\x03\x90\xA2Pa\x1B\x18a \xA5V[PV[`\x04_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[__\x1Ba\x1BL\x81a\x1F\xC5V[_`\x05_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBE\xA1(viL@U\xC7\x1Ft0\x8Fu+\x90'\xCF=UA\x94\0\n6j\xBD\xDF\xC29\xA3\x06`@Q`@Q\x80\x91\x03\x90\xA2PPV[a\x1C \x81V[__\x1Ba\x1B\xF9\x81a\x1F\xC5V[a\x1C\x01a##V[PV[`\x07T\x81V[_`\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[`\x03_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[__\x1B\x81V[`\x08_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x1C\xC7\x82a\x0E\xBBV[a\x1C\xD0\x81a\x1F\xC5V[a\x1C\xDA\x83\x83a!\xAFV[PPPPV[a\x1C\xE8a'BV[`\x06_\x83\x81R` \x01\x90\x81R` \x01_ `@Q\x80a\x01\x80\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01\x80Ta\x1D\xC8\x90a0\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1D\xF4\x90a0\xA6V[\x80\x15a\x1E?W\x80`\x1F\x10a\x1E\x16Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E?V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E\"W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01T\x81R` \x01`\x06\x82\x01T\x81R` \x01`\x07\x82\x01T\x81R` \x01`\x08\x82\x01T\x81R` \x01`\t\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x1E\x9EWa\x1E\x9Da*1V[[`\x03\x81\x11\x15a\x1E\xB0Wa\x1E\xAFa*1V[[\x81R` \x01`\n\x82\x01\x80Ta\x1E\xC4\x90a0\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E\xF0\x90a0\xA6V[\x80\x15a\x1F;W\x80`\x1F\x10a\x1F\x12Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F;V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\x1EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x0B\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x91\x90PV[`\x02_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x1F\xD6\x81a\x1F\xD1a!\xA8V[a#\x84V[PV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[a Ja#\xD5V[`\x02a \\a Wa$\x16V[a$?V[_\x01\x81\x90UPV[a la\x16\xFBV[\x15a \xA3W`@Q\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\x01a \xB7a \xB2a$\x16V[a$?V[_\x01\x81\x90UPV[_a \xCA\x83\x83a\x1C\nV[a!\x9EW`\x01\x80_\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa!;a!\xA8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa!\xA2V[_\x90P[\x92\x91PPV[_3\x90P\x90V[_a!\xBA\x83\x83a\x1C\nV[\x15a\"\x8FW_`\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\",a!\xA8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\"\x93V[_\x90P[\x92\x91PPV[a\"\xA1a$HV[___a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAAa\"\xE2a!\xA8V[`@Qa\"\xEF\x91\x90a-\xBFV[`@Q\x80\x91\x03\x90\xA1V[____a#\x07\x86\x86a$\x88V[\x92P\x92P\x92Pa#\x17\x82\x82a$\xDDV[\x82\x93PPPP\x92\x91PPV[a#+a dV[`\x01__a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa#ma!\xA8V[`@Qa#z\x91\x90a-\xBFV[`@Q\x80\x91\x03\x90\xA1V[a#\x8E\x82\x82a\x1C\nV[a#\xD1W\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#\xC8\x92\x91\x90a=\xA0V[`@Q\x80\x91\x03\x90\xFD[PPV[a#\xDDa&?V[\x15a$\x14W`@Q\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0_\x1B\x90P\x90V[_\x81\x90P\x91\x90PV[a$Pa\x16\xFBV[a$\x86W`@Q\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[___`A\x84Q\x03a$\xC8W___` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa$\xBA\x88\x82\x85\x85a&[V[\x95P\x95P\x95PPPPa$\xD6V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a$\xF0Wa$\xEFa*1V[[\x82`\x03\x81\x11\x15a%\x03Wa%\x02a*1V[[\x03\x15a&;W`\x01`\x03\x81\x11\x15a%\x1DWa%\x1Ca*1V[[\x82`\x03\x81\x11\x15a%0Wa%/a*1V[[\x03a%gW`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a%{Wa%za*1V[[\x82`\x03\x81\x11\x15a%\x8EWa%\x8Da*1V[[\x03a%\xD2W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\xC9\x91\x90a-\xD8V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a%\xE5Wa%\xE4a*1V[[\x82`\x03\x81\x11\x15a%\xF8Wa%\xF7a*1V[[\x03a&:W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&1\x91\x90a,\xB6V[`@Q\x80\x91\x03\x90\xFD[[PPV[_`\x02a&Ra&Ma$\x16V[a$?V[_\x01T\x14\x90P\x90V[___\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a&\x97W_`\x03\x85\x92P\x92P\x92Pa'8V[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa&\xBA\x94\x93\x92\x91\x90a=\xE2V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a&\xDAW=__>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a'+W_`\x01__\x1B\x93P\x93P\x93PPa'8V[\x80___\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[`@Q\x80a\x01\x80\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x03\x81\x11\x15a'\xC4Wa'\xC3a*1V[[\x81R` \x01``\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[__\xFD[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a(\x1E\x82a'\xF5V[\x90P\x91\x90PV[a(.\x81a(\x14V[\x81\x14a(8W__\xFD[PV[_\x815\x90Pa(I\x81a(%V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a(dWa(ca'\xEDV[[_a(q\x84\x82\x85\x01a(;V[\x91PP\x92\x91PPV[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a(\xAE\x81a(zV[\x81\x14a(\xB8W__\xFD[PV[_\x815\x90Pa(\xC9\x81a(\xA5V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a(\xE4Wa(\xE3a'\xEDV[[_a(\xF1\x84\x82\x85\x01a(\xBBV[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a)\x0E\x81a(\xFAV[\x82RPPV[_` \x82\x01\x90Pa)'_\x83\x01\x84a)\x05V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a)?\x81a)-V[\x81\x14a)IW__\xFD[PV[_\x815\x90Pa)Z\x81a)6V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a)uWa)ta'\xEDV[[_a)\x82\x84\x82\x85\x01a)LV[\x91PP\x92\x91PPV[a)\x94\x81a(\x14V[\x82RPPV[_\x81\x90P\x91\x90PV[a)\xAC\x81a)\x9AV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a)\xF4\x82a)\xB2V[a)\xFE\x81\x85a)\xBCV[\x93Pa*\x0E\x81\x85` \x86\x01a)\xCCV[a*\x17\x81a)\xDAV[\x84\x01\x91PP\x92\x91PPV[a*+\x81a)-V[\x82RPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a*oWa*na*1V[[PV[_\x81\x90Pa*\x7F\x82a*^V[\x91\x90PV[_a*\x8E\x82a*rV[\x90P\x91\x90PV[a*\x9E\x81a*\x84V[\x82RPPV[_a\x01\x80\x82\x01\x90Pa*\xB8_\x83\x01\x8Fa)\x8BV[a*\xC5` \x83\x01\x8Ea)\x8BV[a*\xD2`@\x83\x01\x8Da)\xA3V[\x81\x81\x03``\x83\x01Ra*\xE4\x81\x8Ca)\xEAV[\x90Pa*\xF3`\x80\x83\x01\x8Ba)\xA3V[a+\0`\xA0\x83\x01\x8Aa)\xA3V[a+\r`\xC0\x83\x01\x89a*\"V[a+\x1A`\xE0\x83\x01\x88a)\xA3V[a+(a\x01\0\x83\x01\x87a)\xA3V[a+6a\x01 \x83\x01\x86a*\x95V[\x81\x81\x03a\x01@\x83\x01Ra+I\x81\x85a)\xEAV[\x90Pa+Ya\x01`\x83\x01\x84a)\x8BV[\x9D\x9CPPPPPPPPPPPPPV[a+s\x81a)\x9AV[\x81\x14a+}W__\xFD[PV[_\x815\x90Pa+\x8E\x81a+jV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a+\xA9Wa+\xA8a'\xEDV[[_a+\xB6\x84\x82\x85\x01a+\x80V[\x91PP\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a+\xE0Wa+\xDFa+\xBFV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xFDWa+\xFCa+\xC3V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a,\x19Wa,\x18a+\xC7V[[\x92P\x92\x90PV[______`\xA0\x87\x89\x03\x12\x15a,:Wa,9a'\xEDV[[_a,G\x89\x82\x8A\x01a(;V[\x96PP` a,X\x89\x82\x8A\x01a+\x80V[\x95PP`@a,i\x89\x82\x8A\x01a+\x80V[\x94PP``\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\x8AWa,\x89a'\xF1V[[a,\x96\x89\x82\x8A\x01a+\xCBV[\x93P\x93PP`\x80a,\xA9\x89\x82\x8A\x01a)LV[\x91PP\x92\x95P\x92\x95P\x92\x95V[_` \x82\x01\x90Pa,\xC9_\x83\x01\x84a*\"V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a,\xE5Wa,\xE4a'\xEDV[[_a,\xF2\x85\x82\x86\x01a)LV[\x92PP` a-\x03\x85\x82\x86\x01a(;V[\x91PP\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a-\"Wa-!a+\xBFV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-?Wa->a+\xC3V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a-[Wa-Za+\xC7V[[\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15a-yWa-xa'\xEDV[[_a-\x86\x86\x82\x87\x01a)LV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\xA7Wa-\xA6a'\xF1V[[a-\xB3\x86\x82\x87\x01a-\rV[\x92P\x92PP\x92P\x92P\x92V[_` \x82\x01\x90Pa-\xD2_\x83\x01\x84a)\x8BV[\x92\x91PPV[_` \x82\x01\x90Pa-\xEB_\x83\x01\x84a)\xA3V[\x92\x91PPV[a-\xFA\x81a(\x14V[\x82RPPV[a.\t\x81a)\x9AV[\x82RPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a.)\x82a)\xB2V[a.3\x81\x85a.\x0FV[\x93Pa.C\x81\x85` \x86\x01a)\xCCV[a.L\x81a)\xDAV[\x84\x01\x91PP\x92\x91PPV[a.`\x81a)-V[\x82RPPV[a.o\x81a*\x84V[\x82RPPV[_a\x01\x80\x83\x01_\x83\x01Qa.\x8B_\x86\x01\x82a-\xF1V[P` \x83\x01Qa.\x9E` \x86\x01\x82a-\xF1V[P`@\x83\x01Qa.\xB1`@\x86\x01\x82a.\0V[P``\x83\x01Q\x84\x82\x03``\x86\x01Ra.\xC9\x82\x82a.\x1FV[\x91PP`\x80\x83\x01Qa.\xDE`\x80\x86\x01\x82a.\0V[P`\xA0\x83\x01Qa.\xF1`\xA0\x86\x01\x82a.\0V[P`\xC0\x83\x01Qa/\x04`\xC0\x86\x01\x82a.WV[P`\xE0\x83\x01Qa/\x17`\xE0\x86\x01\x82a.\0V[Pa\x01\0\x83\x01Qa/,a\x01\0\x86\x01\x82a.\0V[Pa\x01 \x83\x01Qa/Aa\x01 \x86\x01\x82a.fV[Pa\x01@\x83\x01Q\x84\x82\x03a\x01@\x86\x01Ra/[\x82\x82a.\x1FV[\x91PPa\x01`\x83\x01Qa/ra\x01`\x86\x01\x82a-\xF1V[P\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra/\x95\x81\x84a.uV[\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_a/\xC0a/\xBBa/\xB6\x84a'\xF5V[a/\x9DV[a'\xF5V[\x90P\x91\x90PV[_a/\xD1\x82a/\xA6V[\x90P\x91\x90PV[_a/\xE2\x82a/\xC7V[\x90P\x91\x90PV[a/\xF2\x81a/\xD8V[\x82RPPV[_` \x82\x01\x90Pa0\x0B_\x83\x01\x84a/\xE9V[\x92\x91PPV[\x7FZERO_ADDRESS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a0E`\x0C\x83a)\xBCV[\x91Pa0P\x82a0\x11V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra0r\x81a09V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a0\xBDW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a0\xD0Wa0\xCFa0yV[[P\x91\x90PV[\x7FFEE_TOO_HIGH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a1\n`\x0C\x83a)\xBCV[\x91Pa1\x15\x82a0\xD6V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra17\x81a0\xFEV[\x90P\x91\x90PV[\x7FTOKEN_NOT_SUPPORTED\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a1r`\x13\x83a)\xBCV[\x91Pa1}\x82a1>V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra1\x9F\x81a1fV[\x90P\x91\x90PV[\x7FAMOUNT_IS_ZERO\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a1\xDA`\x0E\x83a)\xBCV[\x91Pa1\xE5\x82a1\xA6V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra2\x07\x81a1\xCEV[\x90P\x91\x90PV[_``\x82\x01\x90Pa2!_\x83\x01\x86a)\x8BV[a2.` \x83\x01\x85a)\x8BV[a2;`@\x83\x01\x84a)\xA3V[\x94\x93PPPPV[a2L\x81a(\xFAV[\x81\x14a2VW__\xFD[PV[_\x81Q\x90Pa2g\x81a2CV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a2\x82Wa2\x81a'\xEDV[[_a2\x8F\x84\x82\x85\x01a2YV[\x91PP\x92\x91PPV[\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a2\xCC`\x0F\x83a)\xBCV[\x91Pa2\xD7\x82a2\x98V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra2\xF9\x81a2\xC0V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a37\x82a)\x9AV[\x91Pa3B\x83a)\x9AV[\x92P\x82\x82\x02a3P\x81a)\x9AV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a3gWa3fa3\0V[[P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a3\xA5\x82a)\x9AV[\x91Pa3\xB0\x83a)\x9AV[\x92P\x82a3\xC0Wa3\xBFa3nV[[\x82\x82\x04\x90P\x92\x91PPV[_a3\xD5\x82a)\x9AV[\x91Pa3\xE0\x83a)\x9AV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a3\xF8Wa3\xF7a3\0V[[\x92\x91PPV[_\x81``\x1B\x90P\x91\x90PV[_a4\x14\x82a3\xFEV[\x90P\x91\x90PV[_a4%\x82a4\nV[\x90P\x91\x90PV[a4=a48\x82a(\x14V[a4\x1BV[\x82RPPV[_\x81\x90P\x91\x90PV[a4]a4X\x82a)\x9AV[a4CV[\x82RPPV[_\x81\x90P\x91\x90PV[a4}a4x\x82a)-V[a4cV[\x82RPPV[_a4\x8E\x82\x88a4,V[`\x14\x82\x01\x91Pa4\x9E\x82\x87a4,V[`\x14\x82\x01\x91Pa4\xAE\x82\x86a4LV[` \x82\x01\x91Pa4\xBE\x82\x85a4lV[` \x82\x01\x91Pa4\xCE\x82\x84a4LV[` \x82\x01\x91P\x81\x90P\x96\x95PPPPPPV[_a4\xEB\x82a)\x9AV[\x91Pa4\xF6\x83a)\x9AV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a5\x0EWa5\ra3\0V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a5\x9D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a5bV[a5\xA7\x86\x83a5bV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_a5\xD9a5\xD4a5\xCF\x84a)\x9AV[a/\x9DV[a)\x9AV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a5\xF2\x83a5\xBFV[a6\x06a5\xFE\x82a5\xE0V[\x84\x84Ta5nV[\x82UPPPPV[__\x90P\x90V[a6\x1Da6\x0EV[a6(\x81\x84\x84a5\xE9V[PPPV[[\x81\x81\x10\x15a6KWa6@_\x82a6\x15V[`\x01\x81\x01\x90Pa6.V[PPV[`\x1F\x82\x11\x15a6\x90Wa6a\x81a5AV[a6j\x84a5SV[\x81\x01` \x85\x10\x15a6yW\x81\x90P[a6\x8Da6\x85\x85a5SV[\x83\x01\x82a6-V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a6\xB0_\x19\x84`\x08\x02a6\x95V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a6\xC8\x83\x83a6\xA1V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a6\xE1\x82a)\xB2V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xFAWa6\xF9a5\x14V[[a7\x04\x82Ta0\xA6V[a7\x0F\x82\x82\x85a6OV[_` \x90P`\x1F\x83\x11`\x01\x81\x14a7@W_\x84\x15a7.W\x82\x87\x01Q\x90P[a78\x85\x82a6\xBDV[\x86UPa7\x9FV[`\x1F\x19\x84\x16a7N\x86a5AV[_[\x82\x81\x10\x15a7uW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa7PV[\x86\x83\x10\x15a7\x92W\x84\x89\x01Qa7\x8E`\x1F\x89\x16\x82a6\xA1V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`@\x82\x01\x90Pa7\xBA_\x83\x01\x85a)\x8BV[a7\xC7` \x83\x01\x84a)\xA3V[\x93\x92PPPV[\x7FFEE_TRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a8\x02`\x13\x83a)\xBCV[\x91Pa8\r\x82a7\xCEV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra8/\x81a7\xF6V[\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a8O\x83\x85a)\xBCV[\x93Pa8\\\x83\x85\x84a86V[a8e\x83a)\xDAV[\x84\x01\x90P\x93\x92PPPV[_`\x80\x82\x01\x90Pa8\x83_\x83\x01\x88a)\x8BV[a8\x90` \x83\x01\x87a)\xA3V[\x81\x81\x03`@\x83\x01Ra8\xA3\x81\x85\x87a8DV[\x90Pa8\xB2``\x83\x01\x84a)\xA3V[\x96\x95PPPPPPV[\x7FUNAUTHORIZED_ORACLE\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a8\xF0`\x13\x83a)\xBCV[\x91Pa8\xFB\x82a8\xBCV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra9\x1D\x81a8\xE4V[\x90P\x91\x90PV[\x7FNOT_PROCESSING\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a9X`\x0E\x83a)\xBCV[\x91Pa9c\x82a9$V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra9\x85\x81a9LV[\x90P\x91\x90PV[\x7FOPERATOR_NOT_ASSIGNED\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a9\xC0`\x15\x83a)\xBCV[\x91Pa9\xCB\x82a9\x8CV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra9\xED\x81a9\xB4V[\x90P\x91\x90PV[\x7FNOT_PENDING\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a:(`\x0B\x83a)\xBCV[\x91Pa:3\x82a9\xF4V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra:U\x81a:\x1CV[\x90P\x91\x90PV[\x7FPAYMENT_EXPIRED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a:\x90`\x0F\x83a)\xBCV[\x91Pa:\x9B\x82a:\\V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra:\xBD\x81a:\x84V[\x90P\x91\x90PV[\x7FNOT_ACTIVE_LP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a:\xF8`\r\x83a)\xBCV[\x91Pa;\x03\x82a:\xC4V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;%\x81a:\xECV[\x90P\x91\x90PV[_a;7\x82\x85a4lV[` \x82\x01\x91Pa;G\x82\x84a4,V[`\x14\x82\x01\x91P\x81\x90P\x93\x92PPPV[\x7FINVALID_SLIP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a;\x8B`\x0C\x83a)\xBCV[\x91Pa;\x96\x82a;WV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;\xB8\x81a;\x7FV[\x90P\x91\x90PV[PV[_a;\xCD_\x83a)\xBCV[\x91Pa;\xD8\x82a;\xBFV[_\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;\xF9\x81a;\xC2V[\x90P\x91\x90PV[\x7FNOT_SENDER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a<4`\n\x83a)\xBCV[\x91Pa<?\x82a<\0V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra<a\x81a<(V[\x90P\x91\x90PV[\x7FDEADLINE_NOT_PASSED\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a<\x9C`\x13\x83a)\xBCV[\x91Pa<\xA7\x82a<hV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra<\xC9\x81a<\x90V[\x90P\x91\x90PV[\x7FREFUND_TRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a=\x04`\x16\x83a)\xBCV[\x91Pa=\x0F\x82a<\xD0V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra=1\x81a<\xF8V[\x90P\x91\x90PV[\x7FDeadline expired\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a=l`\x10\x83a)\xBCV[\x91Pa=w\x82a=8V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra=\x99\x81a=`V[\x90P\x91\x90PV[_`@\x82\x01\x90Pa=\xB3_\x83\x01\x85a)\x8BV[a=\xC0` \x83\x01\x84a*\"V[\x93\x92PPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a=\xDC\x81a=\xC7V[\x82RPPV[_`\x80\x82\x01\x90Pa=\xF5_\x83\x01\x87a*\"V[a>\x02` \x83\x01\x86a=\xD3V[a>\x0F`@\x83\x01\x85a*\"V[a>\x1C``\x83\x01\x84a*\"V[\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \xF0\xBCX\xF6q\xAC\x9E3\x80\xE0\x1E\x92\x0B\xD1\xD9t\xD2\x0C\xD6[\x1D]q\x10\x18B\xD4$\x87\x08-*dsolcC\0\x08\x1E\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106101c2575f3560e01c80636d69fcaf116100f757806391d1485411610095578063c415b95c1161006f578063c415b95c146104d1578063d547741f146104ef578063e66eefc81461050b578063f2e9e4181461053b576101c2565b806391d148541461046557806397da034a14610495578063a217fddf146104b3576101c2565b806376319190116100d157806376319190146104035780637994d1a41461041f5780638456cb591461043d5780638c639a8514610447576101c2565b80636d69fcaf146103ad57806371de2ffc146103c957806372f3e8e1146103e5576101c2565b80632f2ff15d116101645780634dd0301f1161013e5780634dd0301f14610327578063550ebe28146103435780635c975abb1461035f57806368c4ac261461037d576101c2565b80632f2ff15d146102e557806336568abe146103015780633f4ba83a1461031d576101c2565b806312e8e2c3116101a057806312e8e2c31461024d5780631e0a505d14610269578063248a9ca3146102995780632bd735ab146102c9576101c2565b806301e8a6bb146101c657806301ffc9a7146101e25780630716326d14610212575b5f5ffd5b6101e060048036038101906101db919061284f565b610559565b005b6101fc60048036038101906101f791906128cf565b610617565b6040516102099190612914565b60405180910390f35b61022c60048036038101906102279190612960565b610690565b6040516102449c9b9a99989796959493929190612aa4565b60405180910390f35b61026760048036038101906102629190612b94565b610861565b005b610283600480360381019061027e9190612c20565b6108f3565b6040516102909190612cb6565b60405180910390f35b6102b360048036038101906102ae9190612960565b610ebb565b6040516102c09190612cb6565b60405180910390f35b6102e360048036038101906102de9190612960565b610ed8565b005b6102ff60048036038101906102fa9190612ccf565b611201565b005b61031b60048036038101906103169190612ccf565b611223565b005b61032561129e565b005b610341600480360381019061033c9190612d62565b6112b5565b005b61035d6004803603810190610358919061284f565b61163d565b005b6103676116fb565b6040516103749190612914565b60405180910390f35b6103976004803603810190610392919061284f565b61170f565b6040516103a49190612914565b60405180910390f35b6103c760048036038101906103c2919061284f565b61172c565b005b6103e360048036038101906103de9190612960565b611842565b005b6103ed611b1b565b6040516103fa9190612dbf565b60405180910390f35b61041d6004803603810190610418919061284f565b611b40565b005b610427611be7565b6040516104349190612dd8565b60405180910390f35b610445611bed565b005b61044f611c04565b60405161045c9190612dd8565b60405180910390f35b61047f600480360381019061047a9190612ccf565b611c0a565b60405161048c9190612914565b60405180910390f35b61049d611c6e565b6040516104aa9190612dbf565b60405180910390f35b6104bb611c93565b6040516104c89190612cb6565b60405180910390f35b6104d9611c99565b6040516104e69190612dbf565b60405180910390f35b61050960048036038101906105049190612ccf565b611cbe565b005b61052560048036038101906105209190612960565b611ce0565b6040516105329190612f7d565b60405180910390f35b610543611fa0565b6040516105509190612ff8565b60405180910390f35b5f5f1b61056581611fc5565b5f73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16036105d3576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016105ca9061305b565b60405180910390fd5b8160035f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505050565b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19161480610689575061068882611fd9565b5b9050919050565b6006602052805f5260405f205f91509050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690806001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16908060020154908060030180546106ff906130a6565b80601f016020809104026020016040519081016040528092919081815260200182805461072b906130a6565b80156107765780601f1061074d57610100808354040283529160200191610776565b820191905f5260205f20905b81548152906001019060200180831161075957829003601f168201915b505050505090806004015490806005015490806006015490806007015490806008015490806009015f9054906101000a900460ff169080600a0180546107bb906130a6565b80601f01602080910402602001604051908101604052809291908181526020018280546107e7906130a6565b80156108325780601f1061080957610100808354040283529160200191610832565b820191905f5260205f20905b81548152906001019060200180831161081557829003601f168201915b50505050509080600b015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690508c565b5f5f1b61086d81611fc5565b60c88211156108b1576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016108a890613120565b60405180910390fd5b816007819055507f45610d581145924dd7090a5017e5f2b1d6f42213bb2e95707ff86846bbfcb1ca826040516108e79190612dd8565b60405180910390a15050565b5f6108fc612042565b610904612064565b60055f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1661098d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161098490613188565b60405180910390fd5b5f86116109cf576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016109c6906131f0565b60405180910390fd5b8673ffffffffffffffffffffffffffffffffffffffff166323b872dd3330896040518463ffffffff1660e01b8152600401610a0c9392919061320e565b6020604051808303815f875af1158015610a28573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a4c919061326d565b610a8b576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610a82906132e2565b60405180910390fd5b5f61271060075488610a9d919061332d565b610aa7919061339b565b90505f8188610ab691906133cb565b90503389898642604051602001610ad1959493929190613483565b6040516020818303038152906040528051906020012092506040518061018001604052803373ffffffffffffffffffffffffffffffffffffffff1681526020018a73ffffffffffffffffffffffffffffffffffffffff16815260200182815260200187878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081526020018881526020015f8152602001858152602001428152602001611c2042610ba291906134e1565b81526020015f6003811115610bba57610bb9612a31565b5b815260200160405180602001604052805f81525081526020015f73ffffffffffffffffffffffffffffffffffffffff1681525060065f8581526020019081526020015f205f820151815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506020820151816001015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550604082015181600201556060820151816003019081610ca791906136d8565b506080820151816004015560a0820151816005015560c0820151816006015560e082015181600701556101008201518160080155610120820151816009015f6101000a81548160ff02191690836003811115610d0657610d05612a31565b5b021790555061014082015181600a019081610d2191906136d8565b5061016082015181600b015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055509050505f821115610e50578873ffffffffffffffffffffffffffffffffffffffff1663a9059cbb60085f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16846040518363ffffffff1660e01b8152600401610dd09291906137a7565b6020604051808303815f875af1158015610dec573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e10919061326d565b610e4f576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e4690613818565b60405180910390fd5b5b3373ffffffffffffffffffffffffffffffffffffffff16837fad3c6549dd317555f3e8872a10664c9a3312a268f0c9c873a80b9c52f180a07c8b848a8a8d604051610e9f959493929190613870565b60405180910390a35050610eb16120a5565b9695505050505050565b5f60015f8381526020019081526020015f20600101549050919050565b610ee0612042565b60035f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610f6f576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610f6690613906565b60405180910390fd5b5f60065f8381526020019081526020015f20905060016003811115610f9757610f96612a31565b5b816009015f9054906101000a900460ff166003811115610fba57610fb9612a31565b5b14610ffa576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610ff19061396e565b60405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff1681600b015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff160361108b576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611082906139d6565b60405180910390fd5b806001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663a9059cbb82600b015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600201546040518363ffffffff1660e01b81526004016111109291906137a7565b6020604051808303815f875af115801561112c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611150919061326d565b61118f576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611186906132e2565b60405180910390fd5b6002816009015f6101000a81548160ff021916908360038111156111b6576111b5612a31565b5b0217905550817f912edf360d10ba8006466028db26d840a68a22b8db84ef0deb3a7fa9c268eef25f5f1b6040516111ed9190612cb6565b60405180910390a2506111fe6120a5565b50565b61120a82610ebb565b61121381611fc5565b61121d83836120bf565b50505050565b61122b6121a8565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161461128f576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61129982826121af565b505050565b5f5f1b6112aa81611fc5565b6112b2612299565b50565b6112bd612064565b5f60065f8581526020019081526020015f2090505f60038111156112e4576112e3612a31565b5b816009015f9054906101000a900460ff16600381111561130757611306612a31565b5b14611347576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161133e90613a3e565b60405180910390fd5b806008015442111561138e576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161138590613aa6565b60405180910390fd5b60025f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663b5eb4539336040518263ffffffff1660e01b81526004016113e89190612dbf565b602060405180830381865afa158015611403573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611427919061326d565b611466576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161145d90613b0e565b60405180910390fd5b5f843360405160200161147a929190613b2c565b6040516020818303038152906040528051906020012090505f6114e985858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050836122f990919063ffffffff16565b905060045f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161461157a576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161157190613ba1565b60405180910390fd5b6001836009015f6101000a81548160ff021916908360038111156115a1576115a0612a31565b5b02179055503383600b015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055503373ffffffffffffffffffffffffffffffffffffffff16867f9663413acc8b4a21ea9ca7c900fd3bb1fbe4e1ab6e7e60cab595b1a54027025160405161162d90613be2565b60405180910390a3505050505050565b5f5f1b61164981611fc5565b5f73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16036116b7576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016116ae9061305b565b60405180910390fd5b8160045f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505050565b5f5f5f9054906101000a900460ff16905090565b6005602052805f5260405f205f915054906101000a900460ff1681565b5f5f1b61173881611fc5565b5f73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16036117a6576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161179d9061305b565b60405180910390fd5b600160055f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055508173ffffffffffffffffffffffffffffffffffffffff167fd1be2e90bd3d24839d9dd94ad871068e1f9688b02fa43f2a62c9975dfa9de2d760405160405180910390a25050565b61184a612042565b5f60065f8381526020019081526020015f2090503373ffffffffffffffffffffffffffffffffffffffff16815f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16146118ee576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016118e590613c4a565b60405180910390fd5b5f600381111561190157611900612a31565b5b816009015f9054906101000a900460ff16600381111561192457611923612a31565b5b14611964576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161195b90613a3e565b60405180910390fd5b806008015442116119aa576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016119a190613cb2565b60405180910390fd5b806001015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663a9059cbb825f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1683600201546040518363ffffffff1660e01b8152600401611a2e9291906137a7565b6020604051808303815f875af1158015611a4a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a6e919061326d565b611aad576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611aa490613d1a565b60405180910390fd5b6003816009015f6101000a81548160ff02191690836003811115611ad457611ad3612a31565b5b0217905550817fa2c0cfcfdd46ca4b148dde16939db4dbf0481430d552d486f78e076410689be9604051611b0790613d82565b60405180910390a250611b186120a5565b50565b60045f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b5f5f1b611b4c81611fc5565b5f60055f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055508173ffffffffffffffffffffffffffffffffffffffff167fbea12876694c4055c71f74308f752b9027cf3d554194000a366abddfc239a30660405160405180910390a25050565b611c2081565b5f5f1b611bf981611fc5565b611c01612323565b50565b60075481565b5f60015f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b60035f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b5f5f1b81565b60085f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b611cc782610ebb565b611cd081611fc5565b611cda83836121af565b50505050565b611ce8612742565b60065f8381526020019081526020015f20604051806101800160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160028201548152602001600382018054611dc8906130a6565b80601f0160208091040260200160405190810160405280929190818152602001828054611df4906130a6565b8015611e3f5780601f10611e1657610100808354040283529160200191611e3f565b820191905f5260205f20905b815481529060010190602001808311611e2257829003601f168201915b505050505081526020016004820154815260200160058201548152602001600682015481526020016007820154815260200160088201548152602001600982015f9054906101000a900460ff166003811115611e9e57611e9d612a31565b5b6003811115611eb057611eaf612a31565b5b8152602001600a82018054611ec4906130a6565b80601f0160208091040260200160405190810160405280929190818152602001828054611ef0906130a6565b8015611f3b5780601f10611f1257610100808354040283529160200191611f3b565b820191905f5260205f20905b815481529060010190602001808311611f1e57829003601f168201915b50505050508152602001600b82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815250509050919050565b60025f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b611fd681611fd16121a8565b612384565b50565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b61204a6123d5565b600261205c612057612416565b61243f565b5f0181905550565b61206c6116fb565b156120a3576040517fd93c066500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b60016120b76120b2612416565b61243f565b5f0181905550565b5f6120ca8383611c0a565b61219e576001805f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555061213b6121a8565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a4600190506121a2565b5f90505b92915050565b5f33905090565b5f6121ba8383611c0a565b1561228f575f60015f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555061222c6121a8565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a460019050612293565b5f90505b92915050565b6122a1612448565b5f5f5f6101000a81548160ff0219169083151502179055507f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa6122e26121a8565b6040516122ef9190612dbf565b60405180910390a1565b5f5f5f5f6123078686612488565b92509250925061231782826124dd565b82935050505092915050565b61232b612064565b60015f5f6101000a81548160ff0219169083151502179055507f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a25861236d6121a8565b60405161237a9190612dbf565b60405180910390a1565b61238e8282611c0a565b6123d15780826040517fe2517d3f0000000000000000000000000000000000000000000000000000000081526004016123c8929190613da0565b60405180910390fd5b5050565b6123dd61263f565b15612414576040517f3ee5aeb500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005f1b905090565b5f819050919050565b6124506116fb565b612486576040517f8dfc202b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f5f5f60418451036124c8575f5f5f602087015192506040870151915060608701515f1a90506124ba8882858561265b565b9550955095505050506124d6565b5f600285515f1b9250925092505b9250925092565b5f60038111156124f0576124ef612a31565b5b82600381111561250357612502612a31565b5b031561263b576001600381111561251d5761251c612a31565b5b8260038111156125305761252f612a31565b5b03612567576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6002600381111561257b5761257a612a31565b5b82600381111561258e5761258d612a31565b5b036125d257805f1c6040517ffce698f70000000000000000000000000000000000000000000000000000000081526004016125c99190612dd8565b60405180910390fd5b6003808111156125e5576125e4612a31565b5b8260038111156125f8576125f7612a31565b5b0361263a57806040517fd78bce0c0000000000000000000000000000000000000000000000000000000081526004016126319190612cb6565b60405180910390fd5b5b5050565b5f600261265261264d612416565b61243f565b5f015414905090565b5f5f5f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115612697575f600385925092509250612738565b5f6001888888886040515f81526020016040526040516126ba9493929190613de2565b6020604051602081039080840390855afa1580156126da573d5f5f3e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff160361272b575f60015f5f1b93509350935050612738565b805f5f5f1b935093509350505b9450945094915050565b6040518061018001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f8152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f60038111156127c4576127c3612a31565b5b8152602001606081526020015f73ffffffffffffffffffffffffffffffffffffffff1681525090565b5f5ffd5b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61281e826127f5565b9050919050565b61282e81612814565b8114612838575f5ffd5b50565b5f8135905061284981612825565b92915050565b5f60208284031215612864576128636127ed565b5b5f6128718482850161283b565b91505092915050565b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b6128ae8161287a565b81146128b8575f5ffd5b50565b5f813590506128c9816128a5565b92915050565b5f602082840312156128e4576128e36127ed565b5b5f6128f1848285016128bb565b91505092915050565b5f8115159050919050565b61290e816128fa565b82525050565b5f6020820190506129275f830184612905565b92915050565b5f819050919050565b61293f8161292d565b8114612949575f5ffd5b50565b5f8135905061295a81612936565b92915050565b5f60208284031215612975576129746127ed565b5b5f6129828482850161294c565b91505092915050565b61299481612814565b82525050565b5f819050919050565b6129ac8161299a565b82525050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f601f19601f8301169050919050565b5f6129f4826129b2565b6129fe81856129bc565b9350612a0e8185602086016129cc565b612a17816129da565b840191505092915050565b612a2b8161292d565b82525050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60048110612a6f57612a6e612a31565b5b50565b5f819050612a7f82612a5e565b919050565b5f612a8e82612a72565b9050919050565b612a9e81612a84565b82525050565b5f61018082019050612ab85f83018f61298b565b612ac5602083018e61298b565b612ad2604083018d6129a3565b8181036060830152612ae4818c6129ea565b9050612af3608083018b6129a3565b612b0060a083018a6129a3565b612b0d60c0830189612a22565b612b1a60e08301886129a3565b612b286101008301876129a3565b612b36610120830186612a95565b818103610140830152612b4981856129ea565b9050612b5961016083018461298b565b9d9c50505050505050505050505050565b612b738161299a565b8114612b7d575f5ffd5b50565b5f81359050612b8e81612b6a565b92915050565b5f60208284031215612ba957612ba86127ed565b5b5f612bb684828501612b80565b91505092915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f840112612be057612bdf612bbf565b5b8235905067ffffffffffffffff811115612bfd57612bfc612bc3565b5b602083019150836001820283011115612c1957612c18612bc7565b5b9250929050565b5f5f5f5f5f5f60a08789031215612c3a57612c396127ed565b5b5f612c4789828a0161283b565b9650506020612c5889828a01612b80565b9550506040612c6989828a01612b80565b945050606087013567ffffffffffffffff811115612c8a57612c896127f1565b5b612c9689828a01612bcb565b93509350506080612ca989828a0161294c565b9150509295509295509295565b5f602082019050612cc95f830184612a22565b92915050565b5f5f60408385031215612ce557612ce46127ed565b5b5f612cf28582860161294c565b9250506020612d038582860161283b565b9150509250929050565b5f5f83601f840112612d2257612d21612bbf565b5b8235905067ffffffffffffffff811115612d3f57612d3e612bc3565b5b602083019150836001820283011115612d5b57612d5a612bc7565b5b9250929050565b5f5f5f60408486031215612d7957612d786127ed565b5b5f612d868682870161294c565b935050602084013567ffffffffffffffff811115612da757612da66127f1565b5b612db386828701612d0d565b92509250509250925092565b5f602082019050612dd25f83018461298b565b92915050565b5f602082019050612deb5f8301846129a3565b92915050565b612dfa81612814565b82525050565b612e098161299a565b82525050565b5f82825260208201905092915050565b5f612e29826129b2565b612e338185612e0f565b9350612e438185602086016129cc565b612e4c816129da565b840191505092915050565b612e608161292d565b82525050565b612e6f81612a84565b82525050565b5f61018083015f830151612e8b5f860182612df1565b506020830151612e9e6020860182612df1565b506040830151612eb16040860182612e00565b5060608301518482036060860152612ec98282612e1f565b9150506080830151612ede6080860182612e00565b5060a0830151612ef160a0860182612e00565b5060c0830151612f0460c0860182612e57565b5060e0830151612f1760e0860182612e00565b50610100830151612f2c610100860182612e00565b50610120830151612f41610120860182612e66565b50610140830151848203610140860152612f5b8282612e1f565b915050610160830151612f72610160860182612df1565b508091505092915050565b5f6020820190508181035f830152612f958184612e75565b905092915050565b5f819050919050565b5f612fc0612fbb612fb6846127f5565b612f9d565b6127f5565b9050919050565b5f612fd182612fa6565b9050919050565b5f612fe282612fc7565b9050919050565b612ff281612fd8565b82525050565b5f60208201905061300b5f830184612fe9565b92915050565b7f5a45524f5f4144445245535300000000000000000000000000000000000000005f82015250565b5f613045600c836129bc565b915061305082613011565b602082019050919050565b5f6020820190508181035f83015261307281613039565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f60028204905060018216806130bd57607f821691505b6020821081036130d0576130cf613079565b5b50919050565b7f4645455f544f4f5f4849474800000000000000000000000000000000000000005f82015250565b5f61310a600c836129bc565b9150613115826130d6565b602082019050919050565b5f6020820190508181035f830152613137816130fe565b9050919050565b7f544f4b454e5f4e4f545f535550504f52544544000000000000000000000000005f82015250565b5f6131726013836129bc565b915061317d8261313e565b602082019050919050565b5f6020820190508181035f83015261319f81613166565b9050919050565b7f414d4f554e545f49535f5a45524f0000000000000000000000000000000000005f82015250565b5f6131da600e836129bc565b91506131e5826131a6565b602082019050919050565b5f6020820190508181035f830152613207816131ce565b9050919050565b5f6060820190506132215f83018661298b565b61322e602083018561298b565b61323b60408301846129a3565b949350505050565b61324c816128fa565b8114613256575f5ffd5b50565b5f8151905061326781613243565b92915050565b5f60208284031215613282576132816127ed565b5b5f61328f84828501613259565b91505092915050565b7f5452414e534645525f4641494c454400000000000000000000000000000000005f82015250565b5f6132cc600f836129bc565b91506132d782613298565b602082019050919050565b5f6020820190508181035f8301526132f9816132c0565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6133378261299a565b91506133428361299a565b92508282026133508161299a565b9150828204841483151761336757613366613300565b5b5092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f6133a58261299a565b91506133b08361299a565b9250826133c0576133bf61336e565b5b828204905092915050565b5f6133d58261299a565b91506133e08361299a565b92508282039050818111156133f8576133f7613300565b5b92915050565b5f8160601b9050919050565b5f613414826133fe565b9050919050565b5f6134258261340a565b9050919050565b61343d61343882612814565b61341b565b82525050565b5f819050919050565b61345d6134588261299a565b613443565b82525050565b5f819050919050565b61347d6134788261292d565b613463565b82525050565b5f61348e828861342c565b60148201915061349e828761342c565b6014820191506134ae828661344c565b6020820191506134be828561346c565b6020820191506134ce828461344c565b6020820191508190509695505050505050565b5f6134eb8261299a565b91506134f68361299a565b925082820190508082111561350e5761350d613300565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f6008830261359d7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82613562565b6135a78683613562565b95508019841693508086168417925050509392505050565b5f6135d96135d46135cf8461299a565b612f9d565b61299a565b9050919050565b5f819050919050565b6135f2836135bf565b6136066135fe826135e0565b84845461356e565b825550505050565b5f5f905090565b61361d61360e565b6136288184846135e9565b505050565b5b8181101561364b576136405f82613615565b60018101905061362e565b5050565b601f8211156136905761366181613541565b61366a84613553565b81016020851015613679578190505b61368d61368585613553565b83018261362d565b50505b505050565b5f82821c905092915050565b5f6136b05f1984600802613695565b1980831691505092915050565b5f6136c883836136a1565b9150826002028217905092915050565b6136e1826129b2565b67ffffffffffffffff8111156136fa576136f9613514565b5b61370482546130a6565b61370f82828561364f565b5f60209050601f831160018114613740575f841561372e578287015190505b61373885826136bd565b86555061379f565b601f19841661374e86613541565b5f5b8281101561377557848901518255600182019150602085019450602081019050613750565b86831015613792578489015161378e601f8916826136a1565b8355505b6001600288020188555050505b505050505050565b5f6040820190506137ba5f83018561298b565b6137c760208301846129a3565b9392505050565b7f4645455f5452414e534645525f4641494c4544000000000000000000000000005f82015250565b5f6138026013836129bc565b915061380d826137ce565b602082019050919050565b5f6020820190508181035f83015261382f816137f6565b9050919050565b828183375f83830152505050565b5f61384f83856129bc565b935061385c838584613836565b613865836129da565b840190509392505050565b5f6080820190506138835f83018861298b565b61389060208301876129a3565b81810360408301526138a3818587613844565b90506138b260608301846129a3565b9695505050505050565b7f554e415554484f52495a45445f4f5241434c45000000000000000000000000005f82015250565b5f6138f06013836129bc565b91506138fb826138bc565b602082019050919050565b5f6020820190508181035f83015261391d816138e4565b9050919050565b7f4e4f545f50524f43455353494e470000000000000000000000000000000000005f82015250565b5f613958600e836129bc565b915061396382613924565b602082019050919050565b5f6020820190508181035f8301526139858161394c565b9050919050565b7f4f50455241544f525f4e4f545f41535349474e454400000000000000000000005f82015250565b5f6139c06015836129bc565b91506139cb8261398c565b602082019050919050565b5f6020820190508181035f8301526139ed816139b4565b9050919050565b7f4e4f545f50454e44494e470000000000000000000000000000000000000000005f82015250565b5f613a28600b836129bc565b9150613a33826139f4565b602082019050919050565b5f6020820190508181035f830152613a5581613a1c565b9050919050565b7f5041594d454e545f4558504952454400000000000000000000000000000000005f82015250565b5f613a90600f836129bc565b9150613a9b82613a5c565b602082019050919050565b5f6020820190508181035f830152613abd81613a84565b9050919050565b7f4e4f545f4143544956455f4c50000000000000000000000000000000000000005f82015250565b5f613af8600d836129bc565b9150613b0382613ac4565b602082019050919050565b5f6020820190508181035f830152613b2581613aec565b9050919050565b5f613b37828561346c565b602082019150613b47828461342c565b6014820191508190509392505050565b7f494e56414c49445f534c495000000000000000000000000000000000000000005f82015250565b5f613b8b600c836129bc565b9150613b9682613b57565b602082019050919050565b5f6020820190508181035f830152613bb881613b7f565b9050919050565b50565b5f613bcd5f836129bc565b9150613bd882613bbf565b5f82019050919050565b5f6020820190508181035f830152613bf981613bc2565b9050919050565b7f4e4f545f53454e444552000000000000000000000000000000000000000000005f82015250565b5f613c34600a836129bc565b9150613c3f82613c00565b602082019050919050565b5f6020820190508181035f830152613c6181613c28565b9050919050565b7f444541444c494e455f4e4f545f504153534544000000000000000000000000005f82015250565b5f613c9c6013836129bc565b9150613ca782613c68565b602082019050919050565b5f6020820190508181035f830152613cc981613c90565b9050919050565b7f524546554e445f5452414e534645525f4641494c4544000000000000000000005f82015250565b5f613d046016836129bc565b9150613d0f82613cd0565b602082019050919050565b5f6020820190508181035f830152613d3181613cf8565b9050919050565b7f446561646c696e652065787069726564000000000000000000000000000000005f82015250565b5f613d6c6010836129bc565b9150613d7782613d38565b602082019050919050565b5f6020820190508181035f830152613d9981613d60565b9050919050565b5f604082019050613db35f83018561298b565b613dc06020830184612a22565b9392505050565b5f60ff82169050919050565b613ddc81613dc7565b82525050565b5f608082019050613df55f830187612a22565b613e026020830186613dd3565b613e0f6040830185612a22565b613e1c6060830184612a22565b9594505050505056fea2646970667358221220f0bc58f671ac9e3380e01e920bd1d974d20cd65b1d5d71101842d42487082d2a64736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xC2W_5`\xE0\x1C\x80cmi\xFC\xAF\x11a\0\xF7W\x80c\x91\xD1HT\x11a\0\x95W\x80c\xC4\x15\xB9\\\x11a\0oW\x80c\xC4\x15\xB9\\\x14a\x04\xD1W\x80c\xD5Gt\x1F\x14a\x04\xEFW\x80c\xE6n\xEF\xC8\x14a\x05\x0BW\x80c\xF2\xE9\xE4\x18\x14a\x05;Wa\x01\xC2V[\x80c\x91\xD1HT\x14a\x04eW\x80c\x97\xDA\x03J\x14a\x04\x95W\x80c\xA2\x17\xFD\xDF\x14a\x04\xB3Wa\x01\xC2V[\x80cv1\x91\x90\x11a\0\xD1W\x80cv1\x91\x90\x14a\x04\x03W\x80cy\x94\xD1\xA4\x14a\x04\x1FW\x80c\x84V\xCBY\x14a\x04=W\x80c\x8Cc\x9A\x85\x14a\x04GWa\x01\xC2V[\x80cmi\xFC\xAF\x14a\x03\xADW\x80cq\xDE/\xFC\x14a\x03\xC9W\x80cr\xF3\xE8\xE1\x14a\x03\xE5Wa\x01\xC2V[\x80c//\xF1]\x11a\x01dW\x80cM\xD00\x1F\x11a\x01>W\x80cM\xD00\x1F\x14a\x03'W\x80cU\x0E\xBE(\x14a\x03CW\x80c\\\x97Z\xBB\x14a\x03_W\x80ch\xC4\xAC&\x14a\x03}Wa\x01\xC2V[\x80c//\xF1]\x14a\x02\xE5W\x80c6V\x8A\xBE\x14a\x03\x01W\x80c?K\xA8:\x14a\x03\x1DWa\x01\xC2V[\x80c\x12\xE8\xE2\xC3\x11a\x01\xA0W\x80c\x12\xE8\xE2\xC3\x14a\x02MW\x80c\x1E\nP]\x14a\x02iW\x80c$\x8A\x9C\xA3\x14a\x02\x99W\x80c+\xD75\xAB\x14a\x02\xC9Wa\x01\xC2V[\x80c\x01\xE8\xA6\xBB\x14a\x01\xC6W\x80c\x01\xFF\xC9\xA7\x14a\x01\xE2W\x80c\x07\x162m\x14a\x02\x12W[__\xFD[a\x01\xE0`\x04\x806\x03\x81\x01\x90a\x01\xDB\x91\x90a(OV[a\x05YV[\0[a\x01\xFC`\x04\x806\x03\x81\x01\x90a\x01\xF7\x91\x90a(\xCFV[a\x06\x17V[`@Qa\x02\t\x91\x90a)\x14V[`@Q\x80\x91\x03\x90\xF3[a\x02,`\x04\x806\x03\x81\x01\x90a\x02'\x91\x90a)`V[a\x06\x90V[`@Qa\x02D\x9C\x9B\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a*\xA4V[`@Q\x80\x91\x03\x90\xF3[a\x02g`\x04\x806\x03\x81\x01\x90a\x02b\x91\x90a+\x94V[a\x08aV[\0[a\x02\x83`\x04\x806\x03\x81\x01\x90a\x02~\x91\x90a, V[a\x08\xF3V[`@Qa\x02\x90\x91\x90a,\xB6V[`@Q\x80\x91\x03\x90\xF3[a\x02\xB3`\x04\x806\x03\x81\x01\x90a\x02\xAE\x91\x90a)`V[a\x0E\xBBV[`@Qa\x02\xC0\x91\x90a,\xB6V[`@Q\x80\x91\x03\x90\xF3[a\x02\xE3`\x04\x806\x03\x81\x01\x90a\x02\xDE\x91\x90a)`V[a\x0E\xD8V[\0[a\x02\xFF`\x04\x806\x03\x81\x01\x90a\x02\xFA\x91\x90a,\xCFV[a\x12\x01V[\0[a\x03\x1B`\x04\x806\x03\x81\x01\x90a\x03\x16\x91\x90a,\xCFV[a\x12#V[\0[a\x03%a\x12\x9EV[\0[a\x03A`\x04\x806\x03\x81\x01\x90a\x03<\x91\x90a-bV[a\x12\xB5V[\0[a\x03]`\x04\x806\x03\x81\x01\x90a\x03X\x91\x90a(OV[a\x16=V[\0[a\x03ga\x16\xFBV[`@Qa\x03t\x91\x90a)\x14V[`@Q\x80\x91\x03\x90\xF3[a\x03\x97`\x04\x806\x03\x81\x01\x90a\x03\x92\x91\x90a(OV[a\x17\x0FV[`@Qa\x03\xA4\x91\x90a)\x14V[`@Q\x80\x91\x03\x90\xF3[a\x03\xC7`\x04\x806\x03\x81\x01\x90a\x03\xC2\x91\x90a(OV[a\x17,V[\0[a\x03\xE3`\x04\x806\x03\x81\x01\x90a\x03\xDE\x91\x90a)`V[a\x18BV[\0[a\x03\xEDa\x1B\x1BV[`@Qa\x03\xFA\x91\x90a-\xBFV[`@Q\x80\x91\x03\x90\xF3[a\x04\x1D`\x04\x806\x03\x81\x01\x90a\x04\x18\x91\x90a(OV[a\x1B@V[\0[a\x04'a\x1B\xE7V[`@Qa\x044\x91\x90a-\xD8V[`@Q\x80\x91\x03\x90\xF3[a\x04Ea\x1B\xEDV[\0[a\x04Oa\x1C\x04V[`@Qa\x04\\\x91\x90a-\xD8V[`@Q\x80\x91\x03\x90\xF3[a\x04\x7F`\x04\x806\x03\x81\x01\x90a\x04z\x91\x90a,\xCFV[a\x1C\nV[`@Qa\x04\x8C\x91\x90a)\x14V[`@Q\x80\x91\x03\x90\xF3[a\x04\x9Da\x1CnV[`@Qa\x04\xAA\x91\x90a-\xBFV[`@Q\x80\x91\x03\x90\xF3[a\x04\xBBa\x1C\x93V[`@Qa\x04\xC8\x91\x90a,\xB6V[`@Q\x80\x91\x03\x90\xF3[a\x04\xD9a\x1C\x99V[`@Qa\x04\xE6\x91\x90a-\xBFV[`@Q\x80\x91\x03\x90\xF3[a\x05\t`\x04\x806\x03\x81\x01\x90a\x05\x04\x91\x90a,\xCFV[a\x1C\xBEV[\0[a\x05%`\x04\x806\x03\x81\x01\x90a\x05 \x91\x90a)`V[a\x1C\xE0V[`@Qa\x052\x91\x90a/}V[`@Q\x80\x91\x03\x90\xF3[a\x05Ca\x1F\xA0V[`@Qa\x05P\x91\x90a/\xF8V[`@Q\x80\x91\x03\x90\xF3[__\x1Ba\x05e\x81a\x1F\xC5V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x05\xD3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xCA\x90a0[V[`@Q\x80\x91\x03\x90\xFD[\x81`\x03_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\x89WPa\x06\x88\x82a\x1F\xD9V[[\x90P\x91\x90PV[`\x06` R\x80_R`@_ _\x91P\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80`\x02\x01T\x90\x80`\x03\x01\x80Ta\x06\xFF\x90a0\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07+\x90a0\xA6V[\x80\x15a\x07vW\x80`\x1F\x10a\x07MWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07vV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07YW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x04\x01T\x90\x80`\x05\x01T\x90\x80`\x06\x01T\x90\x80`\x07\x01T\x90\x80`\x08\x01T\x90\x80`\t\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90\x80`\n\x01\x80Ta\x07\xBB\x90a0\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xE7\x90a0\xA6V[\x80\x15a\x082W\x80`\x1F\x10a\x08\tWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x082V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\x15W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x0B\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x8CV[__\x1Ba\x08m\x81a\x1F\xC5V[`\xC8\x82\x11\x15a\x08\xB1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xA8\x90a1 V[`@Q\x80\x91\x03\x90\xFD[\x81`\x07\x81\x90UP\x7FEa\rX\x11E\x92M\xD7\t\nP\x17\xE5\xF2\xB1\xD6\xF4\"\x13\xBB.\x95p\x7F\xF8hF\xBB\xFC\xB1\xCA\x82`@Qa\x08\xE7\x91\x90a-\xD8V[`@Q\x80\x91\x03\x90\xA1PPV[_a\x08\xFCa BV[a\t\x04a dV[`\x05_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\t\x8DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\x84\x90a1\x88V[`@Q\x80\x91\x03\x90\xFD[_\x86\x11a\t\xCFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xC6\x90a1\xF0V[`@Q\x80\x91\x03\x90\xFD[\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD30\x89`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x0C\x93\x92\x91\x90a2\x0EV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n(W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nL\x91\x90a2mV[a\n\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\x82\x90a2\xE2V[`@Q\x80\x91\x03\x90\xFD[_a'\x10`\x07T\x88a\n\x9D\x91\x90a3-V[a\n\xA7\x91\x90a3\x9BV[\x90P_\x81\x88a\n\xB6\x91\x90a3\xCBV[\x90P3\x89\x89\x86B`@Q` \x01a\n\xD1\x95\x94\x93\x92\x91\x90a4\x83V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92P`@Q\x80a\x01\x80\x01`@R\x803s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81R` \x01\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x88\x81R` \x01_\x81R` \x01\x85\x81R` \x01B\x81R` \x01a\x1C Ba\x0B\xA2\x91\x90a4\xE1V[\x81R` \x01_`\x03\x81\x11\x15a\x0B\xBAWa\x0B\xB9a*1V[[\x81R` \x01`@Q\x80` \x01`@R\x80_\x81RP\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP`\x06_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01\x90\x81a\x0C\xA7\x91\x90a6\xD8V[P`\x80\x82\x01Q\x81`\x04\x01U`\xA0\x82\x01Q\x81`\x05\x01U`\xC0\x82\x01Q\x81`\x06\x01U`\xE0\x82\x01Q\x81`\x07\x01Ua\x01\0\x82\x01Q\x81`\x08\x01Ua\x01 \x82\x01Q\x81`\t\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\r\x06Wa\r\x05a*1V[[\x02\x17\x90UPa\x01@\x82\x01Q\x81`\n\x01\x90\x81a\r!\x91\x90a6\xD8V[Pa\x01`\x82\x01Q\x81`\x0B\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x90PP_\x82\x11\x15a\x0EPW\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB`\x08_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xD0\x92\x91\x90a7\xA7V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\r\xECW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x10\x91\x90a2mV[a\x0EOW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0EF\x90a8\x18V[`@Q\x80\x91\x03\x90\xFD[[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xAD<eI\xDD1uU\xF3\xE8\x87*\x10fL\x9A3\x12\xA2h\xF0\xC9\xC8s\xA8\x0B\x9CR\xF1\x80\xA0|\x8B\x84\x8A\x8A\x8D`@Qa\x0E\x9F\x95\x94\x93\x92\x91\x90a8pV[`@Q\x80\x91\x03\x90\xA3PPa\x0E\xB1a \xA5V[\x96\x95PPPPPPV[_`\x01_\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[a\x0E\xE0a BV[`\x03_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0FoW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Ff\x90a9\x06V[`@Q\x80\x91\x03\x90\xFD[_`\x06_\x83\x81R` \x01\x90\x81R` \x01_ \x90P`\x01`\x03\x81\x11\x15a\x0F\x97Wa\x0F\x96a*1V[[\x81`\t\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x0F\xBAWa\x0F\xB9a*1V[[\x14a\x0F\xFAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xF1\x90a9nV[`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x0B\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x10\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x82\x90a9\xD6V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x82`\x0B\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x02\x01T`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\x10\x92\x91\x90a7\xA7V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x11,W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11P\x91\x90a2mV[a\x11\x8FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\x86\x90a2\xE2V[`@Q\x80\x91\x03\x90\xFD[`\x02\x81`\t\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\x11\xB6Wa\x11\xB5a*1V[[\x02\x17\x90UP\x81\x7F\x91.\xDF6\r\x10\xBA\x80\x06F`(\xDB&\xD8@\xA6\x8A\"\xB8\xDB\x84\xEF\r\xEB:\x7F\xA9\xC2h\xEE\xF2__\x1B`@Qa\x11\xED\x91\x90a,\xB6V[`@Q\x80\x91\x03\x90\xA2Pa\x11\xFEa \xA5V[PV[a\x12\n\x82a\x0E\xBBV[a\x12\x13\x81a\x1F\xC5V[a\x12\x1D\x83\x83a \xBFV[PPPPV[a\x12+a!\xA8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x12\x8FW`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\x99\x82\x82a!\xAFV[PPPV[__\x1Ba\x12\xAA\x81a\x1F\xC5V[a\x12\xB2a\"\x99V[PV[a\x12\xBDa dV[_`\x06_\x85\x81R` \x01\x90\x81R` \x01_ \x90P_`\x03\x81\x11\x15a\x12\xE4Wa\x12\xE3a*1V[[\x81`\t\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x13\x07Wa\x13\x06a*1V[[\x14a\x13GW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13>\x90a:>V[`@Q\x80\x91\x03\x90\xFD[\x80`\x08\x01TB\x11\x15a\x13\x8EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\x85\x90a:\xA6V[`@Q\x80\x91\x03\x90\xFD[`\x02_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xB5\xEBE93`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xE8\x91\x90a-\xBFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\x03W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14'\x91\x90a2mV[a\x14fW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14]\x90a;\x0EV[`@Q\x80\x91\x03\x90\xFD[_\x843`@Q` \x01a\x14z\x92\x91\x90a;,V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_a\x14\xE9\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x83a\"\xF9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\x04_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x15zW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15q\x90a;\xA1V[`@Q\x80\x91\x03\x90\xFD[`\x01\x83`\t\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\x15\xA1Wa\x15\xA0a*1V[[\x02\x17\x90UP3\x83`\x0B\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x7F\x96cA:\xCC\x8BJ!\xEA\x9C\xA7\xC9\0\xFD;\xB1\xFB\xE4\xE1\xABn~`\xCA\xB5\x95\xB1\xA5@'\x02Q`@Qa\x16-\x90a;\xE2V[`@Q\x80\x91\x03\x90\xA3PPPPPPV[__\x1Ba\x16I\x81a\x1F\xC5V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x16\xB7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16\xAE\x90a0[V[`@Q\x80\x91\x03\x90\xFD[\x81`\x04_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[___\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[`\x05` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[__\x1Ba\x178\x81a\x1F\xC5V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x17\xA6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17\x9D\x90a0[V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x05_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xD1\xBE.\x90\xBD=$\x83\x9D\x9D\xD9J\xD8q\x06\x8E\x1F\x96\x88\xB0/\xA4?*b\xC9\x97]\xFA\x9D\xE2\xD7`@Q`@Q\x80\x91\x03\x90\xA2PPV[a\x18Ja BV[_`\x06_\x83\x81R` \x01\x90\x81R` \x01_ \x90P3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x18\xEEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xE5\x90a<JV[`@Q\x80\x91\x03\x90\xFD[_`\x03\x81\x11\x15a\x19\x01Wa\x19\0a*1V[[\x81`\t\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x19$Wa\x19#a*1V[[\x14a\x19dW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19[\x90a:>V[`@Q\x80\x91\x03\x90\xFD[\x80`\x08\x01TB\x11a\x19\xAAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\xA1\x90a<\xB2V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x82_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x02\x01T`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A.\x92\x91\x90a7\xA7V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1AJW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1An\x91\x90a2mV[a\x1A\xADW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A\xA4\x90a=\x1AV[`@Q\x80\x91\x03\x90\xFD[`\x03\x81`\t\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\x1A\xD4Wa\x1A\xD3a*1V[[\x02\x17\x90UP\x81\x7F\xA2\xC0\xCF\xCF\xDDF\xCAK\x14\x8D\xDE\x16\x93\x9D\xB4\xDB\xF0H\x140\xD5R\xD4\x86\xF7\x8E\x07d\x10h\x9B\xE9`@Qa\x1B\x07\x90a=\x82V[`@Q\x80\x91\x03\x90\xA2Pa\x1B\x18a \xA5V[PV[`\x04_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[__\x1Ba\x1BL\x81a\x1F\xC5V[_`\x05_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBE\xA1(viL@U\xC7\x1Ft0\x8Fu+\x90'\xCF=UA\x94\0\n6j\xBD\xDF\xC29\xA3\x06`@Q`@Q\x80\x91\x03\x90\xA2PPV[a\x1C \x81V[__\x1Ba\x1B\xF9\x81a\x1F\xC5V[a\x1C\x01a##V[PV[`\x07T\x81V[_`\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[`\x03_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[__\x1B\x81V[`\x08_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x1C\xC7\x82a\x0E\xBBV[a\x1C\xD0\x81a\x1F\xC5V[a\x1C\xDA\x83\x83a!\xAFV[PPPPV[a\x1C\xE8a'BV[`\x06_\x83\x81R` \x01\x90\x81R` \x01_ `@Q\x80a\x01\x80\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01\x80Ta\x1D\xC8\x90a0\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1D\xF4\x90a0\xA6V[\x80\x15a\x1E?W\x80`\x1F\x10a\x1E\x16Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E?V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E\"W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01T\x81R` \x01`\x06\x82\x01T\x81R` \x01`\x07\x82\x01T\x81R` \x01`\x08\x82\x01T\x81R` \x01`\t\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x1E\x9EWa\x1E\x9Da*1V[[`\x03\x81\x11\x15a\x1E\xB0Wa\x1E\xAFa*1V[[\x81R` \x01`\n\x82\x01\x80Ta\x1E\xC4\x90a0\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E\xF0\x90a0\xA6V[\x80\x15a\x1F;W\x80`\x1F\x10a\x1F\x12Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F;V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\x1EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x0B\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x91\x90PV[`\x02_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x1F\xD6\x81a\x1F\xD1a!\xA8V[a#\x84V[PV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[a Ja#\xD5V[`\x02a \\a Wa$\x16V[a$?V[_\x01\x81\x90UPV[a la\x16\xFBV[\x15a \xA3W`@Q\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\x01a \xB7a \xB2a$\x16V[a$?V[_\x01\x81\x90UPV[_a \xCA\x83\x83a\x1C\nV[a!\x9EW`\x01\x80_\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa!;a!\xA8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa!\xA2V[_\x90P[\x92\x91PPV[_3\x90P\x90V[_a!\xBA\x83\x83a\x1C\nV[\x15a\"\x8FW_`\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\",a!\xA8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\"\x93V[_\x90P[\x92\x91PPV[a\"\xA1a$HV[___a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAAa\"\xE2a!\xA8V[`@Qa\"\xEF\x91\x90a-\xBFV[`@Q\x80\x91\x03\x90\xA1V[____a#\x07\x86\x86a$\x88V[\x92P\x92P\x92Pa#\x17\x82\x82a$\xDDV[\x82\x93PPPP\x92\x91PPV[a#+a dV[`\x01__a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa#ma!\xA8V[`@Qa#z\x91\x90a-\xBFV[`@Q\x80\x91\x03\x90\xA1V[a#\x8E\x82\x82a\x1C\nV[a#\xD1W\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#\xC8\x92\x91\x90a=\xA0V[`@Q\x80\x91\x03\x90\xFD[PPV[a#\xDDa&?V[\x15a$\x14W`@Q\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0_\x1B\x90P\x90V[_\x81\x90P\x91\x90PV[a$Pa\x16\xFBV[a$\x86W`@Q\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[___`A\x84Q\x03a$\xC8W___` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa$\xBA\x88\x82\x85\x85a&[V[\x95P\x95P\x95PPPPa$\xD6V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a$\xF0Wa$\xEFa*1V[[\x82`\x03\x81\x11\x15a%\x03Wa%\x02a*1V[[\x03\x15a&;W`\x01`\x03\x81\x11\x15a%\x1DWa%\x1Ca*1V[[\x82`\x03\x81\x11\x15a%0Wa%/a*1V[[\x03a%gW`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a%{Wa%za*1V[[\x82`\x03\x81\x11\x15a%\x8EWa%\x8Da*1V[[\x03a%\xD2W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\xC9\x91\x90a-\xD8V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a%\xE5Wa%\xE4a*1V[[\x82`\x03\x81\x11\x15a%\xF8Wa%\xF7a*1V[[\x03a&:W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&1\x91\x90a,\xB6V[`@Q\x80\x91\x03\x90\xFD[[PPV[_`\x02a&Ra&Ma$\x16V[a$?V[_\x01T\x14\x90P\x90V[___\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a&\x97W_`\x03\x85\x92P\x92P\x92Pa'8V[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa&\xBA\x94\x93\x92\x91\x90a=\xE2V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a&\xDAW=__>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a'+W_`\x01__\x1B\x93P\x93P\x93PPa'8V[\x80___\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[`@Q\x80a\x01\x80\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x03\x81\x11\x15a'\xC4Wa'\xC3a*1V[[\x81R` \x01``\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[__\xFD[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a(\x1E\x82a'\xF5V[\x90P\x91\x90PV[a(.\x81a(\x14V[\x81\x14a(8W__\xFD[PV[_\x815\x90Pa(I\x81a(%V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a(dWa(ca'\xEDV[[_a(q\x84\x82\x85\x01a(;V[\x91PP\x92\x91PPV[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a(\xAE\x81a(zV[\x81\x14a(\xB8W__\xFD[PV[_\x815\x90Pa(\xC9\x81a(\xA5V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a(\xE4Wa(\xE3a'\xEDV[[_a(\xF1\x84\x82\x85\x01a(\xBBV[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a)\x0E\x81a(\xFAV[\x82RPPV[_` \x82\x01\x90Pa)'_\x83\x01\x84a)\x05V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a)?\x81a)-V[\x81\x14a)IW__\xFD[PV[_\x815\x90Pa)Z\x81a)6V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a)uWa)ta'\xEDV[[_a)\x82\x84\x82\x85\x01a)LV[\x91PP\x92\x91PPV[a)\x94\x81a(\x14V[\x82RPPV[_\x81\x90P\x91\x90PV[a)\xAC\x81a)\x9AV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a)\xF4\x82a)\xB2V[a)\xFE\x81\x85a)\xBCV[\x93Pa*\x0E\x81\x85` \x86\x01a)\xCCV[a*\x17\x81a)\xDAV[\x84\x01\x91PP\x92\x91PPV[a*+\x81a)-V[\x82RPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a*oWa*na*1V[[PV[_\x81\x90Pa*\x7F\x82a*^V[\x91\x90PV[_a*\x8E\x82a*rV[\x90P\x91\x90PV[a*\x9E\x81a*\x84V[\x82RPPV[_a\x01\x80\x82\x01\x90Pa*\xB8_\x83\x01\x8Fa)\x8BV[a*\xC5` \x83\x01\x8Ea)\x8BV[a*\xD2`@\x83\x01\x8Da)\xA3V[\x81\x81\x03``\x83\x01Ra*\xE4\x81\x8Ca)\xEAV[\x90Pa*\xF3`\x80\x83\x01\x8Ba)\xA3V[a+\0`\xA0\x83\x01\x8Aa)\xA3V[a+\r`\xC0\x83\x01\x89a*\"V[a+\x1A`\xE0\x83\x01\x88a)\xA3V[a+(a\x01\0\x83\x01\x87a)\xA3V[a+6a\x01 \x83\x01\x86a*\x95V[\x81\x81\x03a\x01@\x83\x01Ra+I\x81\x85a)\xEAV[\x90Pa+Ya\x01`\x83\x01\x84a)\x8BV[\x9D\x9CPPPPPPPPPPPPPV[a+s\x81a)\x9AV[\x81\x14a+}W__\xFD[PV[_\x815\x90Pa+\x8E\x81a+jV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a+\xA9Wa+\xA8a'\xEDV[[_a+\xB6\x84\x82\x85\x01a+\x80V[\x91PP\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a+\xE0Wa+\xDFa+\xBFV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xFDWa+\xFCa+\xC3V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a,\x19Wa,\x18a+\xC7V[[\x92P\x92\x90PV[______`\xA0\x87\x89\x03\x12\x15a,:Wa,9a'\xEDV[[_a,G\x89\x82\x8A\x01a(;V[\x96PP` a,X\x89\x82\x8A\x01a+\x80V[\x95PP`@a,i\x89\x82\x8A\x01a+\x80V[\x94PP``\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\x8AWa,\x89a'\xF1V[[a,\x96\x89\x82\x8A\x01a+\xCBV[\x93P\x93PP`\x80a,\xA9\x89\x82\x8A\x01a)LV[\x91PP\x92\x95P\x92\x95P\x92\x95V[_` \x82\x01\x90Pa,\xC9_\x83\x01\x84a*\"V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a,\xE5Wa,\xE4a'\xEDV[[_a,\xF2\x85\x82\x86\x01a)LV[\x92PP` a-\x03\x85\x82\x86\x01a(;V[\x91PP\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a-\"Wa-!a+\xBFV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-?Wa->a+\xC3V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a-[Wa-Za+\xC7V[[\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15a-yWa-xa'\xEDV[[_a-\x86\x86\x82\x87\x01a)LV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\xA7Wa-\xA6a'\xF1V[[a-\xB3\x86\x82\x87\x01a-\rV[\x92P\x92PP\x92P\x92P\x92V[_` \x82\x01\x90Pa-\xD2_\x83\x01\x84a)\x8BV[\x92\x91PPV[_` \x82\x01\x90Pa-\xEB_\x83\x01\x84a)\xA3V[\x92\x91PPV[a-\xFA\x81a(\x14V[\x82RPPV[a.\t\x81a)\x9AV[\x82RPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a.)\x82a)\xB2V[a.3\x81\x85a.\x0FV[\x93Pa.C\x81\x85` \x86\x01a)\xCCV[a.L\x81a)\xDAV[\x84\x01\x91PP\x92\x91PPV[a.`\x81a)-V[\x82RPPV[a.o\x81a*\x84V[\x82RPPV[_a\x01\x80\x83\x01_\x83\x01Qa.\x8B_\x86\x01\x82a-\xF1V[P` \x83\x01Qa.\x9E` \x86\x01\x82a-\xF1V[P`@\x83\x01Qa.\xB1`@\x86\x01\x82a.\0V[P``\x83\x01Q\x84\x82\x03``\x86\x01Ra.\xC9\x82\x82a.\x1FV[\x91PP`\x80\x83\x01Qa.\xDE`\x80\x86\x01\x82a.\0V[P`\xA0\x83\x01Qa.\xF1`\xA0\x86\x01\x82a.\0V[P`\xC0\x83\x01Qa/\x04`\xC0\x86\x01\x82a.WV[P`\xE0\x83\x01Qa/\x17`\xE0\x86\x01\x82a.\0V[Pa\x01\0\x83\x01Qa/,a\x01\0\x86\x01\x82a.\0V[Pa\x01 \x83\x01Qa/Aa\x01 \x86\x01\x82a.fV[Pa\x01@\x83\x01Q\x84\x82\x03a\x01@\x86\x01Ra/[\x82\x82a.\x1FV[\x91PPa\x01`\x83\x01Qa/ra\x01`\x86\x01\x82a-\xF1V[P\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra/\x95\x81\x84a.uV[\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_a/\xC0a/\xBBa/\xB6\x84a'\xF5V[a/\x9DV[a'\xF5V[\x90P\x91\x90PV[_a/\xD1\x82a/\xA6V[\x90P\x91\x90PV[_a/\xE2\x82a/\xC7V[\x90P\x91\x90PV[a/\xF2\x81a/\xD8V[\x82RPPV[_` \x82\x01\x90Pa0\x0B_\x83\x01\x84a/\xE9V[\x92\x91PPV[\x7FZERO_ADDRESS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a0E`\x0C\x83a)\xBCV[\x91Pa0P\x82a0\x11V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra0r\x81a09V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a0\xBDW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a0\xD0Wa0\xCFa0yV[[P\x91\x90PV[\x7FFEE_TOO_HIGH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a1\n`\x0C\x83a)\xBCV[\x91Pa1\x15\x82a0\xD6V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra17\x81a0\xFEV[\x90P\x91\x90PV[\x7FTOKEN_NOT_SUPPORTED\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a1r`\x13\x83a)\xBCV[\x91Pa1}\x82a1>V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra1\x9F\x81a1fV[\x90P\x91\x90PV[\x7FAMOUNT_IS_ZERO\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a1\xDA`\x0E\x83a)\xBCV[\x91Pa1\xE5\x82a1\xA6V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra2\x07\x81a1\xCEV[\x90P\x91\x90PV[_``\x82\x01\x90Pa2!_\x83\x01\x86a)\x8BV[a2.` \x83\x01\x85a)\x8BV[a2;`@\x83\x01\x84a)\xA3V[\x94\x93PPPPV[a2L\x81a(\xFAV[\x81\x14a2VW__\xFD[PV[_\x81Q\x90Pa2g\x81a2CV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a2\x82Wa2\x81a'\xEDV[[_a2\x8F\x84\x82\x85\x01a2YV[\x91PP\x92\x91PPV[\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a2\xCC`\x0F\x83a)\xBCV[\x91Pa2\xD7\x82a2\x98V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra2\xF9\x81a2\xC0V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a37\x82a)\x9AV[\x91Pa3B\x83a)\x9AV[\x92P\x82\x82\x02a3P\x81a)\x9AV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a3gWa3fa3\0V[[P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a3\xA5\x82a)\x9AV[\x91Pa3\xB0\x83a)\x9AV[\x92P\x82a3\xC0Wa3\xBFa3nV[[\x82\x82\x04\x90P\x92\x91PPV[_a3\xD5\x82a)\x9AV[\x91Pa3\xE0\x83a)\x9AV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a3\xF8Wa3\xF7a3\0V[[\x92\x91PPV[_\x81``\x1B\x90P\x91\x90PV[_a4\x14\x82a3\xFEV[\x90P\x91\x90PV[_a4%\x82a4\nV[\x90P\x91\x90PV[a4=a48\x82a(\x14V[a4\x1BV[\x82RPPV[_\x81\x90P\x91\x90PV[a4]a4X\x82a)\x9AV[a4CV[\x82RPPV[_\x81\x90P\x91\x90PV[a4}a4x\x82a)-V[a4cV[\x82RPPV[_a4\x8E\x82\x88a4,V[`\x14\x82\x01\x91Pa4\x9E\x82\x87a4,V[`\x14\x82\x01\x91Pa4\xAE\x82\x86a4LV[` \x82\x01\x91Pa4\xBE\x82\x85a4lV[` \x82\x01\x91Pa4\xCE\x82\x84a4LV[` \x82\x01\x91P\x81\x90P\x96\x95PPPPPPV[_a4\xEB\x82a)\x9AV[\x91Pa4\xF6\x83a)\x9AV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a5\x0EWa5\ra3\0V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a5\x9D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a5bV[a5\xA7\x86\x83a5bV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_a5\xD9a5\xD4a5\xCF\x84a)\x9AV[a/\x9DV[a)\x9AV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a5\xF2\x83a5\xBFV[a6\x06a5\xFE\x82a5\xE0V[\x84\x84Ta5nV[\x82UPPPPV[__\x90P\x90V[a6\x1Da6\x0EV[a6(\x81\x84\x84a5\xE9V[PPPV[[\x81\x81\x10\x15a6KWa6@_\x82a6\x15V[`\x01\x81\x01\x90Pa6.V[PPV[`\x1F\x82\x11\x15a6\x90Wa6a\x81a5AV[a6j\x84a5SV[\x81\x01` \x85\x10\x15a6yW\x81\x90P[a6\x8Da6\x85\x85a5SV[\x83\x01\x82a6-V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a6\xB0_\x19\x84`\x08\x02a6\x95V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a6\xC8\x83\x83a6\xA1V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a6\xE1\x82a)\xB2V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xFAWa6\xF9a5\x14V[[a7\x04\x82Ta0\xA6V[a7\x0F\x82\x82\x85a6OV[_` \x90P`\x1F\x83\x11`\x01\x81\x14a7@W_\x84\x15a7.W\x82\x87\x01Q\x90P[a78\x85\x82a6\xBDV[\x86UPa7\x9FV[`\x1F\x19\x84\x16a7N\x86a5AV[_[\x82\x81\x10\x15a7uW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa7PV[\x86\x83\x10\x15a7\x92W\x84\x89\x01Qa7\x8E`\x1F\x89\x16\x82a6\xA1V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`@\x82\x01\x90Pa7\xBA_\x83\x01\x85a)\x8BV[a7\xC7` \x83\x01\x84a)\xA3V[\x93\x92PPPV[\x7FFEE_TRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a8\x02`\x13\x83a)\xBCV[\x91Pa8\r\x82a7\xCEV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra8/\x81a7\xF6V[\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a8O\x83\x85a)\xBCV[\x93Pa8\\\x83\x85\x84a86V[a8e\x83a)\xDAV[\x84\x01\x90P\x93\x92PPPV[_`\x80\x82\x01\x90Pa8\x83_\x83\x01\x88a)\x8BV[a8\x90` \x83\x01\x87a)\xA3V[\x81\x81\x03`@\x83\x01Ra8\xA3\x81\x85\x87a8DV[\x90Pa8\xB2``\x83\x01\x84a)\xA3V[\x96\x95PPPPPPV[\x7FUNAUTHORIZED_ORACLE\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a8\xF0`\x13\x83a)\xBCV[\x91Pa8\xFB\x82a8\xBCV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra9\x1D\x81a8\xE4V[\x90P\x91\x90PV[\x7FNOT_PROCESSING\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a9X`\x0E\x83a)\xBCV[\x91Pa9c\x82a9$V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra9\x85\x81a9LV[\x90P\x91\x90PV[\x7FOPERATOR_NOT_ASSIGNED\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a9\xC0`\x15\x83a)\xBCV[\x91Pa9\xCB\x82a9\x8CV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra9\xED\x81a9\xB4V[\x90P\x91\x90PV[\x7FNOT_PENDING\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a:(`\x0B\x83a)\xBCV[\x91Pa:3\x82a9\xF4V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra:U\x81a:\x1CV[\x90P\x91\x90PV[\x7FPAYMENT_EXPIRED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a:\x90`\x0F\x83a)\xBCV[\x91Pa:\x9B\x82a:\\V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra:\xBD\x81a:\x84V[\x90P\x91\x90PV[\x7FNOT_ACTIVE_LP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a:\xF8`\r\x83a)\xBCV[\x91Pa;\x03\x82a:\xC4V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;%\x81a:\xECV[\x90P\x91\x90PV[_a;7\x82\x85a4lV[` \x82\x01\x91Pa;G\x82\x84a4,V[`\x14\x82\x01\x91P\x81\x90P\x93\x92PPPV[\x7FINVALID_SLIP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a;\x8B`\x0C\x83a)\xBCV[\x91Pa;\x96\x82a;WV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;\xB8\x81a;\x7FV[\x90P\x91\x90PV[PV[_a;\xCD_\x83a)\xBCV[\x91Pa;\xD8\x82a;\xBFV[_\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;\xF9\x81a;\xC2V[\x90P\x91\x90PV[\x7FNOT_SENDER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a<4`\n\x83a)\xBCV[\x91Pa<?\x82a<\0V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra<a\x81a<(V[\x90P\x91\x90PV[\x7FDEADLINE_NOT_PASSED\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a<\x9C`\x13\x83a)\xBCV[\x91Pa<\xA7\x82a<hV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra<\xC9\x81a<\x90V[\x90P\x91\x90PV[\x7FREFUND_TRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a=\x04`\x16\x83a)\xBCV[\x91Pa=\x0F\x82a<\xD0V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra=1\x81a<\xF8V[\x90P\x91\x90PV[\x7FDeadline expired\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a=l`\x10\x83a)\xBCV[\x91Pa=w\x82a=8V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra=\x99\x81a=`V[\x90P\x91\x90PV[_`@\x82\x01\x90Pa=\xB3_\x83\x01\x85a)\x8BV[a=\xC0` \x83\x01\x84a*\"V[\x93\x92PPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a=\xDC\x81a=\xC7V[\x82RPPV[_`\x80\x82\x01\x90Pa=\xF5_\x83\x01\x87a*\"V[a>\x02` \x83\x01\x86a=\xD3V[a>\x0F`@\x83\x01\x85a*\"V[a>\x1C``\x83\x01\x84a*\"V[\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \xF0\xBCX\xF6q\xAC\x9E3\x80\xE0\x1E\x92\x0B\xD1\xD9t\xD2\x0C\xD6[\x1D]q\x10\x18B\xD4$\x87\x08-*dsolcC\0\x08\x1E\x003",
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
    /**Custom error with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`.
```solidity
error ECDSAInvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignature;
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
        impl ::core::convert::From<ECDSAInvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignature()";
            const SELECTOR: [u8; 4] = [246u8, 69u8, 238u8, 223u8];
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
    /**Custom error with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`.
```solidity
error ECDSAInvalidSignatureLength(uint256 length);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureLength {
        #[allow(missing_docs)]
        pub length: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<ECDSAInvalidSignatureLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureLength) -> Self {
                (value.length,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ECDSAInvalidSignatureLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { length: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureLength(uint256)";
            const SELECTOR: [u8; 4] = [252u8, 230u8, 152u8, 247u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
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
    /**Custom error with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`.
```solidity
error ECDSAInvalidSignatureS(bytes32 s);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureS {
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<ECDSAInvalidSignatureS> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureS) -> Self {
                (value.s,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignatureS {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { s: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureS {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureS(bytes32)";
            const SELECTOR: [u8; 4] = [215u8, 139u8, 206u8, 12u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
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
    /**Event with signature `PaymentCompleted(bytes32,bytes32)` and selector `0x912edf360d10ba8006466028db26d840a68a22b8db84ef0deb3a7fa9c268eef2`.
```solidity
event PaymentCompleted(bytes32 indexed paymentId, bytes32 proofHash);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PaymentCompleted {
        #[allow(missing_docs)]
        pub paymentId: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for PaymentCompleted {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "PaymentCompleted(bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                145u8, 46u8, 223u8, 54u8, 13u8, 16u8, 186u8, 128u8, 6u8, 70u8, 96u8,
                40u8, 219u8, 38u8, 216u8, 64u8, 166u8, 138u8, 34u8, 184u8, 219u8, 132u8,
                239u8, 13u8, 235u8, 58u8, 127u8, 169u8, 194u8, 104u8, 238u8, 242u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    paymentId: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.paymentId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.paymentId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PaymentCompleted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PaymentCompleted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PaymentCompleted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PaymentCreated(bytes32,address,address,uint256,string,uint256)` and selector `0xad3c6549dd317555f3e8872a10664c9a3312a268f0c9c873a80b9c52f180a07c`.
```solidity
event PaymentCreated(bytes32 indexed paymentId, address indexed sender, address token, uint256 amount, string fiatCurrency, uint256 fiatAmount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PaymentCreated {
        #[allow(missing_docs)]
        pub paymentId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fiatCurrency: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub fiatAmount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for PaymentCreated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "PaymentCreated(bytes32,address,address,uint256,string,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                173u8, 60u8, 101u8, 73u8, 221u8, 49u8, 117u8, 85u8, 243u8, 232u8, 135u8,
                42u8, 16u8, 102u8, 76u8, 154u8, 51u8, 18u8, 162u8, 104u8, 240u8, 201u8,
                200u8, 115u8, 168u8, 11u8, 156u8, 82u8, 241u8, 128u8, 160u8, 124u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    paymentId: topics.1,
                    sender: topics.2,
                    token: data.0,
                    amount: data.1,
                    fiatCurrency: data.2,
                    fiatAmount: data.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.fiatCurrency,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fiatAmount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.paymentId.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.paymentId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PaymentCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PaymentCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PaymentCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PaymentProcessing(bytes32,address,string)` and selector `0x9663413acc8b4a21ea9ca7c900fd3bb1fbe4e1ab6e7e60cab595b1a540270251`.
```solidity
event PaymentProcessing(bytes32 indexed paymentId, address indexed operator, string bankReference);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PaymentProcessing {
        #[allow(missing_docs)]
        pub paymentId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub bankReference: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for PaymentProcessing {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "PaymentProcessing(bytes32,address,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                150u8, 99u8, 65u8, 58u8, 204u8, 139u8, 74u8, 33u8, 234u8, 156u8, 167u8,
                201u8, 0u8, 253u8, 59u8, 177u8, 251u8, 228u8, 225u8, 171u8, 110u8, 126u8,
                96u8, 202u8, 181u8, 149u8, 177u8, 165u8, 64u8, 39u8, 2u8, 81u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    paymentId: topics.1,
                    operator: topics.2,
                    bankReference: data.0,
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
                        &self.bankReference,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.paymentId.clone(),
                    self.operator.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.paymentId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PaymentProcessing {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PaymentProcessing> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PaymentProcessing) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PaymentRefunded(bytes32,string)` and selector `0xa2c0cfcfdd46ca4b148dde16939db4dbf0481430d552d486f78e076410689be9`.
```solidity
event PaymentRefunded(bytes32 indexed paymentId, string reason);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PaymentRefunded {
        #[allow(missing_docs)]
        pub paymentId: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for PaymentRefunded {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "PaymentRefunded(bytes32,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                162u8, 192u8, 207u8, 207u8, 221u8, 70u8, 202u8, 75u8, 20u8, 141u8, 222u8,
                22u8, 147u8, 157u8, 180u8, 219u8, 240u8, 72u8, 20u8, 48u8, 213u8, 82u8,
                212u8, 134u8, 247u8, 142u8, 7u8, 100u8, 16u8, 104u8, 155u8, 233u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    paymentId: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.paymentId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.paymentId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PaymentRefunded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PaymentRefunded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PaymentRefunded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PlatformFeeUpdated(uint256)` and selector `0x45610d581145924dd7090a5017e5f2b1d6f42213bb2e95707ff86846bbfcb1ca`.
```solidity
event PlatformFeeUpdated(uint256 newFeePercent);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PlatformFeeUpdated {
        #[allow(missing_docs)]
        pub newFeePercent: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for PlatformFeeUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PlatformFeeUpdated(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                69u8, 97u8, 13u8, 88u8, 17u8, 69u8, 146u8, 77u8, 215u8, 9u8, 10u8, 80u8,
                23u8, 229u8, 242u8, 177u8, 214u8, 244u8, 34u8, 19u8, 187u8, 46u8, 149u8,
                112u8, 127u8, 248u8, 104u8, 70u8, 187u8, 252u8, 177u8, 202u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { newFeePercent: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.newFeePercent),
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
        impl alloy_sol_types::private::IntoLogData for PlatformFeeUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PlatformFeeUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PlatformFeeUpdated) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `SupportedTokenAdded(address)` and selector `0xd1be2e90bd3d24839d9dd94ad871068e1f9688b02fa43f2a62c9975dfa9de2d7`.
```solidity
event SupportedTokenAdded(address indexed token);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SupportedTokenAdded {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for SupportedTokenAdded {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SupportedTokenAdded(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                209u8, 190u8, 46u8, 144u8, 189u8, 61u8, 36u8, 131u8, 157u8, 157u8, 217u8,
                74u8, 216u8, 113u8, 6u8, 142u8, 31u8, 150u8, 136u8, 176u8, 47u8, 164u8,
                63u8, 42u8, 98u8, 201u8, 151u8, 93u8, 250u8, 157u8, 226u8, 215u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { token: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.token.clone())
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
                    &self.token,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SupportedTokenAdded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SupportedTokenAdded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SupportedTokenAdded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `SupportedTokenRemoved(address)` and selector `0xbea12876694c4055c71f74308f752b9027cf3d554194000a366abddfc239a306`.
```solidity
event SupportedTokenRemoved(address indexed token);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SupportedTokenRemoved {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for SupportedTokenRemoved {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SupportedTokenRemoved(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                190u8, 161u8, 40u8, 118u8, 105u8, 76u8, 64u8, 85u8, 199u8, 31u8, 116u8,
                48u8, 143u8, 117u8, 43u8, 144u8, 39u8, 207u8, 61u8, 85u8, 65u8, 148u8,
                0u8, 10u8, 54u8, 106u8, 189u8, 223u8, 194u8, 57u8, 163u8, 6u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { token: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.token.clone())
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
                    &self.token,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SupportedTokenRemoved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SupportedTokenRemoved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SupportedTokenRemoved) -> alloy_sol_types::private::LogData {
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
constructor(address _feeCollector, address _lpRegistryAddress, address _oracleWallet, address _permissionSlipSigner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _feeCollector: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _lpRegistryAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _oracleWallet: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _permissionSlipSigner: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value._feeCollector,
                        value._lpRegistryAddress,
                        value._oracleWallet,
                        value._permissionSlipSigner,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _feeCollector: tuple.0,
                        _lpRegistryAddress: tuple.1,
                        _oracleWallet: tuple.2,
                        _permissionSlipSigner: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
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
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._feeCollector,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._lpRegistryAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._oracleWallet,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._permissionSlipSigner,
                    ),
                )
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
    /**Function with signature `PAYMENT_DEADLINE()` and selector `0x7994d1a4`.
```solidity
function PAYMENT_DEADLINE() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PAYMENT_DEADLINECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`PAYMENT_DEADLINE()`](PAYMENT_DEADLINECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PAYMENT_DEADLINEReturn {
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
            impl ::core::convert::From<PAYMENT_DEADLINECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: PAYMENT_DEADLINECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for PAYMENT_DEADLINECall {
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
            impl ::core::convert::From<PAYMENT_DEADLINEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: PAYMENT_DEADLINEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for PAYMENT_DEADLINEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for PAYMENT_DEADLINECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PAYMENT_DEADLINE()";
            const SELECTOR: [u8; 4] = [121u8, 148u8, 209u8, 164u8];
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
                        let r: PAYMENT_DEADLINEReturn = r.into();
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
                        let r: PAYMENT_DEADLINEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `addSupportedToken(address)` and selector `0x6d69fcaf`.
```solidity
function addSupportedToken(address token) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSupportedTokenCall {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`addSupportedToken(address)`](addSupportedTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSupportedTokenReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<addSupportedTokenCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSupportedTokenCall) -> Self {
                    (value.token,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSupportedTokenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { token: tuple.0 }
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
            impl ::core::convert::From<addSupportedTokenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSupportedTokenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSupportedTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addSupportedTokenReturn {
            fn _tokenize(
                &self,
            ) -> <addSupportedTokenCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addSupportedTokenCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addSupportedTokenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addSupportedToken(address)";
            const SELECTOR: [u8; 4] = [109u8, 105u8, 252u8, 175u8];
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
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addSupportedTokenReturn::_tokenize(ret)
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
    /**Function with signature `claimPayment(bytes32,bytes)` and selector `0x4dd0301f`.
```solidity
function claimPayment(bytes32 paymentId, bytes memory permissionSlip) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimPaymentCall {
        #[allow(missing_docs)]
        pub paymentId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub permissionSlip: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`claimPayment(bytes32,bytes)`](claimPaymentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimPaymentReturn {}
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
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<claimPaymentCall> for UnderlyingRustTuple<'_> {
                fn from(value: claimPaymentCall) -> Self {
                    (value.paymentId, value.permissionSlip)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimPaymentCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        paymentId: tuple.0,
                        permissionSlip: tuple.1,
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
            impl ::core::convert::From<claimPaymentReturn> for UnderlyingRustTuple<'_> {
                fn from(value: claimPaymentReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimPaymentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl claimPaymentReturn {
            fn _tokenize(
                &self,
            ) -> <claimPaymentCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimPaymentCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = claimPaymentReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claimPayment(bytes32,bytes)";
            const SELECTOR: [u8; 4] = [77u8, 208u8, 48u8, 31u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.paymentId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.permissionSlip,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                claimPaymentReturn::_tokenize(ret)
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
    /**Function with signature `claimRefund(bytes32)` and selector `0x71de2ffc`.
```solidity
function claimRefund(bytes32 paymentId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimRefundCall {
        #[allow(missing_docs)]
        pub paymentId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`claimRefund(bytes32)`](claimRefundCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimRefundReturn {}
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
            impl ::core::convert::From<claimRefundCall> for UnderlyingRustTuple<'_> {
                fn from(value: claimRefundCall) -> Self {
                    (value.paymentId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimRefundCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { paymentId: tuple.0 }
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
            impl ::core::convert::From<claimRefundReturn> for UnderlyingRustTuple<'_> {
                fn from(value: claimRefundReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimRefundReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl claimRefundReturn {
            fn _tokenize(
                &self,
            ) -> <claimRefundCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimRefundCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = claimRefundReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claimRefund(bytes32)";
            const SELECTOR: [u8; 4] = [113u8, 222u8, 47u8, 252u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.paymentId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                claimRefundReturn::_tokenize(ret)
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
    /**Function with signature `confirmSettlement(bytes32)` and selector `0x2bd735ab`.
```solidity
function confirmSettlement(bytes32 paymentId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmSettlementCall {
        #[allow(missing_docs)]
        pub paymentId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`confirmSettlement(bytes32)`](confirmSettlementCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmSettlementReturn {}
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
            impl ::core::convert::From<confirmSettlementCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: confirmSettlementCall) -> Self {
                    (value.paymentId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for confirmSettlementCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { paymentId: tuple.0 }
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
            impl ::core::convert::From<confirmSettlementReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: confirmSettlementReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for confirmSettlementReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl confirmSettlementReturn {
            fn _tokenize(
                &self,
            ) -> <confirmSettlementCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for confirmSettlementCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = confirmSettlementReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "confirmSettlement(bytes32)";
            const SELECTOR: [u8; 4] = [43u8, 215u8, 53u8, 171u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.paymentId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                confirmSettlementReturn::_tokenize(ret)
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
    /**Function with signature `createPayment(address,uint256,uint256,string,bytes32)` and selector `0x1e0a505d`.
```solidity
function createPayment(address token, uint256 amount, uint256 fiatAmount, string memory fiatCurrency, bytes32 recipientHash) external returns (bytes32 paymentId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createPaymentCall {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fiatAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fiatCurrency: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub recipientHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`createPayment(address,uint256,uint256,string,bytes32)`](createPaymentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createPaymentReturn {
        #[allow(missing_docs)]
        pub paymentId: alloy::sol_types::private::FixedBytes<32>,
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
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::String,
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
            impl ::core::convert::From<createPaymentCall> for UnderlyingRustTuple<'_> {
                fn from(value: createPaymentCall) -> Self {
                    (
                        value.token,
                        value.amount,
                        value.fiatAmount,
                        value.fiatCurrency,
                        value.recipientHash,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createPaymentCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        token: tuple.0,
                        amount: tuple.1,
                        fiatAmount: tuple.2,
                        fiatCurrency: tuple.3,
                        recipientHash: tuple.4,
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
            impl ::core::convert::From<createPaymentReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createPaymentReturn) -> Self {
                    (value.paymentId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createPaymentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { paymentId: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createPaymentCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createPayment(address,uint256,uint256,string,bytes32)";
            const SELECTOR: [u8; 4] = [30u8, 10u8, 80u8, 93u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fiatAmount),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.fiatCurrency,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.recipientHash),
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
                        let r: createPaymentReturn = r.into();
                        r.paymentId
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
                        let r: createPaymentReturn = r.into();
                        r.paymentId
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `feeCollector()` and selector `0xc415b95c`.
```solidity
function feeCollector() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct feeCollectorCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`feeCollector()`](feeCollectorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct feeCollectorReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<feeCollectorCall> for UnderlyingRustTuple<'_> {
                fn from(value: feeCollectorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for feeCollectorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<feeCollectorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: feeCollectorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for feeCollectorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for feeCollectorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "feeCollector()";
            const SELECTOR: [u8; 4] = [196u8, 21u8, 185u8, 92u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: feeCollectorReturn = r.into();
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
                        let r: feeCollectorReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getPayment(bytes32)` and selector `0xe66eefc8`.
```solidity
function getPayment(bytes32 paymentId) external view returns (IPaymentEscrow.Payment memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPaymentCall {
        #[allow(missing_docs)]
        pub paymentId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    ///Container type for the return parameters of the [`getPayment(bytes32)`](getPaymentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPaymentReturn {
        #[allow(missing_docs)]
        pub _0: <IPaymentEscrow::Payment as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getPaymentCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPaymentCall) -> Self {
                    (value.paymentId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPaymentCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { paymentId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (IPaymentEscrow::Payment,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IPaymentEscrow::Payment as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getPaymentReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPaymentReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPaymentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPaymentCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IPaymentEscrow::Payment as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IPaymentEscrow::Payment,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPayment(bytes32)";
            const SELECTOR: [u8; 4] = [230u8, 110u8, 239u8, 200u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.paymentId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<IPaymentEscrow::Payment as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getPaymentReturn = r.into();
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
                        let r: getPaymentReturn = r.into();
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
    /**Function with signature `lpRegistry()` and selector `0xf2e9e418`.
```solidity
function lpRegistry() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lpRegistryCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`lpRegistry()`](lpRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lpRegistryReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<lpRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: lpRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lpRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<lpRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lpRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lpRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lpRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lpRegistry()";
            const SELECTOR: [u8; 4] = [242u8, 233u8, 228u8, 24u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: lpRegistryReturn = r.into();
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
                        let r: lpRegistryReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `oracleWallet()` and selector `0x97da034a`.
```solidity
function oracleWallet() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct oracleWalletCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`oracleWallet()`](oracleWalletCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct oracleWalletReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<oracleWalletCall> for UnderlyingRustTuple<'_> {
                fn from(value: oracleWalletCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for oracleWalletCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<oracleWalletReturn> for UnderlyingRustTuple<'_> {
                fn from(value: oracleWalletReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for oracleWalletReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for oracleWalletCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "oracleWallet()";
            const SELECTOR: [u8; 4] = [151u8, 218u8, 3u8, 74u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: oracleWalletReturn = r.into();
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
                        let r: oracleWalletReturn = r.into();
                        r._0
                    })
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
    /**Function with signature `payments(bytes32)` and selector `0x0716326d`.
```solidity
function payments(bytes32) external view returns (address sender, address token, uint256 amount, string memory fiatCurrency, uint256 fiatAmount, uint256 exchangeRate, bytes32 recipientHash, uint256 createdAt, uint256 deadline, IPaymentEscrow.PaymentStatus status, string memory bankReference, address operator);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paymentsCall(pub alloy::sol_types::private::FixedBytes<32>);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`payments(bytes32)`](paymentsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paymentsReturn {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fiatCurrency: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub fiatAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub exchangeRate: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub recipientHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub createdAt: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub status: <IPaymentEscrow::PaymentStatus as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub bankReference: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<paymentsCall> for UnderlyingRustTuple<'_> {
                fn from(value: paymentsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paymentsCall {
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                IPaymentEscrow::PaymentStatus,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                <IPaymentEscrow::PaymentStatus as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::String,
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
            impl ::core::convert::From<paymentsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: paymentsReturn) -> Self {
                    (
                        value.sender,
                        value.token,
                        value.amount,
                        value.fiatCurrency,
                        value.fiatAmount,
                        value.exchangeRate,
                        value.recipientHash,
                        value.createdAt,
                        value.deadline,
                        value.status,
                        value.bankReference,
                        value.operator,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paymentsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sender: tuple.0,
                        token: tuple.1,
                        amount: tuple.2,
                        fiatCurrency: tuple.3,
                        fiatAmount: tuple.4,
                        exchangeRate: tuple.5,
                        recipientHash: tuple.6,
                        createdAt: tuple.7,
                        deadline: tuple.8,
                        status: tuple.9,
                        bankReference: tuple.10,
                        operator: tuple.11,
                    }
                }
            }
        }
        impl paymentsReturn {
            fn _tokenize(
                &self,
            ) -> <paymentsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.fiatCurrency,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fiatAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.exchangeRate),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.recipientHash),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.createdAt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                    <IPaymentEscrow::PaymentStatus as alloy_sol_types::SolType>::tokenize(
                        &self.status,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.bankReference,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paymentsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = paymentsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                IPaymentEscrow::PaymentStatus,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "payments(bytes32)";
            const SELECTOR: [u8; 4] = [7u8, 22u8, 50u8, 109u8];
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
                paymentsReturn::_tokenize(ret)
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
    /**Function with signature `permissionSlipSigner()` and selector `0x72f3e8e1`.
```solidity
function permissionSlipSigner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permissionSlipSignerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`permissionSlipSigner()`](permissionSlipSignerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permissionSlipSignerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<permissionSlipSignerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: permissionSlipSignerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for permissionSlipSignerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<permissionSlipSignerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: permissionSlipSignerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for permissionSlipSignerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for permissionSlipSignerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "permissionSlipSigner()";
            const SELECTOR: [u8; 4] = [114u8, 243u8, 232u8, 225u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: permissionSlipSignerReturn = r.into();
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
                        let r: permissionSlipSignerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `platformFeePercent()` and selector `0x8c639a85`.
```solidity
function platformFeePercent() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct platformFeePercentCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`platformFeePercent()`](platformFeePercentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct platformFeePercentReturn {
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
            impl ::core::convert::From<platformFeePercentCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: platformFeePercentCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for platformFeePercentCall {
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
            impl ::core::convert::From<platformFeePercentReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: platformFeePercentReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for platformFeePercentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for platformFeePercentCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "platformFeePercent()";
            const SELECTOR: [u8; 4] = [140u8, 99u8, 154u8, 133u8];
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
                        let r: platformFeePercentReturn = r.into();
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
                        let r: platformFeePercentReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `removeSupportedToken(address)` and selector `0x76319190`.
```solidity
function removeSupportedToken(address token) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeSupportedTokenCall {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`removeSupportedToken(address)`](removeSupportedTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeSupportedTokenReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<removeSupportedTokenCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeSupportedTokenCall) -> Self {
                    (value.token,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeSupportedTokenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { token: tuple.0 }
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
            impl ::core::convert::From<removeSupportedTokenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeSupportedTokenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeSupportedTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl removeSupportedTokenReturn {
            fn _tokenize(
                &self,
            ) -> <removeSupportedTokenCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeSupportedTokenCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeSupportedTokenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeSupportedToken(address)";
            const SELECTOR: [u8; 4] = [118u8, 49u8, 145u8, 144u8];
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
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                removeSupportedTokenReturn::_tokenize(ret)
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
    /**Function with signature `setOracleWallet(address)` and selector `0x01e8a6bb`.
```solidity
function setOracleWallet(address _newOracle) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOracleWalletCall {
        #[allow(missing_docs)]
        pub _newOracle: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setOracleWallet(address)`](setOracleWalletCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOracleWalletReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<setOracleWalletCall> for UnderlyingRustTuple<'_> {
                fn from(value: setOracleWalletCall) -> Self {
                    (value._newOracle,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOracleWalletCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _newOracle: tuple.0 }
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
            impl ::core::convert::From<setOracleWalletReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setOracleWalletReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setOracleWalletReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setOracleWalletReturn {
            fn _tokenize(
                &self,
            ) -> <setOracleWalletCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setOracleWalletCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setOracleWalletReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setOracleWallet(address)";
            const SELECTOR: [u8; 4] = [1u8, 232u8, 166u8, 187u8];
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
                        &self._newOracle,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setOracleWalletReturn::_tokenize(ret)
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
    /**Function with signature `setPermissionSlipSigner(address)` and selector `0x550ebe28`.
```solidity
function setPermissionSlipSigner(address _newSigner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPermissionSlipSignerCall {
        #[allow(missing_docs)]
        pub _newSigner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setPermissionSlipSigner(address)`](setPermissionSlipSignerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPermissionSlipSignerReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<setPermissionSlipSignerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPermissionSlipSignerCall) -> Self {
                    (value._newSigner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPermissionSlipSignerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _newSigner: tuple.0 }
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
            impl ::core::convert::From<setPermissionSlipSignerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPermissionSlipSignerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPermissionSlipSignerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setPermissionSlipSignerReturn {
            fn _tokenize(
                &self,
            ) -> <setPermissionSlipSignerCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setPermissionSlipSignerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setPermissionSlipSignerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setPermissionSlipSigner(address)";
            const SELECTOR: [u8; 4] = [85u8, 14u8, 190u8, 40u8];
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
                        &self._newSigner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setPermissionSlipSignerReturn::_tokenize(ret)
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
    /**Function with signature `setPlatformFee(uint256)` and selector `0x12e8e2c3`.
```solidity
function setPlatformFee(uint256 newFee) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPlatformFeeCall {
        #[allow(missing_docs)]
        pub newFee: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setPlatformFee(uint256)`](setPlatformFeeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPlatformFeeReturn {}
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
            impl ::core::convert::From<setPlatformFeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: setPlatformFeeCall) -> Self {
                    (value.newFee,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setPlatformFeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newFee: tuple.0 }
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
            impl ::core::convert::From<setPlatformFeeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPlatformFeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPlatformFeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setPlatformFeeReturn {
            fn _tokenize(
                &self,
            ) -> <setPlatformFeeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setPlatformFeeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setPlatformFeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setPlatformFee(uint256)";
            const SELECTOR: [u8; 4] = [18u8, 232u8, 226u8, 195u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.newFee),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setPlatformFeeReturn::_tokenize(ret)
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
    /**Function with signature `supportedTokens(address)` and selector `0x68c4ac26`.
```solidity
function supportedTokens(address) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportedTokensCall(pub alloy::sol_types::private::Address);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`supportedTokens(address)`](supportedTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportedTokensReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<supportedTokensCall> for UnderlyingRustTuple<'_> {
                fn from(value: supportedTokensCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for supportedTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            impl ::core::convert::From<supportedTokensReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportedTokensReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportedTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportedTokensCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supportedTokens(address)";
            const SELECTOR: [u8; 4] = [104u8, 196u8, 172u8, 38u8];
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
                        &self.0,
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
                        let r: supportedTokensReturn = r.into();
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
                        let r: supportedTokensReturn = r.into();
                        r._0
                    })
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
    ///Container for all the [`PaymentEscrow`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum PaymentEscrowCalls {
        #[allow(missing_docs)]
        DEFAULT_ADMIN_ROLE(DEFAULT_ADMIN_ROLECall),
        #[allow(missing_docs)]
        PAYMENT_DEADLINE(PAYMENT_DEADLINECall),
        #[allow(missing_docs)]
        addSupportedToken(addSupportedTokenCall),
        #[allow(missing_docs)]
        claimPayment(claimPaymentCall),
        #[allow(missing_docs)]
        claimRefund(claimRefundCall),
        #[allow(missing_docs)]
        confirmSettlement(confirmSettlementCall),
        #[allow(missing_docs)]
        createPayment(createPaymentCall),
        #[allow(missing_docs)]
        feeCollector(feeCollectorCall),
        #[allow(missing_docs)]
        getPayment(getPaymentCall),
        #[allow(missing_docs)]
        getRoleAdmin(getRoleAdminCall),
        #[allow(missing_docs)]
        grantRole(grantRoleCall),
        #[allow(missing_docs)]
        hasRole(hasRoleCall),
        #[allow(missing_docs)]
        lpRegistry(lpRegistryCall),
        #[allow(missing_docs)]
        oracleWallet(oracleWalletCall),
        #[allow(missing_docs)]
        pause(pauseCall),
        #[allow(missing_docs)]
        paused(pausedCall),
        #[allow(missing_docs)]
        payments(paymentsCall),
        #[allow(missing_docs)]
        permissionSlipSigner(permissionSlipSignerCall),
        #[allow(missing_docs)]
        platformFeePercent(platformFeePercentCall),
        #[allow(missing_docs)]
        removeSupportedToken(removeSupportedTokenCall),
        #[allow(missing_docs)]
        renounceRole(renounceRoleCall),
        #[allow(missing_docs)]
        revokeRole(revokeRoleCall),
        #[allow(missing_docs)]
        setOracleWallet(setOracleWalletCall),
        #[allow(missing_docs)]
        setPermissionSlipSigner(setPermissionSlipSignerCall),
        #[allow(missing_docs)]
        setPlatformFee(setPlatformFeeCall),
        #[allow(missing_docs)]
        supportedTokens(supportedTokensCall),
        #[allow(missing_docs)]
        supportsInterface(supportsInterfaceCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
    }
    impl PaymentEscrowCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 232u8, 166u8, 187u8],
            [1u8, 255u8, 201u8, 167u8],
            [7u8, 22u8, 50u8, 109u8],
            [18u8, 232u8, 226u8, 195u8],
            [30u8, 10u8, 80u8, 93u8],
            [36u8, 138u8, 156u8, 163u8],
            [43u8, 215u8, 53u8, 171u8],
            [47u8, 47u8, 241u8, 93u8],
            [54u8, 86u8, 138u8, 190u8],
            [63u8, 75u8, 168u8, 58u8],
            [77u8, 208u8, 48u8, 31u8],
            [85u8, 14u8, 190u8, 40u8],
            [92u8, 151u8, 90u8, 187u8],
            [104u8, 196u8, 172u8, 38u8],
            [109u8, 105u8, 252u8, 175u8],
            [113u8, 222u8, 47u8, 252u8],
            [114u8, 243u8, 232u8, 225u8],
            [118u8, 49u8, 145u8, 144u8],
            [121u8, 148u8, 209u8, 164u8],
            [132u8, 86u8, 203u8, 89u8],
            [140u8, 99u8, 154u8, 133u8],
            [145u8, 209u8, 72u8, 84u8],
            [151u8, 218u8, 3u8, 74u8],
            [162u8, 23u8, 253u8, 223u8],
            [196u8, 21u8, 185u8, 92u8],
            [213u8, 71u8, 116u8, 31u8],
            [230u8, 110u8, 239u8, 200u8],
            [242u8, 233u8, 228u8, 24u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(setOracleWallet),
            ::core::stringify!(supportsInterface),
            ::core::stringify!(payments),
            ::core::stringify!(setPlatformFee),
            ::core::stringify!(createPayment),
            ::core::stringify!(getRoleAdmin),
            ::core::stringify!(confirmSettlement),
            ::core::stringify!(grantRole),
            ::core::stringify!(renounceRole),
            ::core::stringify!(unpause),
            ::core::stringify!(claimPayment),
            ::core::stringify!(setPermissionSlipSigner),
            ::core::stringify!(paused),
            ::core::stringify!(supportedTokens),
            ::core::stringify!(addSupportedToken),
            ::core::stringify!(claimRefund),
            ::core::stringify!(permissionSlipSigner),
            ::core::stringify!(removeSupportedToken),
            ::core::stringify!(PAYMENT_DEADLINE),
            ::core::stringify!(pause),
            ::core::stringify!(platformFeePercent),
            ::core::stringify!(hasRole),
            ::core::stringify!(oracleWallet),
            ::core::stringify!(DEFAULT_ADMIN_ROLE),
            ::core::stringify!(feeCollector),
            ::core::stringify!(revokeRole),
            ::core::stringify!(getPayment),
            ::core::stringify!(lpRegistry),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <setOracleWalletCall as alloy_sol_types::SolCall>::SIGNATURE,
            <supportsInterfaceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <paymentsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setPlatformFeeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <createPaymentCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <confirmSettlementCall as alloy_sol_types::SolCall>::SIGNATURE,
            <grantRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <unpauseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimPaymentCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setPermissionSlipSignerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pausedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <supportedTokensCall as alloy_sol_types::SolCall>::SIGNATURE,
            <addSupportedTokenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimRefundCall as alloy_sol_types::SolCall>::SIGNATURE,
            <permissionSlipSignerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <removeSupportedTokenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <PAYMENT_DEADLINECall as alloy_sol_types::SolCall>::SIGNATURE,
            <pauseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <platformFeePercentCall as alloy_sol_types::SolCall>::SIGNATURE,
            <hasRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <oracleWalletCall as alloy_sol_types::SolCall>::SIGNATURE,
            <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <feeCollectorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <revokeRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getPaymentCall as alloy_sol_types::SolCall>::SIGNATURE,
            <lpRegistryCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for PaymentEscrowCalls {
        const NAME: &'static str = "PaymentEscrowCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 28usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DEFAULT_ADMIN_ROLE(_) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PAYMENT_DEADLINE(_) => {
                    <PAYMENT_DEADLINECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addSupportedToken(_) => {
                    <addSupportedTokenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::claimPayment(_) => {
                    <claimPaymentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::claimRefund(_) => {
                    <claimRefundCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::confirmSettlement(_) => {
                    <confirmSettlementCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createPayment(_) => {
                    <createPaymentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::feeCollector(_) => {
                    <feeCollectorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPayment(_) => {
                    <getPaymentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRoleAdmin(_) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::grantRole(_) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasRole(_) => <hasRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::lpRegistry(_) => {
                    <lpRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::oracleWallet(_) => {
                    <oracleWalletCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused(_) => <pausedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::payments(_) => <paymentsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::permissionSlipSigner(_) => {
                    <permissionSlipSignerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::platformFeePercent(_) => {
                    <platformFeePercentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeSupportedToken(_) => {
                    <removeSupportedTokenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceRole(_) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::revokeRole(_) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setOracleWallet(_) => {
                    <setOracleWalletCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setPermissionSlipSigner(_) => {
                    <setPermissionSlipSignerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setPlatformFee(_) => {
                    <setPlatformFeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportedTokens(_) => {
                    <supportedTokensCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<PaymentEscrowCalls>] = &[
                {
                    fn setOracleWallet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <setOracleWalletCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::setOracleWallet)
                    }
                    setOracleWallet
                },
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn payments(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <paymentsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PaymentEscrowCalls::payments)
                    }
                    payments
                },
                {
                    fn setPlatformFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <setPlatformFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::setPlatformFee)
                    }
                    setPlatformFee
                },
                {
                    fn createPayment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <createPaymentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::createPayment)
                    }
                    createPayment
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn confirmSettlement(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <confirmSettlementCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::confirmSettlement)
                    }
                    confirmSettlement
                },
                {
                    fn grantRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PaymentEscrowCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn renounceRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn unpause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PaymentEscrowCalls::unpause)
                    }
                    unpause
                },
                {
                    fn claimPayment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <claimPaymentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::claimPayment)
                    }
                    claimPayment
                },
                {
                    fn setPermissionSlipSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <setPermissionSlipSignerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::setPermissionSlipSigner)
                    }
                    setPermissionSlipSigner
                },
                {
                    fn paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PaymentEscrowCalls::paused)
                    }
                    paused
                },
                {
                    fn supportedTokens(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <supportedTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::supportedTokens)
                    }
                    supportedTokens
                },
                {
                    fn addSupportedToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <addSupportedTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::addSupportedToken)
                    }
                    addSupportedToken
                },
                {
                    fn claimRefund(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <claimRefundCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::claimRefund)
                    }
                    claimRefund
                },
                {
                    fn permissionSlipSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <permissionSlipSignerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::permissionSlipSigner)
                    }
                    permissionSlipSigner
                },
                {
                    fn removeSupportedToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <removeSupportedTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::removeSupportedToken)
                    }
                    removeSupportedToken
                },
                {
                    fn PAYMENT_DEADLINE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <PAYMENT_DEADLINECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::PAYMENT_DEADLINE)
                    }
                    PAYMENT_DEADLINE
                },
                {
                    fn pause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PaymentEscrowCalls::pause)
                    }
                    pause
                },
                {
                    fn platformFeePercent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <platformFeePercentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::platformFeePercent)
                    }
                    platformFeePercent
                },
                {
                    fn hasRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(PaymentEscrowCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn oracleWallet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <oracleWalletCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::oracleWallet)
                    }
                    oracleWallet
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn feeCollector(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <feeCollectorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::feeCollector)
                    }
                    feeCollector
                },
                {
                    fn revokeRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn getPayment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <getPaymentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::getPayment)
                    }
                    getPayment
                },
                {
                    fn lpRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <lpRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowCalls::lpRegistry)
                    }
                    lpRegistry
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
            ) -> alloy_sol_types::Result<PaymentEscrowCalls>] = &[
                {
                    fn setOracleWallet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <setOracleWalletCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::setOracleWallet)
                    }
                    setOracleWallet
                },
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn payments(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <paymentsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::payments)
                    }
                    payments
                },
                {
                    fn setPlatformFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <setPlatformFeeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::setPlatformFee)
                    }
                    setPlatformFee
                },
                {
                    fn createPayment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <createPaymentCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::createPayment)
                    }
                    createPayment
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn confirmSettlement(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <confirmSettlementCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::confirmSettlement)
                    }
                    confirmSettlement
                },
                {
                    fn grantRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn renounceRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn unpause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::unpause)
                    }
                    unpause
                },
                {
                    fn claimPayment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <claimPaymentCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::claimPayment)
                    }
                    claimPayment
                },
                {
                    fn setPermissionSlipSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <setPermissionSlipSignerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::setPermissionSlipSigner)
                    }
                    setPermissionSlipSigner
                },
                {
                    fn paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::paused)
                    }
                    paused
                },
                {
                    fn supportedTokens(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <supportedTokensCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::supportedTokens)
                    }
                    supportedTokens
                },
                {
                    fn addSupportedToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <addSupportedTokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::addSupportedToken)
                    }
                    addSupportedToken
                },
                {
                    fn claimRefund(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <claimRefundCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::claimRefund)
                    }
                    claimRefund
                },
                {
                    fn permissionSlipSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <permissionSlipSignerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::permissionSlipSigner)
                    }
                    permissionSlipSigner
                },
                {
                    fn removeSupportedToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <removeSupportedTokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::removeSupportedToken)
                    }
                    removeSupportedToken
                },
                {
                    fn PAYMENT_DEADLINE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <PAYMENT_DEADLINECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::PAYMENT_DEADLINE)
                    }
                    PAYMENT_DEADLINE
                },
                {
                    fn pause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::pause)
                    }
                    pause
                },
                {
                    fn platformFeePercent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <platformFeePercentCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::platformFeePercent)
                    }
                    platformFeePercent
                },
                {
                    fn hasRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn oracleWallet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <oracleWalletCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::oracleWallet)
                    }
                    oracleWallet
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn feeCollector(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <feeCollectorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::feeCollector)
                    }
                    feeCollector
                },
                {
                    fn revokeRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn getPayment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <getPaymentCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::getPayment)
                    }
                    getPayment
                },
                {
                    fn lpRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowCalls> {
                        <lpRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowCalls::lpRegistry)
                    }
                    lpRegistry
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
                Self::PAYMENT_DEADLINE(inner) => {
                    <PAYMENT_DEADLINECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addSupportedToken(inner) => {
                    <addSupportedTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::claimPayment(inner) => {
                    <claimPaymentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::claimRefund(inner) => {
                    <claimRefundCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::confirmSettlement(inner) => {
                    <confirmSettlementCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createPayment(inner) => {
                    <createPaymentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::feeCollector(inner) => {
                    <feeCollectorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPayment(inner) => {
                    <getPaymentCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::lpRegistry(inner) => {
                    <lpRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::oracleWallet(inner) => {
                    <oracleWalletCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::payments(inner) => {
                    <paymentsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::permissionSlipSigner(inner) => {
                    <permissionSlipSignerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::platformFeePercent(inner) => {
                    <platformFeePercentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeSupportedToken(inner) => {
                    <removeSupportedTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setOracleWallet(inner) => {
                    <setOracleWalletCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setPermissionSlipSigner(inner) => {
                    <setPermissionSlipSignerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setPlatformFee(inner) => {
                    <setPlatformFeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::supportedTokens(inner) => {
                    <supportedTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::PAYMENT_DEADLINE(inner) => {
                    <PAYMENT_DEADLINECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addSupportedToken(inner) => {
                    <addSupportedTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claimPayment(inner) => {
                    <claimPaymentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claimRefund(inner) => {
                    <claimRefundCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::confirmSettlement(inner) => {
                    <confirmSettlementCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createPayment(inner) => {
                    <createPaymentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::feeCollector(inner) => {
                    <feeCollectorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPayment(inner) => {
                    <getPaymentCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::lpRegistry(inner) => {
                    <lpRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::oracleWallet(inner) => {
                    <oracleWalletCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::payments(inner) => {
                    <paymentsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::permissionSlipSigner(inner) => {
                    <permissionSlipSignerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::platformFeePercent(inner) => {
                    <platformFeePercentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeSupportedToken(inner) => {
                    <removeSupportedTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setOracleWallet(inner) => {
                    <setOracleWalletCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setPermissionSlipSigner(inner) => {
                    <setPermissionSlipSignerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setPlatformFee(inner) => {
                    <setPlatformFeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::supportedTokens(inner) => {
                    <supportedTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`PaymentEscrow`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum PaymentEscrowErrors {
        #[allow(missing_docs)]
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        #[allow(missing_docs)]
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        #[allow(missing_docs)]
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        #[allow(missing_docs)]
        EnforcedPause(EnforcedPause),
        #[allow(missing_docs)]
        ExpectedPause(ExpectedPause),
        #[allow(missing_docs)]
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
    }
    impl PaymentEscrowErrors {
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
            [215u8, 139u8, 206u8, 12u8],
            [217u8, 60u8, 6u8, 101u8],
            [226u8, 81u8, 125u8, 63u8],
            [246u8, 69u8, 238u8, 223u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(ReentrancyGuardReentrantCall),
            ::core::stringify!(AccessControlBadConfirmation),
            ::core::stringify!(ExpectedPause),
            ::core::stringify!(ECDSAInvalidSignatureS),
            ::core::stringify!(EnforcedPause),
            ::core::stringify!(AccessControlUnauthorizedAccount),
            ::core::stringify!(ECDSAInvalidSignature),
            ::core::stringify!(ECDSAInvalidSignatureLength),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::SIGNATURE,
            <AccessControlBadConfirmation as alloy_sol_types::SolError>::SIGNATURE,
            <ExpectedPause as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SIGNATURE,
            <EnforcedPause as alloy_sol_types::SolError>::SIGNATURE,
            <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignature as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for PaymentEscrowErrors {
        const NAME: &'static str = "PaymentEscrowErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 8usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AccessControlBadConfirmation(_) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AccessControlUnauthorizedAccount(_) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignature(_) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureLength(_) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureS(_) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<PaymentEscrowErrors>] = &[
                {
                    fn ReentrancyGuardReentrantCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowErrors> {
                        <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowErrors::ReentrancyGuardReentrantCall)
                    }
                    ReentrancyGuardReentrantCall
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowErrors::AccessControlUnauthorizedAccount)
                    }
                    AccessControlUnauthorizedAccount
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(PaymentEscrowErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
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
            ) -> alloy_sol_types::Result<PaymentEscrowErrors>] = &[
                {
                    fn ReentrancyGuardReentrantCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowErrors> {
                        <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowErrors::ReentrancyGuardReentrantCall)
                    }
                    ReentrancyGuardReentrantCall
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowErrors::AccessControlUnauthorizedAccount)
                    }
                    AccessControlUnauthorizedAccount
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<PaymentEscrowErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(PaymentEscrowErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
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
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encode_raw(
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
    ///Container for all the [`PaymentEscrow`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum PaymentEscrowEvents {
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        PaymentCompleted(PaymentCompleted),
        #[allow(missing_docs)]
        PaymentCreated(PaymentCreated),
        #[allow(missing_docs)]
        PaymentProcessing(PaymentProcessing),
        #[allow(missing_docs)]
        PaymentRefunded(PaymentRefunded),
        #[allow(missing_docs)]
        PlatformFeeUpdated(PlatformFeeUpdated),
        #[allow(missing_docs)]
        RoleAdminChanged(RoleAdminChanged),
        #[allow(missing_docs)]
        RoleGranted(RoleGranted),
        #[allow(missing_docs)]
        RoleRevoked(RoleRevoked),
        #[allow(missing_docs)]
        SupportedTokenAdded(SupportedTokenAdded),
        #[allow(missing_docs)]
        SupportedTokenRemoved(SupportedTokenRemoved),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
    }
    impl PaymentEscrowEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8,
                236u8, 121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8,
                64u8, 48u8, 69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
            ],
            [
                69u8, 97u8, 13u8, 88u8, 17u8, 69u8, 146u8, 77u8, 215u8, 9u8, 10u8, 80u8,
                23u8, 229u8, 242u8, 177u8, 214u8, 244u8, 34u8, 19u8, 187u8, 46u8, 149u8,
                112u8, 127u8, 248u8, 104u8, 70u8, 187u8, 252u8, 177u8, 202u8,
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
                145u8, 46u8, 223u8, 54u8, 13u8, 16u8, 186u8, 128u8, 6u8, 70u8, 96u8,
                40u8, 219u8, 38u8, 216u8, 64u8, 166u8, 138u8, 34u8, 184u8, 219u8, 132u8,
                239u8, 13u8, 235u8, 58u8, 127u8, 169u8, 194u8, 104u8, 238u8, 242u8,
            ],
            [
                150u8, 99u8, 65u8, 58u8, 204u8, 139u8, 74u8, 33u8, 234u8, 156u8, 167u8,
                201u8, 0u8, 253u8, 59u8, 177u8, 251u8, 228u8, 225u8, 171u8, 110u8, 126u8,
                96u8, 202u8, 181u8, 149u8, 177u8, 165u8, 64u8, 39u8, 2u8, 81u8,
            ],
            [
                162u8, 192u8, 207u8, 207u8, 221u8, 70u8, 202u8, 75u8, 20u8, 141u8, 222u8,
                22u8, 147u8, 157u8, 180u8, 219u8, 240u8, 72u8, 20u8, 48u8, 213u8, 82u8,
                212u8, 134u8, 247u8, 142u8, 7u8, 100u8, 16u8, 104u8, 155u8, 233u8,
            ],
            [
                173u8, 60u8, 101u8, 73u8, 221u8, 49u8, 117u8, 85u8, 243u8, 232u8, 135u8,
                42u8, 16u8, 102u8, 76u8, 154u8, 51u8, 18u8, 162u8, 104u8, 240u8, 201u8,
                200u8, 115u8, 168u8, 11u8, 156u8, 82u8, 241u8, 128u8, 160u8, 124u8,
            ],
            [
                189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8,
                81u8, 66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8,
                71u8, 92u8, 58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
            ],
            [
                190u8, 161u8, 40u8, 118u8, 105u8, 76u8, 64u8, 85u8, 199u8, 31u8, 116u8,
                48u8, 143u8, 117u8, 43u8, 144u8, 39u8, 207u8, 61u8, 85u8, 65u8, 148u8,
                0u8, 10u8, 54u8, 106u8, 189u8, 223u8, 194u8, 57u8, 163u8, 6u8,
            ],
            [
                209u8, 190u8, 46u8, 144u8, 189u8, 61u8, 36u8, 131u8, 157u8, 157u8, 217u8,
                74u8, 216u8, 113u8, 6u8, 142u8, 31u8, 150u8, 136u8, 176u8, 47u8, 164u8,
                63u8, 42u8, 98u8, 201u8, 151u8, 93u8, 250u8, 157u8, 226u8, 215u8,
            ],
            [
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8,
                103u8, 11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8,
                253u8, 100u8, 235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(RoleGranted),
            ::core::stringify!(PlatformFeeUpdated),
            ::core::stringify!(Unpaused),
            ::core::stringify!(Paused),
            ::core::stringify!(PaymentCompleted),
            ::core::stringify!(PaymentProcessing),
            ::core::stringify!(PaymentRefunded),
            ::core::stringify!(PaymentCreated),
            ::core::stringify!(RoleAdminChanged),
            ::core::stringify!(SupportedTokenRemoved),
            ::core::stringify!(SupportedTokenAdded),
            ::core::stringify!(RoleRevoked),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE,
            <PlatformFeeUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <Unpaused as alloy_sol_types::SolEvent>::SIGNATURE,
            <Paused as alloy_sol_types::SolEvent>::SIGNATURE,
            <PaymentCompleted as alloy_sol_types::SolEvent>::SIGNATURE,
            <PaymentProcessing as alloy_sol_types::SolEvent>::SIGNATURE,
            <PaymentRefunded as alloy_sol_types::SolEvent>::SIGNATURE,
            <PaymentCreated as alloy_sol_types::SolEvent>::SIGNATURE,
            <RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE,
            <SupportedTokenRemoved as alloy_sol_types::SolEvent>::SIGNATURE,
            <SupportedTokenAdded as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for PaymentEscrowEvents {
        const NAME: &'static str = "PaymentEscrowEvents";
        const COUNT: usize = 12usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Paused)
                }
                Some(<PaymentCompleted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <PaymentCompleted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PaymentCompleted)
                }
                Some(<PaymentCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <PaymentCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PaymentCreated)
                }
                Some(
                    <PaymentProcessing as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PaymentProcessing as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PaymentProcessing)
                }
                Some(<PaymentRefunded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <PaymentRefunded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PaymentRefunded)
                }
                Some(
                    <PlatformFeeUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PlatformFeeUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PlatformFeeUpdated)
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
                Some(
                    <SupportedTokenAdded as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SupportedTokenAdded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SupportedTokenAdded)
                }
                Some(
                    <SupportedTokenRemoved as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SupportedTokenRemoved as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SupportedTokenRemoved)
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
    impl alloy_sol_types::private::IntoLogData for PaymentEscrowEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PaymentCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PaymentCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PaymentProcessing(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PaymentRefunded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PlatformFeeUpdated(inner) => {
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
                Self::SupportedTokenAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SupportedTokenRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PaymentCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PaymentCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PaymentProcessing(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PaymentRefunded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PlatformFeeUpdated(inner) => {
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
                Self::SupportedTokenAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SupportedTokenRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`PaymentEscrow`](self) contract instance.

See the [wrapper's documentation](`PaymentEscrowInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> PaymentEscrowInstance<P, N> {
        PaymentEscrowInstance::<P, N>::new(address, __provider)
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
        _feeCollector: alloy::sol_types::private::Address,
        _lpRegistryAddress: alloy::sol_types::private::Address,
        _oracleWallet: alloy::sol_types::private::Address,
        _permissionSlipSigner: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<PaymentEscrowInstance<P, N>>,
    > {
        PaymentEscrowInstance::<
            P,
            N,
        >::deploy(
            __provider,
            _feeCollector,
            _lpRegistryAddress,
            _oracleWallet,
            _permissionSlipSigner,
        )
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        _feeCollector: alloy::sol_types::private::Address,
        _lpRegistryAddress: alloy::sol_types::private::Address,
        _oracleWallet: alloy::sol_types::private::Address,
        _permissionSlipSigner: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        PaymentEscrowInstance::<
            P,
            N,
        >::deploy_builder(
            __provider,
            _feeCollector,
            _lpRegistryAddress,
            _oracleWallet,
            _permissionSlipSigner,
        )
    }
    /**A [`PaymentEscrow`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`PaymentEscrow`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct PaymentEscrowInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for PaymentEscrowInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("PaymentEscrowInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > PaymentEscrowInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`PaymentEscrow`](self) contract instance.

See the [wrapper's documentation](`PaymentEscrowInstance`) for more details.*/
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
            _feeCollector: alloy::sol_types::private::Address,
            _lpRegistryAddress: alloy::sol_types::private::Address,
            _oracleWallet: alloy::sol_types::private::Address,
            _permissionSlipSigner: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<PaymentEscrowInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                __provider,
                _feeCollector,
                _lpRegistryAddress,
                _oracleWallet,
                _permissionSlipSigner,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            __provider: P,
            _feeCollector: alloy::sol_types::private::Address,
            _lpRegistryAddress: alloy::sol_types::private::Address,
            _oracleWallet: alloy::sol_types::private::Address,
            _permissionSlipSigner: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _feeCollector,
                            _lpRegistryAddress,
                            _oracleWallet,
                            _permissionSlipSigner,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
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
    impl<P: ::core::clone::Clone, N> PaymentEscrowInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> PaymentEscrowInstance<P, N> {
            PaymentEscrowInstance {
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
    > PaymentEscrowInstance<P, N> {
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
        ///Creates a new call builder for the [`PAYMENT_DEADLINE`] function.
        pub fn PAYMENT_DEADLINE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, PAYMENT_DEADLINECall, N> {
            self.call_builder(&PAYMENT_DEADLINECall)
        }
        ///Creates a new call builder for the [`addSupportedToken`] function.
        pub fn addSupportedToken(
            &self,
            token: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, addSupportedTokenCall, N> {
            self.call_builder(&addSupportedTokenCall { token })
        }
        ///Creates a new call builder for the [`claimPayment`] function.
        pub fn claimPayment(
            &self,
            paymentId: alloy::sol_types::private::FixedBytes<32>,
            permissionSlip: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, claimPaymentCall, N> {
            self.call_builder(
                &claimPaymentCall {
                    paymentId,
                    permissionSlip,
                },
            )
        }
        ///Creates a new call builder for the [`claimRefund`] function.
        pub fn claimRefund(
            &self,
            paymentId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, claimRefundCall, N> {
            self.call_builder(&claimRefundCall { paymentId })
        }
        ///Creates a new call builder for the [`confirmSettlement`] function.
        pub fn confirmSettlement(
            &self,
            paymentId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, confirmSettlementCall, N> {
            self.call_builder(&confirmSettlementCall { paymentId })
        }
        ///Creates a new call builder for the [`createPayment`] function.
        pub fn createPayment(
            &self,
            token: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            fiatAmount: alloy::sol_types::private::primitives::aliases::U256,
            fiatCurrency: alloy::sol_types::private::String,
            recipientHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, createPaymentCall, N> {
            self.call_builder(
                &createPaymentCall {
                    token,
                    amount,
                    fiatAmount,
                    fiatCurrency,
                    recipientHash,
                },
            )
        }
        ///Creates a new call builder for the [`feeCollector`] function.
        pub fn feeCollector(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, feeCollectorCall, N> {
            self.call_builder(&feeCollectorCall)
        }
        ///Creates a new call builder for the [`getPayment`] function.
        pub fn getPayment(
            &self,
            paymentId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getPaymentCall, N> {
            self.call_builder(&getPaymentCall { paymentId })
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
        ///Creates a new call builder for the [`lpRegistry`] function.
        pub fn lpRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, lpRegistryCall, N> {
            self.call_builder(&lpRegistryCall)
        }
        ///Creates a new call builder for the [`oracleWallet`] function.
        pub fn oracleWallet(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, oracleWalletCall, N> {
            self.call_builder(&oracleWalletCall)
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(&self) -> alloy_contract::SolCallBuilder<&P, pauseCall, N> {
            self.call_builder(&pauseCall)
        }
        ///Creates a new call builder for the [`paused`] function.
        pub fn paused(&self) -> alloy_contract::SolCallBuilder<&P, pausedCall, N> {
            self.call_builder(&pausedCall)
        }
        ///Creates a new call builder for the [`payments`] function.
        pub fn payments(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, paymentsCall, N> {
            self.call_builder(&paymentsCall(_0))
        }
        ///Creates a new call builder for the [`permissionSlipSigner`] function.
        pub fn permissionSlipSigner(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, permissionSlipSignerCall, N> {
            self.call_builder(&permissionSlipSignerCall)
        }
        ///Creates a new call builder for the [`platformFeePercent`] function.
        pub fn platformFeePercent(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, platformFeePercentCall, N> {
            self.call_builder(&platformFeePercentCall)
        }
        ///Creates a new call builder for the [`removeSupportedToken`] function.
        pub fn removeSupportedToken(
            &self,
            token: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, removeSupportedTokenCall, N> {
            self.call_builder(&removeSupportedTokenCall { token })
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
        ///Creates a new call builder for the [`revokeRole`] function.
        pub fn revokeRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, revokeRoleCall, N> {
            self.call_builder(&revokeRoleCall { role, account })
        }
        ///Creates a new call builder for the [`setOracleWallet`] function.
        pub fn setOracleWallet(
            &self,
            _newOracle: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, setOracleWalletCall, N> {
            self.call_builder(&setOracleWalletCall { _newOracle })
        }
        ///Creates a new call builder for the [`setPermissionSlipSigner`] function.
        pub fn setPermissionSlipSigner(
            &self,
            _newSigner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, setPermissionSlipSignerCall, N> {
            self.call_builder(
                &setPermissionSlipSignerCall {
                    _newSigner,
                },
            )
        }
        ///Creates a new call builder for the [`setPlatformFee`] function.
        pub fn setPlatformFee(
            &self,
            newFee: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, setPlatformFeeCall, N> {
            self.call_builder(&setPlatformFeeCall { newFee })
        }
        ///Creates a new call builder for the [`supportedTokens`] function.
        pub fn supportedTokens(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, supportedTokensCall, N> {
            self.call_builder(&supportedTokensCall(_0))
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
    > PaymentEscrowInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<&P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`PaymentCompleted`] event.
        pub fn PaymentCompleted_filter(
            &self,
        ) -> alloy_contract::Event<&P, PaymentCompleted, N> {
            self.event_filter::<PaymentCompleted>()
        }
        ///Creates a new event filter for the [`PaymentCreated`] event.
        pub fn PaymentCreated_filter(
            &self,
        ) -> alloy_contract::Event<&P, PaymentCreated, N> {
            self.event_filter::<PaymentCreated>()
        }
        ///Creates a new event filter for the [`PaymentProcessing`] event.
        pub fn PaymentProcessing_filter(
            &self,
        ) -> alloy_contract::Event<&P, PaymentProcessing, N> {
            self.event_filter::<PaymentProcessing>()
        }
        ///Creates a new event filter for the [`PaymentRefunded`] event.
        pub fn PaymentRefunded_filter(
            &self,
        ) -> alloy_contract::Event<&P, PaymentRefunded, N> {
            self.event_filter::<PaymentRefunded>()
        }
        ///Creates a new event filter for the [`PlatformFeeUpdated`] event.
        pub fn PlatformFeeUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, PlatformFeeUpdated, N> {
            self.event_filter::<PlatformFeeUpdated>()
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
        ///Creates a new event filter for the [`SupportedTokenAdded`] event.
        pub fn SupportedTokenAdded_filter(
            &self,
        ) -> alloy_contract::Event<&P, SupportedTokenAdded, N> {
            self.event_filter::<SupportedTokenAdded>()
        }
        ///Creates a new event filter for the [`SupportedTokenRemoved`] event.
        pub fn SupportedTokenRemoved_filter(
            &self,
        ) -> alloy_contract::Event<&P, SupportedTokenRemoved, N> {
            self.event_filter::<SupportedTokenRemoved>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<&P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
