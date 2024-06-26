// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: wallet_api/v1/wallet.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package wallet_api.v1;

@kotlin.jvm.JvmName("-initializewalletBalance")
public inline fun walletBalance(block: wallet_api.v1.WalletBalanceKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.WalletBalance =
  wallet_api.v1.WalletBalanceKt.Dsl._create(wallet_api.v1.WalletOuterClass.WalletBalance.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "amount": {
 * "currency_code":"USD",
 * "amount":100,
 * "decimals":2
 * },
 * "file_count": 57
 * }
 * ```
 *
 * Protobuf type `wallet_api.v1.WalletBalance`
 */
public object WalletBalanceKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: wallet_api.v1.WalletOuterClass.WalletBalance.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: wallet_api.v1.WalletOuterClass.WalletBalance.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): wallet_api.v1.WalletOuterClass.WalletBalance = _builder.build()

    /**
     * ```
     * Sum of a particular currency available in the wallet.
     * ```
     *
     * `.common.Amount amount = 1;`
     */
    public var amount: common.Common.Amount
      @JvmName("getAmount")
      get() = _builder.getAmount()
      @JvmName("setAmount")
      set(value) {
        _builder.setAmount(value)
      }
    /**
     * ```
     * Sum of a particular currency available in the wallet.
     * ```
     *
     * `.common.Amount amount = 1;`
     */
    public fun clearAmount() {
      _builder.clearAmount()
    }
    /**
     * ```
     * Sum of a particular currency available in the wallet.
     * ```
     *
     * `.common.Amount amount = 1;`
     * @return Whether the amount field is set.
     */
    public fun hasAmount(): kotlin.Boolean {
      return _builder.hasAmount()
    }
    public val WalletBalanceKt.Dsl.amountOrNull: common.Common.Amount?
      get() = _builder.amountOrNull

    /**
     * ```
     * Total count of a particular banknote
     * ```
     *
     * `int64 file_count = 2;`
     */
    public var fileCount: kotlin.Long
      @JvmName("getFileCount")
      get() = _builder.getFileCount()
      @JvmName("setFileCount")
      set(value) {
        _builder.setFileCount(value)
      }
    /**
     * ```
     * Total count of a particular banknote
     * ```
     *
     * `int64 file_count = 2;`
     */
    public fun clearFileCount() {
      _builder.clearFileCount()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun wallet_api.v1.WalletOuterClass.WalletBalance.copy(block: `wallet_api.v1`.WalletBalanceKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.WalletBalance =
  `wallet_api.v1`.WalletBalanceKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val wallet_api.v1.WalletOuterClass.WalletBalanceOrBuilder.amountOrNull: common.Common.Amount?
  get() = if (hasAmount()) getAmount() else null

