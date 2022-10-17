/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';

/**
 * @category Instructions
 * @category CancelRedeem
 * @category generated
 */
export type CancelRedeemInstructionArgs = {
  root: number[] /* size: 32 */;
};
/**
 * @category Instructions
 * @category CancelRedeem
 * @category generated
 */
export const cancelRedeemStruct = new beet.BeetArgsStruct<
  CancelRedeemInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */;
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['root', beet.uniformFixedSizeArray(beet.u8, 32)],
  ],
  'CancelRedeemInstructionArgs',
);
/**
 * Accounts required by the _cancelRedeem_ instruction
 *
 * @property [] treeAuthority
 * @property [_writable_, **signer**] leafOwner
 * @property [_writable_] merkleTree
 * @property [_writable_] voucher
 * @property [] logWrapper
 * @property [] compressionProgram
 * @category Instructions
 * @category CancelRedeem
 * @category generated
 */
export type CancelRedeemInstructionAccounts = {
  treeAuthority: web3.PublicKey;
  leafOwner: web3.PublicKey;
  merkleTree: web3.PublicKey;
  voucher: web3.PublicKey;
  logWrapper: web3.PublicKey;
  compressionProgram: web3.PublicKey;
};

export const cancelRedeemInstructionDiscriminator = [111, 76, 232, 50, 39, 175, 48, 242];

/**
 * Creates a _CancelRedeem_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category CancelRedeem
 * @category generated
 */
export function createCancelRedeemInstruction(
  accounts: CancelRedeemInstructionAccounts,
  args: CancelRedeemInstructionArgs,
  programId = new web3.PublicKey('BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY'),
) {
  const [data] = cancelRedeemStruct.serialize({
    instructionDiscriminator: cancelRedeemInstructionDiscriminator,
    ...args,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.treeAuthority,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.leafOwner,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.merkleTree,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.voucher,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.logWrapper,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.compressionProgram,
      isWritable: false,
      isSigner: false,
    },
  ];

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  });
  return ix;
}
