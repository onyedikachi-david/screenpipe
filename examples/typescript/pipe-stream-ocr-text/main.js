const INTERVAL = 1 * 60 * 1000; // 1 minute in milliseconds

async function queryScreenpipe() {
    try {
        const now = new Date();
        const oneMinuteAgo = new Date(now.getTime() - INTERVAL);

        const queryParams = `start_time=${oneMinuteAgo.toISOString()}&end_time=${now.toISOString()}&limit=50&content_type=ocr`;

        const result = await pipe.get(`http://localhost:3030/search?${queryParams}`);
        console.log("Retrieved", result.data.length, "items from screenpipe");
        return result.data;
    } catch (error) {
        console.error("Error querying screenpipe:", error);
        return [];
    }
}

async function writeToMarkdown(data) {
    console.log("Writing to markdown", JSON.stringify(data));
    const fileName = `screen-ocr-${new Date().toISOString().replace(/[:.]/g, '-')}.md`;
    const content = data.map(item => `## ${item.content.timestamp}\n\n${item.content.text}\n\n---\n`).join('\n');

    await pipe.writeFile(fileName, content);
    console.log(`Written OCR data to ${fileName}`);
}

async function runOCRTracker() {
    console.log("Starting OCR Tracker");

    while (true) {
        try {
            const screenData = await queryScreenpipe();
            await writeToMarkdown(screenData);
        } catch (error) {
            console.error("Error in OCR tracking:", error);
        }
        await new Promise(resolve => setTimeout(resolve, INTERVAL));
    }
}

// Self-invoking async function to run the OCR tracker
(async () => {
    try {
        await runOCRTracker();
    } catch (error) {
        console.error("Fatal error in OCR Tracker:", error);
    }
})();