# Bonsol Input Types
Bonsol has a variety of input types that can be used to pass data to the prover. These input types help developers deeply integrate with the web and solana.

## Public Inputs
Public inputs are inputs that are passed in the execution request. There are four types of public inputs.
* `PublicData` - A byte array that is passed in the execution request.
* `PublicAccountData` - The pubkey of a solana account that is passed in the execution request. The prover will pull this account data from the solana blockchain and use it as a public input.
* `PublicUrl` - A url that the prover will pull data from and use as a public input.
* `PublicProof` - A proof and its output that the prover will use as a public input.

:::info
A note on pulling data from the solana blockchain or urls. Work is in process to make this more secure, currently the best way to ensure the prover is pulling the correct data is to configure the execution request to verify an input hash. This can be limiting if you are working with rapidly changing data. We hope to allow url input types to have a precompiled http verification circuit that can prove the origin of the data.
:::

## Private Inputs
Private inputs are inputs that are passed in the execution request. There is only one type of private input.

* `PrivateUrl` - A url that the prover will pull data from and use as a private input. This is a complicated one and caveats apply. Once a prover node has claimed the execution request, it must sign a request to the private input server to get the private input. The private input server will return the private input to the prover node. The input is no longer globally private so use this in scenarios where its okay if the prover node can see the input. We recommend looking at Proof Composition through the `PublicProof` input type as an alternative to this.
* `PrivateLocal` - Only used when running local proofs.

## Input Sets

Input sets have been removed due to lack of use.

## Public Proof Inputs
Proof inputs are actually urls to a location on the internet that the prover will pull the data from. The data will be a risc0 receipt. They are too big to fit in a single solana transaction so it usually takes many many transactions to post them to solana. This is why we treat them as url inputs. They must be deliniated from public url inputs so that the prover can add them to the risc0 zkvm in a special way. The bonsol cli has a method to allow you to locally prove something and get the output as a json payload. While json is less efficient than a binary format, it is easier to reason about by more developers. The json file will be hundreds of kilobytes in size and will contain the receipt which is the proof(seail) and the output of the execution.

:::info
Public proof inputs will not be added to the input hash of the execution request. This may change in the future, but for now we must use other techniques to ensure the prover properly handled the proof input and sent the correct proof in.
:::
