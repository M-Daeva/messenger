import { getData, SigningCosmWasmClient, coin } from "./signer";
const { ADDR, CONTR, getAliceClient } = getData(true);

const l = console.log.bind(console);

async function main() {
  const aliceClient = (await getAliceClient(true)) as SigningCosmWasmClient;

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
    body: string,
    tag: Tag,
    rarity: Rarity,
    amount: number
  ) {
    const msg = {
      create_message: { tag, body, rarity },
    };

    const memo = "";

    const funds = [coin(amount, "ujunox")];

    return await aliceClient.execute(
      ADDR.ALICE,
      CONTR.ADDR,
      msg,
      gas,
      memo,
      funds
    );
  }

  //let res = await create_msg("BUIDL!!!", Tag.Juno, Rarity.Epic, 1_000_000);
  //l("\n", res, "\n");

  let msg = {
    create_message: { tag: Tag.Juno, body: "BUIDL!!!", rarity: Rarity.Epic },
  };
  l(msg);

  // let res = await aliceClient.queryContractSmart(CONTR.ADDR, {
  //   get_count: {},
  // });
  // l("\n", res, "\n");
}

main();
