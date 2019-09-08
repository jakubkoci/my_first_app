const path = require("path");
const tape = require("tape");

const {
  Diorama,
  tapeExecutor,
  backwardCompatibilityMiddleware
} = require("@holochain/diorama");

process.on("unhandledRejection", error => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const dnaPath = path.join(__dirname, "../dist/my_first_app.dna.json");
const dna = Diorama.dna(dnaPath, "my_first_app");

const diorama = new Diorama({
  instances: {
    alice: dna,
    bob: dna
  },
  bridges: [],
  debugLog: false,
  executor: tapeExecutor(require("tape")),
  middleware: backwardCompatibilityMiddleware
});

diorama.registerScenario(
  "description of example test",
  async (s, t, { alice }) => {
    // Make a call to a Zome function
    // indicating the function, and passing it an input
    const addr = await alice.call("my_zome", "create_my_entry", {
      entry: { content: "sample content" }
    });
    const result = await alice.call("my_zome", "get_my_entry", {
      address: addr.Ok
    });

    // check for equality of the actual and expected results
    t.deepEqual(result, {
      Ok: { App: ["my_entry", '{"content":"sample content"}'] }
    });
  }
);

diorama.registerScenario("create user, commitment", async (s, t, { alice }) => {
  // Make a call to a Zome function
  // indicating the function, and passing it an input
  const anchor_addr = await alice.call("my_zome", "create_anchor", {});
  console.log("anchor_addr", anchor_addr);
  
  const user_addr = await alice.call("my_zome", "create_user", {
    user: { name: "John Doe" }
  });
  console.log("user_addr", user_addr);

  const result = await alice.call("my_zome", "get_users", {});

  const commitment_addr = await alice.call("my_zome", "create_commitment", {
    commitment: { title: "Attend workshop" },
    user_addr: user_addr.Ok
  });
  console.log("commitment_addr", commitment_addr);
  const commitmentsResult = await alice.call("my_zome", "get_user_commitments", { user_addr: user_addr.Ok });

  // check for equality of the actual and expected results
  t.deepEqual(commitmentsResult, { Ok: { name: "users", items: [{ name: "John Doe" }] } });
});

diorama.run();
