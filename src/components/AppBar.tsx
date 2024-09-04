import { Typography } from "@mui/material";
import PersonIcon from "@mui/icons-material/Person";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import IconButton from "@mui/material/IconButton";
import SyncIcon from "@mui/icons-material/Sync";
import Box from '@mui/material/Box';

export function ButtonAppBar() {
  return (
    <Box sx={{ flexGrow: 1 }}>
      <AppBar position="static">
        <Toolbar>
          <IconButton
            size="large"
            edge="start"
          >
            <SyncIcon />
          </IconButton>

          <Typography variant="h6" component="div" fontFamily={"monospace"} sx={{ flexGrow: 1 }} textAlign="center">
            voxab
          </Typography>

          <IconButton
            size="large"
            edge="end"
          >
            <PersonIcon />
          </IconButton>
        </Toolbar>
      </AppBar>
    </Box>
  );
}
