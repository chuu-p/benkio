import { useState } from "react";
import Flashcard from "../components/Flashcard";
import TinderCard from "react-tinder-card";
import { Container } from "@mui/material";

const flashcards = [
    { sideA: "Hello", sideB: "Hola" },
    { sideA: "Goodbye", sideB: "Adiós" },
    { sideA: "Thank you", sideB: "Gracias" },
    // Add more flashcards as needed
];

function FlashcardScreen() {
    const [currentIndex, setCurrentIndex] = useState(0);

    const handlePass = () => {
        // setCurrentIndex((prevIndex) => (prevIndex + 1) % flashcards.length);
        console.log("Passed");
    };

    const handleFail = () => {
        // setCurrentIndex((prevIndex) => (prevIndex + 1) % flashcards.length);
        console.log("Failed");
    };

    const onSwipe = (direction: "right" | "left" | "up" | "down") => {
        console.log('You swiped: ' + direction)
        if (direction === "right") {
            handlePass();
        } else if (direction === "left") {
            handleFail();
        }
    }

    return (
        <>
            <Container maxWidth="md">
                {flashcards.map((flashcard, index) => {
                    return <TinderCard
                        key={index}
                        onSwipe={onSwipe}
                        preventSwipe={['up', 'down']}>
                        <Flashcard
                            sideA={flashcard.sideA}
                            sideB={flashcard.sideB}
                            onPass={handlePass}
                            onFail={handleFail}
                        />
                    </TinderCard>
                })}
            </Container>
        </>
    );
}

export default FlashcardScreen;