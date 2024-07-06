import React, {useState} from 'react';
import {invoke} from '@tauri-apps/api/tauri';
import {
    AppBar,
    Avatar,
    Box,
    Button,
    CircularProgress,
    Container,
    List,
    ListItem,
    ListItemAvatar,
    ListItemText,
    Paper,
    TextField,
    Toolbar,
    Typography
} from '@mui/material';

interface FingerPrintMeta {
    name: string;
    version: string;
    confidence: number;
    icon: string;
}

interface FingerPrint {
    data: FingerPrintMeta[];
}

const App: React.FC = () => {
    const [url, setUrl] = useState<string>('https://example.com');
    const [result, setResult] = useState<FingerPrint | null>(null);
    const [loading, setLoading] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);

    const analyzeWebsite = async () => {
        setError(null);
        setLoading(true);
        try {
            const response = await invoke<FingerPrint>('web_analyze', {url});
            setResult(response);
        } catch (e) {
            setError('Failed to analyze the website.');
            console.error(e);
        } finally {
            setLoading(false);
        }
    };


    return (
        <Container maxWidth="sm">
            <Box my={4}>
                <AppBar position="static">
                    <Toolbar>
                        <Typography variant="h6" component="div" sx={{flexGrow: 1}}>
                            Web Analyzer
                        </Typography>
                    </Toolbar>
                </AppBar>
                <Box mt={2}>
                    <TextField
                        fullWidth
                        label="Enter URL"
                        variant="outlined"
                        value={url}
                        onChange={(e) => setUrl(e.target.value)}
                        margin="normal"
                    />
                    <Box my={2}>
                        <Button variant="contained" color="primary" onClick={analyzeWebsite} disabled={loading}>
                            Analyze
                        </Button>
                    </Box>
                    {loading && <CircularProgress/>}
                    {error && <Typography color="error">{error}</Typography>}
                    {result && (
                        <Paper elevation={3}>
                            <Box p={2}>
                                <Typography variant="h6">Analysis Result</Typography>
                                <List>
                                    {result.data.map((tech) => (
                                        <ListItem key={tech.name}>
                                            <ListItemAvatar>
                                                <Avatar src={`/icons/${tech.icon}`} alt={tech.name}>
                                                    {tech.name[0].toUpperCase()}
                                                </Avatar>
                                            </ListItemAvatar>
                                            <ListItemText primary={tech.name}/>
                                        </ListItem>
                                    ))}
                                </List>
                            </Box>
                        </Paper>
                    )}
                </Box>
            </Box>
        </Container>
    );
};

export default App;
