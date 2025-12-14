import { invoke } from "@tauri-apps/api/core";
import { Channel } from "@tauri-apps/api/core";

export interface DeviceInfo {
  // 根据Rust代码中的connection模块推断应该包含设备信息字段
  [key: string]: string;
}

/**
 * 注册设备
 * 对应Rust中的register command
 */
export async function register(): Promise<string> {
  return await invoke("start");
}
