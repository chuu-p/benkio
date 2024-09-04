import {
  createRef,
  RefObject,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import Flashcard from "../components/Flashcard";
import TinderCard from "react-tinder-card";
import { Container } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { info } from "@tauri-apps/plugin-log";
import { unmountComponentAtNode } from "react-dom";

type Flashcard = {
  id: number;
  sideA: string;
  sideB: string;
};

async function getFlashcards(): Promise<Flashcard> {
  let res: string = await invoke("get_flashcards", { amount: 1 });
  info(`res ${res}`);
  let parsed: Flashcard[] = JSON.parse(res);
  return parsed[0];
}

function FlashcardScreen() {
  // TODO Implement this for one single flashcard for debugging
  const [flashcards, setFlashcards] = useState<Flashcard>();
  // const itemsRef: React.MutableRefObject<Map<number, any>> = useRef<
  //   Map<number, any>
  // >(new Map());

  const fcRef = useRef(null);

  function getMap(): Map<number, any> {
    if (!itemsRef.current) {
      // Initialize the Map on first usage.
      itemsRef.current = new Map<number, any>();
    }
    return itemsRef.current;
  }

  useEffect(() => {
    async function fetchFlashcards() {
      try {
        info("Fetching flashcards...");
        const cards = await getFlashcards();
        setFlashcards(cards);
      } catch (error) {
        console.error("Failed to fetch flashcards:", error);
      }
    }
    fetchFlashcards();
  }, []);

  const swipe = async (dir: "left" | "right") => {
    info("should swipe");
    info(`direction: ${dir}`);
    // await childRefs[flashcardRef.current].current.swipe(dir);
  };

  const handlePass = async (index: number) => {
    // setCurrentIndex((prevIndex) => (prevIndex + 1) % flashcards.length);
    // flashcardRef.current = index + 1;
    // let div = itemsRef.current.get(index);
    // let card = div?.getElementsByTagName("TinderCard")[0];
    // let card = itemsRef.current[index]?.querySelector("TinderCard");

    info(`index: ${index}`);
    info("Passed");
    fcRef.current.swipe("right");
    // await swipe("right");
  };

  const onCardLeftScreen = (myIdentifier) => {
    console.log(myIdentifier + " left the screen");
  };

  const handleFail = async (index: number) => {
    swipe("left");
    // setCurrentIndex((prevIndex) => (prevIndex + 1) % flashcards.length);
    // flashcardRef.current = index + 1;
    info("Failed");
    await swipe("left");
  };

  return (
    <>
      <Container maxWidth="md">
        <TinderCard
          swipeRequirementType="position"
          swipeThreshold={100}
          preventSwipe={["up", "down"]}
          onCardLeftScreen={() => onCardLeftScreen("fooBar")}
          key={flashcards?.id ?? 0}
          ref={fcRef}
        >
          <Flashcard
            sideA={flashcards?.sideA ?? "0"}
            sideB={flashcards?.sideB ?? "0"}
            onPass={async () => await handlePass(flashcards?.id ?? 0)}
            onFail={async () => await handleFail(flashcards?.id ?? 0)}
          />
        </TinderCard>

        {/* {flashcards.map((flashcard) => {
          return (
            <TinderCard
              swipeRequirementType="position"
              swipeThreshold={100}
              onSwipe={(direction: "right" | "left" | "up" | "down") => {
                onSwipe(direction, flashcard.id);
              }}
              preventSwipe={["up", "down"]}
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
                onPass={async () => await handlePass(flashcard.id)}
                onFail={async () => await handleFail(flashcard.id)}
              />
            </TinderCard>
          );
        })} */}
      </Container>
    </>
  );
}

export default FlashcardScreen;
