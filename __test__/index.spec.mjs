import test from 'ava'
import * as helios from "../index.js"
import * as ethers from "ethers"
import * as fs from "fs/promises"
import { namehash } from '@ensdomains/ensjs/utils';

const vitalikAddress = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

const { data: [{ blockroot, slot }] } = await (await fetch("https://beaconcha.in/api/v1/epoch/finalized/slots")).json();
if (typeof blockroot !== "string") throw new Error("Invalid blockroot");
console.log(`using blockroot ${blockroot} at slot ${slot}`);

const config = new helios.HeliosClientConfig();
config.checkpoint = blockroot;

// Test environment (Docker node:*-slim) doesn't like TLS?
// rpc error on method: get_proof, message: error sending request for url (https://rpc.flashbots.net/): error trying to connect: unexpected EOF (unable to get local issuer certificate)
config.consensusRpc = "http://unstable.mainnet.beacon-api.nimbus.team";
config.executionRpc = "http://cloudflare-eth.com";

const client = helios.HeliosClient.withConfig(config);

await client.waitSynced();

test("chain id", async (t) => {
  const chainId = await client.chainId();
  t.deepEqual(chainId, 1n);
});

test("get block", async (t) => {
  const block = JSON.parse(await client.getBlockByNumber(helios.BlockTag.latest(), false))
  const timestamp = ethers.getBigInt(block.timestamp);
  const now = Math.floor(Date.now() / 1000);
  t.assert(timestamp > now - 300 && timestamp < now + 300);
});

test("vitalik.eth balance", async (t) => {
  const balance = await client.getBalance(vitalikAddress, helios.BlockTag.finalized())
  console.log("balance: " + balance);

  t.assert(typeof balance === "bigint");
  t.assert(balance > 0n);
});

test("vitalik.eth ens resolve", async (t) => {
  const registry = new ethers.Interface(
    await fs.readFile(new URL("./abi/ens_registry.json", import.meta.url), "utf-8"),
  );
  let call = new helios.CallOpts();
  call.setTo("0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e");
  call.setData(ethers.getBytes(registry.encodeFunctionData("resolver", [namehash("vitalik.eth")])));
  const resolverAddress = await client.call(call, helios.BlockTag.latest())
  const resolver = new ethers.Interface(await fs.readFile(new URL("./abi/ens_public_resolver_2.json", import.meta.url), "utf-8"));
  call = new helios.CallOpts();
  call.setTo(resolverAddress.subarray(12, 32));
  call.setData(ethers.getBytes(resolver.encodeFunctionData("addr(bytes32)", [namehash("vitalik.eth")])))
  const outputAddress = await client.call(call, helios.BlockTag.latest())
  t.deepEqual(ethers.hexlify(outputAddress.subarray(12, 32)), ethers.hexlify(vitalikAddress));
});
