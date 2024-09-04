import { Container } from "@mui/material";
import { useRouteError } from "react-router-dom";

export default function Error() {
    const error: any = useRouteError();
    console.error(error);

    return (
        <Container
            maxWidth="md"
            sx={{ "user-select": "none", cursor: "default" }}
        >
            <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh' }}>
                <div id="error-page">
                    <h1>Oops!</h1>
                    <p>Sorry, an unexpected error has occurred.</p>
                    <p>
                        <i>{error.statusText || error.message}</i>
                    </p>
                </div>
            </div>
        </Container>
    );
}
