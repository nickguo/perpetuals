import * as sha3 from "sha3";
import { randomBytes } from "crypto";
import * as secp256k1 from "secp256k1";

export function generateSecp256k1Keypair() {
    // Private key for secp256k1 is just random bytes. Public key is derived afterwards.
    let privateKey;
    do {
      privateKey = randomBytes(32)
    } while (!secp256k1.privateKeyVerify(privateKey));
    // Get the public key in uncompressed format.
    const publicKey = secp256k1.publicKeyCreate(privateKey, false);
    // The library always returns a 65 byte public key that starts with 0x04, so omit the first byte.
    return [privateKey, publicKey.slice(1)];
}

export function secp256k1Sign(privateKey, message) {
    let hash = new sha3.Keccak(256).update(message).digest();
    let sigObj = secp256k1.ecdsaSign(hash, privateKey);

    return {signature: sigObj.signature, recoveryId: sigObj.recid};
}
