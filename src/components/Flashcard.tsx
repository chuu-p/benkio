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
  onBacksideShown: () => void;
}

function Flashcard({
  sideA,
  sideB,
  onPass,
  onFail,
  onBacksideShown,
}: FlashcardProps) {
  const [showAnswer, setShowAnswer] = useState(false);

  const handleShowAnswer = () => {
    setShowAnswer(true);
    onBacksideShown();
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
          {!showAnswer ? (
            <></>
          ) : (
            <>
              <div
                onClick={handleFail}
                style={{
                  position: "absolute",
                  top: 0,
                  left: 0,
                  height: "95%",
                  width: "50%",
                  background:
                    "linear-gradient(to right, rgba(255,0,0,0.1), rgba(0,0,0,0))",
                  cursor: "pointer",
                  zIndex: 1,
                  margin: "20px 20px auto",
                  padding: 2,
                  borderRadius: "5px",
                }}
              />
              <div
                onClick={handlePass}
                style={{
                  position: "absolute",
                  top: 0,
                  right: 0,
                  height: "95%",
                  width: "50%",
                  background:
                    "linear-gradient(to left, rgba(0,255,0,0.1), rgba(0,0,0,0))",
                  cursor: "pointer",
                  zIndex: 1,
                  margin: "20px 20px auto",
                  borderRadius: "5px",
                }}
              />
            </>
          )}
        </CardContent>
      </Card>
    </Container>
  );
}

export default Flashcard;
