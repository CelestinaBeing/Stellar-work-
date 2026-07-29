const { rpc } = require("@stellar/stellar-sdk");
async function run() {
  const server = new rpc.Server("https://soroban-testnet.stellar.org");
  const account = await server.getAccount("GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF");
  console.log(account);
}
run().catch(console.error);
