import { useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { createBrowserRouter, Route, Router, RouterProvider, Routes } from "react-router-dom";
import FlashcardScreen from "./screens/FlashcardScreen";
import { ButtonAppBar } from "./components/AppBar";
import { createTheme, CssBaseline, PaletteMode, ThemeProvider } from "@mui/material";
import { deepOrange } from "@mui/material/colors";

const router = createBrowserRouter([
  {
    path: "/",
    element: <FlashcardScreen />,
  },
]);

export const getDesignTokens = (mode: PaletteMode) => ({
  palette: {
    mode,
    ...(mode === "light"
      ? {
        primary: deepOrange,
      }
      : {
        primary: deepOrange,
      }),
  },
});

function App() {
  // const [greetMsg, setGreetMsg] = useState("");
  // const [name, setName] = useState("");

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }
  const [mode, setMode] = useState<PaletteMode>("dark");
  const theme = useMemo(() => createTheme(getDesignTokens(mode)), [mode]);

  return (
    <>
      <ThemeProvider theme={theme}>
        <CssBaseline />

        <ButtonAppBar />
        <RouterProvider router={router} />
      </ThemeProvider>
    </>

    // <Router location={""} navigator={undefined}>
    //   
    //   <Routes>
    //     <Route path="/" element={} />
    //   </Routes>
    //   {/* <LabelBottomNavigation /> */}
    // </Router>


  );
}

export default App;
