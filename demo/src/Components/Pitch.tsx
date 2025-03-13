import { Name } from "./Name";
import { Subtitle } from "./Subtitle";

export const Pitch = () => {
  return (
    <div className="space-y-4 pt-4">
      <Subtitle text="Run anywhere, even in the browser!" />
      <p className="leading-relaxed text-gray-700">
        Compiled to WebAssembly, <Name /> unlocks:
      </p>

      <ul className="space-y-3 bg-indigo-50 rounded-lg p-4">
        <li className="">
          <span className="text-indigo-600 mr-2">✓</span>
          <span className="flex-1">
            Client-side PDB processing without server backend
          </span>
        </li>
        <li className="">
          <span className="text-indigo-600 mr-2">✓</span>
          <span className="flex-1">
            Real-time visualization and calculations
          </span>
        </li>
        <li className="">
          <span className="text-indigo-600 mr-2">✓</span>
          <span className="flex-1">
            Cross-platform compatibility with JavaScript/TypeScript
          </span>
        </li>
      </ul>
    </div>
  );
};
