/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export type JsBlockTag = BlockTag
export class BlockTag {
  static latest(): JsBlockTag
  static finalized(): JsBlockTag
  static number(x: bigint): JsBlockTag
}
export type JsHeliosClient = HeliosClient
export class HeliosClient {
  static withConfig(config: HeliosClientConfig): JsHeliosClient
  getBalance(address: string | Uint8Array, block_tag: BlockTag): Promise<bigint>
  waitSynced(): Promise<void>
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
