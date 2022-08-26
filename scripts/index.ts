import { getData, SigningCosmWasmClient, coin, Coin } from "./signer";
const { ADDR, CONTR, getAliceClient } = getData(true);

const l = console.log.bind(console);

const gas = {
  amount: [{ denom: "ujunox", amount: "625" }],
  gas: "250000",
};

enum Tag {
  Atom = "Atom",
  Osmo = "Osmo",
  Juno = "Juno",
}

enum Rarity {
  Common = "Common",
  Rare = "Rare",
  Epic = "Epic",
}

async function create_msg(
  client: SigningCosmWasmClient,
  body: string,
  tag: Tag,
  rarity: Rarity,
  amount: number = 0
) {
  const msg = {
    create_message: { tag, body, rarity },
  };

  const memo = "";

  const funds: Coin[] = amount !== 0 ? [coin(amount, "ujunox")] : [];

  return await client.execute(ADDR.ALICE, CONTR.ADDR, msg, gas, memo, funds);
}

async function get_all_msgs(client: SigningCosmWasmClient) {
  const msg = {
    get_messages: {},
  };

  return await client.queryContractSmart(CONTR.ADDR, msg);
}

async function main() {
  const aliceClient = (await getAliceClient(true)) as SigningCosmWasmClient;

  let res = await get_all_msgs(aliceClient);
  l("\n", res, "\n");

  res = await create_msg(aliceClient, "BUIDL!!!", Tag.Juno, Rarity.Common);
  l("\n", res, "\n");

  res = await get_all_msgs(aliceClient);
  l("\n", res, "\n");
}

main();
