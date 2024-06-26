// Generated by the protocol buffer compiler. DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: wallet_api/v1/wallet.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package wallet_api.v1;

@kotlin.jvm.JvmName("-initializelistWalletBalanceDetailsResponse")
public inline fun listWalletBalanceDetailsResponse(block: wallet_api.v1.ListWalletBalanceDetailsResponseKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.ListWalletBalanceDetailsResponse =
  wallet_api.v1.ListWalletBalanceDetailsResponseKt.Dsl._create(wallet_api.v1.WalletOuterClass.ListWalletBalanceDetailsResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * [Example]
 * {
 * "balances": [
 * {
 * "balance": {
 * "amount": {
 * "currency_code":"USD",
 * "amount":5700,
 * "decimals":2
 * },
 * "file_count": 57
 * },
 * "kind": 1
 * }
 * ]
 * }
 * ```
 *
 * Protobuf type `wallet_api.v1.ListWalletBalanceDetailsResponse`
 */
public object ListWalletBalanceDetailsResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: wallet_api.v1.WalletOuterClass.ListWalletBalanceDetailsResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: wallet_api.v1.WalletOuterClass.ListWalletBalanceDetailsResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): wallet_api.v1.WalletOuterClass.ListWalletBalanceDetailsResponse = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class BalancesProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * List of Balances in Wallet.
     * ```
     *
     * `repeated .wallet_api.v1.BalanceDetail balances = 1;`
     */
     public val balances: com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.BalanceDetail, BalancesProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getBalancesList()
      )
    /**
     * ```
     * List of Balances in Wallet.
     * ```
     *
     * `repeated .wallet_api.v1.BalanceDetail balances = 1;`
     * @param value The balances to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addBalances")
    public fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.BalanceDetail, BalancesProxy>.add(value: wallet_api.v1.WalletOuterClass.BalanceDetail) {
      _builder.addBalances(value)
    }
    /**
     * ```
     * List of Balances in Wallet.
     * ```
     *
     * `repeated .wallet_api.v1.BalanceDetail balances = 1;`
     * @param value The balances to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignBalances")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.BalanceDetail, BalancesProxy>.plusAssign(value: wallet_api.v1.WalletOuterClass.BalanceDetail) {
      add(value)
    }
    /**
     * ```
     * List of Balances in Wallet.
     * ```
     *
     * `repeated .wallet_api.v1.BalanceDetail balances = 1;`
     * @param values The balances to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllBalances")
    public fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.BalanceDetail, BalancesProxy>.addAll(values: kotlin.collections.Iterable<wallet_api.v1.WalletOuterClass.BalanceDetail>) {
      _builder.addAllBalances(values)
    }
    /**
     * ```
     * List of Balances in Wallet.
     * ```
     *
     * `repeated .wallet_api.v1.BalanceDetail balances = 1;`
     * @param values The balances to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllBalances")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.BalanceDetail, BalancesProxy>.plusAssign(values: kotlin.collections.Iterable<wallet_api.v1.WalletOuterClass.BalanceDetail>) {
      addAll(values)
    }
    /**
     * ```
     * List of Balances in Wallet.
     * ```
     *
     * `repeated .wallet_api.v1.BalanceDetail balances = 1;`
     * @param index The index to set the value at.
     * @param value The balances to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setBalances")
    public operator fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.BalanceDetail, BalancesProxy>.set(index: kotlin.Int, value: wallet_api.v1.WalletOuterClass.BalanceDetail) {
      _builder.setBalances(index, value)
    }
    /**
     * ```
     * List of Balances in Wallet.
     * ```
     *
     * `repeated .wallet_api.v1.BalanceDetail balances = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearBalances")
    public fun com.google.protobuf.kotlin.DslList<wallet_api.v1.WalletOuterClass.BalanceDetail, BalancesProxy>.clear() {
      _builder.clearBalances()
    }

  }
}
@kotlin.jvm.JvmSynthetic
public inline fun wallet_api.v1.WalletOuterClass.ListWalletBalanceDetailsResponse.copy(block: `wallet_api.v1`.ListWalletBalanceDetailsResponseKt.Dsl.() -> kotlin.Unit): wallet_api.v1.WalletOuterClass.ListWalletBalanceDetailsResponse =
  `wallet_api.v1`.ListWalletBalanceDetailsResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

