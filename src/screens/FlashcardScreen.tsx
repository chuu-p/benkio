import { useEffect, useState } from "react";
import Flashcard from "../components/Flashcard";
import TinderCard from "react-tinder-card";
import { Container } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";


type Flashcard = {
    id: number;
    sideA: string;
    sideB: string;
};

async function getFlashcards(): Promise<Flashcard[]> {
    let res: string = await invoke("get_flashcards", { amount: 10 });
    console.log("res", res);
    let parsed: Flashcard[] = JSON.parse(res);
    return parsed;
}

function FlashcardScreen() {
    const [flashcards, setFlashcards] = useState<Flashcard[]>([]);

    useEffect(() => {
        async function fetchFlashcards() {
            try {
                console.log("Fetching flashcards...");
                const cards = await getFlashcards();
                setFlashcards(cards);
            } catch (error) {
                console.error("Failed to fetch flashcards:", error);
            }
        }
        fetchFlashcards();
    }, []);

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
                        swipeRequirementType="position"
                        swipeThreshold={100}
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