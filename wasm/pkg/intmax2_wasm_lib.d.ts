/* tslint:disable */
/* eslint-disable */
/**
 * Generate a new key pair from a provisional private key.
 * @param {string} provisional_private_key
 * @returns {Promise<Key>}
 */
export function generate_key_from_provisional(provisional_private_key: string): Promise<Key>;
/**
 * Function to take a backup before calling the deposit function of the liquidity contract.
 * You can also get the pubkey_salt_hash from the return value.
 * @param {Config} config
 * @param {string} private_key
 * @param {string} amount
 * @param {number} token_index
 * @returns {Promise<string>}
 */
export function prepare_deposit(config: Config, private_key: string, amount: string, token_index: number): Promise<string>;
/**
 * Function to send a tx request to the block builder. The return value contains information to take a backup.
 * @param {Config} config
 * @param {string} block_builder_url
 * @param {string} private_key
 * @param {(JsTransfer)[]} transfers
 * @returns {Promise<TxRequestResult>}
 */
export function send_tx_request(config: Config, block_builder_url: string, private_key: string, transfers: (JsTransfer)[]): Promise<TxRequestResult>;
/**
 * Function to query the block proposal from the block builder.
 * The return value is the block proposal or null if the proposal is not found.
 * If got an invalid proposal, it will return an error.
 * @param {Config} config
 * @param {string} block_builder_url
 * @param {string} private_key
 * @param {JsTx} tx
 * @returns {Promise<any>}
 */
export function query_proposal(config: Config, block_builder_url: string, private_key: string, tx: JsTx): Promise<any>;
/**
 * In this function, query block proposal from the block builder,
 * and then send the signed tx tree root to the block builder.
 * A backup of the tx is also taken.
 * You need to call send_tx_request before calling this function.
 * The return value is the tx tree root.
 * @param {Config} config
 * @param {string} block_builder_url
 * @param {string} private_key
 * @param {any} tx_request_memo
 * @param {any} proposal
 * @returns {Promise<string>}
 */
export function finalize_tx(config: Config, block_builder_url: string, private_key: string, tx_request_memo: any, proposal: any): Promise<string>;
/**
 * Synchronize the user's balance proof. It may take a long time to generate ZKP.
 * @param {Config} config
 * @param {string} private_key
 * @returns {Promise<void>}
 */
export function sync(config: Config, private_key: string): Promise<void>;
/**
 * Synchronize the user's withdrawal proof, and send request to the withdrawal aggregator.
 * It may take a long time to generate ZKP.
 * @param {Config} config
 * @param {string} private_key
 * @returns {Promise<void>}
 */
export function sync_withdrawals(config: Config, private_key: string): Promise<void>;
/**
 * Get the user's data. It is recommended to sync before calling this function.
 * @param {Config} config
 * @param {string} private_key
 * @returns {Promise<JsUserData>}
 */
export function get_user_data(config: Config, private_key: string): Promise<JsUserData>;
/**
 * Decrypt the deposit data.
 * @param {string} private_key
 * @param {Uint8Array} data
 * @returns {Promise<JsDepositData>}
 */
export function decrypt_deposit_data(private_key: string, data: Uint8Array): Promise<JsDepositData>;
/**
 * Decrypt the transfer data. This is also used to decrypt the withdrawal data.
 * @param {string} private_key
 * @param {Uint8Array} data
 * @returns {Promise<JsTransferData>}
 */
export function decrypt_transfer_data(private_key: string, data: Uint8Array): Promise<JsTransferData>;
/**
 * Decrypt the tx data.
 * @param {string} private_key
 * @param {Uint8Array} data
 * @returns {Promise<JsTxData>}
 */
export function decrypt_tx_data(private_key: string, data: Uint8Array): Promise<JsTxData>;
/**
 * Function to mimic the deposit call of the contract. For development purposes only.
 * @param {string} contract_server_url
 * @param {string} pubkey_salt_hash
 * @param {string} amount
 * @returns {Promise<void>}
 */
