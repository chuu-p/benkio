import { useState } from "react";
import {
  Card,
  CardContent,
  Typography,
  Button,
  Stack,
  Container,
  Divider,
} from "@mui/material";

interface FlashcardProps {
  sideA: string;
  sideB: string;
  onPass: () => void;
  onFail: () => void;
}

function Flashcard({ sideA, sideB, onPass, onFail }: FlashcardProps) {
  const [showAnswer, setShowAnswer] = useState(false);

  const handleShowAnswer = () => {
    setShowAnswer(true);
  };

  const handlePass = () => {
    setShowAnswer(false);
    onPass();
  };

  const handleFail = () => {
    setShowAnswer(false);
    onFail();
  };

  return (
    <Container sx={{ position: "absolute" }}>
      <Card
        sx={{
          maxWidth: "90vw",
          minHeight: "85vh",
          margin: "20px auto",
          padding: 2,
        }}
        onClick={handleShowAnswer}
      >
        <CardContent>
          <Typography variant="h5">{sideA}</Typography>
          {!showAnswer && (
            <>
              <Typography
                variant="body2"
                color="text.secondary"
                align="center"
                marginTop="50vw"
                className="pressable"
              >
                Tap to show answer
              </Typography>
            </>
          )}
          {showAnswer && (
            <>
              <Divider />
              <Typography
                variant="h6"
                color="text.secondary"
                sx={{ marginTop: 2 }}
              >
                {sideB}
              </Typography>
            </>
          )}
        </CardContent>
        <Stack
          direction="row"
          spacing={2}
          justifyContent="center"
          sx={{ padding: 2 }}
        >
          {!showAnswer ? (
            <></>
          ) : (
            <>
              <Button
                variant="contained"
                color="error"
                className="pressable"
                onClick={handleFail}
              >
                Fail
              </Button>
              <Button
                variant="contained"
                color="success"
                className="pressable"
                onClick={handlePass}
              >
                Pass
              </Button>
            </>
          )}
        </Stack>
      </Card>
    </Container>
  );
}

export default Flashcard;
