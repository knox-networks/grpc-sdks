// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: wallet_api/v1/wallet.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package wallet_api.v1;

@kotlin.jvm.JvmName("-initializelistAssetFilesResponse")
public inline fun listAssetFilesResponse(block: wallet_api.v1.ListAssetFilesResponseKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.ListAssetFilesResponse =
  wallet_api.v1.ListAssetFilesResponseKt.Dsl._create(wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "total_owned_funds": 2,
 * "funds": [
 * {
 * "wallet_id": "zWalletIDMultibase58Encoded",
 * "promissory": {
 * "version": "0.1.0",
 * "id": "PROMISSORY_ID",
 * "currency": "USD",
 * "amount": 100,
 * "decimals": 2,
 * "authority": "zAuthorityPublicKeyMultibase58Encoded",
 * "issuer": "zIssuerPublicKeyMultibase58Encoded",
 * "issuance_date": "2006-01-02T15:04:05Z",
 * "block_depth": 3,
 * "authorization_signature": "ByteLengthAuthSignatureVariesByAlgo=",
 * "transfer_signature": "ByteLengthTransferSignatureVariesByAlgo=",
 * "notaries": [
 * {
 * "did": "did:method-name:zDIDMultibase58Encoded",
 * "signature": "ByteLengthSignatureVariesByAlgo="
 * },
 * {
 * "did": "did:method-name:zDIDMultibase58Encoded",
 * "signature": "ByteLengthSignatureVariesByAlgo="
 * }
 * ]
 * }
 * },
 * {
 * "wallet_id": "zWalletIDMultibase58Encoded",
 * "promissory": {
 * "version": "0.1.0",
 * "id": "PROMISSORY_ID",
 * "currency": "USD",
 * "amount": 500,
 * "decimals": 2,
 * "authority": "zAuthorityPublicKeyMultibase58Encoded",
 * "issuer": "zIssuerPublicKeyMultibase58Encoded",
 * "issuance_date": "2006-01-02T15:04:05Z",
 * "block_depth": 3,
 * "authorization_signature": "ByteLengthAuthSignatureVariesByAlgo=",
 * "transfer_signature": "ByteLengthTransferSignatureVariesByAlgo=",
 * "notaries": [
 * {
 * "did": "did:method-name:zDIDMultibase58Encoded",
 * "signature": "ByteLengthSignatureVariesByAlgo="
 * },
 * {
 * "did": "did:method-name:zDIDMultibase58Encoded",
 * "signature": "ByteLengthSignatureVariesByAlgo="
 * }
 * ]
 * }
 * }
 * ]
 * }
 * ```
 *
 * Protobuf type `wallet_api.v1.ListAssetFilesResponse`
 */
public object ListAssetFilesResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): wallet_api.v1.WalletOuterClass.ListAssetFilesResponse = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class FundsProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * List of fund objects.
     * ```
     *
     * `repeated .wallet_api.v1.ListAssetFilesResponse.Fund funds = 1;`
     */
     public val funds: com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund, FundsProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getFundsList()
      )
    /**
     * ```
     * List of fund objects.
     * ```
     *
     * `repeated .wallet_api.v1.ListAssetFilesResponse.Fund funds = 1;`
     * @param value The funds to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addFunds")
    public fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund, FundsProxy>.add(value: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund) {
      _builder.addFunds(value)
    }
    /**
     * ```
     * List of fund objects.
     * ```
     *
     * `repeated .wallet_api.v1.ListAssetFilesResponse.Fund funds = 1;`
     * @param value The funds to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignFunds")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund, FundsProxy>.plusAssign(value: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund) {
      add(value)
    }
    /**
     * ```
     * List of fund objects.
     * ```
     *
     * `repeated .wallet_api.v1.ListAssetFilesResponse.Fund funds = 1;`
     * @param values The funds to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllFunds")
    public fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund, FundsProxy>.addAll(values: kotlin.collections.Iterable<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund>) {
      _builder.addAllFunds(values)
    }
    /**
     * ```
     * List of fund objects.
     * ```
     *
     * `repeated .wallet_api.v1.ListAssetFilesResponse.Fund funds = 1;`
     * @param values The funds to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllFunds")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund, FundsProxy>.plusAssign(values: kotlin.collections.Iterable<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund>) {
      addAll(values)
    }
    /**
     * ```
     * List of fund objects.
     * ```
     *
     * `repeated .wallet_api.v1.ListAssetFilesResponse.Fund funds = 1;`
     * @param index The index to set the value at.
     * @param value The funds to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setFunds")
    public operator fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund, FundsProxy>.set(index: kotlin.Int, value: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund) {
      _builder.setFunds(index, value)
    }
    /**
     * ```
     * List of fund objects.
     * ```
     *
     * `repeated .wallet_api.v1.ListAssetFilesResponse.Fund funds = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearFunds")
    public fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund, FundsProxy>.clear() {
      _builder.clearFunds()
    }


    /**
     * ```
     * Number of funds belonging to the owner.
     * ```
     *
     * `uint32 total_owned_funds = 2;`
     */
    public var totalOwnedFunds: kotlin.Int
      @JvmName("getTotalOwnedFunds")
      get() = _builder.getTotalOwnedFunds()
      @JvmName("setTotalOwnedFunds")
      set(value) {
        _builder.setTotalOwnedFunds(value)
      }
    /**
     * ```
     * Number of funds belonging to the owner.
     * ```
     *
     * `uint32 total_owned_funds = 2;`
     */
    public fun clearTotalOwnedFunds() {
      _builder.clearTotalOwnedFunds()
    }
  }
  @kotlin.jvm.JvmName("-initializenotary")
  public inline fun notary(block: wallet_api.v1.ListAssetFilesResponseKt.NotaryKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary =
    wallet_api.v1.ListAssetFilesResponseKt.NotaryKt.Dsl._create(wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary.newBuilder()).apply { block() }._build()
  /**
   * ```
   * [Example]
   * {
   * "did": "did:method-name:zDIDMultibase58Encoded",
   * "signature": "ByteLengthSignatureVariesByAlgo="
   * }
   * ```
   *
   * Protobuf type `wallet_api.v1.ListAssetFilesResponse.Notary`
   */
  public object NotaryKt {
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    @com.google.protobuf.kotlin.ProtoDslMarker
    public class Dsl private constructor(
      private val _builder: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary.Builder
    ) {
      public companion object {
        @kotlin.jvm.JvmSynthetic
        @kotlin.PublishedApi
        internal fun _create(builder: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary.Builder): Dsl = Dsl(builder)
      }

      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _build(): wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary = _builder.build()

      /**
       * ```
       * DID of Notary
       * ```
       *
       * `string did = 1;`
       */
      public var did: kotlin.String
        @JvmName("getDid")
        get() = _builder.getDid()
        @JvmName("setDid")
        set(value) {
          _builder.setDid(value)
        }
      /**
       * ```
       * DID of Notary
       * ```
       *
       * `string did = 1;`
       */
      public fun clearDid() {
        _builder.clearDid()
      }

      /**
       * ```
       * Signature of Notary
       * ```
       *
       * `bytes signature = 2;`
       */
      public var signature: com.google.protobuf.ByteString
        @JvmName("getSignature")
        get() = _builder.getSignature()
        @JvmName("setSignature")
        set(value) {
          _builder.setSignature(value)
        }
      /**
       * ```
       * Signature of Notary
       * ```
       *
       * `bytes signature = 2;`
       */
      public fun clearSignature() {
        _builder.clearSignature()
      }
    }
  }
  @kotlin.jvm.JvmName("-initializepromissory")
  public inline fun promissory(block: wallet_api.v1.ListAssetFilesResponseKt.PromissoryKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Promissory =
    wallet_api.v1.ListAssetFilesResponseKt.PromissoryKt.Dsl._create(wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Promissory.newBuilder()).apply { block() }._build()
  /**
   * ```
   * [Example]
   * {
   * "version": "0.1.0",
   * "id": "PROMISSORY_ID",
   * "currency": "USD",
   * "amount": 500,
   * "decimals": 2,
   * "authority": "zAuthorityPublicKeyMultibase58Encoded",
   * "issuer": "zIssuerPublicKeyMultibase58Encoded",
   * "issuance_date": "2006-01-02T15:04:05Z",
   * "block_depth": 3,
   * "authorization_signature": "ByteLengthAuthSignatureVariesByAlgo=",
   * "transfer_signature": "ByteLengthTransferSignatureVariesByAlgo=",
   * "notaries": [
   * {
   * "did": "did:method-name:zDIDMultibase58Encoded",
   * "signature": "ByteLengthSignatureVariesByAlgo="
   * },
   * {
   * "did": "did:method-name:zDIDMultibase58Encoded",
   * "signature": "ByteLengthSignatureVariesByAlgo="
   * }
   * ]
   * }
   * ```
   *
   * Protobuf type `wallet_api.v1.ListAssetFilesResponse.Promissory`
   */
  public object PromissoryKt {
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    @com.google.protobuf.kotlin.ProtoDslMarker
    public class Dsl private constructor(
      private val _builder: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Promissory.Builder
    ) {
      public companion object {
        @kotlin.jvm.JvmSynthetic
        @kotlin.PublishedApi
        internal fun _create(builder: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Promissory.Builder): Dsl = Dsl(builder)
      }

      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _build(): wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Promissory = _builder.build()

      /**
       * ```
       * Version of the digital banknote file.
       * ```
       *
       * `string version = 1;`
       */
      public var version: kotlin.String
        @JvmName("getVersion")
        get() = _builder.getVersion()
        @JvmName("setVersion")
        set(value) {
          _builder.setVersion(value)
        }
      /**
       * ```
       * Version of the digital banknote file.
       * ```
       *
       * `string version = 1;`
       */
      public fun clearVersion() {
        _builder.clearVersion()
      }

      /**
       * ```
       * UUID of the digital banknote file.
       * ```
       *
       * `string id = 3;`
       */
      public var id: kotlin.String
        @JvmName("getId")
        get() = _builder.getId()
        @JvmName("setId")
        set(value) {
          _builder.setId(value)
        }
      /**
       * ```
       * UUID of the digital banknote file.
       * ```
       *
       * `string id = 3;`
       */
      public fun clearId() {
        _builder.clearId()
      }

      /**
       * ```
       * ISO-4217 currency code.
       * ```
       *
       * `string currency = 4;`
       */
      public var currency: kotlin.String
        @JvmName("getCurrency")
        get() = _builder.getCurrency()
        @JvmName("setCurrency")
        set(value) {
          _builder.setCurrency(value)
        }
      /**
       * ```
       * ISO-4217 currency code.
       * ```
       *
       * `string currency = 4;`
       */
      public fun clearCurrency() {
        _builder.clearCurrency()
      }

      /**
       * ```
       * Amount of the digital banknote.
       * ```
       *
       * `int64 amount = 5;`
       */
      public var amount: kotlin.Long
        @JvmName("getAmount")
        get() = _builder.getAmount()
        @JvmName("setAmount")
        set(value) {
          _builder.setAmount(value)
        }
      /**
       * ```
       * Amount of the digital banknote.
       * ```
       *
       * `int64 amount = 5;`
       */
      public fun clearAmount() {
        _builder.clearAmount()
      }

      /**
       * ```
       * Decimal precision.
       * ```
       *
       * `uint32 decimals = 6;`
       */
      public var decimals: kotlin.Int
        @JvmName("getDecimals")
        get() = _builder.getDecimals()
        @JvmName("setDecimals")
        set(value) {
          _builder.setDecimals(value)
        }
      /**
       * ```
       * Decimal precision.
       * ```
       *
       * `uint32 decimals = 6;`
       */
      public fun clearDecimals() {
        _builder.clearDecimals()
      }

      /**
       * ```
       * public key of the monetary Authority.
       * ```
       *
       * `string authority = 7;`
       */
      public var authority: kotlin.String
        @JvmName("getAuthority")
        get() = _builder.getAuthority()
        @JvmName("setAuthority")
        set(value) {
          _builder.setAuthority(value)
        }
      /**
       * ```
       * public key of the monetary Authority.
       * ```
       *
       * `string authority = 7;`
       */
      public fun clearAuthority() {
        _builder.clearAuthority()
      }

      /**
       * ```
       * public key of the Issuer.
       * ```
       *
       * `string issuer = 8;`
       */
      public var issuer: kotlin.String
        @JvmName("getIssuer")
        get() = _builder.getIssuer()
        @JvmName("setIssuer")
        set(value) {
          _builder.setIssuer(value)
        }
      /**
       * ```
       * public key of the Issuer.
       * ```
       *
       * `string issuer = 8;`
       */
      public fun clearIssuer() {
        _builder.clearIssuer()
      }

      /**
       * ```
       * RFC3339 timestamp of the issuance date.
       * ```
       *
       * `string issuance_date = 9;`
       */
      public var issuanceDate: kotlin.String
        @JvmName("getIssuanceDate")
        get() = _builder.getIssuanceDate()
        @JvmName("setIssuanceDate")
        set(value) {
          _builder.setIssuanceDate(value)
        }
      /**
       * ```
       * RFC3339 timestamp of the issuance date.
       * ```
       *
       * `string issuance_date = 9;`
       */
      public fun clearIssuanceDate() {
        _builder.clearIssuanceDate()
      }

      /**
       * ```
       * Number of signature blocks, representing the owner history.
       * ```
       *
       * `uint32 block_depth = 10;`
       */
      public var blockDepth: kotlin.Int
        @JvmName("getBlockDepth")
        get() = _builder.getBlockDepth()
        @JvmName("setBlockDepth")
        set(value) {
          _builder.setBlockDepth(value)
        }
      /**
       * ```
       * Number of signature blocks, representing the owner history.
       * ```
       *
       * `uint32 block_depth = 10;`
       */
      public fun clearBlockDepth() {
        _builder.clearBlockDepth()
      }

      /**
       * ```
       * Signature of the latest block hash from the Authority.
       * ```
       *
       * `bytes authorization_signature = 11;`
       */
      public var authorizationSignature: com.google.protobuf.ByteString
        @JvmName("getAuthorizationSignature")
        get() = _builder.getAuthorizationSignature()
        @JvmName("setAuthorizationSignature")
        set(value) {
          _builder.setAuthorizationSignature(value)
        }
      /**
       * ```
       * Signature of the latest block hash from the Authority.
       * ```
       *
       * `bytes authorization_signature = 11;`
       */
      public fun clearAuthorizationSignature() {
        _builder.clearAuthorizationSignature()
      }

      /**
       * ```
       * Signature of the previous owner.
       * ```
       *
       * `bytes transfer_signature = 12;`
       */
      public var transferSignature: com.google.protobuf.ByteString
        @JvmName("getTransferSignature")
        get() = _builder.getTransferSignature()
        @JvmName("setTransferSignature")
        set(value) {
          _builder.setTransferSignature(value)
        }
      /**
       * ```
       * Signature of the previous owner.
       * ```
       *
       * `bytes transfer_signature = 12;`
       */
      public fun clearTransferSignature() {
        _builder.clearTransferSignature()
      }

      /**
       * An uninstantiable, behaviorless type to represent the field in
       * generics.
       */
      @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
      public class NotariesProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
      /**
       * ```
       * List of Notaries.
       * ```
       *
       * `repeated .wallet_api.v1.ListAssetFilesResponse.Notary notaries = 13;`
       */
       public val notaries: com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary, NotariesProxy>
        @kotlin.jvm.JvmSynthetic
        get() = com.google.protobuf.kotlin.DslList(
          _builder.getNotariesList()
        )
      /**
       * ```
       * List of Notaries.
       * ```
       *
       * `repeated .wallet_api.v1.ListAssetFilesResponse.Notary notaries = 13;`
       * @param value The notaries to add.
       */
      @kotlin.jvm.JvmSynthetic
      @kotlin.jvm.JvmName("addNotaries")
      public fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary, NotariesProxy>.add(value: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary) {
        _builder.addNotaries(value)
      }
      /**
       * ```
       * List of Notaries.
       * ```
       *
       * `repeated .wallet_api.v1.ListAssetFilesResponse.Notary notaries = 13;`
       * @param value The notaries to add.
       */
      @kotlin.jvm.JvmSynthetic
      @kotlin.jvm.JvmName("plusAssignNotaries")
      @Suppress("NOTHING_TO_INLINE")
      public inline operator fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary, NotariesProxy>.plusAssign(value: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary) {
        add(value)
      }
      /**
       * ```
       * List of Notaries.
       * ```
       *
       * `repeated .wallet_api.v1.ListAssetFilesResponse.Notary notaries = 13;`
       * @param values The notaries to add.
       */
      @kotlin.jvm.JvmSynthetic
      @kotlin.jvm.JvmName("addAllNotaries")
      public fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary, NotariesProxy>.addAll(values: kotlin.collections.Iterable<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary>) {
        _builder.addAllNotaries(values)
      }
      /**
       * ```
       * List of Notaries.
       * ```
       *
       * `repeated .wallet_api.v1.ListAssetFilesResponse.Notary notaries = 13;`
       * @param values The notaries to add.
       */
      @kotlin.jvm.JvmSynthetic
      @kotlin.jvm.JvmName("plusAssignAllNotaries")
      @Suppress("NOTHING_TO_INLINE")
      public inline operator fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary, NotariesProxy>.plusAssign(values: kotlin.collections.Iterable<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary>) {
        addAll(values)
      }
      /**
       * ```
       * List of Notaries.
       * ```
       *
       * `repeated .wallet_api.v1.ListAssetFilesResponse.Notary notaries = 13;`
       * @param index The index to set the value at.
       * @param value The notaries to set.
       */
      @kotlin.jvm.JvmSynthetic
      @kotlin.jvm.JvmName("setNotaries")
      public operator fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary, NotariesProxy>.set(index: kotlin.Int, value: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary) {
        _builder.setNotaries(index, value)
      }
      /**
       * ```
       * List of Notaries.
       * ```
       *
       * `repeated .wallet_api.v1.ListAssetFilesResponse.Notary notaries = 13;`
       */
      @kotlin.jvm.JvmSynthetic
      @kotlin.jvm.JvmName("clearNotaries")
      public fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary, NotariesProxy>.clear() {
        _builder.clearNotaries()
      }

    }
  }
  @kotlin.jvm.JvmName("-initializefund")
  public inline fun fund(block: wallet_api.v1.ListAssetFilesResponseKt.FundKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund =
    wallet_api.v1.ListAssetFilesResponseKt.FundKt.Dsl._create(wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund.newBuilder()).apply { block() }._build()
  /**
   * ```
   * [Example]
   * {
   * "wallet_id": "zWalletIDMultibase58Encoded",
   * "promissory": {
   * "version": "0.1.0",
   * "id": "PROMISSORY_ID",
   * "currency": "USD",
   * "amount": 100,
   * "decimals": 2,
   * "authority": "zAuthorityPublicKeyMultibase58Encoded",
   * "issuer": "zIssuerPublicKeyMultibase58Encoded",
   * "issuance_date": "2006-01-02T15:04:05Z",
   * "block_depth": 3,
   * "authorization_signature": "ByteLengthAuthSignatureVariesByAlgo=",
   * "transfer_signature": "ByteLengthTransferSignatureVariesByAlgo=",
   * "notaries": [
   * {
   * "did": "did:method-name:zDIDMultibase58Encoded",
   * "signature": "ByteLengthSignatureVariesByAlgo="
   * },
   * {
   * "did": "did:method-name:zDIDMultibase58Encoded",
   * "signature": "ByteLengthSignatureVariesByAlgo="
   * }
   * ]
   * }
   * }
   * ```
   *
   * Protobuf type `wallet_api.v1.ListAssetFilesResponse.Fund`
   */
  public object FundKt {
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    @com.google.protobuf.kotlin.ProtoDslMarker
    public class Dsl private constructor(
      private val _builder: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund.Builder
    ) {
      public companion object {
        @kotlin.jvm.JvmSynthetic
        @kotlin.PublishedApi
        internal fun _create(builder: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund.Builder): Dsl = Dsl(builder)
      }

      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _build(): wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund = _builder.build()

      /**
       * ```
       * UUID of the wallet.
       * ```
       *
       * `string wallet_id = 1;`
       */
      public var walletId: kotlin.String
        @JvmName("getWalletId")
        get() = _builder.getWalletId()
        @JvmName("setWalletId")
        set(value) {
          _builder.setWalletId(value)
        }
      /**
       * ```
       * UUID of the wallet.
       * ```
       *
       * `string wallet_id = 1;`
       */
      public fun clearWalletId() {
        _builder.clearWalletId()
      }

      /**
       * ```
       * Digital Banknote object.
       * ```
       *
       * `.wallet_api.v1.ListAssetFilesResponse.Promissory promissory = 2;`
       */
      public var promissory: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Promissory
        @JvmName("getPromissory")
        get() = _builder.getPromissory()
        @JvmName("setPromissory")
        set(value) {
          _builder.setPromissory(value)
        }
      /**
       * ```
       * Digital Banknote object.
       * ```
       *
       * `.wallet_api.v1.ListAssetFilesResponse.Promissory promissory = 2;`
       */
      public fun clearPromissory() {
        _builder.clearPromissory()
      }
      /**
       * ```
       * Digital Banknote object.
       * ```
       *
       * `.wallet_api.v1.ListAssetFilesResponse.Promissory promissory = 2;`
       * @return Whether the promissory field is set.
       */
      public fun hasPromissory(): kotlin.Boolean {
        return _builder.hasPromissory()
      }
      public val FundKt.Dsl.promissoryOrNull: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Promissory?
        get() = _builder.promissoryOrNull

      /**
       * ```
       * Whether or not the digital banknote is reserved.
       * ```
       *
       * `bool reserved = 3;`
       */
      public var reserved: kotlin.Boolean
        @JvmName("getReserved")
        get() = _builder.getReserved()
        @JvmName("setReserved")
        set(value) {
          _builder.setReserved(value)
        }
      /**
       * ```
       * Whether or not the digital banknote is reserved.
       * ```
       *
       * `bool reserved = 3;`
       */
      public fun clearReserved() {
        _builder.clearReserved()
      }
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.copy(block: `wallet_api.v1`.ListAssetFilesResponseKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.ListAssetFilesResponse =
  `wallet_api.v1`.ListAssetFilesResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

@kotlin.jvm.JvmSynthetic
public inline fun wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary.copy(block: `wallet_api.v1`.ListAssetFilesResponseKt.NotaryKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Notary =
  `wallet_api.v1`.ListAssetFilesResponseKt.NotaryKt.Dsl._create(this.toBuilder()).apply { block() }._build()

@kotlin.jvm.JvmSynthetic
public inline fun wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Promissory.copy(block: `wallet_api.v1`.ListAssetFilesResponseKt.PromissoryKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Promissory =
  `wallet_api.v1`.ListAssetFilesResponseKt.PromissoryKt.Dsl._create(this.toBuilder()).apply { block() }._build()

@kotlin.jvm.JvmSynthetic
public inline fun wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund.copy(block: `wallet_api.v1`.ListAssetFilesResponseKt.FundKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Fund =
  `wallet_api.v1`.ListAssetFilesResponseKt.FundKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.FundOrBuilder.promissoryOrNull: wallet_api.v1.WalletOuterClass.ListAssetFilesResponse.Promissory?
  get() = if (hasPromissory()) getPromissory() else null

