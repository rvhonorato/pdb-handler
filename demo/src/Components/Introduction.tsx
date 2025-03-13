export const Introduction = () => {
  return (
    <div className="space-y-4">
      <p className="text-lg leading-relaxed">
        <span className="font-semibold text-indigo-600">pdb-handler</span>{" "}
        enables developers and researchers to efficiently parse, analyze, and
        manipulate Protein Data Bank (PDB) files. It is built on top of the
        amazing{" "}
        <a
          href="https://github.com/douweschulte/pdbtbx"
          target="_blank"
          className="font-semibold text-indigo-500 hover:text-green-500 hover:underline"
        >
          pdbtbx
        </a>{" "}
        crate. Being a Rust library, it's also possible to compile it to
        WebAssembly, allowing the handling of PDB files directly in the browser,
        without the need of dedicated server.
      </p>
      <p className="text-lg leading-relaxed">
        This library aims to streamline computational structural biology
        workflows with:
      </p>

      <ul className="space-y-3 pl-5 border-r-4 border-l-4 border-indigo-100">
        <li className="">
          <span className="flex-1">
            <b>Zero-copy parsing</b> for memory-efficient handling of large PDB
            files
          </span>
        </li>
        <li className="">
          <b>Type-safe representations</b> of atoms, residues, and molecular
          structures
        </li>
        <li className="">
          <b>Chain/domain operations</b> including selection, filtering, and
          geometric calculations
        </li>
        <li className="">
          <b>WebAssembly support</b> for browser-based structural analysis
        </li>
      </ul>
    </div>
  );
};
