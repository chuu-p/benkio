import { Button, Container } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { info, warn } from "@tauri-apps/plugin-log";
import { useEffect, useState } from "react";
import { Link, Outlet } from "react-router-dom";
import { useParams, useNavigate } from 'react-router-dom';

export default function HomeScreen() {
    const navigate = useNavigate();
    const [level, setLevel] = useState<number>(-1);
    const startKanaChallenge = () => {
        navigate(`/level/${level}`);
    };

    useEffect(() => {
        // Function to fetch the initial level from the database
        const fetchInitialLevel = async () => {
            try {
                info("Fetching the initial level");
                const initialLevel: number = await invoke("get_level"); // Fetch the level from the database
                info(`setLevel: ${initialLevel}`);
                setLevel(initialLevel);
            } catch (error) {
                warn(`Failed to fetch the initial level: ${error}`);
            }
        };

        fetchInitialLevel();
    }, []);

    return (
        <>
            <h1>Current Level: {level}</h1>
            <button onClick={startKanaChallenge}><h1>Start!</h1></button>
        </>
    );
}

