import { useEffect, useMemo, useState } from "react";
import "./App.css";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { ButtonAppBar } from "./components/AppBar";
import {
  Container,
  createTheme,
  CssBaseline,
  PaletteMode,
  ThemeProvider,
} from "@mui/material";
import { deepOrange } from "@mui/material/colors";
import HomeScreen from "./screens/HomeScreen";
import ErrorScreen from "./screens/ErrorScreen";
import KanaScreen from "./screens/KanaScreen";

const router = createBrowserRouter([
  // <Routes>
  //       {/* Dynamic route for levels */}
  //       <Route path="/level/:levelNumber" element={<Level />} />
  //       {/* Default route */}
  //       <Route path="/" element={<Level />} />
  //     </Routes>
  {
    path: "/",
    element: <HomeScreen />,
  },
  { path: "/level/:levelNumber", element: <KanaScreen /> },


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

  // https://stackoverflow.com/a/74637170/26371953
  useEffect(() => {
    document.body.style.overflow = "hidden";
    return () => {
      document.body.style.overflow = "scroll";
    };
  }, []);

  return (
    <>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <ButtonAppBar />
        <Container
          maxWidth="md"
          sx={{ "user-select": "none", cursor: "default" }}>
          <RouterProvider router={router} />
        </Container>
      </ThemeProvider>
    </>
  );
}

export default App;
