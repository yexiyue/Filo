import { ComfyEditor } from "@/components/confyeditor";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/")({
  component: Home,
});

function Home() {
  return (
    <div className="border w-full m-4">
      <ComfyEditor />
    </div>
  );
}
