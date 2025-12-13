import { invoke } from '@tauri-apps/api/core';
import { Channel } from '@tauri-apps/api/core';

export interface DeviceInfo {
  // 根据Rust代码中的connection模块推断应该包含设备信息字段
  [key: string]: string;
}

/**
 * 注册设备
 * 对应Rust中的register command
 */
export async function register(): Promise<string> {
  return await invoke('register');
}

/**
 * 发现其他设备
 * 对应Rust中的discover command
 * @param callback 当发现设备时的回调函数
 */
export async function discover(callback: (deviceInfo: DeviceInfo) => void): Promise<void> {
  const channel = new Channel<DeviceInfo>();
  channel.onmessage = callback;
  return await invoke('discover', { onFound: channel });
}