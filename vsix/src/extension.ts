import { commands, type ExtensionContext, window, workspace } from "vscode";
import {
  LanguageClient,
  type LanguageClientOptions,
  type ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  const restartCommand = commands.registerCommand(
    "docgraph.restartServer",
    async () => {
      if (client) {
        await client.stop();
        await client.start();
      } else {
        startServer(context);
      }
    },
  );

  context.subscriptions.push(restartCommand);

  startServer(context);
}

function startServer(_context: ExtensionContext) {
  const config = workspace.getConfiguration("docgraph");
  const binaryPath = config.get<string>("binaryPath") || "docgraph";

  const serverOptions: ServerOptions = {
    run: { command: binaryPath, args: ["lsp"] },
    debug: { command: binaryPath, args: ["lsp"] },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "markdown" }],
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher("**/*.md"),
    },
  };

  client = new LanguageClient(
    "docgraph",
    "Docgraph Language Server",
    serverOptions,
    clientOptions,
  );

  client.start().catch((err) => {
    window.showErrorMessage(
      `Failed to start Docgraph language server: ${err.message}`,
    );
  });
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
