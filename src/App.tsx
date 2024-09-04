import { useMemo, useState } from "react";
import "./App.css";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import FlashcardScreen from "./screens/FlashcardScreen";
import { ButtonAppBar } from "./components/AppBar";
import {
  createTheme,
  CssBaseline,
  PaletteMode,
  ThemeProvider,
} from "@mui/material";
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
  const [mode, _setMode] = useState<PaletteMode>("dark");
  const theme = useMemo(() => createTheme(getDesignTokens(mode)), [mode]);

  return (
    <>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <ButtonAppBar />
        <RouterProvider router={router} />
      </ThemeProvider>
    </>
  );
}

export default App;
