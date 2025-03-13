import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

// https://vite.dev/config/
export default defineConfig({
  base: "/pdb-handler/",
  plugins: [react(), tailwindcss()],
  server: {
    fs: {
      // Specify `/pkg` here so we can load the types
      allow: ["..", "../../pkg"],
    },
  },
  build: {
    outDir: "../dist",
    emptyOutDir: true,
  },
});
