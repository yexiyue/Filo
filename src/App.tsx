import { useState } from "react";
import reactLogo from "./assets/react.svg";
import "./App.css";
import { register, discover, DeviceInfo } from "./commands";

function App() {
  const [deviceList, setDeviceList] = useState<DeviceInfo[]>([]);
  const [isDiscovering, setIsDiscovering] = useState(false);

  async function handleRegister() {
    try {
      await register();
      alert("设备注册成功！");
    } catch (error) {
      console.error("注册失败:", error);
      alert("设备注册失败");
    }
  }

  async function handleDiscover() {
    if (isDiscovering) return;
    
    setIsDiscovering(true);
    setDeviceList([]);
    
    try {
      await discover((deviceInfo) => {
        setDeviceList(prev => [...prev, deviceInfo]);
      });
    } catch (error) {
      console.error("发现设备过程中出错:", error);
      setIsDiscovering(false);
    }
  }

  return (
    <main className="container">
      <h1>Filo 设备发现演示</h1>

      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      
      <p>点击下方按钮注册设备或发现网络中的其他设备。</p>

      <div className="row">
        <button onClick={handleRegister}>注册设备</button>
        <button onClick={handleDiscover} disabled={isDiscovering}>
          {isDiscovering ? "发现中..." : "发现设备"}
        </button>
      </div>

      {deviceList.length > 0 && (
        <div>
          <h3>发现的设备:</h3>
          <ul>
            {deviceList.map((device, index) => (
              <li key={index}>
                <strong>主机名:</strong> {device.hostname}<br/>
                <strong>平台:</strong> {device.platform}<br/>
                <strong>系统类型:</strong> {device.os_type}<br/>
                <strong>系统版本:</strong> {device.os_version}<br/>
                <strong>架构:</strong> {device.os_arch}<br/>
                <strong>设备ID:</strong> {device.device_id}
              </li>
            ))}
          </ul>
        </div>
      )}
    </main>
  );
}

export default App;
