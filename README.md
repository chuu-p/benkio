# voxab

Mobile app for language learning.

## Sequence

### 1. Kana: Mini-SRS
- plus one per day do all due cards in main kana deck
- Level System
   - Level 1: 
      - show a i u e o hiragana (display special cases like hi-chi here)
      - check a i u e o (play sound after successful check)
      - if all a i u e o are passed in a row 
      - add a e u i o to main kana deck
      - next level
   - Level 2: show ka ki ku ke ko hiragana
      - show ka ki ku ke ko hiragana
      - check ka ki ku ke ko
      - if all ka ki ku ke ko are passed in a row 
      - add a e u i o to main kana deck
      - next level
   - ...
   - Level 4: ...

#### timing

~2 Weeks (~4 Level per day)

9 Level Hiragana
1 Bonus Level (Katakana Diacritics) ka -> ga, etc.
1 Bonus Level (nya, sha, etc.)
9 Level Katakana
1 Bonus Level (Katakana Diacritics)
1 Bonus Level (Extended Katakana)
1 Bonus Level (nya, sha, etc.)

#### checking

instead of pass/fail give the user either a keyboard or multiple choice buttons to enter
-> this way the user cannot cheat
kana keyboard (thanks @ben-kerman)
```
 [k ] [g ] |
 [s ] [z ] | [ a ]
 [t ] [d ] | [ i ]
   [ n ]   | [ u ]
 [h][b][p] | [ e ]
 [   y   ] | [ o ]
 [r ] [w ] |
```

### 2. Learn first 1000 words (kanji)

- 1000 words learning through flashcards (like anki)
- basic grammar learing through single articles (not flashcards) https://guidetojapanese.org/learn/category/grammar-guide/basic-grammar/

only do 20 new cards per day -> extra counter per day

### 3. Creating own flashcards

- have a few standard flashcard templates

#### sentence mining with netflix/crunchyroll anime series

This can work on mobile with [asbplayer](https://chromewebstore.google.com/detail/asbplayer-language-learni/hkledmpjpaehamkiehglnbelcpdflcab) [killergerbah/asbplayer](https://github.com/killergerbah/asbplayer)
And [kiwibrowser](https://play.google.com/store/apps/details?id=com.kiwibrowser.browser&hl=en)
-> this works only on android

firefox mobile+asbplayer  should be the best option for ios

- mobile browser firefox with voxab addon -> login to netflix or crunchyroll
- when floating button is pressed -> screenshot -> extract subtitles from page source 
  - create card from subtitles -> dictionary entry -> etc.
- ~maybe fork mobile firefox as module for this app~

save media files to media folder
-> save references to media files in sqlite db
images: <img src="file.jpg">
sounds: [sound:file.mp3] # unsure if this is possible, if you can extract this from netflix or crunchyroll

#### sentence mining with light novels / browser text

- someday...
- syosetu
- highlight sentence -> choose word for definition -> create card
- should work for wikipedia and other normal websites

#### sentence mining someday

- epubs, pdfs
- mangas

## Features

- [x] flashcard view
  - [x] Display Side A, "Show Answer" button, Show answer everywhere on screen
  - [x] "Show Answer" -> Display Side A, Side B, Pass/Fail buttons -> zap OR swipe left/right like tinder to pass/fail
  - [x] only be able to swipe left/right when answer is shown!
- [ ] flashcard backend
  - [ ] get due cards for today

1. Kana lernen ->
   - ~one week of doing only kana-cards
   - Suspend cards, that are passed very quickly multiple times (just kana)
   - then they leave the deck
2. Learn first 1000 words (kanji) +
   - 1000 words learning through flashcards
   - basic grammar learing through single articles (not flashcards) https://guidetojapanese.org/learn/category/grammar-guide/basic-grammar/
3. creating own flashcards
   - screenshot -> ocr -> text -> flashcard

## Technical Backlog

- [ ] Add [JSON Schema](https://json-schema.org/) for Message passing between front- and backend

## References

Kana are taken from https://github.com/shlchoi/kana

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
