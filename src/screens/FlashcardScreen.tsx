import { useEffect, useRef, useState } from "react";
import { Container } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { debug, info } from "@tauri-apps/plugin-log";
import type { Flashcard } from "../models/flashcard.d.ts";
import FlashcardComponent from "../components/FlashcardComponent.tsx";


async function getFlashcards(): Promise<Flashcard[]> {
  let response: string = await invoke("get_flashcards_for_today");
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
    let response: string = await invoke("update_flashcard", { id: flashcardId, passed: true });
    info(response);
    // invoke("get_flashcards_for_today", { id: flashcardId, passed: true })
    //   .then((message: string) => info(message))
    //   .catch((error) => error(error));
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
        <div>0 new | 1 false | 60 due</div>
        {flashcards.map((flashcard) => {
          return (
            <FlashcardComponent
              sideA={flashcard.content_front}
              sideB={flashcard.content_back}
              onPass={async () => await handlePress(flashcard.id, "pass")}
              onFail={async () => await handlePress(flashcard.id, "fail")}
              onBacksideShown={() => handleBacksideShown(flashcard.id)}
            />
          );
        })}
      </Container>
    </>
  );
}

export default FlashcardScreen;
