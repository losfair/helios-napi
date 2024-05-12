import test from 'ava'
import * as helios from "../index.js"

const config = new helios.HeliosClientConfig();
config.checkpoint = "0xf38911c84ca0d777fba2732fdc9548e0dd33fa48e67f45378d30290c69521ad4";
const client = helios.HeliosClient.withConfig(config);

await client.waitSynced();

test("vitalik.eth balance", async (t) => {
  const balance = await client.getBalance("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045", helios.BlockTag.finalized())
  console.log("balance: " + balance);

  t.assert(typeof balance === "bigint");
  t.assert(balance > 0n);
});
