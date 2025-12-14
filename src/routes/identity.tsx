import { createFileRoute } from "@tanstack/react-router";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { useSecretStore } from "@/hooks/use-secret-store";

export const Route = createFileRoute("/identity")({
  component: Identity,
});

function Identity() {
  const { keypair, deviceId, _hasHydrated } = useSecretStore();

  if (!_hasHydrated) {
    return (
      <div className="container py-6">
        <p className="text-muted-foreground">正在初始化...</p>
      </div>
    );
  }

  return (
    <div className="container py-6 space-y-6">
      <div>
        <h1 className="text-3xl font-bold">身份管理</h1>
        <p className="text-muted-foreground">设备密钥对已自动生成并存储</p>
      </div>

      {keypair && (
        <Card>
          <CardHeader>
            <CardTitle>密钥对</CardTitle>
            <CardDescription>
              Ed25519 密钥对 ({keypair.length} bytes)
            </CardDescription>
          </CardHeader>
          <CardContent>
            <code className="text-xs break-all">
              {keypair.map((b) => b.toString(16).padStart(2, "0")).join("")}
            </code>
          </CardContent>
        </Card>
      )}

      {deviceId && (
        <Card>
          <CardHeader>
            <CardTitle>设备 ID</CardTitle>
            <CardDescription>从公钥派生的设备标识</CardDescription>
          </CardHeader>
          <CardContent>
            <code className="text-sm break-all">{deviceId}</code>
          </CardContent>
        </Card>
      )}

      <Button
        variant="destructive"
        onClick={() => useSecretStore.persist.clearStorage()}
      >
        清除存储
      </Button>
    </div>
  );
}
