import React, { useState } from 'react';
import { Card, CardContent, Typography, Button, Stack } from '@mui/material';

interface FlashcardProps {
    sideA: string;
    sideB: string;
}

function Flashcard({ sideA, sideB }: FlashcardProps) {
    const [showAnswer, setShowAnswer] = useState(false);

    const handleShowAnswer = () => {
        setShowAnswer(true);
    };

    const handlePass = () => {
        alert('Pass');
        setShowAnswer(false);
    };

    const handleFail = () => {
        alert('Fail');
        setShowAnswer(false);
    };

    return (
        <Card sx={{ maxWidth: 345, margin: '20px auto', padding: 2 }}>
            <CardContent>
                <Typography variant="h5" component="div">
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
                    <Button variant="contained" onClick={handleShowAnswer}>
                        Show Answer
                    </Button>
                ) : (
                    <>
                        <Button variant="contained" color="success" onClick={handlePass}>
                            Pass
                        </Button>
                        <Button variant="contained" color="error" onClick={handleFail}>
                            Fail
                        </Button>
                    </>
                )}
            </Stack>
        </Card>
    );
}

export default Flashcard;
