import { useEffect, useRef, useState } from "react";
import Flashcard from "../components/Flashcard";
import TinderCard from "react-tinder-card";
import { Container } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { debug, info } from "@tauri-apps/plugin-log";

type Flashcard = {
  id: number;
  sideA: string;
  sideB: string;
};

async function getFlashcards(): Promise<Flashcard[]> {
  let response: string = await invoke("get_flashcards", { amount: 10 });
  info(`response ${response}`);
  let parsed: Flashcard[] = JSON.parse(response);
  return parsed;
}

function FlashcardScreen() {
  const [flashcards, setFlashcards] = useState<Flashcard[]>([]);
  const [backsideShown, setBacksideShown] = useState<{
    [key: number]: boolean;
  }>({});
  const itemsRef = useRef<Map<number, any> | null>(null);

  // Init Cards
  useEffect(() => {
    async function fetchFlashcards() {
      try {
        info("Fetching flashcards...");
        const cards = await getFlashcards();
        setFlashcards(cards);
        setBacksideShown(
          cards.reduce(
            (acc, card) => {
              acc[card.id] = false;
              return acc;
            },
            {} as { [key: number]: boolean },
          ),
        );
      } catch (error) {
        console.error("Failed to fetch flashcards:", error);
      }
    }
    fetchFlashcards();
  }, []);

  // https://react.dev/learn/manipulating-the-dom-with-refs
  function getMap(): Map<number, any> {
    if (!itemsRef.current) {
      // Initialize the Map on first usage.
      itemsRef.current = new Map<number, any>();
    }
    return itemsRef.current;
  }

  const removeFlashcardById = (id: number) => {
    setFlashcards((prevFlashcards) =>
      prevFlashcards.filter((flashcard) => flashcard.id !== id),
    );
  };

  const handlePress = async (
    flashcardId: number,
    operation: "pass" | "fail",
  ) => {
    debug(`flashcardId: ${flashcardId}, operation: ${operation}`);
    const map = getMap();
    const node = map.get(flashcardId);
    const direction = operation == "pass" ? "right" : "left";
    await node?.swipe(direction);
    map.delete(flashcardId);
    removeFlashcardById(flashcardId);
  };

  const handleBacksideShown = (flashcardId: number) => {
    setBacksideShown((prevState) => ({
      ...prevState,
      [flashcardId]: true,
    }));
  };

  return (
    <>
      <Container
        maxWidth="md"
        sx={{ "user-select": "none", cursor: "default" }}
      >
        {flashcards.map((flashcard) => {
          return (
            <TinderCard
              swipeRequirementType="position"
              swipeThreshold={100}
              preventSwipe={[
                "up",
                "down",
                ...(!backsideShown[flashcard.id] ? ["left", "right"] : []),
              ]} // Prevent swipe if card sideB has not been shown
              key={flashcard.id}
              ref={(node) => {
                const map = getMap();
                if (node) {
                  map.set(flashcard.id, node);
                } else {
                  map.delete(flashcard.id);
                }
              }}
            >
              <Flashcard
                sideA={flashcard.sideA}
                sideB={flashcard.sideB}
                onPass={async () => await handlePress(flashcard.id, "pass")}
                onFail={async () => await handlePress(flashcard.id, "fail")}
                onBacksideShown={() => handleBacksideShown(flashcard.id)}
              />
            </TinderCard>
          );
        })}
      </Container>
    </>
  );
}

export default FlashcardScreen;
