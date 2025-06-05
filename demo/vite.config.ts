import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

// https://vite.dev/config/
export default defineConfig({
  base: "/pdb-handler/",
  plugins: [react(), tailwindcss()],
  server: {},
  build: {
    outDir: "../dist",
    emptyOutDir: true,
  },
  optimizeDeps: {
    exclude: ["pdb-handler-wasm"],
  },
});
