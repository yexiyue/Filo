import { Plate, usePlateEditor } from "platejs/react";

import { Editor, EditorContainer } from "@/components/ui/editor";
import { BasicBlocksKit } from "../editor/plugins/basic-blocks-kit";
import { MarkdownKit } from "../editor/plugins/markdown-kit";
import { AutoformatKit } from "../editor/plugins/autoformat-kit";
import { CodeBlockKit } from "../editor/plugins/code-block-kit";
import { BasicMarksKit } from "../editor/plugins/basic-marks-kit";
import { CalloutKit } from "../editor/plugins/callout-kit";
import { SlashKit } from "../editor/plugins/slash-kit";

export function ComfyEditor() {
  const editor = usePlateEditor({
    plugins: [
      ...BasicBlocksKit,
      ...MarkdownKit,
      ...AutoformatKit,
      ...CodeBlockKit,
      ...BasicMarksKit,
      ...CalloutKit,
      ...SlashKit,
    ],
  }); // Initializes the editor instance

  return (
    <Plate editor={editor}>
      {/* Provides editor context */}
      <EditorContainer className="w-full h-full overflow-auto">
        {/* Styles the editor area */}
        <Editor
          variant="fullWidth"
          className="px-8!"
          placeholder="Type your amazing content here..."
        />
      </EditorContainer>
    </Plate>
  );
}
