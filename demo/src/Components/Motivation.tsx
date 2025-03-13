import { Subtitle } from "./Subtitle";
import { Name } from "./Name";

export const Motivation = () => {
  return (
    <div className="space-y-4 pt-4">
      <Subtitle text="Why?" />
      <p className="leading-relaxed text-gray-700">
        The main goal of <Name /> is to provide a modern and fast library for
        processing PDB files, moving beyond solutions typically implemented in
        dynamic languages or legacy C/C++ code. This library delivers
        exceptional performance with a clean, type-safe API while ensuring
        cross-platform compatibility—making it ideal for both traditional
        applications and web-based tools through WebAssembly.
      </p>
    </div>
  );
};
