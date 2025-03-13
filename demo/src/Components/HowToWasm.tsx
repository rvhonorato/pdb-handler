import SyntaxHighlighter from "react-syntax-highlighter";
import { Subtitle } from "./Subtitle";

export const HowToWasm = () => {
  const initSnippet = `export const PDBProcessingComponent = () = > {
  const [wasmApi, setWasmApi] = useState<PdbHandlerApi | undefined>();
  const [fileData, setFileData] = useState<Uint8Array | null>(null);

  useEffect(() => {
    async function loadWasm() {
      try {
        // initialize the WASM module
        await init();
        // create an instance of the the pdb handler api
        const api = new PdbHandlerApi();
        // set the API in the state
        setWasmApi(api);
      } catch (error) {
        console.error("Failed to initialize WASM:", error);
      }
    }
    loadWasm();
  }, [fileData]);

  const handleProcess = () => {
    if (wasmApi && fileData) {
      const chainResult = wasmApi.list_chains(fileData);
      const unknownresResult = wasmApi.list_unknown_residues(fileData);
      const moltypeResult = wasmApi.guess_moltype(fileData);
      const listresidueResult = wasmApi.list_residues(fileData);
      const chainincontactResult = wasmApi.chains_in_contact(fileData);
    }
  };
`;

  return (
    <>
      <div className="space-y-4 pt-4">
        <Subtitle text="How to use it via WebAssembly?" />
        <p className="leading-relaxed text-gray-700">
          First, initialize the WASM module in your React component. This async
          loading pattern ensures proper resource management and error handling:
        </p>

        <div className="text-left mr-50 ml-50">
          <SyntaxHighlighter language="typescript">
            {initSnippet}
          </SyntaxHighlighter>
        </div>

        <p className="text-center leading-relaxed text-gray-700">
          Check the{" "}
          <a
            href="https://github.com/rvhonorato/pdb-handler/tree/main/demo"
            target="_blank"
            className="text-xl font-semibold text-indigo-500 hover:text-green-500 hover:underline"
          >
            repository of this demo
          </a>{" "}
          for examples!
        </p>
      </div>
    </>
  );
};
