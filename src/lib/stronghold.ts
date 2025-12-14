import { StateStorage } from "zustand/middleware";
import { Client, Stronghold } from "@tauri-apps/plugin-stronghold";
import { appDataDir } from "@tauri-apps/api/path";

class StrongholdStorage implements StateStorage {
  private store: Awaited<ReturnType<Client["getStore"]>>;
  private stronghold: Stronghold;

  private constructor(
    store: Awaited<ReturnType<Client["getStore"]>>,
    stronghold: Stronghold
  ) {
    this.store = store;
    this.stronghold = stronghold;
  }

  static async create(password: string, clientName = "filo") {
    const vaultPath = `${await appDataDir()}/vault.hold`;
    const stronghold = await Stronghold.load(vaultPath, password);
    let client: Client;
    try {
      client = await stronghold.loadClient(clientName);
    } catch {
      client = await stronghold.createClient(clientName);
    }
    return new StrongholdStorage(client.getStore(), stronghold);
  }

  getItem = async (name: string) => {
    const data = await this.store.get(name);
    if (!data) return null;
    return new TextDecoder().decode(new Uint8Array(data));
  };

  setItem = async (name: string, value: string) => {
    const data = Array.from(new TextEncoder().encode(value));
    await this.store.insert(name, data);
    await this.stronghold.save();
  };

  removeItem = async (name: string) => {
    await this.store.remove(name);
    await this.stronghold.save();
  };
}

export const strongholdStorage = await StrongholdStorage.create(
  "filo-stronghold"
);
