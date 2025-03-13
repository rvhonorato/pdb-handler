import { useState, useEffect } from "react";
import "./App.css";
import "./index.css";
import { PdbHandlerApi, default as init } from "pdb-handler-wasm";

function App() {
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
    <Container>
      <Header />
      <hr />
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
                <Result />
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
    </Container>
  );
}

type ContainerProps = {
  children: React.ReactNode;
};

const Container = ({ children }: ContainerProps) => {
  return (
    <div className="container p-10 m-auto border border-slate-200 bg-stone-50">
      {children}
    </div>
  );
};

const Result = () => {
  return (
    <>
      🎉 Check the <code>console</code> to see the output of the functions! 🎉
    </>
  );
};

const Header = () => {
  return (
    <div className="text-left">
      <h1 className="text-center text-3xl text-bold m-4">
        pdb-handler WebAssembly Demo
      </h1>
      <p className="p-2">
        pdb-handler is a Rust library that facilitates the handling of
        structural data in PDB format. This library provides robust tools for
        parsing, manipulating, and analyzing protein structure files in the
        standard Protein Data Bank (PDB) format.
      </p>
      <p className="text-center text-bold text-xl">Overview</p>
      <p className="p-2">
        pdb-handler was developed to support scientific research software that
        works with protein structural data. It offers a comprehensive set of
        functions for working with PDB files while maintaining high performance
        and memory efficiency.
      </p>

      <p className="text-center text-bold text-xl">WebAssembly Support</p>
      <p className="p-2">
        This page demonstrates how pdb-handler can be run directly in the
        browser via WebAssembly (WASM). By compiling the Rust code to WASM,
        researchers and developers can leverage the library's capabilities in
        web applications without sacrificing performance.
      </p>
      <p className="p-2">The WASM implementation allows for: </p>
      <ul className="text-gray-700">
        <li>- Client-side processing of PDB files </li>
        <li>- Real-time analysis without server dependencies</li>
        <li>- Cross-platform compatibility</li>
      </ul>
      <p className="p-2">
        The examples below show how to integrate pdb-handler into your web
        projects and utilize its core functionality within a browser
        environment. Would you like me to add more specific details about the
        library's features or the WASM implementation?{" "}
      </p>
    </div>
  );
};

export default App;
