// @generated
/// RemittanceRequest defines the input to a remittance operation in same or different currencies, including the recipient, the amount inclusive of currency in which remittance is desired, the promissory files to support the remittance and an optional exchange rate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemittanceRequest {
    /// Recipient public key to remit the promissory files to.
    #[prost(message, optional, tag="1")]
    pub recipient: ::core::option::Option<super::common::DynamicVerifier>,
    /// Total amount to remit, residual batch amounts returned to sender.
    #[prost(message, optional, tag="2")]
    pub amount: ::core::option::Option<super::common::Amount>,
    /// Batch of promissory files to be remitted; Promissory files need to be signed to treasury service. Promissory files of a batch must be of the same currency.
    #[prost(bytes="vec", repeated, tag="3")]
    pub promissory_files: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// Optional exchange rate required to perform conversion, otherwise will use spot rate If the batch of promissory files is of a different currency than the currency of the remittance request, then the exchange rate is required to convert the promissory files to the currency of the remittance request.
    #[prost(double, tag="4")]
    pub exchange_rate: f64,
}
/// RemittanceResponse is returned in response to a RemittanceRequest and includes the files sent to recipient at the applicable exchange rate, files representing a residual amount  left after the conversion if any and a fee.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemittanceResponse {
    /// Batch of exchanged promissory files remitted to the recipient of the request.
    #[prost(bytes="vec", repeated, tag="1")]
    pub exchanged_promissory_files: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// Transaction fee amount associated with remittance.
    #[prost(message, optional, tag="2")]
    pub fee_amount: ::core::option::Option<super::common::Amount>,
    /// Exchange rate used to perform conversion, if foreign currency transaction.
    #[prost(double, tag="3")]
    pub exchange_rate: f64,
    /// Batch of promissory files returned to the sender for (if any) residual amounts.
    #[prost(bytes="vec", repeated, tag="4")]
    pub refunded_promissory_files: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// GetSupplyRequest requests the count of files of various denominations for a given currency.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSupplyRequest {
    /// The currency code of the requested supply.
    #[prost(string, tag="1")]
    pub currency_code: ::prost::alloc::string::String,
}
/// The Denomination count object lists the count of the files of specific denominations held in the treasury vault, and the total value held in files of that denomination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenominationCount {
    /// The amount of the denomination.
    #[prost(message, optional, tag="1")]
    pub denomination: ::core::option::Option<super::common::Amount>,
    /// The number of promissories of this amount held in supply.
    #[prost(uint32, tag="2")]
    pub count: u32,
    /// The total value represented by promissories of this denomination.
    #[prost(message, optional, tag="3")]
    pub total: ::core::option::Option<super::common::Amount>,
}
/// GetSupplyResponse provides a list of file denominations in the Treasury service's Vault.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSupplyResponse {
    /// A list of denomination counts of the promissories held in supply.
    #[prost(message, repeated, tag="1")]
    pub supply: ::prost::alloc::vec::Vec<DenominationCount>,
}
include!("treasury.tonic.rs");
// @@protoc_insertion_point(module)