export function mimic_deposit(contract_server_url: string, pubkey_salt_hash: string, amount: string): Promise<void>;
export class Config {
  free(): void;
  /**
   * @param {string} store_vault_server_url
   * @param {string} block_validity_prover_url
   * @param {string} balance_prover_url
   * @param {string} withdrawal_aggregator_url
   * @param {bigint} deposit_timeout
   * @param {bigint} tx_timeout
   * @returns {Config}
   */
  static new(store_vault_server_url: string, block_validity_prover_url: string, balance_prover_url: string, withdrawal_aggregator_url: string, deposit_timeout: bigint, tx_timeout: bigint): Config;
/**
 * URL of the balance prover
 */
  balance_prover_url: string;
/**
 * URL of the block validity prover
 */
  block_validity_prover_url: string;
/**
 * Time to reach the rollup contract after taking a backup of the deposit
 * If this time is exceeded, the deposit backup will be ignored
 */
  deposit_timeout: bigint;
/**
 * URL of the store vault server
 */
  store_vault_server_url: string;
/**
 * Time to reach the rollup contract after sending a tx request
 * If this time is exceeded, the tx request will be ignored
 */
  tx_timeout: bigint;
/**
 * URL of the withdrawal aggregator
 */
  withdrawal_aggregator_url: string;
}
export class JsDepositData {
  free(): void;
  amount: string;
  deposit_salt: string;
  pubkey_salt_hash: string;
  token_index: number;
}
export class JsGenericAddress {
  free(): void;
  /**
   * @param {boolean} is_pubkey
   * @param {string} data
   */
  constructor(is_pubkey: boolean, data: string);
  data: string;
  is_pubkey: boolean;
}
export class JsTransfer {
  free(): void;
  /**
   * @param {JsGenericAddress} recipient
   * @param {number} token_index
   * @param {string} amount
   * @param {string} salt
   */
  constructor(recipient: JsGenericAddress, token_index: number, amount: string, salt: string);
  amount: string;
  recipient: JsGenericAddress;
  salt: string;
  token_index: number;
}
export class JsTransferData {
  free(): void;
  sender: string;
  transfer: JsTransfer;
}
export class JsTx {
  free(): void;
  nonce: number;
  transfer_tree_root: string;
}
export class JsTxData {
  free(): void;
  transfers: (JsTransfer)[];
  tx: JsTx;
}
export class JsUserData {
  free(): void;
  balances: (TokenBalance)[];
  block_number: number;
  deposit_lpt: bigint;
  private_commitment: string;
  processed_deposit_uuids: (string)[];
  processed_transfer_uuids: (string)[];
  processed_tx_uuids: (string)[];
  processed_withdrawal_uuids: (string)[];
  pubkey: string;
  transfer_lpt: bigint;
  tx_lpt: bigint;
  withdrawal_lpt: bigint;
}
export class Key {
  free(): void;
  privkey: string;
  pubkey: string;
}
export class TokenBalance {
  free(): void;
  amount: string;
  is_insufficient: boolean;
  token_index: number;
}
export class TxRequestResult {
  free(): void;
  memo: any;
  tx: JsTx;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_key_free: (a: number, b: number) => void;
  readonly __wbg_get_key_privkey: (a: number) => Array;
  readonly __wbg_set_key_privkey: (a: number, b: number, c: number) => void;
  readonly __wbg_get_key_pubkey: (a: number) => Array;
  readonly __wbg_set_key_pubkey: (a: number, b: number, c: number) => void;
  readonly __wbg_txrequestresult_free: (a: number, b: number) => void;
  readonly __wbg_get_txrequestresult_tx: (a: number) => number;
  readonly __wbg_set_txrequestresult_tx: (a: number, b: number) => void;
  readonly __wbg_get_txrequestresult_memo: (a: number) => number;
  readonly __wbg_set_txrequestresult_memo: (a: number, b: number) => void;
  readonly generate_key_from_provisional: (a: number, b: number) => number;
  readonly prepare_deposit: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly send_tx_request: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly query_proposal: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly finalize_tx: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly sync: (a: number, b: number, c: number) => number;
  readonly sync_withdrawals: (a: number, b: number, c: number) => number;
  readonly get_user_data: (a: number, b: number, c: number) => number;
  readonly decrypt_deposit_data: (a: number, b: number, c: number, d: number) => number;
  readonly decrypt_transfer_data: (a: number, b: number, c: number, d: number) => number;
  readonly decrypt_tx_data: (a: number, b: number, c: number, d: number) => number;
  readonly mimic_deposit: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly __wbg_jsgenericaddress_free: (a: number, b: number) => void;
  readonly __wbg_get_jsgenericaddress_is_pubkey: (a: number) => number;
  readonly __wbg_set_jsgenericaddress_is_pubkey: (a: number, b: number) => void;
  readonly jsgenericaddress_new: (a: number, b: number, c: number) => number;
  readonly __wbg_jstransfer_free: (a: number, b: number) => void;
  readonly __wbg_get_jstransfer_recipient: (a: number) => number;
  readonly __wbg_set_jstransfer_recipient: (a: number, b: number) => void;
  readonly __wbg_get_jstransfer_token_index: (a: number) => number;
  readonly __wbg_set_jstransfer_token_index: (a: number, b: number) => void;
  readonly __wbg_get_jstransfer_amount: (a: number) => Array;
  readonly __wbg_set_jstransfer_amount: (a: number, b: number, c: number) => void;
  readonly __wbg_get_jstransfer_salt: (a: number) => Array;
  readonly __wbg_set_jstransfer_salt: (a: number, b: number, c: number) => void;
  readonly jstransfer_new: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly __wbg_get_jstx_nonce: (a: number) => number;
  readonly __wbg_set_jstx_nonce: (a: number, b: number) => void;
  readonly __wbg_jsdepositdata_free: (a: number, b: number) => void;
  readonly __wbg_get_jsdepositdata_deposit_salt: (a: number) => Array;
  readonly __wbg_set_jsdepositdata_deposit_salt: (a: number, b: number, c: number) => void;
  readonly __wbg_get_jsdepositdata_pubkey_salt_hash: (a: number) => Array;
  readonly __wbg_set_jsdepositdata_pubkey_salt_hash: (a: number, b: number, c: number) => void;
  readonly __wbg_get_jsdepositdata_token_index: (a: number) => number;
  readonly __wbg_set_jsdepositdata_token_index: (a: number, b: number) => void;
  readonly __wbg_get_jsdepositdata_amount: (a: number) => Array;
  readonly __wbg_set_jsdepositdata_amount: (a: number, b: number, c: number) => void;
  readonly __wbg_jstransferdata_free: (a: number, b: number) => void;
  readonly __wbg_get_jstransferdata_transfer: (a: number) => number;
  readonly __wbg_set_jstransferdata_transfer: (a: number, b: number) => void;
  readonly __wbg_jstxdata_free: (a: number, b: number) => void;
  readonly __wbg_get_jstxdata_tx: (a: number) => number;
  readonly __wbg_get_jstxdata_transfers: (a: number) => Array;
  readonly __wbg_set_jstxdata_transfers: (a: number, b: number, c: number) => void;
  readonly __wbg_jsuserdata_free: (a: number, b: number) => void;
  readonly __wbg_get_jsuserdata_pubkey: (a: number) => Array;
  readonly __wbg_set_jsuserdata_pubkey: (a: number, b: number, c: number) => void;
  readonly __wbg_get_jsuserdata_block_number: (a: number) => number;
  readonly __wbg_set_jsuserdata_block_number: (a: number, b: number) => void;
  readonly __wbg_get_jsuserdata_balances: (a: number) => Array;
  readonly __wbg_set_jsuserdata_balances: (a: number, b: number, c: number) => void;
  readonly __wbg_get_jsuserdata_private_commitment: (a: number) => Array;
  readonly __wbg_set_jsuserdata_private_commitment: (a: number, b: number, c: number) => void;
  readonly __wbg_get_jsuserdata_deposit_lpt: (a: number) => number;
  readonly __wbg_set_jsuserdata_deposit_lpt: (a: number, b: number) => void;
  readonly __wbg_get_jsuserdata_transfer_lpt: (a: number) => number;
  readonly __wbg_set_jsuserdata_transfer_lpt: (a: number, b: number) => void;
  readonly __wbg_get_jsuserdata_tx_lpt: (a: number) => number;
  readonly __wbg_set_jsuserdata_tx_lpt: (a: number, b: number) => void;
  readonly __wbg_get_jsuserdata_withdrawal_lpt: (a: number) => number;
  readonly __wbg_set_jsuserdata_withdrawal_lpt: (a: number, b: number) => void;
  readonly __wbg_get_jsuserdata_processed_deposit_uuids: (a: number) => Array;
  readonly __wbg_set_jsuserdata_processed_deposit_uuids: (a: number, b: number, c: number) => void;
  readonly __wbg_get_jsuserdata_processed_transfer_uuids: (a: number) => Array;
  readonly __wbg_set_jsuserdata_processed_transfer_uuids: (a: number, b: number, c: number) => void;
  readonly __wbg_get_jsuserdata_processed_tx_uuids: (a: number) => Array;
  readonly __wbg_set_jsuserdata_processed_tx_uuids: (a: number, b: number, c: number) => void;
  readonly __wbg_get_jsuserdata_processed_withdrawal_uuids: (a: number) => Array;
  readonly __wbg_set_jsuserdata_processed_withdrawal_uuids: (a: number, b: number, c: number) => void;
  readonly __wbg_tokenbalance_free: (a: number, b: number) => void;
  readonly __wbg_get_tokenbalance_is_insufficient: (a: number) => number;
  readonly __wbg_set_tokenbalance_is_insufficient: (a: number, b: number) => void;
  readonly __wbg_jstx_free: (a: number, b: number) => void;
  readonly __wbg_get_tokenbalance_token_index: (a: number) => number;
  readonly __wbg_set_jstx_transfer_tree_root: (a: number, b: number, c: number) => void;
  readonly __wbg_set_jsgenericaddress_data: (a: number, b: number, c: number) => void;
  readonly __wbg_set_jstransferdata_sender: (a: number, b: number, c: number) => void;
  readonly __wbg_set_tokenbalance_amount: (a: number, b: number, c: number) => void;
  readonly __wbg_set_jstxdata_tx: (a: number, b: number) => void;
  readonly __wbg_set_tokenbalance_token_index: (a: number, b: number) => void;
  readonly __wbg_get_jstx_transfer_tree_root: (a: number) => Array;
  readonly __wbg_get_jsgenericaddress_data: (a: number) => Array;
  readonly __wbg_get_jstransferdata_sender: (a: number) => Array;
  readonly __wbg_get_tokenbalance_amount: (a: number) => Array;
  readonly __wbg_config_free: (a: number, b: number) => void;
  readonly __wbg_get_config_store_vault_server_url: (a: number) => Array;
  readonly __wbg_set_config_store_vault_server_url: (a: number, b: number, c: number) => void;
  readonly __wbg_get_config_block_validity_prover_url: (a: number) => Array;
  readonly __wbg_set_config_block_validity_prover_url: (a: number, b: number, c: number) => void;
  readonly __wbg_get_config_balance_prover_url: (a: number) => Array;
  readonly __wbg_set_config_balance_prover_url: (a: number, b: number, c: number) => void;
  readonly __wbg_get_config_withdrawal_aggregator_url: (a: number) => Array;
  readonly __wbg_set_config_withdrawal_aggregator_url: (a: number, b: number, c: number) => void;
  readonly __wbg_get_config_deposit_timeout: (a: number) => number;
  readonly __wbg_set_config_deposit_timeout: (a: number, b: number) => void;
  readonly __wbg_get_config_tx_timeout: (a: number) => number;
  readonly __wbg_set_config_tx_timeout: (a: number, b: number) => void;
  readonly config_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly closure533_externref_shim: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly closure650_externref_shim: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
