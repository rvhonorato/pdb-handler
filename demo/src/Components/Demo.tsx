import { useState, useEffect, ReactElement } from "react";
import { PdbHandlerApi, default as init } from "pdb-handler-wasm";

export const Demo = () => {
  const [wasmApi, setWasmApi] = useState<PdbHandlerApi | undefined>();
  const [loading, setLoading] = useState(true);
  const [fileData, setFileData] = useState<Uint8Array | null>(null);
  const [executed, setExecuted] = useState(false);
  const [output, setOutput] = useState<ReactElement>();

  async function loadWasm() {
    try {
      await init(); // initialize the WASM module
      const api = new PdbHandlerApi(); // create an instance of the the pdb handler api
      setWasmApi(api); // set the api in state
      setLoading(false);
    } catch (error) {
      console.error("Failed to initialize WASM:", error);
      setLoading(false);
    }
  }
  useEffect(() => {
    loadWasm();
  }, [fileData, executed]);

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setExecuted(false);
    const file = event.target.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (e) => {
        if (e.target?.result instanceof ArrayBuffer) {
          const data = new Uint8Array(e.target.result); // convert to Uint8Array
          setFileData(data);
        }
      };
      reader.readAsArrayBuffer(file);
    }
  };

  const handleReload = () => {
    setFileData(null);
    setExecuted(false);
    loadWasm();
    // window.location.reload();
  };

  const handleProcess = () => {
    if (wasmApi && fileData) {
      console.log("Calling WebAssembly functions...");

      const chainResult = wasmApi.list_chains(fileData);
      const unknownresResult = wasmApi.list_unknown_residues(fileData);
      const moltypeResult = wasmApi.guess_moltype(fileData);
      const listresidueResult = wasmApi.list_residues(fileData);
      const chainincontactResult = wasmApi.chains_in_contact(fileData);

      console.log("list_chains:", chainResult);
      console.log("list_unknown_residues: ", unknownresResult);
      console.log("guess_moltype ", moltypeResult);
      console.log("list_residues: ", listresidueResult);
      console.log("chains_in_contact: ", chainincontactResult);

      console.log("Done!");

      const formattedOutput = formatOutput(
        chainResult,
        unknownresResult,
        moltypeResult,
        listresidueResult,
        chainincontactResult,
      );

      setOutput(formattedOutput);
      setExecuted(true);
    }
  };

  const formatOutput = (
    chainResult: string[],
    unknownresResult: Map<string, string[]>,
    moltypeResult: Map<string, string[]>,
    listresidueResult: Map<string, string[]>,
    chainincontactResult: string[][],
  ) => {
    return (
      <div className="max-w-2xl mx-auto space-y-8 bg-white p-6 rounded-lg shadow-md">
        {/* Chains Section */}
        <div>
          <h3 className="text-lg font-semibold mb-4 text-blue-600 border-b-2 border-blue-100 pb-2">
            Chains
          </h3>
          <ul className="list-disc list-inside space-y-2 text-gray-700">
            {chainResult.map((chain, index) => (
              <li
                key={index}
                className="pl-2 hover:bg-blue-50 rounded px-2 py-1"
              >
                <span className="font-mono bg-blue-100 px-2 rounded text-blue-800">
                  {chain}
                </span>
              </li>
            ))}
          </ul>
        </div>

        {/* Unknown Residues Section */}
        <div>
          <h3 className="text-lg font-semibold mb-4 text-amber-600 border-b-2 border-amber-100 pb-2">
            Unknown Residues
          </h3>
          <ul className="space-y-3">
            {Array.from(unknownresResult.entries()).map(
              ([chain, residues], index) => (
                <li key={index} className="bg-amber-50 p-3 rounded-lg">
                  <span className="font-medium text-amber-800">{chain}:</span>
                  <span className="ml-2 text-amber-700">
                    {residues.join(", ")}
                  </span>
                </li>
              ),
            )}
          </ul>
        </div>

        {/* Molecular Types Section */}
        <div>
          <h3 className="text-lg font-semibold mb-4 text-emerald-600 border-b-2 border-emerald-100 pb-2">
            Molecular Types
          </h3>
          <ul className="grid grid-cols-1 md:grid-cols-2 gap-3">
            {Array.from(moltypeResult.entries()).map(
              ([chain, types], index) => (
                <li key={index} className="bg-emerald-50 p-3 rounded-lg">
                  <span className="font-medium text-emerald-800">{chain}:</span>
                  <span className="ml-2 text-emerald-700">
                    {types.join(", ")}
                  </span>
                </li>
              ),
            )}
          </ul>
        </div>

        {/* Residues Section */}
        <div>
          <h3 className="text-lg font-semibold mb-4 text-purple-600 border-b-2 border-purple-100 pb-2">
            Residues
          </h3>
          <ul className="space-y-3">
            {Array.from(listresidueResult.entries()).map(
              ([chain, residues], index) => (
                <li key={index} className="bg-purple-50 p-3 rounded-lg">
                  <span className="font-medium text-purple-800">{chain}:</span>
                  <span className="ml-2 text-purple-700">
                    {residues.join(", ")}
                  </span>
                </li>
              ),
            )}
          </ul>
        </div>

        {/* Chains in Contact Section */}
        <div>
          <h3 className="text-lg font-semibold mb-4 text-rose-600 border-b-2 border-rose-100 pb-2">
            Chains in Contact
          </h3>
          <ul className="flex flex-wrap gap-2">
            {chainincontactResult.map((contact, index) => (
              <li
                key={index}
                className="bg-rose-100 px-3 py-1 rounded-full text-rose-800 text-sm font-medium"
              >
                {contact.join(" ↔ ")}
              </li>
            ))}
          </ul>
        </div>
      </div>
    );
  };

  return (
    <div className="bg-stone-50 border-indigo-100 border m-auto p-5 rounded">
      {loading ? (
        <p>Loading WebAssembly module...</p>
      ) : wasmApi ? (
        <div className="demo-section">
          <div className="flex flex-col items-center space-y-2">
            <label
              htmlFor="file-input"
              className={`cursor-pointer bg-blue-500 text-white m-5 px-5 py-3 rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-300 ${executed ? "hidden" : ""}`}
            >
              Select File
            </label>
            <input
              className={`text-center m-5 file:border-none file:bg-transparent file:text-transparent file:cursor-pointer border border-slate-300 bg-neutral-100 text-neutral-800 px-4 py-2 rounded-md focus:ring-2 focus:ring-blue-300 focus:outline-none ${
                executed ? "hidden" : ""
              }`}
              id="file-input"
              type="file"
              // className={`m-5 border border-slate-300 bg-neutral-100 text-neutral-800 px-4 py-2 rounded-md focus:ring-2 focus:ring-blue-300 focus:outline-none ${executed ? "hidden" : ""}`}
              onChange={handleFileChange}
            />
          </div>
          <input type="button" onClick={handleProcess} />
          <button
            className={`bg-green-500 hover:bg-white text-white hover:text-green-500 cursor-pointer border hover:border-green-500 font-bold py-2 px-4 rounded ${executed ? "hidden" : ""}`}
            onClick={handleProcess}
          >
            Run
          </button>
          <div>
            {executed ? (
              <>
                {output}
                <br />
                <button
                  onClick={handleReload}
                  className="bg-cyan-500 hover:bg-white text-white hover:text-green-500 cursor-pointer border hover:border-green-500 font-bold p-2 m-4 rounded "
                >
                  Run another!
                </button>
              </>
            ) : (
              <></>
            )}
          </div>
        </div>
      ) : (
        <p className="error">
          Failed to load WebAssembly module, check console for details
        </p>
      )}
    </div>
  );
};
