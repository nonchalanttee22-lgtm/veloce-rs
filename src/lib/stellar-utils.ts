import { Contract, networks, Server } from '@stellar/stellar-sdk';

export const VELOCE_CONTRACT_ID = 'YOUR_CONTRACT_ID_HERE';

export const server = new Server('https://soroban-testnet.stellar.org');

export const getContract = () => {
  return new Contract(VELOCE_CONTRACT_ID);
};

// Logic to calculate client-side "drip" visualizer
export const calculateFlow = (velocity: number, startTime: number) => {
  const now = Math.floor(Date.now() / 1000);
  const elapsed = now - startTime;
  return elapsed * velocity;
};
