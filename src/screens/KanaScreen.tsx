import { Button, Paper, Table, TableCell, TableContainer, TableRow } from '@mui/material';
import { useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';

function KanaLevel1() {
    const [lessonShown, setLessionShown] = useState<Boolean>(true);

    const kanaTable = [
        ["あ", "a"],
        ["い", "i"],
        ["う", "u"],
        ["え", "e"],
        ["お", "o"],
    ];

    return (
        <>
            {lessonShown ?
                <div>
                    <h1>Level 1: LESSON</h1>
                    {/* Display あ-a い-i う-u え-e お-o in a table */}
                    <TableContainer component={Paper}>
                        <Table>
                            {
                                kanaTable.map((row, index) => (
                                    <TableRow key={index}>
                                        <TableCell align="center"><h2>{row[0]}</h2></TableCell>
                                        <TableCell align="center"><h2>{row[1]}</h2></TableCell>
                                    </TableRow>
                                ))
                            }
                        </Table>
                    </TableContainer>
                    <br />
                    <Button variant='contained' color='success' onClick={() => setLessionShown(false)}>Got it!</Button>
                </div>
                :
                <div>
                    <h1>Level 1: CHALLENGE</h1>
                    <Button variant='contained' color='info' onClick={() => { setLessionShown(true) }}>Show Lesson</Button>
                </div>
            }
        </>
    );
}



export default function KanaScreen() {
    // Get the current level from the route parameter
    const { levelNumber } = useParams<{ levelNumber: string }>();
    const navigate = useNavigate();
    const [trainingStarted, setTrainingStarted] = useState<Boolean>(false);


    // Parse the level number and ensure it's a number
    const currentLevel = parseInt(levelNumber || '1', 10);

    // Handler to navigate to the next level
    const goToNextLevel = () => {
        const nextLevel = currentLevel + 1;
        navigate(`/level/${nextLevel}`);
    };

    return (
        <>
            {

                (currentLevel == 1) ?
                    <KanaLevel1 />
                    :
                    <>
                        <h1>Kana Level {currentLevel}</h1>
                        <button onClick={goToNextLevel}>Go to Level {currentLevel + 1}</button>
                    </>

            }
        </>
    );
};
