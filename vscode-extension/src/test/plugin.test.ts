import * as assert from "node:assert";
import MarkdownIt from "markdown-it";
import { beforeEach, describe, it } from "vitest";
import { docgraphPlugin } from "../markdown-it-docgraph";

describe("markdown-it-docgraph plugin", () => {
	let md: MarkdownIt;

	beforeEach(() => {
		md = new MarkdownIt();
		md.use(docgraphPlugin);
	});

	it("should render {document} transparently", () => {
		const input = [
			"```{document} title",
			":kind: req",
			":id: 1",
			"",
			"- list item",
			"```",
		].join("\n");

		const result = md.render(input);

		assert.ok(result.includes("<ul>"), "Should contain ul");
		assert.ok(result.includes("<li>list item</li>"), "Should contain li");
		assert.ok(!result.includes("<pre>"), "Should not contain pre/code block");
		assert.ok(
			!result.includes(":kind: req"),
			"Should not contain option lines",
		);
	});

	it("should render :class: docgraph transparently", () => {
		const input = [
			"```{admonition} title",
			":class: docgraph",
			":id: 2",
			"",
			"paragraph text",
			"```",
		].join("\n");

		const result = md.render(input);

		assert.ok(
			result.includes("<p>paragraph text</p>"),
			"Should render paragraph",
		);
		assert.ok(!result.includes("<pre>"), "Should not contain pre/code block");
	});

	it("should NOT affect normal code blocks", () => {
		const input = ["```python", 'print("hello")', "```"].join("\n");

		const result = md.render(input);
		assert.ok(
			result.includes('<pre><code class="language-python">'),
			"Should render normal code block",
		);
		assert.ok(
			result.includes('print("hello")') ||
				result.includes("print(&quot;hello&quot;)"),
			"Should contain code content",
		);
	});

	it("should handle nested markdown", () => {
		const input = [
			"```{document}",
			":kind: r",
			"",
			"# Heading",
			"",
			"| h1 | h2 |",
			"| -- | -- |",
			"| c1 | c2 |",
			"```",
		].join("\n");

		const result = md.render(input);
		assert.ok(result.includes("<h1>Heading</h1>"), "Should render heading");
		assert.ok(result.includes("<table>"), "Should render table");
	});
});
