import type BN from 'bn.js';
import type {ReturnNumber} from '@727-ventures/typechain-types';

export enum LangError {
	couldNotReadInput = 'CouldNotReadInput'
}

export enum Error {
	insufficientBalance = 'InsufficientBalance',
	transferFailed = 'TransferFailed'
}

export type AccountId = string | number[]

