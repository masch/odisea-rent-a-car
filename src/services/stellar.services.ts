import { AccountBalance } from "../interfaces/account";
import { IAccountBalanceResponse } from "../interfaces/balance";
import { IKeypair } from "../interfaces/keypair";
import {
  STELLAR_NETWORK,
  HORIZON_URL,
  STELLAR_FRIENDBOT_URL,
} from "../utils/constants";
import { Horizon, Keypair } from "@stellar/stellar-sdk";

export class StellarService {
  private network: string;
  private horizonUrl: string;
  private server: Horizon.Server;
  private friendBotUrl: string;

  constructor() {
    this.network = STELLAR_NETWORK as string;
    this.horizonUrl = HORIZON_URL as string;
    this.friendBotUrl = STELLAR_FRIENDBOT_URL as string;

    this.server = new Horizon.Server(this.horizonUrl, { allowHttp: true });
  }

  createAccount(): IKeypair {
    const pair = Keypair.random();

    return {
      publicKey: pair.publicKey(),
      secretKey: pair.secret(),
    };
  }

  async fundAccount(publicKey: string): Promise<boolean> {
    try {
      if (this.network !== "testnet") {
        throw new Error("Friendbot is only available on testnet");
      }

      const response = await fetch(`${this.friendBotUrl}?addr=${publicKey}`);

      if (!response.ok) {
        return false;
      }

      return true;
    } catch (error: unknown) {
      throw new Error(
        `Error when funding account with Friendbot: ${error as string}`,
      );
    }
  }

  private async getAccount(address: string): Promise<Horizon.AccountResponse> {
    try {
      return await this.server.loadAccount(address);
    } catch (error) {
      console.error(error);
      throw new Error("Account not found");
    }
  }

  async getAccountBalance(publicKey: string): Promise<AccountBalance[]> {
    const account = await this.getAccount(publicKey);

    return account.balances.map((b) => ({
      assetCode:
        b.asset_type === "native"
          ? "XLM"
          : (b as IAccountBalanceResponse).asset_code,

      amount: b.balance,
    }));
  }
}

export const stellarService = new StellarService();
