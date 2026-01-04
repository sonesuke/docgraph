import type MarkdownIt from "markdown-it";
import type Renderer from "markdown-it/lib/renderer";
import type Token from "markdown-it/lib/token";

export function docgraphPlugin(md: MarkdownIt) {
	const originalFence = md.renderer.rules.fence;

	md.renderer.rules.fence = (
		tokens: Token[],
		idx: number,
		options: MarkdownIt.Options,
		env: any,
		self: Renderer,
	) => {
		const token = tokens[idx];
		const info = token.info ? token.info.trim() : "";

		// Check if enabled (default true)
		// In VS Code context, we might not have direct access to config here easily without passing it down,
		// but typically markdown-it plugins run in webview where vscode API is limited.
		// However, the `extendMarkdownIt` in extension.ts runs in the extension host.
		// We can check the environment validation or just implemented the logic.
		// The requirement says:
		// Docgraph node criteria (preview side):
		// 1. Directive name is "document".
		// 2. Any directive, if the :class: option contains "docgraph".

		const isDocumentDirective = info.startsWith("{document}");
		// We need to parse options to check for :class: docgraph if it's not {document}
		// Simplified check for :class: docgraph in the content or info string if possible?
		// options are usually inside the block content for directives.

		let isDocgraph = isDocumentDirective;

		// If not explicitly {document}, check for :class: docgraph in the content's option lines
		// But we shouldn't parse the whole content if we don't need to.
		// The standard usually puts options at the top.
		if (!isDocgraph) {
			const lines = token.content.split("\n");
			for (const line of lines) {
				const trimmed = line.trim();
				if (trimmed === "") {
					continue;
				} // Skip empty lines? directives usually have options immediately.
				if (!trimmed.startsWith(":")) {
					break;
				} // End of options
				if (trimmed.startsWith(":class:") && trimmed.includes("docgraph")) {
					isDocgraph = true;
					break;
				}
			}
		}

		if (isDocgraph) {
			// Guard against infinite recursion if we re-render
			if (env?.docgraphTransparent) {
				// If we are already transparent, we shouldn't be here ideally,
				// but if we are, just render as empty or raw to avoid loop?
				// Actually, if we re-render the *content*, it shouldn't contain the fence itself again
				// unless the content has nested fences.
				// The re-rendering is: md.render(newContent)
				// So if newContent has fences, they will be processed.
			}

			// Remove option lines from content
			const lines = token.content.split("\n");
			const newLines = [];
			let inOptions = true;

			for (const line of lines) {
				if (inOptions) {
					const trimmed = line.trim();
					if (trimmed === "") {
						// Empty line might mean end of options or just empty line between options
						// Usually options are contiguous at the top.
						// Let's assume options are contiguous lines starting with ':'
						// But wait, "input: ... - a -b".
						// The user example had:
						// :kind: requirement
						// :id: RQ-1
						// <empty line>
						// - a
						if (newLines.length > 0) {
							// If we have pushed content? No.
							// options must be at the start.
						}
						// If we hit an empty line, and we are looking for options...
						// The spec says: "Remove option lines (key-value pairs) from the beginning of token.content"
						newLines.push(line);
						if (trimmed === "") {
							inOptions = false;
						} // Maybe?
						// Let's stick to: remove lines starting with `: \w +: ` at the beginning.
					} else if (trimmed.startsWith(":")) {
						// It's an option line, skip it
					} else {
						// Not an option line, so options are done
						inOptions = false;
						newLines.push(line);
					}
				} else {
					newLines.push(line);
				}
			}

			// Join back
			const cleanContent = newLines.join("\n");

			// Render the inner content as normal markdown
			// accessing md instance to render.
			// We need to set a flag to avoid potential issues?
			// Actually recursion happens if we render *the same fence*.
			// We are rendering the *inner content* which does NOT have the {document} fence.
			// So infinite recursion is unlikely unless the user nested {document} inside {document}
			// which is possible.
			// If nested, it will just call this plugin again, which is correct behavior.

			return md.render(cleanContent, env);
		}

		// Fallback to default renderer
		return originalFence
			? originalFence(tokens, idx, options, env, self)
			: self.renderToken(tokens, idx, options);
	};
}
