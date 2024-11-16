import chai from 'chai';
const { expect } = chai;
import { Noir } from '@noir-lang/noir_js';
import { BarretenbergBackend } from '@noir-lang/backend_barretenberg';
import { BackendInstances, Circuits, Noirs } from '../types.js';
import hre from 'hardhat';
const { viem } = hre;
import { compile, createFileManager } from '@noir-lang/noir_wasm';
import { join, resolve } from 'path';
import { ProofData } from '@noir-lang/types';
import { bytesToHex } from 'viem';

async function getCircuit(name: string) {
  const basePath = resolve(join('../noir', name));
  const fm = createFileManager(basePath);
  const compiled = await compile(fm, basePath);
  if (!('program' in compiled)) {
    throw new Error('Compilation failed');
  }
  return compiled.program;
}

describe('It compiles noir program code, receiving circuit bytes and abi object.', () => {
  let circuits: Circuits;
  let backends: BackendInstances;
  let noirs: Noirs;

  const mainInput = {
    intermediate_root :"0x12be21783d2eceb6968e86d597ff5d8939cd7955816500ea128793a35e1ff220",
    leaf_index1 : "0x2",
    new_leaf1 : "0x2a28adb2b2752c6f2561c56004a2a5665382ff6e39d980a16331002b539cc9a2",
    new_path1 : ["0x04fd3da9756f25c72ca8990437b7f7b58e7ca48bfc21e65e7978320db8b1e5c5", "0x046394ae1ebbf494f2cd2c2d37171099510d099489c9accef59f90512d5f0477", "0x26feab79f46e178b393804a1f69909eab52d2db77b53e01af496d277ad724780", "0x2c58b6be5ce4dff5c323326a011a8d110498ee6e3ba0c625484af2777044e5ea"],
    };

  before(async () => {
    circuits = {
      main: await getCircuit('main'),
      recursive: await getCircuit('recursion'),
    };
    backends = {
      main: new BarretenbergBackend(circuits.main, { threads: 8 }),
      recursive: new BarretenbergBackend(circuits.recursive, { threads: 8 }),
    };
    noirs = {
      main: new Noir(circuits.main, backends.main),
      recursive: new Noir(circuits.recursive, backends.recursive),
    };
  });

  after(async () => {
    await backends.main.destroy();
    await backends.recursive.destroy();
  });

  describe('Recursive flow', async () => {
    let recursiveInputs: any;
    let intermediateProof: ProofData;
    let finalProof: ProofData;

    describe.only('Proof generation', async () => {
      it('Should generate an intermediate proof', async () => {
        const { witness } = await noirs.main.execute(mainInput);
        intermediateProof = await backends.main.generateProof(witness);
    
        const { proof, publicInputs } = intermediateProof;
        expect(proof instanceof Uint8Array).to.be.true;
    
        const verified = await backends.main.verifyProof({ proof, publicInputs });
        expect(verified).to.be.true;
    
        const numPublicInputs = 1;
        const { proofAsFields, vkAsFields, vkHash } =
          await backends.main.generateRecursiveProofArtifacts(
            { publicInputs, proof },
            numPublicInputs,
          );
    
        // Log the generated proof artifacts for debugging purposes
        console.log("Generated Verification Key:", JSON.stringify(vkAsFields, null, 2));
        console.log("Generated Proof:", JSON.stringify(proofAsFields, null, 2));
        console.log("Generated Key Hash:", vkHash);
    
        expect(vkAsFields).to.be.of.length(114);
        expect(vkHash).to.be.a('string');
    
        recursiveInputs = {
          verification_key: vkAsFields,
          proof: proofAsFields,
          public_inputs: [mainInput.intermediate_root], // Check if this should match your inputs correctly
          key_hash: vkHash,
        };
        
        // Log the recursive inputs before proceeding to generate the final proof
        console.log("Recursive Inputs:", JSON.stringify(recursiveInputs, null, 2));
      });
    
      it('Should generate a final proof with a recursive input', async () => {
        finalProof = await noirs.recursive.generateProof(recursiveInputs);
        expect(finalProof.proof instanceof Uint8Array).to.be.true;
      });
    });
    
    describe('Proof verification', async () => {
      let verifierContract: any;

      before(async () => {
        verifierContract = await viem.deployContract('UltraVerifier');
      });

      it('Should verify off-chain', async () => {
        const verified = await noirs.recursive.verifyProof(finalProof);
        expect(verified).to.be.true;
      });

      it('Should verify on-chain', async () => {
        const verified = await verifierContract.read.verify(
          bytesToHex(finalProof.proof),
          finalProof.publicInputs,
        );
        expect(verified).to.be.true;
      });
    });
  });
});
