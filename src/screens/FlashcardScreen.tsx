import { useState } from "react";
import Flashcard from "../components/Flashcard";

const flashcards = [
    { sideA: "Hello", sideB: "Hola" },
    { sideA: "Goodbye", sideB: "Adiós" },
    { sideA: "Thank you", sideB: "Gracias" },
    // Add more flashcards as needed
];

function FlashcardScreen() {
    const [currentIndex, setCurrentIndex] = useState(0);

    const handlePass = () => {
        setCurrentIndex((prevIndex) => (prevIndex + 1) % flashcards.length);
    };

    const handleFail = () => {
        setCurrentIndex((prevIndex) => (prevIndex + 1) % flashcards.length);
    };
    return (
        <>
            <Flashcard
                sideA={flashcards[currentIndex].sideA}
                sideB={flashcards[currentIndex].sideB}
                onPass={handlePass}
                onFail={handleFail}
            />
        </>
    );
}

export default FlashcardScreen;