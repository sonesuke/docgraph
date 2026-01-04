import * as vscode from "vscode";
import { docgraphPlugin } from "./markdown-it-docgraph";

export function activate(_context: vscode.ExtensionContext) {
	return {
		extendMarkdownIt(md: any) {
			const config = vscode.workspace.getConfiguration("docgraph");
			if (config.get<boolean>("preview.enable", true)) {
				return md.use(docgraphPlugin);
			}
			return md;
		},
	};
}
