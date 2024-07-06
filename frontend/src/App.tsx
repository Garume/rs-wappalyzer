import React, {useState} from 'react';
import {
    AppBar,
    Avatar,
    Box,
    Button,
    CircularProgress,
    Container,
    Input,
    List,
    ListItem,
    ListItemAvatar,
    ListItemText,
    Paper,
    TextField,
    Toolbar,
    Typography
} from '@mui/material';

// 環境変数に基づいて条件分岐
const isTauri = import.meta.env.VITE_BUILD_TARGET === 'tauri';

let analyzeWebsite: (url: string, jsonData: string[]) => Promise<any>;

if (isTauri) {
    // Tauri用のコード
    import('@tauri-apps/api/tauri').then(({invoke}) => {
        analyzeWebsite = async (url: string, jsons: string[]) => {
            return await invoke('web_analyze', {url, jsons});
        };
    });
} else {
    // WASM用のコード
    import('./wasm/src_wasm').then((module) => {
        module.default().then(() => console.log("WASM initialized"));
        analyzeWebsite = async (url: string, jsons: string[]) => {
            return await module.web_analyze(url, jsons);
        };
    });
}

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
    const [jsonData, setJsonData] = useState<string[]>([]);

    const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const files = event.target.files;
        if (files) {
            const filePromises = Array.from(files).map((file) => {
                return new Promise<string>((resolve, reject) => {
                    const reader = new FileReader();
                    reader.onload = (e) => {
                        resolve(e.target?.result as string);
                    };
                    reader.onerror = reject;
                    reader.readAsText(file);
                });
            });

            Promise.all(filePromises)
                .then((data) => setJsonData(data))
                .catch((error) => console.error("Failed to read files:", error));
        }
    };

    const handleAnalyze = async () => {
        setError(null);
        setLoading(true);
        try {
            const response = await analyzeWebsite(url, jsonData);
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
                        <Input type="file" inputProps={{multiple: true}} onChange={handleFileChange}/>
                    </Box>
                    <Box my={2}>
                        <Button variant="contained" color="primary" onClick={handleAnalyze} disabled={loading}>
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
