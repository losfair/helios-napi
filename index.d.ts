/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export type JsBlockTag = BlockTag
export class BlockTag {
  static latest(): JsBlockTag
  static finalized(): JsBlockTag
  static number(x: bigint): JsBlockTag
}
export type JsCallOpts = CallOpts
export class CallOpts {
  constructor()
  setFrom(address: string | Uint8Array): void
  setTo(address: string | Uint8Array): void
  setGas(gas: string | bigint): void
  setGasPrice(gas_price: string | bigint): void
  setValue(value: string | bigint): void
  setData(data: Uint8Array): void
}
export type JsHeliosClient = HeliosClient
export class HeliosClient {
  static withConfig(config: HeliosClientConfig): JsHeliosClient
  waitSynced(): Promise<void>
  getBalance(address: string | Uint8Array, block_tag: BlockTag): Promise<bigint>
  getCode(address: string | Uint8Array, block_tag: BlockTag): Promise<Buffer>
  getNonce(address: string | Uint8Array, block_tag: BlockTag): Promise<bigint>
  call(callOpts: CallOpts, blockTag: BlockTag): Promise<Buffer>
  estimateGas(callOpts: CallOpts): Promise<bigint>
  getBlockNumber(): Promise<bigint>
  getBlockByNumber(blockTag: BlockTag, fullTx: boolean): Promise<string>
  chainId(): Promise<bigint>
}
export type JsHeliosClientConfig = HeliosClientConfig
export class HeliosClientConfig {
  network?: string
  consensusRpc?: string
  executionRpc?: string
  dataDir?: string
  checkpoint?: string
  constructor(network?: string, consensusRpc?: string, executionRpc?: string, dataDir?: string, checkpoint?: string)
}
