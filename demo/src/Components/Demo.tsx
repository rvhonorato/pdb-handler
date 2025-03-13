import { useState, useEffect } from "react";
import { PdbHandlerApi, default as init } from "pdb-handler-wasm";

export const Demo = () => {
  const [wasmApi, setWasmApi] = useState<PdbHandlerApi | undefined>();
  const [loading, setLoading] = useState(true);
  const [fileData, setFileData] = useState<Uint8Array | null>(null);
  const [executed, setExecuted] = useState(false);

  useEffect(() => {
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

    loadWasm();
  }, [fileData]);

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
    window.location.reload();
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

      setExecuted(true);
    }
  };

  return (
    <div className="bg-stone-200 border-indigo-100 border m-auto p-5 rounded">
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
              id="file-input"
              type="file"
              className={`m-5 border border-slate-300 bg-neutral-100 text-neutral-800 px-4 py-2 rounded-md focus:ring-2 focus:ring-blue-300 focus:outline-none ${executed ? "hidden" : ""}`}
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
