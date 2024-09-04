import { useState } from 'react';
import { Card, CardContent, Typography, Button, Stack, Container } from '@mui/material';

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
        // FIXME position absolute is not working
        <Card
            sx={{ maxWidth: "90vw", minHeight: "90vh", margin: '20px auto', padding: 2 }}
            onClick={handleShowAnswer}
        >
            <Container sx={{ position: "absolute"}}>
            <CardContent>
                <Typography variant="h5">
                    {sideA}
                </Typography>
                {showAnswer && (
                    <Typography variant="h6" color="text.secondary" sx={{ marginTop: 2 }}>
                        {sideB}
                    </Typography>
                )}
            </CardContent>
            <Stack direction="row" spacing={2} justifyContent="center" sx={{ padding: 2 }}>
                {!showAnswer ? (
                    <></>
                ) : (
                    <>
                        <Button variant="contained" color="error" onClick={handleFail}>
                            Fail
                        </Button>
                        <Button variant="contained" color="success" onClick={handlePass}>
                            Pass
                        </Button>
                    </>
                )}
            </Stack>
            </Container>
        </Card>
    );
}

export default Flashcard;
