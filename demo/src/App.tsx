import { useState } from "react";
import "./App.css";
import "./index.css";
import { Container } from "./Components/Container";
import { HowToRust } from "./Components/HowToRust";
import { HowToWasm } from "./Components/HowToWasm";
import { Introduction } from "./Components/Introduction";
import { Links } from "./Components/Links";
import { Motivation } from "./Components/Motivation";
import { Pitch } from "./Components/Pitch";
import { Title } from "./Components/Title";
import { Demo } from "./Components/Demo";
import { ProjectStatus } from "./Components/ProjectStatus";
import { Footer } from "./Components/Footer";

export const App = () => {
  const [showDemo, setShowDemo] = useState(false);

  return (
    <Container>
      <Title />
      <Introduction />
      <Links />
      {showDemo && <Demo />}
      <button
        onClick={() => setShowDemo(!showDemo)}
        className="mx-auto mt-8 px-6 py-3 bg-green-600 hover:bg-green-700 text-white font-semibold rounded-lg shadow-lg transition-all duration-200 transform hover:scale-105 flex items-center"
      >
        <span className="mr-2">🚀</span>
        {showDemo ? "Hide Demo" : "Try It Now!"}
      </button>
      <Motivation />
      <Pitch />
      <HowToRust />
      <HowToWasm />
      <ProjectStatus />
      <Footer />
    </Container>
  );
};
