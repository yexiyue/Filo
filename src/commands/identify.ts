import { invoke } from "@tauri-apps/api/core";

export async function generateKeypair(): Promise<number[]> {
  return await invoke("generate_keypair");
}

export async function registerKeypair(keypair: number[]): Promise<string> {
  return await invoke("register_keypair", { keypair });
}
