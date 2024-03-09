import type BN from 'bn.js';

export enum LangError {
	couldNotReadInput = 'CouldNotReadInput'
}

export enum Error {
	insufficientBalance = 'InsufficientBalance',
	transferFailed = 'TransferFailed'
}

export type AccountId = string | number[]

