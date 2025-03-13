import SyntaxHighlighter from "react-syntax-highlighter";
import { Subtitle } from "./Subtitle";
import { Name } from "./Name";

export const HowToRust = () => {
  const snippet = `use pdbtbx::PDB;
use pdb_handler::chains_in_contact;
let (mut pdb, _errors) = pdbtbx::open("2oob.pdb").unwrap();
let contacting_chains = chains_in_contact(&pdb);
for (chain_a, chain_b) in contacting_chains {
  println!("Chains {} and {} are in contact", chain_a, chain_b);
}
`;
  return (
    <>
      <Subtitle text="How to use it in Rust 🦀?" />
      <p className="leading-relaxed text-gray-700">
        In the simple example below, <Name /> can go check which chains in a
        given protein structure are in contact with each other:
      </p>
      <div className="text-left mr-50 ml-50">
        <SyntaxHighlighter language="rust">{snippet}</SyntaxHighlighter>
      </div>
      <p className="leading-relaxed text-gray-700">
        Check{" "}
        <a
          href="https://docs.rs/pdb-handler"
          target="_blank"
          className="text-xl font-semibold text-indigo-500 hover:text-green-500 hover:underline"
        >
          docs.rs/pdb-handler
        </a>{" "}
        for examples!
      </p>
    </>
  );
};
