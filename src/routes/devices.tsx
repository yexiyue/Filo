import { createFileRoute } from "@tanstack/react-router";
import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { 
  register as registerDevice, 
  discover as discoverDevices, 
  type DeviceInfo 
} from "@/commands";

export const Route = createFileRoute("/devices")({
  component: Devices,
});

function Devices() {
  const [devices, setDevices] = useState<DeviceInfo[]>([]);
  const [isDiscovering, setIsDiscovering] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleRegister = async () => {
    try {
      setError(null);
      await registerDevice();
    } catch (err) {
      console.error("注册设备失败:", err);
      setError("注册设备失败: " + (err as Error).message);
    }
  };

  const handleDiscover = async () => {
    if (isDiscovering) return;
    
    try {
      setError(null);
      setIsDiscovering(true);
      setDevices([]);
      
      await discoverDevices((deviceInfo: DeviceInfo) => {
        setDevices(prev => {
          // 避免重复添加相同设备
          if (prev.some(d => d.device_id === deviceInfo.device_id)) {
            return prev;
          }
          return [...prev, deviceInfo];
        });
      });
    } catch (err) {
      console.error("发现设备失败:", err);
      setError("发现设备失败: " + (err as Error).message);
      setIsDiscovering(false);
    }
  };

  // 清理发现过程
  useEffect(() => {
    return () => {
      setIsDiscovering(false);
    };
  }, []);

  return (
    <div className="container py-6 space-y-6">
      <div>
        <h1 className="text-3xl font-bold">设备管理</h1>
        <p className="text-muted-foreground">注册您的设备并在网络上发现其他设备</p>
      </div>

      <div className="flex gap-4">
        <Button onClick={handleRegister}>注册设备</Button>
        <Button 
          onClick={handleDiscover} 
          disabled={isDiscovering}
          variant={isDiscovering ? "secondary" : "default"}
        >
          {isDiscovering ? "正在发现设备..." : "发现设备"}
        </Button>
      </div>

      {error && (
        <Card className="border-destructive">
          <CardHeader>
            <CardTitle className="text-destructive">错误</CardTitle>
          </CardHeader>
          <CardContent>
            <p>{error}</p>
          </CardContent>
        </Card>
      )}

      {devices.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>发现的设备</CardTitle>
            <CardDescription>在网络中找到 {devices.length} 台设备</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            {devices.map((device, index) => (
              <div key={device.device_id || index}>
                <div className="flex items-center justify-between">
                  <div>
                    <h3 className="font-semibold">{device.hostname || "未知主机"}</h3>
                    <div className="flex flex-wrap gap-2 mt-1">
                      <Badge variant="outline">{device.platform || "未知平台"}</Badge>
                      <Badge variant="outline">{device.os_type || "未知系统类型"}</Badge>
                      <Badge variant="outline">{device.os_arch || "未知架构"}</Badge>
                    </div>
                  </div>
                  <Badge>{device.device_id?.substring(0, 8) || "无ID"}</Badge>
                </div>
                
                {device.os_version && (
                  <p className="text-sm text-muted-foreground mt-1">
                    系统版本: {device.os_version}
                  </p>
                )}
                
                {index < devices.length - 1 && <Separator className="mt-4" />}
              </div>
            ))}
          </CardContent>
        </Card>
      )}

      {isDiscovering && devices.length === 0 && (
        <Card>
          <CardContent className="py-8 text-center">
            <p className="text-muted-foreground">正在搜索网络中的设备...</p>
          </CardContent>
        </Card>
      )}
    </div>
  );
}